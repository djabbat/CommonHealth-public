"""AI/tests/test_morning_brief.py — MB1 (2026-05-04)."""
from __future__ import annotations

import pytest


@pytest.fixture
def isolated(tmp_path, monkeypatch):
    monkeypatch.setenv("AI_DIAGNOSTIC_DB", str(tmp_path / "dl.db"))
    monkeypatch.setenv("AIM_EVAL_CASES_DIR", str(tmp_path / "cases"))
    monkeypatch.setenv("AIM_EVAL_ARCHIVE_DIR", str(tmp_path / "arch"))
    (tmp_path / "cases").mkdir()
    import importlib, sys
    for m in (
        "AI.ai.diagnostic_ledger",
        "AI.ai.regression_detector",
        "AI.ai.case_archiver",
        "AI.ai.doctor",
        "AI.ai.morning_brief",
    ):
        if m in sys.modules:
            importlib.reload(sys.modules[m])
    return tmp_path


# ── individual sections ────────────────────────────────────────


def test_section_regression_no_baseline(isolated):
    from AI.ai.morning_brief import _section_regression
    text, bad = _section_regression()
    assert "no baseline" in text
    assert bad is False


def test_section_regression_flags_regression(isolated, tmp_path):
    p1 = tmp_path / "r1.md"
    p1.write_text("`agents/x.py:1`")
    p2 = tmp_path / "r2.md"
    p2.write_text("`agents/x.py:1` and `agents/new.py:42`")
    from AI.ai.diagnostic_ledger import record
    record(model="m", grade="B", n_refs=1, n_with_line=1, crit=0,
           report_path=str(p1), ts="2026-05-03T10:00:00")
    record(model="m", grade="C", n_refs=2, n_with_line=2, crit=1,
           report_path=str(p2), ts="2026-05-04T10:00:00")
    from AI.ai.morning_brief import _section_regression
    text, bad = _section_regression()
    assert bad is True
    assert "REGRESSED" in text
    assert "agents/new.py:42" in text


def test_section_regression_celebrates_improvement(isolated, tmp_path):
    p1 = tmp_path / "r1.md"
    p1.write_text("`agents/x.py:1` and `agents/y.py:2`")
    p2 = tmp_path / "r2.md"
    p2.write_text("`agents/x.py:1`")
    from AI.ai.diagnostic_ledger import record
    record(model="m", grade="D", n_refs=2, n_with_line=2, crit=2,
           report_path=str(p1), ts="2026-05-03T10:00:00")
    record(model="m", grade="B", n_refs=1, n_with_line=1, crit=1,
           report_path=str(p2), ts="2026-05-04T10:00:00")
    from AI.ai.morning_brief import _section_regression
    text, bad = _section_regression()
    assert bad is False
    assert "IMPROVED" in text


def test_section_ledger_empty(isolated):
    from AI.ai.morning_brief import _section_ledger
    assert "no diagnostic runs" in _section_ledger()


def test_section_ledger_renders_metrics(isolated):
    from AI.ai.diagnostic_ledger import record
    record(model="m", grade="B", n_refs=10, n_with_line=8, crit=1,
           retry_used=True)
    record(model="m", grade="B", n_refs=10, n_with_line=10, crit=0)
    from AI.ai.morning_brief import _section_ledger
    text = _section_ledger()
    assert "2 runs" in text
    assert "compliance" in text
    assert "retry fired" in text


def test_section_archive_empty(isolated):
    from AI.ai.morning_brief import _section_archive
    assert "no resolved cases" in _section_archive()


def test_section_archive_lists_candidates(isolated, tmp_path):
    """Plant 1 stale FE1 case + 1 ledger row pointing at unrelated finding."""
    from AI.ai.findings_to_evals import write_cases
    written = write_cases(["agents/x.py:1"])
    assert written
    import datetime as dt, os
    target = dt.datetime.now().timestamp() - (10 * 86400)
    os.utime(written[0], (target, target))
    other = tmp_path / "report.md"
    other.write_text("`agents/y.py:99`")
    from AI.ai.diagnostic_ledger import record
    record(model="m", grade="B", n_refs=1, n_with_line=1, crit=0,
           report_path=str(other))
    from AI.ai.morning_brief import _section_archive
    text = _section_archive()
    assert "ready to archive" in text


# ── render ──────────────────────────────────────────────────────


def test_render_calm_when_clean(isolated):
    from AI.ai.morning_brief import render
    text = render()
    assert "🟢" in text
    assert "healthy" in text.lower()


