"""AI/tests/test_run_self_diagnostic.py — closes last high-severity
finding from the D-grade audit (2026-05-03)."""
from __future__ import annotations

import datetime as dt
import sys

import pytest


@pytest.fixture
def isolated(tmp_path, monkeypatch):
    monkeypatch.setenv("AIM_HOME", str(tmp_path / "home"))
    monkeypatch.setenv("AI_DIAGNOSTIC_DB", str(tmp_path / "dl.db"))
    import importlib
    for mod in ("AI.ai.run_self_diagnostic", "AI.ai.diagnostic_ledger",
                "AI.ai.safety_gate"):
        if mod in sys.modules:
            importlib.reload(sys.modules[mod])
    return tmp_path


# ── _api_key resolution ─────────────────────────────────────────


def test_api_key_from_env(isolated, monkeypatch):
    monkeypatch.setenv("DEEPSEEK_API_KEY", "sk-from-env")
    from AI.ai.run_self_diagnostic import _api_key
    assert _api_key() == "sk-from-env"


def test_api_key_from_aim_env_file(isolated, tmp_path, monkeypatch):
    monkeypatch.delenv("DEEPSEEK_API_KEY", raising=False)
    aim_env = tmp_path / ".aim_env"
    aim_env.write_text(
        "# comment\n"
        "OTHER=foo\n"
        "DEEPSEEK_API_KEY=sk-from-file\n"
        "ANOTHER=bar\n"
    )
    monkeypatch.setattr("pathlib.Path.home", lambda: tmp_path)
    import importlib, AI.ai.run_self_diagnostic as r
    importlib.reload(r)
    assert r._api_key() == "sk-from-file"


def test_api_key_strips_quotes(isolated, tmp_path, monkeypatch):
    monkeypatch.delenv("DEEPSEEK_API_KEY", raising=False)
    aim_env = tmp_path / ".aim_env"
    aim_env.write_text("export DEEPSEEK_API_KEY='sk-quoted'\n")
    monkeypatch.setattr("pathlib.Path.home", lambda: tmp_path)
    import importlib, AI.ai.run_self_diagnostic as r
    importlib.reload(r)
    assert r._api_key() == "sk-quoted"


def test_api_key_missing(isolated, tmp_path, monkeypatch):
    monkeypatch.delenv("DEEPSEEK_API_KEY", raising=False)
    monkeypatch.setattr("pathlib.Path.home", lambda: tmp_path)
    import importlib, AI.ai.run_self_diagnostic as r
    importlib.reload(r)
    assert r._api_key() is None


# ── _post_deepseek (with stubbed httpx) ─────────────────────────


def test_post_deepseek_raises_without_key(isolated, monkeypatch, tmp_path):
    monkeypatch.delenv("DEEPSEEK_API_KEY", raising=False)
    monkeypatch.setattr("pathlib.Path.home", lambda: tmp_path)
    import importlib, AI.ai.run_self_diagnostic as r
    importlib.reload(r)
    with pytest.raises(RuntimeError, match="DEEPSEEK_API_KEY"):
        r._post_deepseek("hi", "deepseek-chat")


def test_post_deepseek_happy_path(isolated, monkeypatch):
    monkeypatch.setenv("DEEPSEEK_API_KEY", "sk-test")

    class FakeResp:
        def raise_for_status(self): pass
        def json(self):
            return {"choices": [{"message": {"content": "REPORT BODY"}}]}

    captured = {}

    class FakeClient:
        def __init__(self, *a, **kw): pass
        def __enter__(self): return self
        def __exit__(self, *a): return False
        def post(self, url, json, headers):
            captured["url"] = url
            captured["model"] = json["model"]
            captured["headers"] = headers
            return FakeResp()

    import httpx
    monkeypatch.setattr(httpx, "Client", FakeClient)
    import importlib, AI.ai.run_self_diagnostic as r
    importlib.reload(r)
    out = r._post_deepseek("PROMPT", "deepseek-chat")
    assert out == "REPORT BODY"
    assert captured["model"] == "deepseek-chat"
    assert captured["headers"]["Authorization"].startswith("Bearer ")


def test_post_deepseek_propagates_http_errors(isolated, monkeypatch):
    monkeypatch.setenv("DEEPSEEK_API_KEY", "sk-x")

    class FakeResp:
        def raise_for_status(self):
            raise RuntimeError("503 Service Unavailable")
        def json(self): return {}

    class FakeClient:
        def __init__(self, *a, **kw): pass
        def __enter__(self): return self
        def __exit__(self, *a): return False
        def post(self, *a, **kw): return FakeResp()

    import httpx
    monkeypatch.setattr(httpx, "Client", FakeClient)
    import importlib, AI.ai.run_self_diagnostic as r
    importlib.reload(r)
    with pytest.raises(RuntimeError, match="503"):
        r._post_deepseek("p", "deepseek-chat")


