"""AI/ai/regression_alert.py — RA1 (2026-05-04).

When a regression is detected by `regression_detector`, push an alert
through the runtime's `notify_mux` (Telegram / email / log).

Direction rule preserved: AI/ → agents/ is allowed; agents/ never
imports from AI/.

Public API:
    check_and_alert() -> Alert | None
"""
from __future__ import annotations

import dataclasses
import logging
from typing import Optional

log = logging.getLogger("ai.regression_alert")


@dataclasses.dataclass
class Alert:
    fired: bool
    title: str
    body: str
    channels: list[str]


def _format(r) -> tuple[str, str]:
    title = (f"AIM/AI regression — {len(r.new_findings)} new "
             f"finding(s)")
    crit_delta = ""
    if r.prev_crit is not None and r.curr_crit is not None:
        crit_delta = (f"\ncrit: {r.prev_crit} → {r.curr_crit} "
                      f"(Δ {r.curr_crit - r.prev_crit:+d})")
    new_list = "\n".join(f"  • {f}" for f in sorted(r.new_findings)[:10])
    if len(r.new_findings) > 10:
        new_list += f"\n  (+{len(r.new_findings) - 10} more)"
    body = (
        f"between {r.prev_ts[:19]} and {r.curr_ts[:19]}:\n"
        f"grade: {r.prev_grade or '?'} → {r.curr_grade or '?'}"
        f"{crit_delta}\n\nnew findings:\n{new_list}"
    )
    return (title, body)


def check_and_alert(*, dry_run: bool = False) -> Optional[Alert]:
    """If the latest two ledger rows show regression, push to notify_mux.

    Returns the Alert struct (with `fired` boolean) on regression, or
    None if no baseline / not regressed. With `dry_run=True`, the
    alert is built but the notification side-effect is skipped.
    """
    from AI.ai.regression_detector import detect
    r = detect()
    if not r.have_baseline:
        return None
    if not r.regressed:
        return None
    title, body = _format(r)
    if dry_run:
        return Alert(fired=False, title=title, body=body, channels=[])

    channels: list[str] = []
    try:
        from agents.notify import notify as _notify
        full = f"{title}\n\n{body}"
        res = _notify(full, subject=title, level="high",
                      source="ai.regression_alert",
                      dedup_key=f"regression:{r.curr_ts[:10]}",
                      dedup_window_minutes=720.0)
        if getattr(res, "delivered_via", None):
            channels = [res.delivered_via]
    except Exception as e:
        log.warning("notify unavailable: %s", e)
    return Alert(fired=bool(channels), title=title, body=body,
                 channels=channels)
