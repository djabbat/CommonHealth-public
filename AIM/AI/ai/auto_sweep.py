"""AI/ai/auto_sweep.py — AS1 (2026-05-04).

A maintenance routine that runs the side-effect-light parts of the
closed loop on a schedule (cron / systemd):

  1. Fingerprint the current SELF_DIAGNOSTIC_PROMPT.md (PV1).
  2. Validate every yaml case in AIM_EVAL_CASES_DIR (CV1).
  3. Archive stale FE1-generated regression cases (CA1).
  4. Compute prompt-impact deltas (PI1).
  5. Render a compact maintenance report.

Doctor wiring (DR2) and morning brief (MB1) are NOT called here —
those are read-only and meant for human ad-hoc inspection. Sweep is
the *write-back* maintenance complement.

Public API:
    sweep(*, dry_run=False) -> SweepResult
    summary(*, dry_run=False) -> str
"""
from __future__ import annotations

import dataclasses
import datetime as dt
import logging
from typing import Optional

log = logging.getLogger("ai.auto_sweep")


@dataclasses.dataclass
class SweepResult:
    started_at: str
    finished_at: str
    prompt_recorded: bool
    prompt_changed: Optional[bool]      # None if first-time
    n_cases_validated: int
    n_cases_invalid: int
    n_archived_candidates: int
    n_archived_moved: int
    n_prompt_revisions: int
    n_phantom_removed: int = 0
    notes: list[str] = dataclasses.field(default_factory=list)

    @property
    def all_clean(self) -> bool:
        return (self.n_cases_invalid == 0)


def _now() -> str:
    return dt.datetime.now().isoformat(timespec="seconds")


def sweep(*, dry_run: bool = False) -> SweepResult:
    started = _now()
    notes: list[str] = []

    # Step 1: fingerprint prompt
    prompt_recorded = False
    prompt_changed: Optional[bool] = None
    try:
        from AI.ai.prompt_versions import (
            fingerprint, history, record_current,
        )
        existing = history()
        cur_fp = fingerprint()
        if not existing:
            prompt_changed = None
        else:
            prompt_changed = (cur_fp.sha256 != existing[-1].sha256)
        if not dry_run:
            record_current()
            prompt_recorded = True
    except FileNotFoundError:
        notes.append("prompt file missing")
    except Exception as e:
        notes.append(f"prompt step failed: {e}")

    # Step 2: validate cases
    n_val = 0
    n_inv = 0
    try:
        from AI.ai.case_validator import validate_dir
        rep = validate_dir()
        n_val = rep.n_cases
        n_inv = rep.n_failed
        if n_inv:
            for s in rep.statuses:
                if not s.ok:
                    notes.append(f"invalid case: {s.path.name} "
                                  f"({len(s.issues)} issue(s))")
    except Exception as e:
        notes.append(f"case validator failed: {e}")

    # Step 3: archive stale cases
    n_cands = 0
    n_moved = 0
    try:
        from AI.ai.case_archiver import archive
        res = archive(dry_run=dry_run)
        n_cands = res.n_candidates
        n_moved = res.n_moved
    except Exception as e:
        notes.append(f"archive step failed: {e}")

    # Step 4: prompt impact (read-only)
    n_revs = 0
    try:
        from AI.ai.prompt_impact import impact_per_revision
        n_revs = len(impact_per_revision())
    except Exception as e:
        notes.append(f"impact step failed: {e}")

    # Step 5: prune phantom ledger rows (test-fixture leftovers).
    # Done BEFORE score snapshot so the score reflects post-cleanup state.
    n_phantom = 0
    try:
        from AI.ai.diagnostic_ledger import prune_phantom
        res = prune_phantom(dry_run=dry_run)
        n_phantom = res["would_remove"] if dry_run else res["removed"]
    except Exception as e:
        notes.append(f"prune step failed: {e}")

    # Step 6: prune expired finding-suppressions
    if not dry_run:
        try:
            from AI.ai.finding_suppressions import prune_expired
            n_exp = prune_expired()
            if n_exp:
                notes.append(f"removed {n_exp} expired suppression(s)")
        except Exception as e:
            notes.append(f"suppression prune failed: {e}")

    # Step 7: snapshot the health score (post-cleanup) so the daily
    # trend reflects production state, not test pollution.
    if not dry_run:
        try:
            from AI.ai.health_score import record as record_score
            record_score()
        except Exception as e:
            notes.append(f"score snapshot failed: {e}")

    return SweepResult(
        started_at=started,
        finished_at=_now(),
        prompt_recorded=prompt_recorded,
        prompt_changed=prompt_changed,
        n_cases_validated=n_val,
        n_cases_invalid=n_inv,
        n_archived_candidates=n_cands,
        n_archived_moved=n_moved,
        n_prompt_revisions=n_revs,
        n_phantom_removed=n_phantom,
        notes=notes,
    )


def summary(*, dry_run: bool = False) -> str:
    r = sweep(dry_run=dry_run)
    parts = [f"🧹 Auto-sweep ({'dry-run' if dry_run else 'live'}) — "
             f"{r.started_at}"]
    if r.prompt_changed is None and r.prompt_recorded:
        parts.append("  • prompt fingerprint recorded for the first time")
    elif r.prompt_changed is True and r.prompt_recorded:
        parts.append("  • prompt CHANGED — new revision logged")
    elif r.prompt_changed is False:
        parts.append("  • prompt unchanged")
    parts.append(f"  • cases validated: {r.n_cases_validated} "
                  f"({r.n_cases_invalid} invalid)")
    if r.n_archived_candidates:
        if dry_run:
            parts.append(f"  • would archive: {r.n_archived_candidates} "
                          "stale regression case(s)")
        else:
            parts.append(f"  • archived: {r.n_archived_moved} stale "
                          "regression case(s)")
    else:
        parts.append("  • no cases ready for archive")
    if r.n_phantom_removed:
        verb = "would remove" if dry_run else "removed"
        parts.append(f"  • {verb} {r.n_phantom_removed} phantom "
                      "ledger row(s)")
    parts.append(f"  • prompt revisions tracked: {r.n_prompt_revisions}")
    if r.notes:
        parts.append("  notes:")
        for n in r.notes:
            parts.append(f"    - {n}")
    parts.append(f"  finished {r.finished_at}")
    return "\n".join(parts)
