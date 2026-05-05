"""AI/ai/doctor.py — DR2 (2026-05-04).

Smoke-test every AI/ai/* module + verify the wiring assumptions a
fresh checkout depends on. Runs in O(seconds), no network, no model
calls — pure local introspection.

Returns a list of Probe results so callers (CLI, CI, dashboard) can
decide what to do.

Public API:
    diagnose() -> list[Probe]
    summary() -> str
"""
from __future__ import annotations

import dataclasses
import importlib
import logging
import os
from pathlib import Path
from typing import Callable, Optional

log = logging.getLogger("ai.doctor")


@dataclasses.dataclass
class Probe:
    name: str
    ok: bool
    detail: str
    severity: str = "info"   # info | warn | crit


def _project_root() -> Path:
    return Path(__file__).resolve().parent.parent.parent


# ── individual probes ───────────────────────────────────────────


def _probe_modules() -> Probe:
    """Every AI/ai/*.py must import without error."""
    failed: list[str] = []
    ai_dir = _project_root() / "AI" / "ai"
    for p in sorted(ai_dir.glob("*.py")):
        if p.name.startswith("_") or p.name == "__init__.py":
            continue
        modname = f"AI.ai.{p.stem}"
        try:
            importlib.import_module(modname)
        except Exception as e:
            failed.append(f"{modname}: {type(e).__name__}: {e}")
    if failed:
        return Probe(name="modules", ok=False, severity="crit",
                     detail=f"{len(failed)} import failures:\n  "
                             + "\n  ".join(failed))
    n = sum(1 for p in ai_dir.glob("*.py")
             if not p.name.startswith("_") and p.name != "__init__.py")
    return Probe(name="modules", ok=True,
                 detail=f"{n} AI/ai modules import cleanly")


def _probe_direction_rule() -> Probe:
    """`agents/` must not import from AI/ — fundamental contract."""
    from AI.ai.self_diagnostic import _direction_rule_status
    out = _direction_rule_status()
    if out["clean"]:
        return Probe(name="direction_rule", ok=True,
                     detail="agents/ → AI/ imports: 0 (clean)")
    return Probe(name="direction_rule", ok=False, severity="crit",
                 detail="agents/ imports AI/ — direction rule violated:\n  "
                         + "\n  ".join(out["violations"][:5]))


def _probe_db_writable() -> Probe:
    """Diagnostic ledger DB path must be writable."""
    from AI.ai.diagnostic_ledger import db_path
    p = db_path()
    try:
        p.parent.mkdir(parents=True, exist_ok=True)
        marker = p.parent / ".doctor_probe"
        marker.write_text("ok", encoding="utf-8")
        marker.unlink()
    except OSError as e:
        return Probe(name="db_writable", ok=False, severity="crit",
                     detail=f"{p} — {e}")
    return Probe(name="db_writable", ok=True,
                 detail=f"{p.parent} is writable")


def _probe_artifacts_dir() -> Probe:
    """AI/artifacts/ must exist or be creatable."""
    p = _project_root() / "AI" / "artifacts"
    try:
        p.mkdir(parents=True, exist_ok=True)
    except OSError as e:
        return Probe(name="artifacts_dir", ok=False, severity="warn",
                     detail=f"{p} — {e}")
    n = sum(1 for _ in p.glob("self_diag_*.md")
             if "_request_" not in _.name)
    return Probe(name="artifacts_dir", ok=True,
                 detail=f"{p} ({n} reports)")


def _probe_latest_report_parseable() -> Probe:
    """If a self_diag_*.md exists, it must parse without crashing."""
    p = _project_root() / "AI" / "artifacts"
    cands = sorted(c for c in p.glob("self_diag_*.md")
                    if "_request_" not in c.name)
    if not cands:
        return Probe(name="latest_report", ok=True,
                     severity="info",
                     detail="(no reports yet — first run pending)")
    latest = cands[-1]
    try:
        from AI.ai.meta_evaluator import parse_report
        parsed = parse_report(latest.read_text(encoding="utf-8",
                                                  errors="replace"))
    except Exception as e:
        return Probe(name="latest_report", ok=False, severity="warn",
                     detail=f"parse failed: {type(e).__name__}: {e}")
    return Probe(name="latest_report", ok=True,
                 detail=f"{latest.name} → grade={parsed.grade} "
                         f"refs={len(parsed.findings)} "
                         f"compliance={parsed.line_compliance:.0%}")


def _probe_api_key() -> Probe:
    """DEEPSEEK_API_KEY presence (warn, not crit — diagnostic still
    parseable / fix_planner usable without it)."""
    from AI.ai.run_self_diagnostic import _api_key
    if _api_key():
        return Probe(name="api_key", ok=True,
                     detail="DEEPSEEK_API_KEY resolved")
    return Probe(name="api_key", ok=False, severity="warn",
                 detail="DEEPSEEK_API_KEY missing — "
                         "run_self_diagnostic.run() will fail")


_PROBES: list[Callable[[], Probe]] = [
    _probe_modules,
    _probe_direction_rule,
    _probe_db_writable,
    _probe_artifacts_dir,
    _probe_latest_report_parseable,
    _probe_api_key,
]


# ── orchestrate ─────────────────────────────────────────────────


def diagnose() -> list[Probe]:
    out: list[Probe] = []
    for fn in _PROBES:
        try:
            out.append(fn())
        except Exception as e:
            out.append(Probe(name=fn.__name__.lstrip("_probe_"),
                              ok=False, severity="crit",
                              detail=f"probe crashed: "
                                      f"{type(e).__name__}: {e}"))
    return out


def has_critical_failure(probes: Optional[list[Probe]] = None) -> bool:
    probes = probes if probes is not None else diagnose()
    return any((not p.ok) and p.severity == "crit" for p in probes)


# ── render ──────────────────────────────────────────────────────


def summary() -> str:
    probes = diagnose()
    lines = [f"🩺 AI/ doctor — {len(probes)} probes"]
    crit = sum(1 for p in probes if not p.ok and p.severity == "crit")
    warn = sum(1 for p in probes if not p.ok and p.severity == "warn")
    if crit == 0 and warn == 0:
        lines.append("  ✅ all probes ok")
    else:
        lines.append(f"  {crit} crit · {warn} warn")
    for p in probes:
        mark = "✅" if p.ok else ("❌" if p.severity == "crit" else "⚠")
        lines.append(f"  {mark} {p.name}: {p.detail}")
    return "\n".join(lines)
