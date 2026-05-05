"""AI/ai/regression_detector.py — RD1 (2026-05-04).

Compare the two most recent self-diagnostic runs in the ledger and
flag NEW critical findings: file:line refs that appear in the latest
report but did NOT appear in the previous one.

Use case: each morning the cron fires `run_self_diagnostic`, the report
is logged to the ledger, and `aim diag --regress` shows whether the
last 24h introduced new high-severity issues.

Public API:
    detect() -> Regression
    summary() -> str
"""
from __future__ import annotations

import dataclasses
import logging
from pathlib import Path
from typing import Optional

log = logging.getLogger("ai.regression_detector")


@dataclasses.dataclass
class Regression:
    have_baseline: bool
    prev_ts: Optional[str]
    curr_ts: Optional[str]
    prev_grade: Optional[str]
    curr_grade: Optional[str]
    prev_crit: Optional[int]
    curr_crit: Optional[int]
    prev_findings: set[str]
    curr_findings: set[str]
    new_findings: set[str]
    fixed_findings: set[str]

    @property
    def grade_improved(self) -> bool:
        """A→F: lower letter is better. None values → no signal."""
        if self.prev_grade is None or self.curr_grade is None:
            return False
        return self.curr_grade < self.prev_grade

    @property
    def grade_worsened(self) -> bool:
        if self.prev_grade is None or self.curr_grade is None:
            return False
        return self.curr_grade > self.prev_grade

    @property
    def regressed(self) -> bool:
        """True if NEW critical issues appeared OR crit count went up.

        BUT — if the overall grade IMPROVED (e.g. D → C), don't flag
        regression even when new refs appear: a more thorough model
        finds more issues, that's quality going up, not down.
        """
        if self.grade_improved:
            return False
        if self.new_findings:
            return True
        if (self.prev_crit is not None and self.curr_crit is not None
                and self.curr_crit > self.prev_crit):
            return True
        return False

    @property
    def improved(self) -> bool:
        return (
            (not self.regressed) and
            (self.grade_improved or
             bool(self.fixed_findings) or
             (self.prev_crit is not None and self.curr_crit is not None
              and self.curr_crit < self.prev_crit))
        )


def _findings_for(report_path: Optional[str]) -> set[str]:
    if not report_path:
        return set()
    p = Path(report_path)
    if not p.exists():
        return set()
    try:
        text = p.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return set()
    from AI.ai.meta_evaluator import parse_report
    return parse_report(text).findings


def detect() -> Regression:
    """Pull the last two ledger rows; diff their finding sets."""
    from AI.ai.diagnostic_ledger import recent
    rows = recent(n=2)
    if len(rows) < 2:
        return Regression(
            have_baseline=False,
            prev_ts=rows[0].ts if rows else None,
            curr_ts=None,
            prev_grade=None, curr_grade=None,
            prev_crit=None, curr_crit=None,
            prev_findings=set(), curr_findings=set(),
            new_findings=set(), fixed_findings=set(),
        )
    prev, curr = rows[0], rows[1]
    pf = _findings_for(prev.report_path)
    cf = _findings_for(curr.report_path)
    new_set = cf - pf
    fixed_set = pf - cf
    # Filter out any findings the user has explicitly suppressed.
    try:
        from AI.ai.finding_suppressions import filter_findings
        new_set = set(filter_findings(new_set))
        # Note: fixed_findings stay — they're informational, not alerting.
    except Exception as e:
        log.debug("suppression filter skipped: %s", e)
    return Regression(
        have_baseline=True,
        prev_ts=prev.ts, curr_ts=curr.ts,
        prev_grade=prev.grade, curr_grade=curr.grade,
        prev_crit=prev.crit, curr_crit=curr.crit,
        prev_findings=pf, curr_findings=cf,
        new_findings=new_set,
        fixed_findings=fixed_set,
    )


def summary() -> str:
    r = detect()
    if not r.have_baseline:
        return ("(no baseline — need at least 2 diagnostic runs in the "
                "ledger before regression detection works)")
    parts = [
        f"🔍 Regression check — {r.prev_ts[:19]} → {r.curr_ts[:19]}",
        f"  grade:  {r.prev_grade or '?'} → {r.curr_grade or '?'}",
        f"  crit:   {r.prev_crit if r.prev_crit is not None else '?'} → "
        f"{r.curr_crit if r.curr_crit is not None else '?'}",
        f"  new findings:   {len(r.new_findings)}",
        f"  fixed findings: {len(r.fixed_findings)}",
    ]
    if r.regressed:
        parts.append("  ⚠ REGRESSED — new critical issues this run:")
        for f in sorted(r.new_findings)[:10]:
            parts.append(f"    • {f}")
        if len(r.new_findings) > 10:
            parts.append(f"    (+{len(r.new_findings) - 10} more)")
    elif r.improved:
        parts.append("  ✅ IMPROVED")
        for f in sorted(r.fixed_findings)[:5]:
            parts.append(f"    • fixed: {f}")
    else:
        parts.append("  = stable (no new findings, no fixes)")
    return "\n".join(parts)
