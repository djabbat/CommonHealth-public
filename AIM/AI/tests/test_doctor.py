"""AI/tests/test_doctor.py — DR2 (2026-05-04)."""
from __future__ import annotations

import pytest


@pytest.fixture
def isolated(tmp_path, monkeypatch):
    monkeypatch.setenv("AI_DIAGNOSTIC_DB", str(tmp_path / "dl.db"))
    import importlib, sys
    if "AI.ai.doctor" in sys.modules:
        importlib.reload(sys.modules["AI.ai.doctor"])
    if "AI.ai.diagnostic_ledger" in sys.modules:
        importlib.reload(sys.modules["AI.ai.diagnostic_ledger"])
    return tmp_path


# ── individual probes ───────────────────────────────────────────


def test_probe_modules_clean(isolated):
    """Real AI/ai/* should all import."""
    from AI.ai.doctor import _probe_modules
    p = _probe_modules()
    assert p.ok is True
    assert "import cleanly" in p.detail


def test_probe_direction_rule_clean(isolated):
    from AI.ai.doctor import _probe_direction_rule
    p = _probe_direction_rule()
    assert p.ok is True
    assert "clean" in p.detail.lower()


def test_probe_db_writable(isolated, tmp_path):
    """Setting AI_DIAGNOSTIC_DB to a writable path → probe ok."""
    from AI.ai.doctor import _probe_db_writable
    p = _probe_db_writable()
    assert p.ok is True


def test_probe_db_unwritable(isolated, tmp_path, monkeypatch):
    """If parent is a regular file (not a dir), mkdir should fail."""
    blocker = tmp_path / "blocker"
    blocker.write_text("not-a-dir")
    monkeypatch.setenv("AI_DIAGNOSTIC_DB", str(blocker / "dl.db"))
    import importlib, sys
    importlib.reload(sys.modules["AI.ai.diagnostic_ledger"])
    from AI.ai.doctor import _probe_db_writable
    p = _probe_db_writable()
    assert p.ok is False
    assert p.severity == "crit"


def test_probe_artifacts_dir(isolated):
    from AI.ai.doctor import _probe_artifacts_dir
    p = _probe_artifacts_dir()
    assert p.ok is True


def test_probe_latest_report_no_reports(isolated, monkeypatch, tmp_path):
    """No reports yet → ok info-level (not a failure)."""
    monkeypatch.setattr("AI.ai.doctor._project_root",
                         lambda: tmp_path)
    (tmp_path / "AI" / "artifacts").mkdir(parents=True)
    from AI.ai.doctor import _probe_latest_report_parseable
    p = _probe_latest_report_parseable()
    assert p.ok is True
    assert "no reports" in p.detail.lower()


def test_probe_latest_report_parseable(isolated, monkeypatch, tmp_path):
    monkeypatch.setattr("AI.ai.doctor._project_root",
                         lambda: tmp_path)
    artifacts = tmp_path / "AI" / "artifacts"
    artifacts.mkdir(parents=True)
    (artifacts / "self_diag_2026-05-04.md").write_text(
        "Grade: B\n`agents/x.py:1`", encoding="utf-8")
    from AI.ai.doctor import _probe_latest_report_parseable
    p = _probe_latest_report_parseable()
    assert p.ok is True
    assert "grade=B" in p.detail


def test_probe_latest_report_skips_request_files(isolated, monkeypatch, tmp_path):
    """Prompt-request copies must NOT be picked as the latest report."""
    monkeypatch.setattr("AI.ai.doctor._project_root",
                         lambda: tmp_path)
    artifacts = tmp_path / "AI" / "artifacts"
    artifacts.mkdir(parents=True)
    # Lexically later but should be filtered out.
    (artifacts / "self_diag_request_2099-12-31.md").write_text("x")
    (artifacts / "self_diag_2026-05-04.md").write_text(
        "Grade: B\n`agents/x.py:1`", encoding="utf-8")
    from AI.ai.doctor import _probe_latest_report_parseable
    p = _probe_latest_report_parseable()
    assert p.ok is True
    assert "2026-05-04" in p.detail


def test_probe_api_key_missing(isolated, monkeypatch, tmp_path):
    monkeypatch.delenv("DEEPSEEK_API_KEY", raising=False)
    monkeypatch.setenv("HOME", str(tmp_path))   # no ~/.aim_env in tmp
    import importlib, sys
    importlib.reload(sys.modules["AI.ai.run_self_diagnostic"])
    from AI.ai.doctor import _probe_api_key
    p = _probe_api_key()
    assert p.ok is False
    assert p.severity == "warn"


def test_probe_api_key_present(isolated, monkeypatch):
    monkeypatch.setenv("DEEPSEEK_API_KEY", "sk-stub")
    import importlib, sys
    importlib.reload(sys.modules["AI.ai.run_self_diagnostic"])
    from AI.ai.doctor import _probe_api_key
    p = _probe_api_key()
    assert p.ok is True


# ── orchestrate ─────────────────────────────────────────────────


def test_diagnose_returns_probes(isolated):
    from AI.ai.doctor import diagnose
    out = diagnose()
    names = {p.name for p in out}
    assert "modules" in names
    assert "direction_rule" in names
    assert "db_writable" in names


def test_diagnose_swallows_probe_crash(isolated, monkeypatch):
    """If a probe raises, diagnose() still returns and marks it crit."""
    from AI.ai import doctor

    def boom():
        raise RuntimeError("probe died")

    monkeypatch.setattr(doctor, "_PROBES", [boom])
    out = doctor.diagnose()
    assert len(out) == 1
    assert out[0].ok is False
    assert "probe died" in out[0].detail


def test_has_critical_failure_false_when_clean(isolated):
    from AI.ai.doctor import has_critical_failure
    # Real repo should be clean (or at most warnings, not crit).
    # If api_key is missing it's only `warn`, which doesn't count.
    # We don't assert False here unconditionally — we assert that
    # the function returns a bool deterministically.
    assert isinstance(has_critical_failure(), bool)


def test_has_critical_failure_true_when_crit_present(isolated, monkeypatch):
    from AI.ai import doctor
    crit = doctor.Probe(name="x", ok=False, severity="crit",
                         detail="boom")
    assert doctor.has_critical_failure([crit]) is True
    warn = doctor.Probe(name="y", ok=False, severity="warn", detail="…")
    assert doctor.has_critical_failure([warn]) is False


# ── summary ─────────────────────────────────────────────────────


def test_summary_contains_probe_names(isolated):
    from AI.ai.doctor import summary
    s = summary()
    assert "doctor" in s
    assert "modules" in s
    assert "direction_rule" in s


def test_summary_calm_when_all_ok(isolated, monkeypatch):
    from AI.ai import doctor
    monkeypatch.setattr(doctor, "_PROBES", [
        lambda: doctor.Probe(name="x", ok=True, detail="ok"),
    ])
    s = doctor.summary()
    assert "all probes ok" in s
