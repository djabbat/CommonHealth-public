"""AI/ai/gap_detector.py — capability-gap detector (S11, 2026-05-03).

Walks session JSONL logs (`~/.cache/aim/sessions/*.jsonl`) and finds
*tasks where AIM gave up* — the final answer matches one of these
surrender patterns:

    "I cannot help"      "I don't have access to"     "(interrupted)"
    "не могу помочь"     "у меня нет доступа"          "I'm sorry, I"
    "I'm not able"       "outside my capabilities"
    ERROR: prefix when emitted as the FINAL answer (vs intermediate)

For each gap we cluster the originating tasks (Jaccard on key terms),
producing a "missing capability" report:

    - cluster theme (top key terms)
    - n sessions where this gap appeared
    - representative task
    - suggested next step (new tool? prompt patch? domain expansion?)

Public API:
    surrenders(window_days=14) -> list[Surrender]
    gaps(window_days=14, threshold=0.20) -> list[Gap]
    summary(window_days=14) -> str
"""
from __future__ import annotations

import collections
import dataclasses
import datetime as dt
import json
import logging
import os
import re
from pathlib import Path
from typing import Iterable, Optional

log = logging.getLogger("ai.gap_detector")


def sessions_dir() -> Path:
    env = os.environ.get("AIM_SESSIONS_DIR")
    if env:
        return Path(env).expanduser()
    return Path.home() / ".cache" / "aim" / "sessions"


# ── surrender detection ─────────────────────────────────────────


_SURRENDER_PATTERNS = [
    # Broad "I cannot X" / "I can't X" — AI surrender almost always uses these.
    re.compile(r"\bI\s+(?:cannot|can'?t)\b", re.I),
    re.compile(r"\bI\s+(?:don'?t|do\s+not)\s+have\s+access\b", re.I),
    re.compile(r"\bI'?m\s+(?:not\s+able|unable)\b", re.I),
    re.compile(r"\boutside\s+my\s+capabilities\b", re.I),
    re.compile(r"\bI'?m\s+sorry\s+I\b", re.I),
    re.compile(r"\(interrupted\)", re.I),
    # Russian — also broad.
    re.compile(r"\bне\s+могу\b", re.I),
    re.compile(r"\bу\s+меня\s+нет\s+доступа\b", re.I),
    re.compile(r"\bне\s+умею\b", re.I),
]


def _is_surrender(answer: str) -> bool:
    if not isinstance(answer, str):
        return False
    if answer.strip().startswith("ERROR:"):
        return True
    return any(pat.search(answer) for pat in _SURRENDER_PATTERNS)


# ── token helpers (shared with reflexion_cluster) ───────────────


_TOKEN_RE = re.compile(r"[A-Za-zА-Яа-яЁё][\w-]{3,}")
_FILLERS = {"the", "and", "for", "with", "that", "this", "from", "they",
            "your", "have", "should", "would", "must", "make", "more",
            "когда", "если", "также", "может", "очень", "уже", "будет"}


def _tokens(s: str) -> set[str]:
    return {w.lower() for w in _TOKEN_RE.findall(s)
            if w.lower() not in _FILLERS}


def _jaccard(a: set[str], b: set[str]) -> float:
    if not a or not b:
        return 0.0
    return len(a & b) / max(1, len(a | b))


# ── data ─────────────────────────────────────────────────────────


@dataclasses.dataclass
class Surrender:
    session: str
    task: str
    answer: str
    ts: Optional[str]


@dataclasses.dataclass
class Gap:
    theme: list[str]              # top key terms across surrenders
    surrenders: list[Surrender]
    representative: str           # the most representative task
    suggestion: str               # heuristic next-step recommendation

    @property
    def n(self) -> int:
        return len(self.surrenders)


# ── walk session logs ───────────────────────────────────────────


