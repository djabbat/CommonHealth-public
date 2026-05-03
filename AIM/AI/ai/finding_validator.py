"""AI/ai/finding_validator.py — FV1 (2026-05-04).

Heuristic auto-validator for diagnostic findings. The DeepSeek runs
have produced ~7% signal / 93% noise (per 2026-05-03 audit). Most
false positives are claims about code that's plainly contradicted by
the code itself: "no UNIQUE constraint" when CREATE UNIQUE INDEX is
literally on the page; "returns None" when the function has a typed
return annotation.

This module reads each finding's claim, locates the cited file, and
runs cheap pattern checks. If the claim is contradicted by the code,
the finding is auto-flagged FALSE. Otherwise UNVERIFIED (we don't
have enough data to confirm — pass to human).

Public API:
    classify(claim_text, file_path) -> Verdict
    audit_report(report_text) -> AuditReport
    summary(report_text) -> str
"""
from __future__ import annotations

import dataclasses
import logging
import re
from pathlib import Path
from typing import Iterable, Optional

log = logging.getLogger("ai.finding_validator")


@dataclasses.dataclass
class Verdict:
    status: str          # "false_positive" | "unverified" | "true"
    rule: str            # which rule fired
    evidence: str        # short note explaining the verdict


@dataclasses.dataclass
class FindingAudit:
    excerpt: str         # first 120 chars of finding text
    file_ref: Optional[str]
    verdict: Verdict


@dataclasses.dataclass
class AuditReport:
    n_findings: int
    n_false: int
    n_unverified: int
    n_true: int
    audits: list[FindingAudit]


# Patterns that contradict specific claim shapes
_CLAIM_RULES: list[tuple[re.Pattern[str], re.Pattern[str], str, str]] = [
    # Claim: "no UNIQUE constraint" / "no CREATE TABLE" / "no PRIMARY KEY"
    (
        re.compile(r"no\s+(UNIQUE|PRIMARY KEY|CREATE TABLE)", re.I),
        re.compile(r"CREATE\s+(?:UNIQUE\s+)?(?:INDEX|TABLE)|UNIQUE\s+INDEX|PRIMARY\s+KEY", re.I),
        "claim_negates_existing_sql",
        "claim says SQL artifact missing, but file contains it",
    ),
    # Claim: "returns None implicitly" / "no return type"
    (
        re.compile(r"returns\s+None\s+implicitly|no\s+return\s+type|inconsistent\s+return\s+type",
                    re.I),
        re.compile(r"->\s*(?:Optional\[)?(?:list|dict|set|tuple|str|int|float|bool|Path|"
                    r"[A-Z]\w+)", re.I),
        "claim_negates_typed_return",
        "claim says return type missing, but function has -> annotation",
    ),
    # Claim: "crashes on missing dir/file" / "no FileNotFoundError handling"
    (
        re.compile(r"crashes?\s+(?:on|with)\s+(?:missing|FileNotFoundError|absent)|"
                    r"no\s+FileNotFoundError\s+handling|"
                    r"production\s+crash\s+on\s+missing", re.I),
        re.compile(r"if\s+not\s+\S+\.exists\(\)|except\s+\(?FileNotFoundError"
                    r"|except\s+OSError|errors=[\"']replace[\"']", re.I),
        "claim_negates_existence_guard",
        "claim says missing-file crashes, but file has explicit exists() / "
        "OSError guard",
    ),
    # Claim: "no citation_guard" / "no verify_no_fabricated"
    (
        re.compile(r"no\s+citation_guard|no\s+verify_no_fabricated|"
                    r"unverified\s+emit", re.I),
        re.compile(r"citation_guard|_verify_no_fabricated_citations|"
                    r"verify\(strict=True\)", re.I),
        "claim_negates_citation_guard",
        "claim says citation guard missing, but file imports / calls it",
    ),
    # Claim: "no concurrency lock" / "no thread safety"
    (
        re.compile(r"no\s+(?:thread\s+)?lock|no\s+thread\s+safety|race\s+condition", re.I),
        re.compile(r"threading\.RLock|threading\.Lock|with\s+_LOCK", re.I),
        "claim_negates_lock",
        "claim says no lock, but file uses threading.RLock/Lock",
    ),
]


