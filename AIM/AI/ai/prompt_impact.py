"""AI/ai/prompt_impact.py — PI1 (2026-05-04).

Did tightening the diagnostic prompt actually move metrics?

Joins `prompt_versions` (PV1) with `runs` (DG1) by timestamp ordering
and computes BEFORE-vs-AFTER metric averages around each prompt
revision. Surfaces whether avg compliance / avg crit shifted
meaningfully after the change.

Public API:
    impact_per_revision() -> list[ImpactRow]
    summary() -> str
"""
from __future__ import annotations

import dataclasses
import logging
from typing import Optional

log = logging.getLogger("ai.prompt_impact")


@dataclasses.dataclass
class ImpactRow:
    revision_ts: str
    sha_prefix: str
    n_runs_before: int
    n_runs_after: int
    avg_compliance_before: Optional[float]
    avg_compliance_after: Optional[float]
    avg_crit_before: Optional[float]
    avg_crit_after: Optional[float]

    @property
    def compliance_delta(self) -> Optional[float]:
        if (self.avg_compliance_before is None
                or self.avg_compliance_after is None):
            return None
        return self.avg_compliance_after - self.avg_compliance_before

    @property
    def crit_delta(self) -> Optional[float]:
        if (self.avg_crit_before is None
                or self.avg_crit_after is None):
            return None
        return self.avg_crit_after - self.avg_crit_before


def _avg(xs: list[float]) -> Optional[float]:
    return sum(xs) / len(xs) if xs else None


def impact_per_revision() -> list[ImpactRow]:
    """For each prompt revision, compute window stats from runs whose
    ts fall before vs. after the revision ts.

    A run is "before" if `run.ts < revision.ts` AND `run.ts` is at or
    after the previous revision (or unbounded if first revision). A
    run is "after" if `run.ts >= revision.ts` AND `run.ts` is before
    the next revision (or unbounded if last revision).
    """
    from AI.ai.diagnostic_ledger import all_rows
    from AI.ai.prompt_versions import history

    revs = history()
    if not revs:
        return []
    runs = all_rows()
    out: list[ImpactRow] = []
    for i, rev in enumerate(revs):
        prev_ts = revs[i - 1].ts if i > 0 else None
        next_ts = revs[i + 1].ts if i + 1 < len(revs) else None

        before = [r for r in runs
                   if r.ts < rev.ts
                   and (prev_ts is None or r.ts >= prev_ts)]
        after = [r for r in runs
                  if r.ts >= rev.ts
                  and (next_ts is None or r.ts < next_ts)]

        avg_comp_before = _avg([r.compliance for r in before])
        avg_comp_after = _avg([r.compliance for r in after])
        crit_before = [r.crit for r in before if r.crit is not None]
        crit_after = [r.crit for r in after if r.crit is not None]
        out.append(ImpactRow(
            revision_ts=rev.ts or "?",
            sha_prefix=rev.sha256[:8],
            n_runs_before=len(before),
            n_runs_after=len(after),
            avg_compliance_before=avg_comp_before,
            avg_compliance_after=avg_comp_after,
            avg_crit_before=_avg(crit_before) if crit_before else None,
            avg_crit_after=_avg(crit_after) if crit_after else None,
        ))
    return out


def _fmt_pct(v: Optional[float]) -> str:
    return f"{v:.0%}" if v is not None else "-"


def _fmt_float(v: Optional[float]) -> str:
    return f"{v:.1f}" if v is not None else "-"


def _fmt_delta_pct(v: Optional[float]) -> str:
    if v is None:
        return ""
    return f"  ({v:+.0%})"


def _fmt_delta_float(v: Optional[float]) -> str:
    if v is None:
        return ""
    return f"  ({v:+.1f})"


def summary() -> str:
    rows = impact_per_revision()
    if not rows:
        return "(no prompt revisions recorded — run record_current first)"
    parts = ["📊 Prompt-impact analysis"]
    for r in rows:
        ts = r.revision_ts[:19] if r.revision_ts != "?" else "?"
        parts.append(f"\nrev {r.sha_prefix}  {ts}")
        parts.append(f"  runs: {r.n_runs_before} before / "
                      f"{r.n_runs_after} after")
        parts.append(
            f"  compliance: {_fmt_pct(r.avg_compliance_before)} → "
            f"{_fmt_pct(r.avg_compliance_after)}"
            f"{_fmt_delta_pct(r.compliance_delta)}"
        )
        parts.append(
            f"  avg crit:   {_fmt_float(r.avg_crit_before)} → "
            f"{_fmt_float(r.avg_crit_after)}"
            f"{_fmt_delta_float(r.crit_delta)}"
        )
    return "\n".join(parts)
