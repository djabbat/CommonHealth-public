"""AI/ai/self_modify.py — S6 (2026-05-04, framework only).

S6 closes the loop: AIM modifies its own `agents/` code based on
diagnostic findings + eval gate.

This module is intentionally **scaffolded but disabled** until at
least 4 weeks of accumulated baselines exist (see roadmap rule). The
framework is here so the loop can flip on the moment the baseline
threshold is reached, with all safety mechanisms already tested.

Direction rule: S6 LIVES in AI/ but ITS WRITE TARGET is `agents/` —
worktree isolation makes that one-way. We never short-circuit and
mutate the live tree.

Public API:
    can_self_modify() -> Verdict
    propose(finding) -> Proposal
    apply(proposal, *, dry_run=True) -> ApplyResult
"""
from __future__ import annotations

import dataclasses
import datetime as dt
import logging
import os
from pathlib import Path
from typing import Optional

log = logging.getLogger("ai.self_modify")


@dataclasses.dataclass
class Verdict:
    allowed: bool
    reasons: list[str]
    n_baseline_runs: int
    baseline_age_days: float


@dataclasses.dataclass
class Proposal:
    finding_ref: str        # `agents/x.py:42`
    target_path: Path
    summary: str            # short rationale
    patch_unified: str      # unified diff or empty
    eval_case_id: Optional[str] = None


@dataclasses.dataclass
class ApplyResult:
    proposal: Proposal
    applied: bool
    worktree_path: Optional[Path]
    pre_eval_score: Optional[float]
    post_eval_score: Optional[float]
    notes: list[str]


# ── safety prerequisites ────────────────────────────────────────


_MIN_BASELINE_RUNS = 28        # ~4 wks daily
_MIN_BASELINE_AGE_DAYS = 28


def can_self_modify() -> Verdict:
    """S6 only fires once baseline is mature. We require BOTH a long
    enough wall-clock window AND enough sampled runs — neither alone
    is sufficient (one cron firing all month with no real responses
    must NOT unlock the gate)."""
    reasons: list[str] = []
    n_runs = 0
    age_days = 0.0
    try:
        from AI.ai.diagnostic_ledger import all_rows
        rows = all_rows()
        n_runs = len(rows)
        if rows:
            try:
                first = dt.datetime.fromisoformat(rows[0].ts)
                age_days = (dt.datetime.now() - first).total_seconds() / 86400.0
            except (ValueError, TypeError):
                age_days = 0.0
    except Exception as e:
        reasons.append(f"ledger unavailable: {e}")

    if n_runs < _MIN_BASELINE_RUNS:
        reasons.append(
            f"baseline runs {n_runs} < {_MIN_BASELINE_RUNS} required"
        )
    if age_days < _MIN_BASELINE_AGE_DAYS:
        reasons.append(
            f"baseline age {age_days:.1f}d < {_MIN_BASELINE_AGE_DAYS}d required"
        )

    # Hard kill-switch via env so an emergency stop is one command:
    #   export AI_SELF_MODIFY_DISABLED=1
    if os.environ.get("AI_SELF_MODIFY_DISABLED"):
        reasons.append("AI_SELF_MODIFY_DISABLED env set")

    return Verdict(
        allowed=not reasons,
        reasons=reasons,
        n_baseline_runs=n_runs,
        baseline_age_days=age_days,
    )


# ── propose ─────────────────────────────────────────────────────


def propose(finding_ref: str) -> Proposal:
    """Build a Proposal struct for `finding_ref`. Currently only
    populates metadata — patch generation lands when can_self_modify()
    returns allowed."""
    parts = finding_ref.split(":")
    path = parts[0]
    return Proposal(
        finding_ref=finding_ref,
        target_path=Path(path),
        summary=f"(stub) framework proposal for {finding_ref}",
        patch_unified="",
    )


# ── apply ───────────────────────────────────────────────────────


def apply(proposal: Proposal, *, dry_run: bool = True) -> ApplyResult:
    """Apply a Proposal in an isolated worktree, then run S1 evals
    pre/post and decide whether to merge.

    For now: refuses unless can_self_modify() allows AND dry_run.
    Real mutation lands when baseline is mature."""
    notes: list[str] = []
    v = can_self_modify()
    if not v.allowed:
        notes.append("can_self_modify denied: " + "; ".join(v.reasons))
        return ApplyResult(
            proposal=proposal, applied=False, worktree_path=None,
            pre_eval_score=None, post_eval_score=None, notes=notes,
        )

    # Even when verdict passes, never mutate without explicit consent.
    # This branch is reached only after L_CONSENT integration in a
    # later wave — for now we force dry_run.
    if not dry_run:
        notes.append("forced to dry_run — live mutation not yet enabled")
    notes.append("(framework: would isolate worktree, apply patch, "
                  "run S1 evals pre/post, merge if Δscore >= 0.05 & p ≤ 0.05)")
    return ApplyResult(
        proposal=proposal, applied=False, worktree_path=None,
        pre_eval_score=None, post_eval_score=None, notes=notes,
    )


def summary() -> str:
    v = can_self_modify()
    if v.allowed:
        return ("🟢 self-modify gate OPEN — baseline mature.\n"
                f"  runs={v.n_baseline_runs}  age={v.baseline_age_days:.1f}d")
    parts = ["🔒 self-modify gate CLOSED",
             f"  runs={v.n_baseline_runs}  age={v.baseline_age_days:.1f}d"]
    for r in v.reasons:
        parts.append(f"  - {r}")
    return "\n".join(parts)