# ── output path ─────────────────────────────────────────────────


def test_output_path_uses_today(isolated, monkeypatch):
    import AI.ai.run_self_diagnostic as r
    p = r._output_path(today=dt.date(2026, 1, 15))
    assert p.name == "self_diag_2026-01-15.md"
    assert p.parent.name == "artifacts"


def test_output_path_creates_artifacts_dir(isolated, monkeypatch, tmp_path):
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    p = r._output_path(today=dt.date(2026, 1, 15))
    assert p.parent.exists()


# ── run() pipeline ──────────────────────────────────────────────


def test_run_writes_report(isolated, monkeypatch, tmp_path):
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    monkeypatch.setattr(r, "_post_deepseek",
                        lambda prompt, model, **kw: f"REPORT for {model}")
    # Avoid touching the real AI/CLAUDE.md prompt file: stub build_prompt.
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "FAKE PROMPT")
    out = r.run(model="deepseek-chat", verbose=False, skip_safety_gate=True)
    assert out.exists()
    assert out.read_text() == "REPORT for deepseek-chat"


def test_run_falls_back_to_chat_when_reasoner_fails(
    isolated, monkeypatch, tmp_path, capsys,
):
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)

    calls: list[str] = []

    def post(prompt, model, **kw):
        calls.append(model)
        if model == "deepseek-reasoner":
            raise RuntimeError("rate-limited")
        return "FALLBACK BODY"

    monkeypatch.setattr(r, "_post_deepseek", post)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "PROMPT")
    out = r.run(model="deepseek-reasoner", verbose=True, skip_safety_gate=True)
    assert out.read_text() == "FALLBACK BODY"
    assert calls == ["deepseek-reasoner", "deepseek-chat"]


def test_run_propagates_when_non_reasoner_fails(
    isolated, monkeypatch, tmp_path,
):
    """If the caller picked a specific non-reasoner model and it fails,
    we DON'T silently retry on a different model — we propagate."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    monkeypatch.setattr(r, "_post_deepseek",
                        lambda *a, **kw: (_ for _ in ()).throw(
                            RuntimeError("explicit fail")))
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "PROMPT")
    with pytest.raises(RuntimeError, match="explicit fail"):
        r.run(model="deepseek-chat", verbose=False, skip_safety_gate=True)


# ── line compliance signal ──────────────────────────────────────


def test_run_warns_on_low_line_compliance(
    isolated, monkeypatch, tmp_path, capsys,
):
    """Verbose run should print a warning when the model returned refs
    without `:line` (ignoring L_VERIFIABILITY)."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    bad_report = ("# Audit\n\nGrade: D\n\n"
                  "Found bug at `agents/x.py` and `agents/y.py`.\n")
    monkeypatch.setattr(r, "_post_deepseek",
                        lambda *a, **kw: bad_report)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "P")
    r.run(model="deepseek-chat", verbose=True, compliance_retry=False, skip_safety_gate=True)
    out = capsys.readouterr().out
    assert "line_compliance=0%" in out
    assert "low line compliance" in out


def test_run_quiet_when_line_compliance_high(
    isolated, monkeypatch, tmp_path, capsys,
):
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    good = ("# Audit\n\nGrade: B\n\n"
            "Issue at `agents/x.py:10` and `agents/y.py:42`.\n")
    monkeypatch.setattr(r, "_post_deepseek", lambda *a, **kw: good)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "P")
    r.run(model="deepseek-chat", verbose=True, compliance_retry=False, skip_safety_gate=True)
    out = capsys.readouterr().out
    assert "line_compliance=100%" in out
    assert "low line compliance" not in out


# ── compliance retry ────────────────────────────────────────────


def test_run_retries_on_low_compliance_and_uses_better_response(
    isolated, monkeypatch, tmp_path, capsys,
):
    """First response: 0% compliance → corrective retry. Second
    response: 100% → run() saves the BETTER one."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)

    calls: list[str] = []
    def post(prompt, model, **kw):
        calls.append(prompt)
        if len(calls) == 1:
            return "Bug at `agents/x.py` and `agents/y.py`."   # 0%
        return "Bug at `agents/x.py:1` and `agents/y.py:2`."   # 100%

    monkeypatch.setattr(r, "_post_deepseek", post)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "BASE PROMPT")
    out_path = r.run(model="deepseek-chat", verbose=True, skip_safety_gate=True)
    saved = out_path.read_text()
    assert ":1" in saved and ":2" in saved
    assert len(calls) == 2
    # The retry prompt must include the corrective suffix.
    assert "REPEATED INSTRUCTION" in calls[1]
    out = capsys.readouterr().out
    assert "retry compliance=100%" in out


def test_run_retry_keeps_first_when_retry_no_better(
    isolated, monkeypatch, tmp_path, capsys,
):
    """If retry doesn't improve compliance, keep the first response."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    bad = "Bug at `agents/x.py`."  # 0%
    monkeypatch.setattr(r, "_post_deepseek", lambda *a, **kw: bad)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "P")
    out_path = r.run(model="deepseek-chat", verbose=True, skip_safety_gate=True)
    assert out_path.read_text() == bad
    assert "did not improve" in capsys.readouterr().out


