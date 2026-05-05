"""AI/tests/test_pipeline_integration.py — INT1 (2026-05-04).

End-to-end smoke test of the full closed-loop pipeline:

   stub DeepSeek → run_self_diagnostic → DG1 ledger
                                       → RD1 regression detect
                                       → S14 fix_planner
                                       → FE1 findings_to_evals
                                       → CV1 case_validator
                                       → CA1 case_archiver
                                       → DB1 dashboard

If any module's contract drifts (signature, shape, env-var name),
this single test trips, even if individual unit tests still pass.
"""
from __future__ import annotations

import datetime as dt
import os

import pytest


@pytest.fixture
def isolated(tmp_path, monkeypatch):
    monkeypatch.setenv("AI_DIAGNOSTIC_DB", str(tmp_path / "dl.db"))
    monkeypatch.setenv("AI_DISTILL_DB", str(tmp_path / "distill.db"))
    monkeypatch.setenv("AIM_HOME", str(tmp_path / "home"))
    monkeypatch.setenv("AIM_SESSIONS_DIR", str(tmp_path / "sessions"))
    monkeypatch.setenv("AIM_EVAL_CASES_DIR", str(tmp_path / "cases"))
    monkeypatch.setenv("AIM_EVAL_ARCHIVE_DIR", str(tmp_path / "arch"))
    (tmp_path / "cases").mkdir()
    import importlib, sys
    for m in (
        "AI.ai.diagnostic_ledger",
        "AI.ai.regression_detector",
        "AI.ai.distillation_tracker",
        "AI.ai.gap_detector",
        "AI.ai.reflexion_cluster",
        "AI.ai.fix_planner",
        "AI.ai.findings_to_evals",
        "AI.ai.case_validator",
        "AI.ai.case_archiver",
        "AI.ai.dashboard",
        "AI.ai.run_self_diagnostic",
    ):
        if m in sys.modules:
            importlib.reload(sys.modules[m])
    return tmp_path


def _set_age(p, days):
    target = dt.datetime.now().timestamp() - (days * 86400)
    os.utime(p, (target, target))


# ── full pipeline ───────────────────────────────────────────────


