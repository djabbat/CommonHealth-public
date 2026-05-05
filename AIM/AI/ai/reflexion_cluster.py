"""AI/ai/reflexion_cluster.py — group failure themes (S10, 2026-05-03).

Pulls accumulated reflexion notes from
`agents.reflexion.recent_reflections` (or feedback memory directly)
and clusters them by topic so we can produce ONE targeted prompt
patch per cluster instead of dozens of one-off corrections.

Strategy (deterministic, no LLM needed for clustering):
  1. Tokenise each note (≥4 chars, no fillers).
  2. Compute Jaccard similarity between notes.
  3. Greedy single-link clustering: notes with similarity ≥ threshold
     join the same cluster.
  4. Each cluster gets a "theme" — top-K most-shared key terms.
  5. (optional) Suggest a prompt-patch hint built from those terms.

Output:
  * `Cluster.notes`     — list of note texts
  * `Cluster.theme`     — sorted top key terms
  * `Cluster.suggestion` — short string the user can paste into a
                          prompt as a "remember:" hint

Public API:
    cluster(notes, *, threshold=0.25) -> list[Cluster]
    clusters_from_memory(window_days=180, threshold=0.25) -> list[Cluster]
    summary(threshold=0.25) -> str
"""
from __future__ import annotations

import dataclasses
import datetime as dt
import logging
import re
from pathlib import Path
from typing import Iterable, Optional

log = logging.getLogger("ai.reflexion_cluster")


# ── tokenisation ────────────────────────────────────────────────


_TOKEN_RE = re.compile(r"[A-Za-zА-Яа-яЁё][\w-]{3,}")
_FILLERS = {
    # English
    "the", "and", "for", "with", "that", "this", "from", "they", "their",
    "your", "user", "model", "agent", "must", "should", "after", "into",
    "have", "been", "would", "could", "make", "make", "more", "very",
    "when", "what", "which", "where", "while", "doesn", "don", "didn",
    # Russian
    "когда", "если", "чтобы", "также", "может", "нужно", "очень",
    "будут", "будет", "пока", "потом", "только", "ровно", "будем",
    "уже", "его", "ему", "она", "нам", "наш", "наша", "наше",
}


def _tokens(s: str) -> set[str]:
    return {w.lower() for w in _TOKEN_RE.findall(s)
            if w.lower() not in _FILLERS}


def _jaccard(a: set[str], b: set[str]) -> float:
    if not a or not b:
        return 0.0
    return len(a & b) / max(1, len(a | b))


# ── data ─────────────────────────────────────────────────────────


@dataclasses.dataclass
class Cluster:
    notes: list[str]
    theme: list[str]            # top key terms
    representative: str         # the longest note in the cluster

    @property
    def n(self) -> int:
        return len(self.notes)

    @property
    def suggestion(self) -> str:
        if not self.theme:
            return self.representative[:200]
        terms = ", ".join(self.theme[:5])
        return (f"Remember when handling {terms}: "
                f"{self.representative.strip()[:200]}")


# ── cluster ──────────────────────────────────────────────────────


def cluster(notes: Iterable[str], *,
            threshold: float = 0.25) -> list[Cluster]:
    """Greedy single-link clustering by Jaccard token overlap."""
    items: list[tuple[str, set[str]]] = []
    for n in notes:
        if not isinstance(n, str):
            continue
        n = n.strip()
        if len(n) < 20:
            continue
        toks = _tokens(n)
        if not toks:
            continue
        items.append((n, toks))

    clusters: list[list[tuple[str, set[str]]]] = []
    for note, toks in items:
        attached = False
        for cl in clusters:
            for _, ct in cl:
                if _jaccard(toks, ct) >= threshold:
                    cl.append((note, toks))
                    attached = True
                    break
            if attached:
                break
        if not attached:
            clusters.append([(note, toks)])

    out: list[Cluster] = []
    for cl in clusters:
        # Theme: tokens shared by ≥half the notes in the cluster.
        from collections import Counter
        ctr: Counter = Counter()
        for _, ts in cl:
            ctr.update(ts)
        threshold_count = max(1, len(cl) // 2)
        theme = [t for t, c in ctr.most_common(20)
                 if c >= threshold_count]
        rep = max((n for n, _ in cl), key=len)
        out.append(Cluster(
            notes=[n for n, _ in cl],
            theme=theme[:6],
            representative=rep,
        ))
    out.sort(key=lambda c: -c.n)
    return out


# ── pull reflexions from memory ─────────────────────────────────


def _from_feedback_memory(window_days: int = 180) -> list[str]:
    base = (Path.home() / ".claude" / "projects" /
            "-home-oem" / "memory")
    if not base.exists():
        return []
    cutoff = dt.datetime.now() - dt.timedelta(days=window_days)
    out: list[str] = []
    for p in base.glob("feedback_*.md"):
        try:
            mtime = dt.datetime.fromtimestamp(p.stat().st_mtime)
        except OSError:
            continue
        if mtime < cutoff:
            continue
        try:
            text = p.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        if text.startswith("---"):
            end = text.find("\n---", 3)
            if end != -1:
                text = text[end + 4:]
        text = text.strip()
        if len(text) >= 20:
            out.append(text)
    return out


def _from_reflexion_buckets(n_per_bucket: int = 8) -> list[str]:
    try:
        from agents import reflexion as rfx
    except Exception:
        return []
    base = getattr(rfx, "_store_dir", None)
    if base is None:
        return []
    try:
        d = base()
    except Exception:
        return []
    if not d.exists():
        return []
    out: list[str] = []
    import json as _json
    for p in d.glob("*.jsonl"):
        try:
            lines = p.read_text(encoding="utf-8").splitlines()[-n_per_bucket:]
        except OSError:
            continue
        for line in lines:
            try:
                rec = _json.loads(line)
            except Exception:
                continue
            s = rec.get("summary") or ""
            if isinstance(s, str) and len(s) >= 20:
                out.append(s)
    return out


def clusters_from_memory(window_days: int = 180,
                          threshold: float = 0.25) -> list[Cluster]:
    notes = _from_feedback_memory(window_days=window_days)
    notes += _from_reflexion_buckets()
    return cluster(notes, threshold=threshold)


# ── reporting ────────────────────────────────────────────────────


def summary(threshold: float = 0.25) -> str:
    cls = clusters_from_memory(threshold=threshold)
    if not cls:
        return "(no reflexions to cluster yet)"
    lines = [f"🧩 Reflexion clusters — {len(cls)} themes"]
    for c in cls[:8]:
        theme = ", ".join(c.theme[:4]) if c.theme else "(no shared theme)"
        lines.append(f"  • [{c.n} notes] {theme}")
        lines.append(f"      → suggestion: {c.suggestion[:160]}")
    return "\n".join(lines)