def test_render_alarms_when_regressed(isolated, tmp_path):
    p1 = tmp_path / "r1.md"
    p1.write_text("`agents/x.py:1`")
    p2 = tmp_path / "r2.md"
    p2.write_text("`agents/x.py:1` and `agents/new.py:42`")
    from AI.ai.diagnostic_ledger import record
    record(model="m", grade="B", n_refs=1, n_with_line=1, crit=0,
           report_path=str(p1), ts="2026-05-03T10:00:00")
    record(model="m", grade="C", n_refs=2, n_with_line=2, crit=1,
           report_path=str(p2), ts="2026-05-04T10:00:00")
    from AI.ai.morning_brief import render
    text = render()
    assert "needs attention" in text
    assert "REGRESSED" in text


def test_render_handles_doctor_crash(isolated, monkeypatch):
    """If a section raises, brief still renders (graceful degradation)."""
    from AI.ai import morning_brief
    def boom():
        raise RuntimeError("doctor died")
    monkeypatch.setattr(morning_brief, "_section_doctor",
                         lambda: (f"unavailable: doctor died", False))
    text = morning_brief.render()
    assert "AIM/AI" in text
    assert "unavailable" in text


def test_render_includes_all_sections(isolated):
    from AI.ai.morning_brief import render
    text = render()
    assert "## High-criticality deadlines" in text
    assert "## Wiring" in text
    assert "## Regression check" in text
    assert "## Diagnostic trend" in text
    assert "## Case archive" in text


# ── deadlines section ───────────────────────────────────────────


def test_section_deadlines_calm_when_none(isolated, monkeypatch):
    """No deadline scanner data → calm message."""
    import sys, agents as _agents_pkg
    fake = type(sys)("agents.deadline_scanner")
    fake.scan_memory = lambda today: []
    monkeypatch.setitem(sys.modules, "agents.deadline_scanner", fake)
    monkeypatch.setattr(_agents_pkg, "deadline_scanner", fake,
                         raising=False)
    from AI.ai.morning_brief import _section_deadlines
    text = _section_deadlines()
    assert "no high-criticality" in text


def test_section_deadlines_lists_high_pending(isolated, monkeypatch):
    import sys, datetime as dt, dataclasses
    import agents as _agents_pkg

    @dataclasses.dataclass
    class FakeDeadline:
        when: dt.date
        label: str
        source: str = ""
        kind: str = "memory"
        criticality: str = "high"

    today = dt.date.today()
    fake_rows = [
        FakeDeadline(when=today, label="Lezhava reply due"),
        FakeDeadline(when=today + dt.timedelta(days=3),
                      label="Submit PATE demo design"),
        FakeDeadline(when=today - dt.timedelta(days=5),
                      label="OLD missed", criticality="medium"),
    ]
    fake = type(sys)("agents.deadline_scanner")
    fake.scan_memory = lambda today=today: fake_rows
    monkeypatch.setitem(sys.modules, "agents.deadline_scanner", fake)
    monkeypatch.setattr(_agents_pkg, "deadline_scanner", fake,
                         raising=False)
    from AI.ai.morning_brief import _section_deadlines
    text = _section_deadlines()
    assert "2 high-criticality" in text
    assert "TODAY" in text
    assert "Lezhava" in text
    assert "+3d" in text
    assert "OLD missed" not in text   # past + medium → excluded


def test_section_deadlines_truncates_long_list(isolated, monkeypatch):
    import sys, datetime as dt, dataclasses
    import agents as _agents_pkg

    @dataclasses.dataclass
    class FakeDeadline:
        when: dt.date
        label: str
        source: str = ""
        kind: str = "memory"
        criticality: str = "high"

    today = dt.date.today()
    rows = [FakeDeadline(when=today + dt.timedelta(days=i),
                          label=f"Item {i}")
            for i in range(8)]
    fake = type(sys)("agents.deadline_scanner")
    fake.scan_memory = lambda today=today: rows
    monkeypatch.setitem(sys.modules, "agents.deadline_scanner", fake)
    monkeypatch.setattr(_agents_pkg, "deadline_scanner", fake,
                         raising=False)
    from AI.ai.morning_brief import _section_deadlines
    text = _section_deadlines()
    assert "8 high-criticality" in text
    assert "+3 more" in text


def test_section_deadlines_handles_scanner_crash(isolated, monkeypatch):
    import sys, agents as _agents_pkg
    fake = type(sys)("agents.deadline_scanner")
    fake.scan_memory = lambda today: (_ for _ in ()).throw(
        RuntimeError("scanner died"))
    monkeypatch.setitem(sys.modules, "agents.deadline_scanner", fake)
    monkeypatch.setattr(_agents_pkg, "deadline_scanner", fake,
                         raising=False)
    from AI.ai.morning_brief import _section_deadlines
    text = _section_deadlines()
    assert "scan failed" in text or "died" in text