def _file_matches(content: str, rebuilt_pattern: re.Pattern[str]) -> bool:
    return bool(rebuilt_pattern.search(content))


def classify(claim_text: str, file_path: Path) -> Verdict:
    """Run cheap rule checks against the cited file. Returns
    `false_positive` if a contradiction rule fires, else `unverified`."""
    if not file_path.exists():
        return Verdict(
            status="unverified", rule="no_file",
            evidence=f"file not found: {file_path}",
        )
    try:
        content = file_path.read_text(encoding="utf-8", errors="replace")
    except OSError as e:
        return Verdict(
            status="unverified", rule="read_error",
            evidence=f"{type(e).__name__}: {e}",
        )
    for claim_re, contradict_re, rule_name, evidence in _CLAIM_RULES:
        if claim_re.search(claim_text) and contradict_re.search(content):
            return Verdict(
                status="false_positive", rule=rule_name, evidence=evidence,
            )
    return Verdict(
        status="unverified", rule="no_match",
        evidence="no rule fired; claim cannot be auto-rejected",
    )


# ── parsing markdown report into individual findings ────────────


_FINDING_LINE_RE = re.compile(
    r"\*\*`?([\w./_-]+\.py)`?\*\*",
)
_BACKTICK_FILE_RE = re.compile(r"`([\w./_-]+\.py)(?::\d+)?(?::\w+)?`")


def _extract_file_ref(line: str) -> Optional[str]:
    m = _FINDING_LINE_RE.search(line)
    if m:
        return m.group(1)
    m = _BACKTICK_FILE_RE.search(line)
    if m:
        return m.group(1)
    return None


def _split_into_findings(report_text: str) -> Iterable[str]:
    """Pick out lines that look like severity-tagged findings."""
    sev_re = re.compile(r"→\s*\*\*(crit|high|med|low)\*\*", re.I)
    for line in report_text.splitlines():
        if sev_re.search(line):
            yield line.strip()


def audit_report(report_text: str,
                  *, repo_root: Optional[Path] = None) -> AuditReport:
    if repo_root is None:
        from AI.ai.run_self_diagnostic import project_root
        repo_root = project_root()
    audits: list[FindingAudit] = []
    n_false = 0
    n_unverified = 0
    n_true = 0
    for line in _split_into_findings(report_text):
        ref = _extract_file_ref(line)
        if ref is None:
            audits.append(FindingAudit(
                excerpt=line[:120], file_ref=None,
                verdict=Verdict(status="unverified", rule="no_file_ref",
                                  evidence="line had severity but no file ref"),
            ))
            n_unverified += 1
            continue
        # Resolve to actual path. The model often emits bare module name
        # (`distillation_tracker.py`) — try in AI/ai/ first, then agents/.
        candidates = [
            repo_root / ref,
            repo_root / "AI" / "ai" / ref.split("/")[-1],
            repo_root / "agents" / ref.split("/")[-1],
        ]
        path = next((c for c in candidates if c.exists()), candidates[0])
        v = classify(line, path)
        audits.append(FindingAudit(excerpt=line[:120], file_ref=ref,
                                     verdict=v))
        if v.status == "false_positive":
            n_false += 1
        elif v.status == "true":
            n_true += 1
        else:
            n_unverified += 1
    return AuditReport(
        n_findings=len(audits),
        n_false=n_false,
        n_unverified=n_unverified,
        n_true=n_true,
        audits=audits,
    )


def summary(report_text: str,
             *, repo_root: Optional[Path] = None) -> str:
    a = audit_report(report_text, repo_root=repo_root)
    if a.n_findings == 0:
        return "(no severity-tagged findings in report)"
    parts = [
        f"🔍 Finding validator — {a.n_findings} findings",
        f"  ❌ false positive: {a.n_false}",
        f"  ❓ unverified:     {a.n_unverified}",
        f"  ✅ true:           {a.n_true}",
    ]
    if a.n_false:
        parts.append("\n False-positive examples:")
        for au in a.audits:
            if au.verdict.status == "false_positive":
                parts.append(f"  • [{au.file_ref}] {au.verdict.rule}: "
                              f"{au.verdict.evidence}")
                if len([1 for x in a.audits
                         if x.verdict.status == "false_positive"]) > 5:
                    break
    return "\n".join(parts)
