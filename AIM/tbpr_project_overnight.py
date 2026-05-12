#!/usr/bin/env python3
"""
TBPR мульти-агентный цикл.

Фаза 1 — Чемпионат: 6 циклов, разные агенты пробуют, лучший = чемпион.
Фаза 2 — Вызов: каждому агенту 3 попытки побить чемпиона.
После 2 ухудшений подряд — переход к следующему агенту.

Usage:
    python3 tbpr_project_overnight.py                    # все проекты
    python3 tbpr_project_overnight.py --project MCOA     # один проект
"""

import os
import sys
import re
import json
import logging
import argparse
from pathlib import Path
from datetime import datetime
from collections import defaultdict

sys.path.insert(0, str(Path(__file__).resolve().parent))
from llm import ask, ask_reasoner

# ─── Настройки ────────────────────────────────────────────────────────────────
HERE = Path(__file__).resolve().parent
LC_ROOT = HERE.parent
OUTPUT_DIR = HERE / "tbpr_project_output"
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

TARGET_SCORE = 45  # 45/55 = 81.8%
SCORE_MAX = 55
CHAMPION_CYCLES = 6     # сколько циклов в чемпионате
CHALLENGE_ATTEMPTS = 3  # попыток на агента в фазе вызова

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[
        logging.StreamHandler(),
        logging.FileHandler(str(OUTPUT_DIR / "tbpr_project.log"), mode="a")
    ]
)
log = logging.getLogger("tbpr_project")

# ─── Агенты ───────────────────────────────────────────────────────────────────
AGENTS = [
    {"name": "reasoner", "fix": lambda p, **kw: ask_reasoner(p, max_tokens=16384, temperature=0.3, timeout=600)},
    {"name": "chat",     "fix": lambda p, **kw: ask(p, model="deepseek-chat", max_tokens=8192, temperature=0.2, timeout=120)},
]

# ─── Список проектов ─────────────────────────────────────────────────────────
PROJECTS = {
    "CDATA": LC_ROOT / "CDATA" / "CONCEPT.md",
    "MCOA": LC_ROOT / "MCOA" / "CONCEPT.md",
    "Ze": LC_ROOT / "Ze" / "CONCEPT.md",
    "AIM": LC_ROOT / "AIM" / "CONCEPT.md",
    "BioSense": LC_ROOT / "BioSense" / "CONCEPT.md",
}

EXPERIMENTS = {
    "MitoROS": LC_ROOT / "CDATA/experiments/MitoROS/CONCEPT.md",
    "Telomere": LC_ROOT / "CDATA/experiments/Telomere/CONCEPT.md",
    "Proteostasis": LC_ROOT / "CDATA/experiments/Proteostasis/CONCEPT.md",
    "EpigeneticDrift": LC_ROOT / "CDATA/experiments/EpigeneticDrift/CONCEPT.md",
    "CellLineageTree": LC_ROOT / "CDATA/experiments/CellLineageTree/CONCEPT.md",
    "AutomatedMicroscopy": LC_ROOT / "CDATA/experiments/AutomatedMicroscopy/CONCEPT.md",
    "E0": LC_ROOT / "CDATA/experiments/E0/CONCEPT.md",
}

ALL_PROJECTS = {**PROJECTS, **EXPERIMENTS}

# ─── Промпты ──────────────────────────────────────────────────────────────────

SYSTEM_TBPR_PROJECT = """You are a grant review panel conducting a TRIPLE-BLIND PEER REVIEW (TBPR) of a project concept document.

Evaluate this document as a scientific research project proposal (EIC Pathfinder / ERC / NIH format).

Produce EXACTLY the following structure:

# Triple-Blind Peer Review: PROJECT [Name]

## Evaluation Framework: [EIC Pathfinder / ERC AdG / NIH R01] (choose best fit)

---

# REVIEWER A: [Role]

## Scores
| # | Criterion | Score (1-5) | Rationale |
|---|-----------|-------------|-----------|
| 1 | Impact/Significance | X | ... |
| 2 | Approach/Methodology | X | ... |
| 3 | Innovation/Novelty | X | ... |
| 4 | Preliminary Data | X | ... |
| 5 | PI & Team | X | ... |
| 6 | Feasibility | X | ... |
| 7 | Experimental Design | X | ... |
| 8 | Budget & Resources | X | ... |
| 9 | Presentation/Clarity | X | ... |
| 10 | Ethics & Open Science | X | ... |
| 11 | Overall Evaluation | X | ... |

**Score Sum: XX/55**

## Detailed Comments

---

# REVIEWER B: [Role]

## Scores [Same table]
**Score Sum: XX/55**

## Fluff & Padding Audit

---

# REVIEWER C: [Role]

## Scores [Same table]
**Score Sum: XX/55**

## Counter-argument & Bias Audit

---

# Combined Verdict

## Overall Assessment

## Recommendation
**[ACCEPT / REVISE_MAJOR / REVISE_MINOR / REJECT]**

## Combined Score: **XX/55**

## Top 3 Actions for Author
1. ...
2. ...
3. ...

IMPORTANT: Each of 11 criteria 1-5 per reviewer. Score Sum/55. Combined = MIN of 3."""