def test_full_pipeline_first_run(isolated, monkeypatch, tmp_path):
    """Run 1: virgin pipeline — diagnostic posts, ledger records,
    no baseline yet, fix_planner still emits plan, FE1 generates cases,
    CV1 validates them, CA1 has nothing to do, dashboard renders."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)

    report = (
        "# AIM/AI Self-Diagnostic Report\n"
        "Grade: B\ncrit: 1\nhigh: 2\nmed: 0\nlow: 0\n\n"
        "Bug at `AI/ai/distillation_tracker.py:42` — race in record().\n"
        "Bug at `AI/ai/eval_synthesiser.py:120` — no L_VERIFIABILITY.\n"
        "Stale TODO at `agents/x.py:7`.\n"
    )
    monkeypatch.setattr(r, "_post_deepseek", lambda *a, **kw: report)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "P")

    out = r.run(model="deepseek-chat", verbose=False,
                 compliance_retry=False, skip_safety_gate=True)
    assert out.exists()

    # Ledger captured the run
    from AI.ai.diagnostic_ledger import recent
    rows = recent(n=5)
    assert len(rows) == 1
    assert rows[0].grade == "B"
    assert rows[0].compliance == 1.0   # all 3 refs carry :line
    assert rows[0].crit == 1

    # No baseline yet → regression check is calm
    from AI.ai.regression_detector import detect
    assert detect().have_baseline is False

    # fix_planner emits actionable plan with path-aware advice
    from AI.ai.meta_evaluator import parse_report
    from AI.ai.fix_planner import plan
    parsed = parse_report(report)
    fp = plan(parsed.findings, context_lines=0)
    assert fp.n_files == 3
    advice_blob = " ".join(f.suggestion for f in fp.files)
    assert "WAL" in advice_blob               # distillation_tracker hint
    assert "L_VERIFIABILITY" in advice_blob  # eval_synthesiser hint

    # FE1 turns findings into yaml cases
    from AI.ai.findings_to_evals import write_cases
    written = write_cases(parsed.findings)
    assert len(written) == 3
    case_names = {p.name for p in written}
    assert any("distillation-tracker" in n for n in case_names)

    # CV1 validates them — all schema-clean
    from AI.ai.case_validator import validate_dir
    rep = validate_dir()
    assert rep.n_cases == 3
    assert rep.all_ok is True

    # CA1 has nothing to archive (cases are brand new)
    from AI.ai.case_archiver import candidates
    assert candidates(min_age_days=3) == []

    # DB1 dashboard renders without crashing
    from AI.ai.dashboard import render
    text = render()
    assert "AIM/AI Dashboard" in text
    assert "1 runs" in text


def test_full_pipeline_two_runs_with_regression(isolated, monkeypatch, tmp_path):
    """Run 1 finds X. Run 2 finds X + Y. RD1 must flag Y as new."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "P")

    report1 = "Grade: B\ncrit: 1\nBug at `agents/x.py:1`.\n"
    monkeypatch.setattr(r, "_post_deepseek", lambda *a, **kw: report1)

    # Force ts ordering by faking the output filename
    out1 = tmp_path / "AI" / "artifacts" / "self_diag_2026-05-03.md"
    out1.parent.mkdir(parents=True)
    monkeypatch.setattr(r, "_output_path", lambda today=None: out1)

    r.run(model="deepseek-chat", verbose=False, compliance_retry=False, skip_safety_gate=True)

    report2 = ("Grade: D\ncrit: 2\nBug at `agents/x.py:1` and "
                "`agents/new.py:42`.\n")
    monkeypatch.setattr(r, "_post_deepseek", lambda *a, **kw: report2)
    out2 = tmp_path / "AI" / "artifacts" / "self_diag_2026-05-04.md"
    monkeypatch.setattr(r, "_output_path", lambda today=None: out2)

    # Bump ledger ts by 1 day so chronology is right.
    import time
    time.sleep(0.01)   # microsecond precision is enough on modern Linux
    r.run(model="deepseek-chat", verbose=False, compliance_retry=False, skip_safety_gate=True)

    from AI.ai.regression_detector import detect
    reg = detect()
    assert reg.have_baseline is True
    assert reg.regressed is True
    assert "agents/new.py:42" in reg.new_findings


def test_archiver_picks_up_resolved_finding(isolated, monkeypatch, tmp_path):
    """Generate a case for X. Plant a ledger row whose report does NOT
    flag X. Age the case beyond min_age_days. CA1 must mark it archivable."""
    from AI.ai.findings_to_evals import write_cases
    written = write_cases(["agents/x.py:42"])
    assert written
    _set_age(written[0], 10)

    # Ledger has a recent run with a DIFFERENT finding.
    other_report = tmp_path / "report.md"
    other_report.write_text("Bug at `agents/y.py:99`")
    from AI.ai.diagnostic_ledger import record
    record(model="m", grade="B", n_refs=1, n_with_line=1, crit=0,
           report_path=str(other_report))

    from AI.ai.case_archiver import archive
    res = archive(min_age_days=3, dry_run=False)
    assert res.n_moved == 1
    assert (res.archive_dir / written[0].name).exists()
    assert not written[0].exists()   # original is gone


# ── direction-rule guard (live recheck) ─────────────────────────


def test_direction_rule_still_clean_after_overnight_wave(isolated):
    """Overnight built modules (DG1/RD1/RA1/FE1/CV1/CA1/DB1/DR2)
    expanded the AI/ subtree significantly. Make sure none of them
    accidentally injected a back-import."""
    from AI.ai.self_diagnostic import _direction_rule_status
    out = _direction_rule_status()
    assert out["clean"], (
        f"agents/ → AI/ violations: {out['violations']}"
    )
