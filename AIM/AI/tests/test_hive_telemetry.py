"""AI/tests/test_hive_telemetry.py — HV1 worker (2026-05-04)."""
from __future__ import annotations

import json

import pytest


@pytest.fixture
def isolated(tmp_path, monkeypatch):
    monkeypatch.setenv("AI_DIAGNOSTIC_DB", str(tmp_path / "dl.db"))
    monkeypatch.setenv("HOME", str(tmp_path / "home"))
    fake = tmp_path / "PROMPT.md"
    fake.write_text("v1 prompt\n", encoding="utf-8")
    monkeypatch.setenv("AI_DIAGNOSTIC_PROMPT", str(fake))
    monkeypatch.setenv("AIM_SESSIONS_DIR", str(tmp_path / "sessions"))
    monkeypatch.delenv("AIM_HIVE_QUEEN_URL", raising=False)
    import importlib, sys
    for m in ("AI.ai.diagnostic_ledger", "AI.ai.prompt_versions",
              "AI.ai.reflexion_cluster", "AI.ai.finding_suppressions",
              "AI.ai.hive_telemetry"):
        if m in sys.modules:
            importlib.reload(sys.modules[m])
    return tmp_path


# ── _scrub: PII enforcement (L_PRIVACY) ────────────────────────


def test_scrub_blocks_email(isolated):
    from AI.ai.hive_telemetry import _scrub
    with pytest.raises(ValueError, match="L_PRIVACY"):
        _scrub({"data": "contact alice@example.com"})


def test_scrub_blocks_phone(isolated):
    from AI.ai.hive_telemetry import _scrub
    with pytest.raises(ValueError, match="L_PRIVACY"):
        _scrub("call +995579095713")


def test_scrub_blocks_user_path(isolated):
    from AI.ai.hive_telemetry import _scrub
    with pytest.raises(ValueError, match="L_PRIVACY"):
        _scrub("path /home/oem/Desktop/secret")


def test_scrub_blocks_full_name(isolated):
    from AI.ai.hive_telemetry import _scrub
    with pytest.raises(ValueError, match="L_PRIVACY"):
        _scrub("from Jaba Tkemaladze")


def test_scrub_blocks_pmid(isolated):
    from AI.ai.hive_telemetry import _scrub
    with pytest.raises(ValueError, match="L_PRIVACY"):
        _scrub("citation PMID 36583780")


def test_scrub_blocks_doi(isolated):
    from AI.ai.hive_telemetry import _scrub
    with pytest.raises(ValueError, match="L_PRIVACY"):
        _scrub("see 10.5281/zenodo.12345")


def test_scrub_passes_clean(isolated):
    from AI.ai.hive_telemetry import _scrub
    out = _scrub({"counts": {"a": 1, "b": 2}, "tags": ["fast", "ok"]})
    assert out["counts"]["a"] == 1


def test_scrub_recurses_nested(isolated):
    from AI.ai.hive_telemetry import _scrub
    with pytest.raises(ValueError):
        _scrub({"deep": {"deeper": [{"leaked": "user@bad.com"}]}})


# ── contribution payload shape ─────────────────────────────────


def test_contribution_has_required_keys(isolated):
    from AI.ai.hive_telemetry import contribution
    p = contribution()
    for key in ("v", "ts", "worker_id", "ledger", "prompt",
                "skills", "reflexion", "suppressions", "system"):
        assert key in p


def test_contribution_empty_when_no_data(isolated):
    from AI.ai.hive_telemetry import contribution
    p = contribution()
    assert p["ledger"]["n_runs"] == 0


def test_contribution_includes_real_metrics_when_available(isolated):
    from AI.ai.diagnostic_ledger import record
    record(model="m", grade="B", n_refs=10, n_with_line=8, crit=1)
    record(model="m", grade="C", n_refs=10, n_with_line=5, crit=2)
    from AI.ai.hive_telemetry import contribution
    p = contribution()
    assert p["ledger"]["n_runs"] == 2
    assert 0 < p["ledger"]["avg_compliance"] < 1


def test_contribution_does_not_leak_report_paths(isolated, tmp_path):
    """Even if ledger has report_path with /home/<user>/, the payload
    must NOT contain it."""
    from AI.ai.diagnostic_ledger import record
    record(model="m", grade="B", n_refs=1, n_with_line=1,
           report_path=str(tmp_path / "user_secret.md"))
    from AI.ai.hive_telemetry import contribution
    p = contribution()
    blob = json.dumps(p)
    assert "user_secret" not in blob
    assert str(tmp_path) not in blob


def test_contribution_blocked_if_pii_in_signal(isolated, monkeypatch):
    """If a signal builder accidentally returns PII, _scrub must
    raise — overall contribution() should propagate."""
    import AI.ai.hive_telemetry as ht
    monkeypatch.setattr(ht, "_system_signal",
                         lambda: {"name": "Jaba Tkemaladze"})
    with pytest.raises(ValueError, match="L_PRIVACY"):
        ht.contribution()


def test_worker_id_stable(isolated):
    from AI.ai.hive_telemetry import _worker_id
    a = _worker_id()
    b = _worker_id()
    assert a == b
    assert len(a) == 16


def test_worker_id_different_per_install(isolated, tmp_path, monkeypatch):
    """If salt file is different, worker_id is different even on same
    hostname. Simulates a fresh install on another machine."""
    from AI.ai.hive_telemetry import _worker_id
    a = _worker_id()
    # Wipe salt; force regeneration.
    import shutil
    shutil.rmtree(tmp_path / "home" / ".cache" / "aim", ignore_errors=True)
    b = _worker_id()
    assert a != b


# ── contribute() send path ─────────────────────────────────────


def test_contribute_dry_run_no_send(isolated):
    from AI.ai.hive_telemetry import contribute
    res = contribute(dry_run=True)
    assert res.sent is False
    assert "dry_run" in " ".join(res.notes)
    assert res.payload  # built but not sent


def test_contribute_no_url_skips(isolated):
    from AI.ai.hive_telemetry import contribute
    res = contribute(dry_run=False)
    assert res.sent is False
    assert any("queen URL" in n for n in res.notes)


def test_contribute_sends_when_url_set(isolated, monkeypatch):
    import AI.ai.hive_telemetry as ht

    class FakeResp:
        status_code = 200
        def raise_for_status(self): pass
        def json(self): return {"ok": True, "n_received": 1}

    captured = {}
    class FakeHttpx:
        @staticmethod
        def post(url, json, timeout):
            captured["url"] = url
            captured["payload"] = json
            return FakeResp()

    monkeypatch.setattr("httpx.post", FakeHttpx.post)

    res = ht.contribute(queen_url="https://queen.example.com")
    assert res.sent is True
    assert captured["url"].endswith("/v1/hive/contribute")
    assert "worker_id" in captured["payload"]


def test_contribute_handles_http_error(isolated, monkeypatch):
    def fake_post(url, json, timeout):
        raise RuntimeError("503 Service Unavailable")
    monkeypatch.setattr("httpx.post", fake_post)
    from AI.ai.hive_telemetry import contribute
    res = contribute(queen_url="https://queen.example.com")
    assert res.sent is False
    assert any("503" in n for n in res.notes)


# ── preview ────────────────────────────────────────────────────


def test_preview_returns_valid_json(isolated):
    from AI.ai.hive_telemetry import preview
    text = preview()
    payload = json.loads(text)
    assert payload["v"] == 1


def test_summary_renders(isolated):
    from AI.ai.hive_telemetry import summary
    s = summary()
    assert "Hive telemetry" in s
    assert "dry-run" in s