def test_run_no_retry_when_compliance_already_good(
    isolated, monkeypatch, tmp_path,
):
    """100% compliance on first try → no retry call."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    calls = []
    def post(prompt, model, **kw):
        calls.append(model)
        return "Bug at `agents/x.py:1`."
    monkeypatch.setattr(r, "_post_deepseek", post)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "P")
    r.run(model="deepseek-chat", verbose=False, skip_safety_gate=True)
    assert len(calls) == 1


def test_run_compliance_retry_disabled(
    isolated, monkeypatch, tmp_path,
):
    """compliance_retry=False → never retries, even on 0%."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    calls = []
    def post(prompt, model, **kw):
        calls.append(model)
        return "Bug at `agents/x.py`."  # 0%
    monkeypatch.setattr(r, "_post_deepseek", post)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "P")
    r.run(model="deepseek-chat", verbose=False, compliance_retry=False, skip_safety_gate=True)
    assert len(calls) == 1


def test_run_retry_swallows_retry_exception(
    isolated, monkeypatch, tmp_path, capsys,
):
    """If the retry POST raises, run() keeps the first response."""
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    state = {"i": 0}
    bad = "Bug at `agents/x.py`."
    def post(prompt, model, **kw):
        state["i"] += 1
        if state["i"] == 1:
            return bad
        raise RuntimeError("retry boom")
    monkeypatch.setattr(r, "_post_deepseek", post)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "P")
    out_path = r.run(model="deepseek-chat", verbose=True, skip_safety_gate=True)
    assert out_path.read_text() == bad
    assert "retry failed" in capsys.readouterr().out


# ── cost_monitor integration ────────────────────────────────────


def test_post_deepseek_records_cost(isolated, monkeypatch):
    """Successful DeepSeek call should call cost_monitor.record with
    the API's usage counts."""
    monkeypatch.setenv("DEEPSEEK_API_KEY", "sk-stub")
    captured: dict = {}

    class FakeResponse:
        status_code = 200
        def raise_for_status(self): pass
        def json(self):
            return {
                "choices": [{"message": {"content": "report body"}}],
                "usage": {"prompt_tokens": 1234,
                           "completion_tokens": 567},
            }

    class FakeClient:
        def __init__(self, *a, **kw): pass
        def __enter__(self): return self
        def __exit__(self, *a): return False
        def post(self, *a, **kw): return FakeResponse()

    import sys, agents as _agents_pkg
    fake_cm = type(sys)("agents.cost_monitor")
    def fake_record(**kw):
        captured.update(kw)
    fake_cm.record = fake_record
    monkeypatch.setitem(sys.modules, "agents.cost_monitor", fake_cm)
    monkeypatch.setattr(_agents_pkg, "cost_monitor", fake_cm,
                         raising=False)

    monkeypatch.setattr("httpx.Client", FakeClient)

    import AI.ai.run_self_diagnostic as r
    out = r._post_deepseek("PROMPT", "deepseek-chat")
    assert out == "report body"
    assert captured["model"] == "deepseek-chat"
    assert captured["input_tokens"] == 1234
    assert captured["output_tokens"] == 567
    assert captured["provider"] == "deepseek"


def test_post_deepseek_skips_cost_when_no_usage(isolated, monkeypatch):
    """If DeepSeek doesn't return usage, don't crash."""
    monkeypatch.setenv("DEEPSEEK_API_KEY", "sk-stub")
    called = []

    class FakeResponse:
        def raise_for_status(self): pass
        def json(self):
            return {"choices": [{"message": {"content": "x"}}]}

    class FakeClient:
        def __init__(self, *a, **kw): pass
        def __enter__(self): return self
        def __exit__(self, *a): return False
        def post(self, *a, **kw): return FakeResponse()

    import sys, agents as _agents_pkg
    fake_cm = type(sys)("agents.cost_monitor")
    fake_cm.record = lambda **kw: called.append(kw)
    monkeypatch.setitem(sys.modules, "agents.cost_monitor", fake_cm)
    monkeypatch.setattr(_agents_pkg, "cost_monitor", fake_cm,
                         raising=False)
    monkeypatch.setattr("httpx.Client", FakeClient)

    import AI.ai.run_self_diagnostic as r
    out = r._post_deepseek("P", "deepseek-chat")
    assert out == "x"
    assert called == []   # no record call


