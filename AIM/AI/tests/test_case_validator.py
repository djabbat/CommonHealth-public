"""AI/tests/test_case_validator.py — CV1 (2026-05-04)."""
from __future__ import annotations

import textwrap

import pytest


@pytest.fixture
def isolated(tmp_path, monkeypatch):
    cases = tmp_path / "cases"
    cases.mkdir()
    monkeypatch.setenv("AIM_EVAL_CASES_DIR", str(cases))
    import importlib, sys
    if "AI.ai.case_validator" in sys.modules:
        importlib.reload(sys.modules["AI.ai.case_validator"])
    return cases


def write(setup, name, body):
    p = setup / f"{name}.yaml"
    p.write_text(textwrap.dedent(body).lstrip(), encoding="utf-8")
    return p


# ── _validate_doc ────────────────────────────────────────────────


def test_validate_doc_complete():
    from AI.ai.case_validator import _validate_doc
    doc = {"id": "c1", "task": "do x",
           "rubrics": {"min_length": 1}}
    assert _validate_doc(doc) == []


def test_validate_doc_non_mapping():
    from AI.ai.case_validator import _validate_doc
    issues = _validate_doc(["not", "a", "dict"])
    assert any("mapping" in i for i in issues)


def test_validate_doc_missing_keys():
    from AI.ai.case_validator import _validate_doc
    issues = _validate_doc({"id": "c"})
    assert any("task" in i for i in issues)
    assert any("rubrics" in i for i in issues)


def test_validate_doc_empty_id():
    from AI.ai.case_validator import _validate_doc
    issues = _validate_doc({"id": "", "task": "x", "rubrics": {"min_length": 0}})
    assert any("`id`" in i and "empty" in i for i in issues)


def test_validate_doc_unknown_rubric():
    from AI.ai.case_validator import _validate_doc
    issues = _validate_doc({
        "id": "c", "task": "x",
        "rubrics": {"unknown_key": True, "min_length": 1},
    })
    assert any("unknown rubric" in i for i in issues)


def test_validate_doc_rubric_type_check():
    from AI.ai.case_validator import _validate_doc
    issues = _validate_doc({
        "id": "c", "task": "x",
        "rubrics": {"contains_all": "not-a-list", "min_length": 1},
    })
    assert any("contains_all" in i for i in issues)


def test_validate_doc_min_max_swap():
    from AI.ai.case_validator import _validate_doc
    issues = _validate_doc({
        "id": "c", "task": "x",
        "rubrics": {"min_length": 100, "max_length": 50},
    })
    assert any("min_length > max_length" in i for i in issues)


def test_validate_doc_empty_rubrics():
    from AI.ai.case_validator import _validate_doc
    issues = _validate_doc({"id": "c", "task": "x", "rubrics": {}})
    assert any("rubrics" in i.lower() and "empty" in i for i in issues)


def test_validate_doc_tags_must_be_list():
    from AI.ai.case_validator import _validate_doc
    issues = _validate_doc({"id": "c", "task": "x",
                              "rubrics": {"min_length": 1},
                              "tags": "not-list"})
    assert any("tags" in i for i in issues)


# ── validate_one ─────────────────────────────────────────────────


def test_validate_one_clean(isolated):
    p = write(isolated, "c1", """
        id: c1
        task: do something
        rubrics:
          min_length: 1
    """)
    from AI.ai.case_validator import validate_one
    s = validate_one(p)
    assert s.ok is True
    assert s.case_id == "c1"


def test_validate_one_yaml_error(isolated):
    p = isolated / "broken.yaml"
    p.write_text("id: c1\n  task: badly: indented:")
    from AI.ai.case_validator import validate_one
    s = validate_one(p)
    assert s.ok is False
    assert any("yaml parse" in i for i in s.issues)


def test_validate_one_missing_file(isolated):
    from AI.ai.case_validator import validate_one
    s = validate_one(isolated / "nope.yaml")
    assert s.ok is False
    assert any("does not exist" in i for i in s.issues)


# ── validate_dir / Report ───────────────────────────────────────


def test_validate_dir_empty(isolated):
    from AI.ai.case_validator import validate_dir
    r = validate_dir()
    assert r.n_cases == 0
    assert r.all_ok is True


def test_validate_dir_mixed(isolated):
    write(isolated, "good", """
        id: good
        task: x
        rubrics:
          min_length: 1
    """)
    write(isolated, "bad", """
        id: bad
        task: x
    """)   # missing rubrics
    from AI.ai.case_validator import validate_dir
    r = validate_dir()
    assert r.n_cases == 2
    assert r.n_ok == 1
    assert r.n_failed == 1
    assert r.all_ok is False


def test_validate_dir_uses_explicit_path(tmp_path):
    cases = tmp_path / "explicit"
    cases.mkdir()
    (cases / "c.yaml").write_text(
        "id: c\ntask: x\nrubrics:\n  min_length: 1\n", encoding="utf-8")
    from AI.ai.case_validator import validate_dir
    r = validate_dir(cases)
    assert r.n_cases == 1
    assert r.all_ok is True


def test_validate_dir_handles_missing_dir(tmp_path):
    from AI.ai.case_validator import validate_dir
    r = validate_dir(tmp_path / "no-such-dir")
    assert r.n_cases == 0


# ── summary ─────────────────────────────────────────────────────


def test_summary_empty(isolated):
    from AI.ai.case_validator import summary
    assert "no eval cases" in summary()


def test_summary_all_ok(isolated):
    write(isolated, "c", """
        id: c
        task: x
        rubrics:
          min_length: 1
    """)
    from AI.ai.case_validator import summary
    assert "all cases pass" in summary()


def test_summary_lists_failures(isolated):
    write(isolated, "broken", """
        id: broken
        task: x
    """)   # missing rubrics
    from AI.ai.case_validator import summary
    s = summary()
    assert "broken.yaml" in s
    assert "rubrics" in s


# ── round-trip with FE1 ─────────────────────────────────────────


def test_fe1_emitted_cases_validate_clean(isolated):
    """Cases produced by `findings_to_evals.write_cases` must pass
    validate_dir without issues."""
    from AI.ai.findings_to_evals import write_cases
    written = write_cases(["agents/x.py:1", "AI/ai/y.py:42"])
    assert len(written) == 2
    from AI.ai.case_validator import validate_dir
    r = validate_dir()
    assert r.all_ok is True
    assert r.n_cases == 2
