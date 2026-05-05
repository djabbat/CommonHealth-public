#!/usr/bin/env python3
"""scripts/daily_brief.py — Daily morning brief (P4, 2026-05-02).

Run from systemd timer or cron at 09:00. Renders all project briefs +
the cross-project deadline summary, then sends to Telegram (or stdout
if AIM_TG_DRYRUN=1 / no token).

Usage:
    python -m scripts.daily_brief                    # send to Telegram
    AIM_TG_DRYRUN=1 python -m scripts.daily_brief    # stdout only

Env vars consumed:
    TELEGRAM_BOT_TOKEN   (or AIM_TG_BOT_TOKEN)
    AIM_TELEGRAM_CHAT_ID — chat id for the brief; if absent, stdout only.
    AIM_BRIEF_HEAD       — optional preamble line.
"""
from __future__ import annotations

import datetime as dt
import logging
import os
import sys
from pathlib import Path

# Make AIM importable when invoked via systemd (cwd is /).
HERE = Path(__file__).resolve().parent.parent
if str(HERE) not in sys.path:
    sys.path.insert(0, str(HERE))

logging.basicConfig(level=os.environ.get("AIM_LOGLEVEL", "INFO"))
log = logging.getLogger("aim.daily_brief")


def render_brief(today: dt.date | None = None) -> str:
    today = today or dt.date.today()
    from agents import project_owner as po
    from agents import deadline_scanner as ds
    parts: list[str] = []

    head = os.environ.get("AIM_BRIEF_HEAD")
    if head is None:
        # B1 (2026-05-03): auto-generate a smart preamble unless the env
        # var was set explicitly. Empty string still wins (suppresses).
        try:
            from agents.brief_preamble import compose as _compose
            head = _compose(today=today)
        except Exception:
            head = ""
    if head:
        parts.append(head)

    parts.append(f"☀️ AIM daily brief — {today.isoformat()}")
    parts.append("")
    parts.append(po.all_briefs(today=today))

    parts.append("")
    parts.append("———")
    parts.append("")
    parts.append(ds.summary(today=today))
    return "\n".join(parts)


def send_telegram(text: str) -> bool:
    """POST text to Telegram. Returns True on 200, False on failure / missing config."""
    token = os.environ.get("TELEGRAM_BOT_TOKEN") or os.environ.get("AIM_TG_BOT_TOKEN")
    chat = os.environ.get("AIM_TELEGRAM_CHAT_ID")
    if not token or not chat:
        log.warning("Telegram not configured (need TELEGRAM_BOT_TOKEN + AIM_TELEGRAM_CHAT_ID)")
        return False
    try:
        import httpx
    except ImportError:
        log.error("httpx not installed; cannot send to Telegram")
        return False
    # Telegram has a 4096-char message limit. Chunk longer briefs.
    LIMIT = 3800
    chunks = [text[i:i + LIMIT] for i in range(0, len(text), LIMIT)] or [text]
    ok = True
    with httpx.Client(timeout=10) as cl:
        for i, body in enumerate(chunks):
            r = cl.post(f"https://api.telegram.org/bot{token}/sendMessage",
                        json={"chat_id": chat, "text": body,
                              "disable_web_page_preview": True})
            if r.status_code != 200:
                log.error("Telegram %d: %s", r.status_code, r.text[:200])
                ok = False
                break
    return ok


def main() -> int:
    text = render_brief()
    if os.environ.get("AIM_TG_DRYRUN") == "1":
        print(text)
        return 0
    # B2 (2026-05-03): respect quiet_hours + delivery channels from prefs.
    try:
        from agents import brief_preferences as bp
        prefs = bp.load()
        if bp.in_quiet_hours(prefs=prefs):
            log.info("inside quiet hours; brief suppressed")
            return 0
        channels = bp.daily_channels(prefs)
    except Exception:
        channels = ["telegram", "stdout"]
    try:
        from agents.notify import notify
        result = notify(text, channels=tuple(channels),
                         subject="AIM daily brief",
                         level="info", source="daily_brief",
                         dedup_key=f"daily:{__import__('datetime').date.today()}",
                         dedup_window_minutes=18 * 60)
        if result.delivered_via:
            log.info("daily brief sent via %s (%d chars)",
                     result.delivered_via, len(text))
            return 0
    except Exception as e:
        log.warning("notify-based delivery failed: %s", e)
    # Final fallback: legacy single-channel telegram, then stdout.
    if send_telegram(text):
        log.info("daily brief sent (%d chars)", len(text))
        return 0
    print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
