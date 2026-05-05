"""AI/ai/hive_telemetry.py — HV1 worker side (2026-05-04).

Each AIM worker periodically packages anonymized signals about its
operation and POSTs them to the queen for aggregation. Critical: all
PII must be stripped before send. The L_PRIVACY contract is the spine.

What we send (allowed):
  - aggregate counters (ledger row count, retry rate, score)
  - hashed prompt fingerprint (sha256 of SELF_DIAGNOSTIC_PROMPT.md)
  - reflexion theme labels (anonymized — only category words)
  - skill usage frequencies (skill_id + count, no body)
  - eval pass/fail counts per case_id
  - compliance metric (numeric)

What we NEVER send (blocked):
  - phone numbers, emails, names
  - file paths, project names
  - patient data
  - prompt/response content
  - actual finding text (only counts)

Public API:
    contribution() -> dict           — build the payload (read-only)
    preview() -> str                 — human-friendly view of what would send
    contribute(*, dry_run=False) -> Result
"""
from __future__ import annotations

import contextlib
import dataclasses
import datetime as dt
import json
import logging
import os
import re
import sqlite3
from pathlib import Path
from typing import Any, Optional

log = logging.getLogger("ai.hive_telemetry")


@dataclasses.dataclass
class ContributionResult:
    sent: bool
    payload: dict
    queen_response: Optional[dict]
    notes: list[str]


# Patterns that, if found in any string about to be sent, cause REJECT.
_PII_PATTERNS = [
    re.compile(r"\b[\w._%+-]+@[\w.-]+\.[A-Za-z]{2,}\b"),    # email
    re.compile(r"\+\d{6,}"),                                  # phone
    re.compile(r"/home/\w+|/Users/\w+|C:\\Users\\\w+"),        # user path
    re.compile(r"\b[A-Z][a-z]+ [A-Z][a-z]+\b"),                # first+last name
    re.compile(r"\bPMID[: ]?\d+|10\.\d{4,}/\S+"),              # publication ids
]


def _scrub(value: Any) -> Any:
    """Recursively check value for PII; raise if found."""
    if isinstance(value, str):
        for p in _PII_PATTERNS:
            if p.search(value):
                raise ValueError(
                    f"L_PRIVACY blocked: PII pattern matched in {value[:60]!r}"
                )
        return value
    if isinstance(value, (list, tuple)):
        return type(value)(_scrub(v) for v in value)
    if isinstance(value, dict):
        return {k: _scrub(v) for k, v in value.items()}
    return value


# ── signal builders ─────────────────────────────────────────────


def _ledger_signal() -> dict:
    """Aggregate counts from DG1 ledger. No raw rows leave."""
    try:
        from AI.ai.diagnostic_ledger import all_rows, trend
    except Exception:
        return {}
    rows = all_rows()
    if not rows:
        return {"n_runs": 0}
    t = trend()
    grade_dist = t.get("grade_dist", {})
    return {
        "n_runs": t["n_runs"],
        "avg_compliance": round(t.get("avg_compliance", 0.0), 3),
        "avg_crit": round(t.get("avg_crit", 0.0), 2),
        "retry_share": round(t.get("retry_share", 0.0), 3),
        "grade_dist": grade_dist,
        # DELIBERATELY no ts ranges, no model strings, no report paths
    }


def _prompt_signal() -> dict:
    """sha256 of current prompt — fingerprint only, never body."""
    try:
        from AI.ai.prompt_versions import fingerprint
        fp = fingerprint()
        return {
            "sha256": fp.sha256,
            "byte_count": fp.byte_count,
            "line_count": fp.line_count,
        }
    except Exception:
        return {}


def _skill_signal() -> dict:
    """Skill usage frequencies — id + count only, no body."""
    # Stub: this would walk session JSONL logs and count skill invocations.
    # For now we return empty pending real session-log integration.
    return {"skill_invocations": {}}


def _reflexion_signal() -> dict:
    """Anonymized reflexion theme labels."""
    try:
        from AI.ai.reflexion_cluster import clusters_from_memory
        clusters = clusters_from_memory()
    except Exception:
        return {"clusters": []}
    out = []
    for c in clusters[:20]:
        # Only theme labels (already anonymized topic words). NOT bodies.
        if hasattr(c, "theme") and c.theme:
            out.append({"theme": list(c.theme)[:5],
                          "n": int(getattr(c, "n", 0))})
    return {"clusters": out}