def make_fix_prompt(concept_text, tbpr_review, project_name, version, agent_name):
    return f"""=== ORIGINAL CONCEPT {project_name} (version {version}) ===
Concept document for {project_name}.

{concept_text}

=== TBPR REVIEW ===
{tbpr_review}

===

Produce the COMPLETE revised concept document incorporating ALL fixable recommendations.
Return ONLY the full revised document, no commentary."""


# ─── Парсинг ──────────────────────────────────────────────────────────────────

def parse_all_scores(tbpr_text: str) -> dict:
    scores = {}
    # Combined Score блок
    cb = re.search(r'##\s*Combined\s+Score.*?(?=\n##|\Z)', tbpr_text, re.DOTALL)
    if cb:
        abcs = re.findall(r'(Reviewer\s+[ABC])\s+Score\s+Sum:\s*(\d+)/55', cb.group(0))
        for name, val in abcs:
            scores[name.strip()] = int(val)
    if scores:
        return scores
    # **Score Sum** | | **30 / 55**
    m = re.findall(r'\*\*Score\s+Sum\*\*[^|]*\|\s*\*\*(\d+)\s*/\s*55\*\*', tbpr_text)
    if m:
        for i, val in enumerate(m[:3]):
            scores[f'R{i+1}'] = int(val)
        return scores
    # Score Sum: XX/55
    m = re.findall(r'Score\s+Sum[^:]*?[:=]\s*(\d+)\s*/\s*55', tbpr_text)
    if m:
        for i, val in enumerate(m[:3]):
            scores[f'R{i+1}'] = int(val)
        return scores
    # XX/55
    m = re.findall(r'(?<![\d/])([1-5][0-9]?|55)\s*/\s*55(?![\d/])', tbpr_text)
    if m:
        unique = []
        for v in m:
            val = int(v)
            if 1 <= val <= 55 and val not in unique:
                unique.append(val)
        for i, val in enumerate(unique[:3]):
            scores[f'R{i+1}'] = val
    return scores


def parse_combined_score(tbpr_text: str) -> int:
    patterns = [
        r'Combined\s+Score\s*=\s*MIN\([^)]+\)\s*=\s*(\d+)\s*/\s*55',
        r'Combined\s+Score[^=]*=\s*\*?\*?(\d+)\s*/\s*55\*?\*?',
        r'Combined\s+score[^=]*=\s*\*?\*?(\d+)\s*/\s*55\*?\*?',
        r'=\s*\*\*(\d+)\s*/\s*55',
        r'MIN\([^)]+\)\s*=\s*\*?\*?(\d+)\s*/\s*55\*?\*?',
        r'Combined\s+Score[^:]*:\s*(\d+)\s*/\s*55',
    ]
    for p in patterns:
        m = re.search(p, tbpr_text)
        if m:
            try:
                return int(m.group(1))
            except (ValueError, IndexError):
                continue
    scores = parse_all_scores(tbpr_text)
    if scores:
        return min(scores.values())
    return 0


def parse_verdict(tbpr_text: str) -> str:
    for p in [r'Recommendation:\s*\*\*([^*]+)\*\*',
              r'##\s*Combined\s+verdict[^#]*\n\*\*([^*]+)\*\*',
              r'(ACCEPT|REJECT|REVISE)']:
        m = re.search(p, tbpr_text)
        if m:
            return m.group(1).strip()
    return "UNKNOWN"


# ─── Ядро ─────────────────────────────────────────────────────────────────────