def _events(path: Path,
            cutoff: Optional[dt.datetime] = None) -> Iterable[dict]:
    try:
        with path.open(encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    ev = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if cutoff is not None:
                    ts = ev.get("ts") or ev.get("timestamp")
                    if isinstance(ts, str):
                        try:
                            evt = dt.datetime.fromisoformat(ts)
                        except ValueError:
                            continue
                        if evt < cutoff:
                            continue
                    elif isinstance(ts, (int, float)):
                        if dt.datetime.fromtimestamp(ts) < cutoff:
                            continue
                yield ev
    except OSError:
        return


def surrenders(window_days: int = 14) -> list[Surrender]:
    """Walk every session JSONL and return cases that ended in surrender."""
    sd = sessions_dir()
    if not sd.exists():
        return []
    cutoff = dt.datetime.now() - dt.timedelta(days=window_days)
    out: list[Surrender] = []
    for p in sorted(sd.glob("*.jsonl")):
        # Track the latest "start" task per file, then check final.
        current_task: Optional[str] = None
        current_ts: Optional[str] = None
        for ev in _events(p, cutoff=cutoff):
            kind = ev.get("type")
            if kind == "start":
                current_task = ev.get("task") or current_task
                current_ts = ev.get("ts") or current_ts
            elif kind in ("final", "error"):
                ans = ev.get("answer") or ev.get("error") or ""
                if _is_surrender(ans):
                    out.append(Surrender(
                        session=p.stem,
                        task=str(current_task or "")[:300],
                        answer=str(ans)[:300],
                        ts=current_ts,
                    ))
    return out


# ── cluster surrenders into gaps ────────────────────────────────


def _suggestion_for(theme: list[str], rep: str) -> str:
    if not theme:
        return f"Investigate failures on: {rep[:120]}"
    head = ", ".join(theme[:4])
    # Heuristic next-step: if 'access' / 'permission' in theme → tool gap;
    # 'fact' / 'pubmed' → citation/grounding gap; else prompt patch.
    low = " ".join(theme).lower()
    if any(t in low for t in ("access", "permission", "auth", "право",
                                "доступ")):
        return f"Likely missing tool / scope: {head}. Add MCP server or "\
               "expand bash whitelist."
    if any(t in low for t in ("pubmed", "citation", "doi", "pmid")):
        return f"Citation / grounding gap: {head}. Add literature lookup "\
               "before emit."
    if any(t in low for t in ("language", "translate", "georgian",
                                "грузин")):
        return f"Translation gap: {head}. Wire i18n delegate or DeepSeek."
    return f"Prompt patch candidate: '{head}' — see clusters in S10."


def gaps(window_days: int = 14,
          threshold: float = 0.20,
          *,
          surrender_list: Optional[list["Surrender"]] = None,
          ) -> list[Gap]:
    """Cluster surrenders into capability gaps.

    `surrender_list` lets callers pre-compute / mock surrenders. When
    None we fetch via `surrenders()`. CRIT-3 fix (2026-05-03): we
    explicitly materialise the input as a list before iterating twice
    (once for tokenisation, once for cluster representative selection),
    so callers passing a generator don't trigger StopIteration.
    """
    surr = (list(surrender_list)
            if surrender_list is not None
            else surrenders(window_days=window_days))
    if not surr:
        return []
    items = [(s, _tokens(s.task + " " + s.answer)) for s in surr]
    clusters: list[list[tuple[Surrender, set[str]]]] = []
    for s, toks in items:
        attached = False
        for cl in clusters:
            for _, ct in cl:
                if _jaccard(toks, ct) >= threshold:
                    cl.append((s, toks))
                    attached = True
                    break
            if attached:
                break
        if not attached:
            clusters.append([(s, toks)])

    out: list[Gap] = []
    for cl in clusters:
        from collections import Counter
        ctr: Counter = Counter()
        for _, ts in cl:
            ctr.update(ts)
        keep = max(1, len(cl) // 2)
        theme = [t for t, c in ctr.most_common(20) if c >= keep][:6]
        rep = max((s for s, _ in cl), key=lambda s: len(s.task)).task
        out.append(Gap(
            theme=theme,
            surrenders=[s for s, _ in cl],
            representative=rep,
            suggestion=_suggestion_for(theme, rep),
        ))
    out.sort(key=lambda g: -g.n)
    return out


def summary(window_days: int = 14) -> str:
    g = gaps(window_days=window_days)
    if not g:
        return f"(no capability gaps detected over last {window_days}d)"
    lines = [f"🕳 Capability gaps — {len(g)} clusters / "
             f"{sum(x.n for x in g)} surrenders / last {window_days}d"]
    for cluster in g[:8]:
        theme = ", ".join(cluster.theme[:4]) if cluster.theme else "(no theme)"
        lines.append(f"  • [{cluster.n} surrenders] {theme}")
        lines.append(f"      → {cluster.suggestion[:140]}")
    return "\n".join(lines)
