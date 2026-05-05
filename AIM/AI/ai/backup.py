"""AI/ai/backup.py — BK1 (2026-05-04).

JSON snapshot of every persistent DB AIM/AI uses, for backup / migration
/ debugging. Safe to run any time — read-only, no writes back.

Public API:
    snapshot() -> dict
    write_snapshot(path) -> Path
    restore(path, *, dry_run=False) -> dict
"""
from __future__ import annotations

import contextlib
import datetime as dt
import json
import logging
import sqlite3
from pathlib import Path
from typing import Optional

log = logging.getLogger("ai.backup")


_TABLES = (
    "runs",
    "tier_runs",
    "prompt_versions",
    "health_scores",
    "finding_suppressions",
)


def _diag_db_path() -> Path:
    from AI.ai.diagnostic_ledger import db_path
    return db_path()


def _distill_db_path() -> Path:
    from AI.ai.distillation_tracker import db_path
    return db_path()


def _dump_db(p: Path) -> dict:
    """Read every known table from the DB. Missing tables → []."""
    out: dict[str, list[dict]] = {}
    if not p.exists():
        return out
    with contextlib.closing(sqlite3.connect(p)) as conn:
        conn.row_factory = sqlite3.Row
        cur = conn.execute(
            "SELECT name FROM sqlite_master "
            "WHERE type='table' AND name NOT LIKE 'sqlite_%'"
        )
        names = [r[0] for r in cur.fetchall()]
        for name in names:
            if name not in _TABLES:
                continue
            rows = conn.execute(f"SELECT * FROM {name}").fetchall()
            out[name] = [dict(r) for r in rows]
    return out


def snapshot() -> dict:
    return {
        "version": 1,
        "created_at": dt.datetime.now().isoformat(timespec="seconds"),
        "diagnostic_db": {
            "path": str(_diag_db_path()),
            "tables": _dump_db(_diag_db_path()),
        },
        "distillation_db": {
            "path": str(_distill_db_path()),
            "tables": _dump_db(_distill_db_path()),
        },
    }


def write_snapshot(path: Optional[Path] = None) -> Path:
    s = snapshot()
    if path is None:
        try:
            from AI.ai.run_self_diagnostic import ai_root
            base = ai_root() / "artifacts"
        except Exception:
            base = Path.home() / ".cache" / "aim"
        base.mkdir(parents=True, exist_ok=True)
        ts = dt.datetime.now().strftime("%Y-%m-%dT%H%M%S")
        path = base / f"backup_{ts}.json"
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(s, ensure_ascii=False, indent=2),
                     encoding="utf-8")
    return path


def _restore_db(p: Path, table_data: dict[str, list[dict]],
                  *, dry_run: bool) -> dict:
    """Re-insert rows from a snapshot back into a DB. UNIQUE indexes
    must already match — we use INSERT OR IGNORE so duplicates are
    silently skipped."""
    counts: dict[str, int] = {}
    if dry_run:
        return {name: len(rows) for name, rows in table_data.items()}
    p.parent.mkdir(parents=True, exist_ok=True)
    with contextlib.closing(sqlite3.connect(p)) as conn:
        for name, rows in table_data.items():
            if not rows:
                counts[name] = 0
                continue
            # Make sure the table exists by re-touching the source module.
            if name == "runs":
                from AI.ai.diagnostic_ledger import _connect as c
                c().close()
            elif name == "tier_runs":
                from AI.ai.distillation_tracker import _connect as c
                c().close()
            elif name == "prompt_versions":
                from AI.ai.prompt_versions import _connect as c
                c().close()
            elif name == "health_scores":
                from AI.ai.health_score import _connect as c
                c().close()
            elif name == "finding_suppressions":
                from AI.ai.finding_suppressions import _connect as c
                c().close()
            cols = list(rows[0].keys())
            placeholders = ",".join("?" * len(cols))
            stmt = (f"INSERT OR IGNORE INTO {name} "
                     f"({','.join(cols)}) VALUES ({placeholders})")
            inserted = 0
            for r in rows:
                conn.execute(stmt, [r[c] for c in cols])
                inserted += 1
            conn.commit()
            counts[name] = inserted
    return counts


def restore(path: Path, *, dry_run: bool = False) -> dict:
    p = Path(path)
    if not p.exists():
        raise FileNotFoundError(p)
    payload = json.loads(p.read_text(encoding="utf-8"))
    if payload.get("version") != 1:
        raise ValueError(f"unsupported snapshot version: "
                          f"{payload.get('version')!r}")
    diag_counts = _restore_db(
        _diag_db_path(),
        payload.get("diagnostic_db", {}).get("tables", {}),
        dry_run=dry_run,
    )
    distill_counts = _restore_db(
        _distill_db_path(),
        payload.get("distillation_db", {}).get("tables", {}),
        dry_run=dry_run,
    )
    return {"diagnostic_db": diag_counts,
             "distillation_db": distill_counts,
             "dry_run": dry_run}


def summary() -> str:
    s = snapshot()
    parts = ["📦 Backup snapshot inventory"]
    for db_key in ("diagnostic_db", "distillation_db"):
        parts.append(f"  {db_key}: {s[db_key]['path']}")
        for tname, rows in sorted(s[db_key]["tables"].items()):
            parts.append(f"    • {tname:24s} {len(rows):>5d} rows")
    return "\n".join(parts)