def run_tbpr(text: str, name: str, tag: str) -> str:
    log.info(f"[{name}] TBPR {tag}...")
    prompt = f"""=== PROJECT CONCEPT for TBPR REVIEW ({tag}) ===
Project: {name}

{text}

===

Produce complete Triple-Blind Peer Review.
3 reviewers, each scores 11 criteria 1-5. Score Sum/55. Combined = MIN."""
    return ask_reasoner(prompt, max_tokens=16384, temperature=0.1)


def fix_with_agent(text: str, review: str, name: str, tag: str, agent: dict) -> str:
    log.info(f"[{name}] Fix {tag} с агентом {agent['name']}...")
    prompt = make_fix_prompt(text, review, name, tag, agent["name"])
    try:
        return agent["fix"](prompt)
    except Exception as e:
        log.error(f"[{name}] Агент {agent['name']} ошибка: {e}")
        return text


def save(path: Path, content: str, desc: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def log_history(history: list, name: str):
    log.info(f"[{name}] === Score History ===")
    for h in history:
        s = "; ".join(f"{k}={v}" for k, v in h.get("scores", {}).items())
        log.info(f"  v{h['agent']}: {s} → {h['combined']}/{SCORE_MAX} ({h['verdict']}){' ⭐' if h.get('best') else ''}")


def run_project(name: str, concept_path: Path):
    if not concept_path.is_file():
        log.warning(f"[{name}] CONCEPT.md not found: {concept_path}")
        return

    orig = concept_path.read_text(encoding="utf-8")
    log.info(f"[{name}] Started. {len(orig)} chars, {len(orig.splitlines())} lines")

    out = OUTPUT_DIR / name
    out.mkdir(parents=True, exist_ok=True)

    history = []
    best_score = 0
    best_text = orig
    best_tag = "original"

    champion = None       # имя агента-чемпиона
    champion_text = None  # текст чемпиона

    # ─── Фаза 1 — Чемпионат: 6 циклов ──────────────────────────────────────
    log.info(f"\n{'='*50}")
    log.info(f"[{name}] ФАЗА 1 — ЧЕМПИОНАТ ({CHAMPION_CYCLES} циклов)")
    log.info(f"{'='*50}")

    for i in range(CHAMPION_CYCLES):
        agent = AGENTS[i % len(AGENTS)]
        tag = f"champ_{i+1}_{agent['name']}"
        log.info(f"\n[{name}] Чемпионат {i+1}/{CHAMPION_CYCLES} — агент {agent['name']}")

        # TBPR
        tbpr = run_tbpr(best_text, name, tag)
        save(out / f"tbpr_{tag}.md", tbpr, f"TBPR {tag}")

        scores = parse_all_scores(tbpr)
        cs = parse_combined_score(tbpr)
        if cs == 0 and scores:
            cs = min(scores.values())
        vd = parse_verdict(tbpr)

        # Показываем результат ДО фикса
        score_str = "; ".join(f"{k}={v}" for k, v in scores.items())
        is_best_ever = cs > best_score

        entry = {"agent": tag, "scores": scores, "combined": cs, "verdict": vd, "best": False}
        history.append(entry)
        log.info(f"  Scores: {score_str} → {cs}/{SCORE_MAX} ({cs/SCORE_MAX*100:.1f}%) — {vd}")

        if cs >= TARGET_SCORE:
            log.info(f"\n🎉 [{name}] ЦЕЛЬ ДОСТИГНУТА! {cs}/{SCORE_MAX}")
            save(out / f"CONCEPT_{tag}_ACCEPTED.md", best_text, "Accepted")
            concept_path.write_text(best_text, encoding="utf-8")
            log_history(history, name)
            return

        # Фикс агентом
        fixed = fix_with_agent(best_text, tbpr, name, tag, agent)
        save(out / f"CONCEPT_{tag}_fixed.md", fixed, f"Fixed by {agent['name']}")

        if is_best_ever:
            best_score = cs
            best_text = best_text  # это текст ДО фикса, который дал лучший скор
            best_tag = tag
            # Но для следующего цикла используем фикс
            best_text = fixed
            champion = agent["name"]
            champion_text = fixed
            entry["best"] = True
            log.info(f"  🆕 Лучший результат! Чемпион: {champion} ⭐")
        else:
            # Откат к лучшей версии
            log.info(f"  ⬇ Score {cs} < best {best_score}. Откат к {best_tag}")
            best_text = best_text  # остаётся предыдущий best

    # Если чемпион не определён — берём лучший текст
    if champion is None:
        champion = best_tag
        champion_text = best_text
    log.info(f"\n[{name}] ЧЕМПИОН: {champion} со скором {best_score}/{SCORE_MAX}")
    save(out / f"CHAMPION_{champion}_{best_score}.md", champion_text, f"Champion {champion}")

    # ─── Фаза 2 — Вызов: каждому агенту 3 попытки ──────────────────────────
    log.info(f"\n{'='*50}")
    log.info(f"[{name}] ФАЗА 2 — ВЫЗОВ (каждому агенту {CHALLENGE_ATTEMPTS} попыток)")
    log.info(f"{'='*50}")

    current_text = champion_text
    current_best = best_score

    for agent in AGENTS:
        if agent["name"] == champion:
            log.info(f"[{name}] Агент {agent['name']} — чемпион, пропускаем.")
            continue

        fails_in_row = 0
        for attempt in range(1, CHALLENGE_ATTEMPTS + 1):
            if fails_in_row >= 2:
                log.info(f"[{name}]   → Агент {agent['name']}: 2 ухудшения подряд. Переход к следующему.")
                break

            tag = f"challenge_{agent['name']}_a{attempt}"
            log.info(f"\n[{name}] Вызов: {agent['name']} попытка {attempt}/{CHALLENGE_ATTEMPTS}")

            # TBPR на текущем тексте
            tbpr = run_tbpr(current_text, name, tag)
            save(out / f"tbpr_{tag}.md", tbpr, f"TBPR {tag}")

            scores = parse_all_scores(tbpr)
            cs = parse_combined_score(tbpr)
            if cs == 0 and scores:
                cs = min(scores.values())
            vd = parse_verdict(tbpr)

            score_str = "; ".join(f"{k}={v}" for k, v in scores.items())
            entry = {"agent": tag, "scores": scores, "combined": cs, "verdict": vd, "best": False}
            history.append(entry)
            log.info(f"  Scores: {score_str} → {cs}/{SCORE_MAX} ({cs/SCORE_MAX*100:.1f}%) — {vd}")

            if cs >= TARGET_SCORE:
                log.info(f"\n🎉 [{name}] ЦЕЛЬ ДОСТИГНУТА! {cs}/{SCORE_MAX}")
                save(out / f"CONCEPT_{tag}_ACCEPTED.md", current_text, "Accepted")
                concept_path.write_text(current_text, encoding="utf-8")
                log_history(history, name)
                return

            if cs > current_best:
                log.info(f"  🆕 Новый лучший! {cs} > {current_best}. Чемпион сменён: {agent['name']} ⭐")
                current_best = cs
                # Фикс агентом
                fixed = fix_with_agent(current_text, tbpr, name, tag, agent)
                save(out / f"CONCEPT_{tag}_fixed.md", fixed, f"Fixed by {agent['name']}")
                current_text = fixed
                fails_in_row = 0
                entry["best"] = True
            else:
                log.info(f"  ⬇ Score {cs} <= best {current_best}. Откат к чемпиону.")
                current_text = champion_text
                fails_in_row += 1

    # ─── Финал ──────────────────────────────────────────────────────────────
    log.info(f"\n[{name}] ИТОГО: лучший скор {current_best}/{SCORE_MAX} ({current_best/SCORE_MAX*100:.1f}%)")
    if current_text != orig:
        save(out / f"CONCEPT_BEST_{current_best}.md", current_text, "Best CONCEPT")
        concept_path.write_text(current_text, encoding="utf-8")
        log.info(f"[{name}] → CONCEPT.md обновлён до лучшей версии!")
    log_history(history, name)


def main():
    parser = argparse.ArgumentParser(description="TBPR Multi-Agent Overnight Cycle")
    parser.add_argument("--project", type=str, default="")
    args = parser.parse_args()

    projects = {args.project: ALL_PROJECTS[args.project]} if args.project else ALL_PROJECTS

    for name, cpath in projects.items():
        log.info(f"\n{'#'*60}")
        log.info(f"# ЗАПУСК: {name}")
        log.info(f"{'#'*60}")
        run_project(name, cpath)

    log.info(f"\n{'='*60}")
    log.info("OVERALL RESULT")
    for name in projects:
        best = "?"
        log.info(f"  {name}: done")
    log.info(f"{'='*60}")


if __name__ == "__main__":
    main()
