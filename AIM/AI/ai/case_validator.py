"""AI/ai/case_validator.py — CV1 (2026-05-04).

Validate every yaml case in AIM_EVAL_CASES_DIR — load with PyYAML,
verify required keys, sanity-check rubric shapes. Catches a malformed
auto-generated case (e.g. from FE1) before the eval harness trips on
it during a run.

Public API:
    validate_dir(path=None) -> Report
    validate_one(path) -> CaseStatus
    summary() -> str
"""
from __future__ import annotations

import dataclasses
import logging
import os
from pathlib import Path
from typing import Optional

log = logging.getLogger("ai.case_validator")


@dataclasses.dataclass
class CaseStatus:
    path: Path
    ok: bool
    case_id: Optional[str]
    issues: list[str]


@dataclasses.dataclass
class Report:
    n_cases: int
    n_ok: int
    n_failed: int
    statuses: list[CaseStatus]

    @property
    def all_ok(self) -> bool:
        return self.n_failed == 0


def _cases_dir(dest: Optional[Path] = None) -> Path:
    if dest is not None:
        return Path(dest)
    env = os.environ.get("AIM_EVAL_CASES_DIR")
    if env:
        return Path(env)
    return Path.home() / ".cache" / "aim" / "eval_cases"


_REQUIRED_KEYS = ("id", "task", "rubrics")
_KNOWN_RUBRICS = {
    "min_length", "max_length", "contains_all", "contains_any",
    "forbid_any", "regex_must_match", "regex_must_not_match",
    "json_keys",
}


def _validate_doc(doc: object) -> list[str]:
    issues: list[str] = []
    if not isinstance(doc, dict):
        return [f"top-level must be a mapping, got {type(doc).__name__}"]
    for k in _REQUIRED_KEYS:
        if k not in doc:
            issues.append(f"missing required key: {k!r}")
    cid = doc.get("id")
    if cid is not None and not isinstance(cid, str):
        issues.append("`id` must be a string")
    elif isinstance(cid, str) and not cid.strip():
        issues.append("`id` is empty")
    task = doc.get("task")
    if task is not None and not isinstance(task, str):
        issues.append("`task` must be a string")
    elif isinstance(task, str) and not task.strip():
        issues.append("`task` is empty")
    rubrics = doc.get("rubrics")
    if rubrics is not None and not isinstance(rubrics, dict):
        issues.append("`rubrics` must be a mapping")
    elif isinstance(rubrics, dict):
        if not rubrics:
            issues.append("`rubrics` is empty — every case needs at least one")
        unknown = set(rubrics.keys()) - _KNOWN_RUBRICS
        if unknown:
            issues.append(f"unknown rubric keys: {sorted(unknown)}")
        for k in ("contains_all", "contains_any", "forbid_any"):
            if k in rubrics and not isinstance(rubrics[k], list):
                issues.append(f"rubric {k!r} must be a list")
        for k in ("min_length", "max_length"):
            if k in rubrics and not isinstance(rubrics[k], int):
                issues.append(f"rubric {k!r} must be an int")
        if ("min_length" in rubrics and "max_length" in rubrics
                and isinstance(rubrics["min_length"], int)
                and isinstance(rubrics["max_length"], int)
                and rubrics["min_length"] > rubrics["max_length"]):
            issues.append("min_length > max_length")
    tags = doc.get("tags")
    if tags is not None and not isinstance(tags, list):
        issues.append("`tags` must be a list")
    return issues


def validate_one(path: Path) -> CaseStatus:
    p = Path(path)
    try:
        import yaml
    except ImportError:
        return CaseStatus(path=p, ok=False, case_id=None,
                           issues=["PyYAML not installed"])
    if not p.exists():
        return CaseStatus(path=p, ok=False, case_id=None,
                           issues=["file does not exist"])
    try:
        text = p.read_text(encoding="utf-8")
    except OSError as e:
        return CaseStatus(path=p, ok=False, case_id=None,
                           issues=[f"read failed: {e}"])
    try:
        doc = yaml.safe_load(text)
    except yaml.YAMLError as e:
        return CaseStatus(path=p, ok=False, case_id=None,
                           issues=[f"yaml parse: {e}"])
    issues = _validate_doc(doc)
    cid = doc.get("id") if isinstance(doc, dict) else None
    return CaseStatus(path=p, ok=not issues, case_id=cid, issues=issues)


def validate_dir(path: Optional[Path] = None) -> Report:
    target = _cases_dir(path)
    statuses: list[CaseStatus] = []
    if not target.exists():
        return Report(n_cases=0, n_ok=0, n_failed=0, statuses=[])
    for p in sorted(target.glob("*.yaml")):
        statuses.append(validate_one(p))
    n_ok = sum(1 for s in statuses if s.ok)
    return Report(
        n_cases=len(statuses),
        n_ok=n_ok,
        n_failed=len(statuses) - n_ok,
        statuses=statuses,
    )


def summary(path: Optional[Path] = None) -> str:
    r = validate_dir(path)
    if r.n_cases == 0:
        return "(no eval cases found)"
    head = (f"📋 Case validator — {r.n_cases} cases "
            f"({r.n_ok} ok / {r.n_failed} failed)")
    if r.all_ok:
        return head + "\n  ✅ all cases pass schema check"
    parts = [head]
    for s in r.statuses:
        if s.ok:
            continue
        parts.append(f"  ❌ {s.path.name}  ({s.case_id or '?'})")
        for i in s.issues:
            parts.append(f"      • {i}")
    return "\n".join(parts)
