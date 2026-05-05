"""AI/ai/morning_brief.py — MB1 (2026-05-04).

A single-shot wake-up briefing for AIM/AI subproject state. Like
`aim brief` but specifically for AI/ — what happened overnight, what
needs attention.

Pulls from doctor (wiring), regression_detector (any bugs?), ledger
(trend), case_archiver (anything to retire?), and surfaces *only*
the lines worth reading first thing in the morning.

Public API:
    render() -> str
"""
from __future__ import annotations

import logging
from typing import Optional

log = logging.getLogger("ai.morning_brief")


def _section_doctor() -> tuple[str, bool]:
    from AI.ai.doctor import diagnose, has_critical_failure
    probes = diagnose()
    crit = [p for p in probes if not p.ok and p.severity == "crit"]
    warn = [p for p in probes if not p.ok and p.severity == "warn"]
    if not crit and not warn:
        return ("✅ wiring clean — all probes ok", False)
    parts: list[str] = []
    if crit:
        parts.append(f"❌ {len(crit)} critical wiring issue(s):")
        for p in crit:
            parts.append(f"   • {p.name}: {p.detail.splitlines()[0]}")
    if warn:
        parts.append(f"⚠ {len(warn)} warning(s):")
        for p in warn:
            parts.append(f"   • {p.name}: {p.detail.splitlines()[0]}")
    return ("\n".join(parts), bool(crit))


def _section_regression() -> tuple[str, bool]:
    from AI.ai.regression_detector import detect
    try:
        r = detect()
    except Exception as e:
        return (f"⚠ regression check unavailable: {e}", False)
    if not r.have_baseline:
        return ("(no baseline yet — first 2 diagnostic runs needed)", False)
    if r.regressed:
        new_list = ", ".join(sorted(r.new_findings)[:3])
        more = (f" +{len(r.new_findings) - 3} more"
                if len(r.new_findings) > 3 else "")
        return (
            f"❌ REGRESSED — {len(r.new_findings)} new finding(s): "
            f"{new_list}{more}",
            True,
        )
    if r.improved:
        return (f"✅ IMPROVED — {len(r.fixed_findings)} finding(s) fixed",
                False)
    return ("= stable since last run", False)


def _section_ledger() -> str:
    from AI.ai.diagnostic_ledger import trend
    t = trend()
    if t["n_runs"] == 0:
        return "(no diagnostic runs in ledger)"
    parts = [
        f"{t['n_runs']} runs · "
        f"avg compliance {t['avg_compliance']:.0%} · "
        f"avg crit {t['avg_crit']:.1f}"
    ]
    if t["retry_share"] > 0:
        parts.append(f"  retry fired in {t['retry_share']:.0%} of runs")
    return "\n".join(parts)


def _section_archive() -> str:
    try:
        from AI.ai.case_archiver import candidates
        cands = candidates()
    except Exception as e:
        return f"(archive scan failed: {e})"
    if not cands:
        return "(no resolved cases to archive)"
    return (f"{len(cands)} regression case(s) ready to archive — "
            "run `aim diag --archive-cases` to retire")


def _section_deadlines() -> str:
    """High-crit pending deadlines (from agents/deadline_scanner)."""
    try:
        import datetime as dt
        from agents.deadline_scanner import scan_memory
    except Exception as e:
        return f"(deadline scanner unavailable: {e})"
    today = dt.date.today()
    try:
        rows = scan_memory(today=today)
    except Exception as e:
        return f"(deadline scan failed: {e})"
    high_pending = [r for r in rows
                     if r.criticality == "high" and r.when >= today]
    if not high_pending:
        return "(no high-criticality pending deadlines)"
    high_pending.sort(key=lambda r: r.when)
    parts = [f"{len(high_pending)} high-criticality pending:"]
    for r in high_pending[:5]:
        days = (r.when - today).days
        when = "TODAY" if days == 0 else f"+{days}d"
        # trim label to first 80 chars, single-line
        label = r.label.replace("\n", " ")[:80]
        parts.append(f"  • {when:>5}  {r.when}  {label}")
    if len(high_pending) > 5:
        parts.append(f"  (+{len(high_pending) - 5} more)")
    return "\n".join(parts)


def render() -> str:
    """Render the morning brief.

    Format: header line ("good morning" status) + sections. The header
    summarises whether anything actually needs attention; sections give
    detail.
    """
    doctor_text, doctor_crit = _section_doctor()
    regr_text, regr_bad = _section_regression()
    ledger_text = _section_ledger()
    archive_text = _section_archive()

    overall_bad = doctor_crit or regr_bad
    headline = ("⚠ AIM/AI needs attention this morning"
                if overall_bad
                else "🟢 AIM/AI is healthy this morning")

    deadlines_text = _section_deadlines()
    parts = [
        f"# {headline}",
        "",
        "## High-criticality deadlines",
        deadlines_text,
        "",
        "## Wiring",
        doctor_text,
        "",
        "## Regression check",
        regr_text,
        "",
        "## Diagnostic trend",
        ledger_text,
        "",
        "## Case archive",
        archive_text,
    ]
    return "\n".join(parts)