def test_post_deepseek_swallows_cost_monitor_error(isolated, monkeypatch):
    """If cost_monitor isn't importable / record() raises, the call
    still returns the response — accounting must never block AI."""
    monkeypatch.setenv("DEEPSEEK_API_KEY", "sk-stub")

    class FakeResponse:
        def raise_for_status(self): pass
        def json(self):
            return {"choices": [{"message": {"content": "ok"}}],
                    "usage": {"prompt_tokens": 10,
                                "completion_tokens": 5}}

    class FakeClient:
        def __init__(self, *a, **kw): pass
        def __enter__(self): return self
        def __exit__(self, *a): return False
        def post(self, *a, **kw): return FakeResponse()

    import sys, agents as _agents_pkg
    fake_cm = type(sys)("agents.cost_monitor")
    def boom(**kw): raise RuntimeError("ledger down")
    fake_cm.record = boom
    monkeypatch.setitem(sys.modules, "agents.cost_monitor", fake_cm)
    monkeypatch.setattr(_agents_pkg, "cost_monitor", fake_cm,
                         raising=False)
    monkeypatch.setattr("httpx.Client", FakeClient)

    import AI.ai.run_self_diagnostic as r
    out = r._post_deepseek("P", "deepseek-chat")
    assert out == "ok"


def test_run_save_false_returns_devnull(isolated, monkeypatch):
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "_post_deepseek",
                        lambda p, m, **kw: "ignored")
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "PROMPT")
    out = r.run(model="deepseek-chat", save=False, verbose=False, skip_safety_gate=True)
    assert str(out) == "/dev/null"


def test_run_verbose_prints_progress(isolated, monkeypatch, tmp_path, capsys):
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    monkeypatch.setattr(r, "_post_deepseek",
                        lambda p, m, **kw: "BODY")
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "PROMPT")
    r.run(model="deepseek-chat", verbose=True, skip_safety_gate=True)
    out = capsys.readouterr().out
    assert "querying DeepSeek" in out
    assert "saved" in out


# ── _main entrypoint ─────────────────────────────────────────────


def test_main_success(isolated, monkeypatch, tmp_path, capsys):
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    monkeypatch.setattr(r, "_post_deepseek",
                        lambda p, m, **kw: "OK")
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "PROMPT")
    monkeypatch.setattr(sys, "argv",
                        ["run_self_diagnostic", "--quiet", "--force"])
    rc = r._main()
    assert rc == 0


def test_main_blocked_by_safety_gate(isolated, monkeypatch, capsys, tmp_path):
    """_main without --force must respect the safety gate. Plant a
    fresh ledger row → cooldown blocks → rc != 0."""
    import datetime as dt
    from AI.ai.diagnostic_ledger import record
    fresh = (dt.datetime.now() - dt.timedelta(minutes=10)).isoformat()
    record(model="m", grade="B", n_refs=1, n_with_line=1, ts=fresh)
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "PROMPT")
    monkeypatch.setattr(sys, "argv", ["run_self_diagnostic", "--quiet"])
    rc = r._main()
    assert rc == 1


def test_main_force_bypasses_gate(isolated, monkeypatch, tmp_path):
    """--force must bypass the cooldown gate."""
    import datetime as dt
    from AI.ai.diagnostic_ledger import record
    fresh = (dt.datetime.now() - dt.timedelta(minutes=10)).isoformat()
    record(model="m", grade="B", n_refs=1, n_with_line=1, ts=fresh)
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "ai_root", lambda: tmp_path)
    monkeypatch.setattr(r, "_post_deepseek", lambda p, m, **kw: "OK")
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "PROMPT")
    monkeypatch.setattr(sys, "argv",
                        ["run_self_diagnostic", "--quiet", "--force"])
    rc = r._main()
    assert rc == 0


def test_main_failure_returns_1(isolated, monkeypatch, capsys):
    import AI.ai.run_self_diagnostic as r
    monkeypatch.setattr(r, "_post_deepseek",
                        lambda *a, **kw: (_ for _ in ()).throw(
                            RuntimeError("API down")))
    monkeypatch.setattr("AI.ai.self_diagnostic.build_prompt",
                        lambda: "PROMPT")
    monkeypatch.setattr(sys, "argv",
                        ["run_self_diagnostic", "--quiet",
                         "--model", "deepseek-chat"])
    rc = r._main()
    err = capsys.readouterr().err
    assert rc == 1
    assert "ERROR" in err