def _suppression_signal() -> dict:
    """Count of active suppressions — no refs leak (file:line could leak
    project structure). Just total."""
    try:
        from AI.ai.finding_suppressions import active
        return {"n_active_suppressions": len(active())}
    except Exception:
        return {}


def _system_signal() -> dict:
    """Identifies AIM version + basic setup metadata. No host info."""
    return {
        "aim_version": "AI-hive-1",
        "python_major_minor": ".".join(__import__("sys").version.split()[0]
                                          .split(".")[:2]),
    }


# ── public API ──────────────────────────────────────────────────


def contribution() -> dict:
    """Build the payload. Apply scrub. Returns dict ready for upload."""
    payload = {
        "v": 1,
        "ts": dt.datetime.now().isoformat(timespec="seconds"),
        "worker_id": _worker_id(),
        "ledger": _ledger_signal(),
        "prompt": _prompt_signal(),
        "skills": _skill_signal(),
        "reflexion": _reflexion_signal(),
        "suppressions": _suppression_signal(),
        "system": _system_signal(),
    }
    # Hard L_PRIVACY enforcement before return.
    return _scrub(payload)


def _worker_id() -> str:
    """Stable but anonymous ID per machine. Hash of hostname salted with
    a per-install random — avoids cross-machine correlation by queen."""
    import hashlib
    salt_path = Path.home() / ".cache" / "aim" / "hive_salt"
    if salt_path.exists():
        salt = salt_path.read_text().strip()
    else:
        salt = os.urandom(16).hex()
        salt_path.parent.mkdir(parents=True, exist_ok=True)
        salt_path.write_text(salt, encoding="utf-8")
    base = (os.uname().nodename + salt).encode()
    return hashlib.sha256(base).hexdigest()[:16]


def preview() -> str:
    """Human-friendly view of what we would send. Nothing transmitted."""
    p = contribution()
    return json.dumps(p, indent=2, ensure_ascii=False)


def contribute(*, dry_run: bool = False,
                queen_url: Optional[str] = None) -> ContributionResult:
    """Build payload and POST to queen. Returns ContributionResult."""
    notes: list[str] = []
    try:
        payload = contribution()
    except ValueError as e:
        return ContributionResult(
            sent=False, payload={}, queen_response=None,
            notes=[f"L_PRIVACY blocked: {e}"],
        )

    if dry_run:
        notes.append("dry_run — not transmitted")
        return ContributionResult(sent=False, payload=payload,
                                    queen_response=None, notes=notes)

    url = queen_url or os.environ.get("AIM_HIVE_QUEEN_URL")
    if not url:
        notes.append("no queen URL configured (AIM_HIVE_QUEEN_URL)")
        return ContributionResult(sent=False, payload=payload,
                                    queen_response=None, notes=notes)

    try:
        import httpx
        r = httpx.post(f"{url.rstrip('/')}/v1/hive/contribute",
                        json=payload, timeout=30)
        r.raise_for_status()
        return ContributionResult(sent=True, payload=payload,
                                    queen_response=r.json(),
                                    notes=notes)
    except Exception as e:
        notes.append(f"upload failed: {type(e).__name__}: {e}")
        return ContributionResult(sent=False, payload=payload,
                                    queen_response=None, notes=notes)


def summary(*, dry_run: bool = True) -> str:
    res = contribute(dry_run=dry_run)
    parts = [f"🐝 Hive telemetry — {'dry-run' if dry_run else 'live'}"]
    if res.payload.get("ledger", {}).get("n_runs", 0) == 0:
        parts.append("  (no diagnostic activity yet — payload mostly empty)")
    else:
        led = res.payload["ledger"]
        parts.append(f"  ledger: {led['n_runs']} runs, "
                      f"avg compliance {led['avg_compliance']:.0%}")
    parts.append(f"  worker_id: {res.payload.get('worker_id', '?')}")
    if res.notes:
        for n in res.notes:
            parts.append(f"  - {n}")
    return "\n".join(parts)
