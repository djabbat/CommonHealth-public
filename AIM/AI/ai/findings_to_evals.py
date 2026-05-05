"""AI/ai/findings_to_evals.py — FE1 (2026-05-04).

Convert shared findings (file:line refs) from the diagnostic pipeline
into eval cases that codify those concerns as regression checks.

Why: today shared_findings are *noticed* (S13 stable_run), *triaged*
(S14 fix_planner), and *trended* (DG1 ledger), but never stored as
eval gates. After a fix lands, we don't have a regression case that
trips when the same bug returns. This module closes that gap.

Public API:
    case_from_finding(ref) -> CaseSpec
    generate_cases(refs) -> list[CaseSpec]
    write_cases(refs, *, dest=None) -> list[Path]
"""
from __future__ import annotations

import dataclasses
import json
import logging
import os
import re
from pathlib import Path
from typing import Iterable, Optional

log = logging.getLogger("ai.findings_to_evals")

_REF_RE = re.compile(r"^([\w./_-]+\.py)(?::(\d+))?$")


@dataclasses.dataclass
class CaseSpec:
    id: str
    task: str
    rubrics: dict
    tags: list[str]


def _slug(ref: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", ref.lower()).strip("-")


def _extract_path(ref: str) -> Optional[tuple[str, Optional[int]]]:
    m = _REF_RE.match(ref.strip().lstrip("./"))
    if not m:
        return None
    line = int(m.group(2)) if m.group(2) else None
    return (m.group(1), line)


def case_from_finding(ref: str) -> Optional[CaseSpec]:
    """Build a CaseSpec for a single finding ref. Returns None if the
    ref is not a parseable file:line reference."""
    parsed = _extract_path(ref)
    if parsed is None:
        return None
    path, line = parsed
    suffix = f"-l{line}" if line else ""
    cid = f"regr-{_slug(path)}{suffix}"

    line_part = f" at line {line}" if line else ""
    task = (f"Audit `{path}`{line_part}: identify the regression that "
            f"a previous self-diagnostic flagged here, and propose the "
            f"smallest patch that closes it without breaking adjacent "
            f"behaviour.")

    rubrics = {
        # The auditor must reference the path + (if present) the line.
        "contains_all": [path] + ([str(line)] if line else []),
        # Some signal the model thought through it.
        "min_length": 200,
        # Forbid handwave — must be specific.
        "forbid_any": ["probably", "should be fine", "looks ok"],
    }

    tags = ["regression", "from-diagnostic"]
    if "/tests/" in path or path.startswith("tests/"):
        tags.append("test-gap")
    elif "/AI/ai/" in path or path.startswith("AI/ai/"):
        tags.append("ai-subproject")
    elif path.startswith("agents/"):
        tags.append("agents-runtime")

    return CaseSpec(id=cid, task=task, rubrics=rubrics, tags=tags)


def generate_cases(refs: Iterable[str]) -> list[CaseSpec]:
    """Build CaseSpecs for every parseable ref. Skips refs that don't
    look like file paths (e.g. URLs, free-text)."""
    out: list[CaseSpec] = []
    seen_ids: set[str] = set()
    for ref in refs:
        spec = case_from_finding(ref)
        if spec is None or spec.id in seen_ids:
            continue
        seen_ids.add(spec.id)
        out.append(spec)
    return out


def _cases_dir(dest: Optional[Path] = None) -> Path:
    if dest is not None:
        return Path(dest)
    env = os.environ.get("AIM_EVAL_CASES_DIR")
    if env:
        return Path(env)
    return Path.home() / ".cache" / "aim" / "eval_cases"


def _yaml_dump(spec: CaseSpec) -> str:
    """Tiny stdlib-only YAML emitter (we accept the shape that
    `agents/evals.py` understands)."""
    parts: list[str] = [
        f"id: {spec.id}",
        f"task: |",
    ]
    for line in spec.task.splitlines() or [spec.task]:
        parts.append(f"  {line}")
    parts.append("rubrics:")
    for k, v in spec.rubrics.items():
        if isinstance(v, list):
            inner = ", ".join(json.dumps(x) for x in v)
            parts.append(f"  {k}: [{inner}]")
        else:
            parts.append(f"  {k}: {json.dumps(v)}")
    parts.append("tags: [" + ", ".join(json.dumps(t) for t in spec.tags) + "]")
    return "\n".join(parts) + "\n"


def write_cases(refs: Iterable[str], *,
                 dest: Optional[Path] = None,
                 overwrite: bool = False) -> list[Path]:
    """Write one yaml file per generated case to `dest` (default
    AIM_EVAL_CASES_DIR). Returns the list of paths actually written.
    By default skips files that already exist (idempotent)."""
    target = _cases_dir(dest)
    target.mkdir(parents=True, exist_ok=True)
    written: list[Path] = []
    for spec in generate_cases(refs):
        p = target / f"{spec.id}.yaml"
        if p.exists() and not overwrite:
            continue
        p.write_text(_yaml_dump(spec), encoding="utf-8")
        written.append(p)
    return written


def summary(refs: Iterable[str]) -> str:
    specs = generate_cases(refs)
    if not specs:
        return "(no eval cases generated — refs were unparseable)"
    return (f"📋 Generated {len(specs)} regression eval cases\n"
            + "\n".join(f"  • {s.id}" for s in specs[:15])
            + (f"\n  (+{len(specs) - 15} more)"
               if len(specs) > 15 else ""))
