"""AI/ai/finding_suppressions.py — FS1 (2026-05-04).

Some diagnostic findings are persistent false-positives or
intentional code (e.g. a TODO that's part of the design, a known
limitation that has a roadmap entry). RA1 alerts must not fire on
those forever.

This module keeps a small SQLite table of suppressed file:line refs.
Suppressions can have an expiry (`until_ts`) — the test of "still
suppressed?" is `until_ts is NULL or until_ts > now`. After expiry
the finding becomes alertable again.

Public API:
    suppress(ref, *, reason="", until=None) -> Suppression
    unsuppress(ref) -> bool
    is_suppressed(ref) -> bool
    active() -> list[Suppression]
    filter_findings(refs) -> list[str]
    summary() -> str
"""
from __future__ import annotations

import contextlib
import dataclasses
import datetime as dt
import logging
import os
import sqlite3
import threading
from pathlib import Path
from typing import Iterable, Optional

log = logging.getLogger("ai.finding_suppressions")

_LOCK = threading.RLock()


def db_path() -> Path:
    """Reuse the diagnostic ledger DB so suppressions live next to
    the metrics they explain."""
    env = os.environ.get("AI_DIAGNOSTIC_DB")
    if env:
        return Path(env)
    return Path.home() / ".cache" / "aim" / "diagnostic_ledger.db"


def _connect() -> sqlite3.Connection:
    p = db_path()
    p.parent.mkdir(parents=True, exist_ok=True)
    conn = sqlite3.connect(p, isolation_level=None, timeout=30)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA synchronous=NORMAL")
    conn.execute("""
        CREATE TABLE IF NOT EXISTS finding_suppressions (
            ref       TEXT PRIMARY KEY,
            reason    TEXT NOT NULL DEFAULT '',
            created_ts TEXT NOT NULL,
            until_ts  TEXT
        )
    """)
    return conn


@dataclasses.dataclass
class Suppression:
    ref: str
    reason: str
    created_ts: str
    until_ts: Optional[str]

    @property
    def active_now(self) -> bool:
        if self.until_ts is None:
            return True
        try:
            until = dt.datetime.fromisoformat(self.until_ts)
        except (ValueError, TypeError):
            return True
        return dt.datetime.now() < until


def suppress(ref: str, *,
              reason: str = "",
              until: Optional[dt.datetime] = None) -> Suppression:
    if not ref or not ref.strip():
        raise ValueError("ref must be non-empty")
    ref = ref.strip()
    created = dt.datetime.now().isoformat()
    until_ts = until.isoformat() if until else None
    with _LOCK, contextlib.closing(_connect()) as conn:
        conn.execute(
            "INSERT OR REPLACE INTO finding_suppressions"
            "(ref, reason, created_ts, until_ts) VALUES (?, ?, ?, ?)",
            (ref, reason, created, until_ts),
        )
    return Suppression(ref=ref, reason=reason,
                        created_ts=created, until_ts=until_ts)


def unsuppress(ref: str) -> bool:
    """Remove a suppression. Returns True if a row was deleted."""
    with _LOCK, contextlib.closing(_connect()) as conn:
        cur = conn.execute(
            "DELETE FROM finding_suppressions WHERE ref = ?", (ref,)
        )
        return cur.rowcount > 0


def _all_rows() -> list[Suppression]:
    with _LOCK, contextlib.closing(_connect()) as conn:
        rows = conn.execute(
            "SELECT ref, reason, created_ts, until_ts "
            "FROM finding_suppressions ORDER BY created_ts ASC"
        ).fetchall()
    return [Suppression(ref=r[0], reason=r[1],
                          created_ts=r[2], until_ts=r[3])
            for r in rows]


def active() -> list[Suppression]:
    """Suppressions that are currently in effect (not expired)."""
    return [s for s in _all_rows() if s.active_now]


def is_suppressed(ref: str) -> bool:
    for s in active():
        if s.ref == ref:
            return True
    return False


def filter_findings(refs: Iterable[str]) -> list[str]:
    """Return refs minus any currently-suppressed."""
    blocked = {s.ref for s in active()}
    return [r for r in refs if r not in blocked]


def summary() -> str:
    rows = _all_rows()
    if not rows:
        return "(no finding suppressions)"
    act = [r for r in rows if r.active_now]
    expired = [r for r in rows if not r.active_now]
    parts = [f"🔇 Finding suppressions — {len(act)} active, "
             f"{len(expired)} expired"]
    for s in act[:15]:
        until = f" (until {s.until_ts[:10]})" if s.until_ts else ""
        reason = f" — {s.reason}" if s.reason else ""
        parts.append(f"  • {s.ref}{until}{reason}")
    if len(act) > 15:
        parts.append(f"  (+{len(act) - 15} more)")
    return "\n".join(parts)


def prune_expired() -> int:
    """Delete rows whose `until_ts` has passed. Returns count removed."""
    rows = _all_rows()
    expired_refs = [r.ref for r in rows if not r.active_now]
    if not expired_refs:
        return 0
    with _LOCK, contextlib.closing(_connect()) as conn:
        for i in range(0, len(expired_refs), 500):
            batch = expired_refs[i:i + 500]
            placeholders = ",".join("?" * len(batch))
            conn.execute(
                f"DELETE FROM finding_suppressions WHERE ref IN "
                f"({placeholders})",
                batch,
            )
    return len(expired_refs)
