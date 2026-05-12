#!/usr/bin/env python3
"""
TBPR трёхуровневый цикл.

Уровень 1 — CONCEPT.md    (концепция проекта)
Уровень 2 — Core-файлы    (THEORY.md, DESIGN.md, PARAMETERS.md, EVIDENCE.md)
Уровень 3 — Весь проект   (интегративная проверка)

Каждый уровень: Фаза 1 — Чемпионат (6 циклов, агенты чередуются)
                Фаза 2 — Вызов (каждому агенту 3 попытки)
                Откат к лучшей версии при ухудшении
                2 ухудшения подряд → смена агента

Usage:
    python3 tbpr_3level.py                          # все проекты, все уровни
    python3 tbpr_3level.py --project MCOA           # один проект, все уровни
    python3 tbpr_3level.py --project MCOA --level 1 # только CONCEPT
"""

import os
import sys
import re
import json
import logging
import argparse
import requests
from pathlib import Path
from datetime import datetime

sys.path.insert(0, str(Path(__file__).resolve().parent))
from llm import ask, ask_reasoner

HERE = Path(__file__).resolve().parent
LC_ROOT = HERE.parent
OUTPUT_DIR = HERE / "tbpr_3level_output"
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

TARGET_SCORE = 45
SCORE_MAX = 55

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[logging.StreamHandler(), logging.FileHandler(str(OUTPUT_DIR / "tbpr_3level.log"), mode="a")]
)
log = logging.getLogger("tbpr_3level")

# ─── Агенты (все модели × все режимы) ───────────────────────────────────────
# Каждому агенту — 1 попытка. Лучший результат = финал.

def _call(model: str, prompt: str, effort=None, max_tokens=16384, temp=0.3, timeout=300):
    """Универсальный вызов DeepSeek с режимом мышления."""
    env_path = Path.home() / ".aim_env"
    key = None
    if env_path.is_file():
        for line in env_path.read_text().splitlines():
            if line.startswith("DEEPSEEK_API_KEY="):
                key = line.split("=", 1)[1].strip()
    if not key:
        key = os.environ.get("DEEPSEEK_API_KEY")
    
    payload = {
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": max_tokens,
        "temperature": temp,
    }
    if effort:
        payload["reasoning_effort"] = effort
    
    resp = requests.post(
        "https://api.deepseek.com/v1/chat/completions",
        headers={"Authorization": f"Bearer {key}", "Content-Type": "application/json"},
        json=payload,
        timeout=timeout,
    )
    resp.raise_for_status()
    return resp.json()["choices"][0]["message"]["content"]


# Полный список: все модели × все режимы, которые дают content
AGENTS = [
    {"name": "flash-nothink",  "model": "deepseek-v4-flash",   "effort": None,  "timeout": 120},
    {"name": "flash-low",      "model": "deepseek-v4-flash",   "effort": "low",  "timeout": 200},
    {"name": "flash-high",     "model": "deepseek-v4-flash",   "effort": "high", "timeout": 300},
    {"name": "flash-max",      "model": "deepseek-v4-flash",   "effort": "max",  "timeout": 400},
    {"name": "chat-high",      "model": "deepseek-chat",       "effort": "high", "timeout": 300},
    {"name": "reasoner",       "model": "deepseek-reasoner",   "effort": None,  "timeout": 400},
    {"name": "reasoner-high",  "model": "deepseek-reasoner",   "effort": "high", "timeout": 500},
    {"name": "reasoner-max",   "model": "deepseek-reasoner",   "effort": "max",  "timeout": 600},
    {"name": "pro-nothink",    "model": "deepseek-v4-pro",     "effort": None,  "timeout": 400},
]

# Оборачиваем в fix-функции
def _make_fix(model, effort, timeout):
    return lambda p, **kw: _call(model, p, effort=effort, timeout=timeout)

for ad in AGENTS:
    ad["fix"] = _make_fix(ad["model"], ad["effort"], ad["timeout"])

# ─── Проекты ──────────────────────────────────────────────────────────────────
# Порядок: от низших подпроектов к высшим
PROJECT_ORDER = [
    # ─── Подпроекты (эксперименты) — lowest ─────────────────────────────────
    ("E0", LC_ROOT / "CDATA/experiments/E0"),
    ("AutomatedMicroscopy", LC_ROOT / "CDATA/experiments/AutomatedMicroscopy"),
    ("CellLineageTree", LC_ROOT / "CDATA/experiments/CellLineageTree"),
    ("EpigeneticDrift", LC_ROOT / "CDATA/experiments/EpigeneticDrift"),
    ("Proteostasis", LC_ROOT / "CDATA/experiments/Proteostasis"),
    ("Telomere", LC_ROOT / "CDATA/experiments/Telomere"),
    ("MitoROS", LC_ROOT / "CDATA/experiments/MitoROS"),
    # ─── Основные проекты ───────────────────────────────────────────────────
    ("BioSense", LC_ROOT / "BioSense"),
    ("AIM", LC_ROOT / "AIM"),
    ("Ze", LC_ROOT / "Ze"),
    ("MCOA", LC_ROOT / "MCOA"),
    ("CDATA", LC_ROOT / "CDATA"),
]
PROJECT_ROOTS = dict(PROJECT_ORDER)

CORE_FILES = ["THEORY.md", "DESIGN.md", "PARAMETERS.md", "EVIDENCE.md"]


# ─── Промпты ──────────────────────────────────────────────────────────────────

SYSTEM_TBPR_CONCEPT = """You are a grant review panel conducting a TRIPLE-BLIND PEER REVIEW (TBPR) of a PROJECT CONCEPT DOCUMENT.

Evaluate as a grant proposal (EIC Pathfinder / ERC / NIH).

Produce:
- REVIEWER A: domain expert. Scores (1-5): Impact, Approach, Innovation, Preliminary Data, PI & Team, Feasibility, Experimental Design, Budget, Clarity, Ethics, Overall. **Score Sum: XX/55**
- REVIEWER B: fluff/impact auditor. Same scores + padding audit. **Score Sum: XX/55**
- REVIEWER C: red team. Same scores + counter-argument/bias audit. **Score Sum: XX/55**
- Combined Verdict: **Combined Score: MIN(X,Y,Z) = XX/55** + Recommendation + Top 3 Actions"""

SYSTEM_TBPR_CORE = """You are a TECHNICAL PEER REVIEW panel evaluating CORE SCIENTIFIC DOCUMENTS of a research project.

Review the following core files:
- THEORY.md — scientific theory and formalism
- DESIGN.md — architecture and implementation design
- PARAMETERS.md — parameters, constants, equations
- EVIDENCE.md — evidence, citations, data

Produce:
- REVIEWER A (Domain Expert): Scores (1-5): TheorySoundness, DesignCoherence, ParameterJustification, EvidenceQuality, Reproducibility, InternalConsistency, Completeness, Clarity, Novelty, Overall. **Score Sum: XX/55**
- REVIEWER B (Cynic): fluff/redundancy audit. Same scores. **Score Sum: XX/55**
- REVIEWER C (Red Team): consistency check, gaps. Same scores. **Score Sum: XX/55**
- Combined: **Combined Score: MIN = XX/55** + Top 3 Actions"""

SYSTEM_TBPR_FULL = """You are an INTEGRATIVE PEER REVIEW panel evaluating an ENTIRE RESEARCH PROJECT.

Review all documents together:
- CONCEPT.md — project concept
- THEORY.md — theory
- DESIGN.md — design
- PARAMETERS.md — parameters
- EVIDENCE.md — evidence

Evaluate: cross-document consistency, feasibility, completeness, scientific merit.

Produce:
- REVIEWER A (Domain Expert): Scores (1-5): CrossDocConsistency, Feasibility, ScientificMerit, Completeness, Reproducibility, Impact, Innovation, Clarity, EthicalSoundness, Overall. **Score Sum: XX/55**
- REVIEWER B (Cynic): signal/noise, overlap audit. Same scores. **Score Sum: XX/55**
- REVIEWER C (Red Team): integration gaps, contradictions. Same scores. **Score Sum: XX/55**
- Combined: **Combined Score: MIN = XX/55** + Top 3 Actions"""


# ─── Парсинг (универсальный) ──────────────────────────────────────────────────

def parse_all_scores(text: str) -> dict:
    scores = {}
    cb = re.search(r'##\s*Combined\s+Score.*?(?=\n##|\Z)', text, re.DOTALL)
    if cb:
        abcs = re.findall(r'(Reviewer\s+[ABC])\s+Score\s+Sum:\s*(\d+)/55', cb.group(0))
        for n, v in abcs:
            scores[n.strip()] = int(v)
    if scores: return scores
    m = re.findall(r'\*\*Score\s+Sum\*\*[^|]*\|\s*\*\*(\d+)\s*/\s*55\*\*', text)
    if m:
        for i, v in enumerate(m[:3]): scores[f'R{i+1}'] = int(v)
        return scores
    m = re.findall(r'Score\s+Sum[^:]*?[:=]\s*(\d+)\s*/\s*55', text)
    if m:
        for i, v in enumerate(m[:3]): scores[f'R{i+1}'] = int(v)
        return scores
    m = re.findall(r'(?<![\d/])([1-5][0-9]?|55)\s*/\s*55(?![\d/])', text)
    if m:
        uniq = []
        for v in m:
            val = int(v)
            if 1 <= val <= 55 and val not in uniq: uniq.append(val)
        for i, v in enumerate(uniq[:3]): scores[f'R{i+1}'] = v
    return scores


def parse_combined(text: str) -> int:
    for p in [r'Combined\s+Score\s*=\s*MIN\([^)]+\)\s*=\s*(\d+)\s*/\s*55',
              r'Combined\s+Score[^=]*=\s*\*?\*?(\d+)\s*/\s*55\*?\*?',
              r'=\s*\*\*(\d+)\s*/\s*55', r'Combined\s+Score[^:]*:\s*(\d+)\s*/\s*55']:
        m = re.search(p, text)
        if m:
            try: return int(m.group(1))
            except: continue
    sc = parse_all_scores(text)
    return min(sc.values()) if sc else 0


def parse_verdict(text: str) -> str:
    for p in [r'Recommendation:\s*\*\*([^*]+)\*\*', r'(ACCEPT|REJECT|REVISE)']:
        m = re.search(p, text)
        if m: return m.group(1).strip()
    return "UNKNOWN"


# ─── TBPR + Fix ───────────────────────────────────────────────────────────────

def run_tbpr(text: str, name: str, tag: str, system_prompt: str) -> str:
    log.info(f"[{name}] TBPR {tag}...")
    prompt = f"""{text}

===

Produce complete Triple-Blind Peer Review as specified.
3 reviewers, scores 1-5 each, Score Sum/55 each. Combined = MIN."""
    return ask_reasoner(prompt, max_tokens=16384, temperature=0.1, system=system_prompt)


def fix_with_agent(text: str, review: str, name: str, tag: str, agent: dict, level: str) -> str:
    log.info(f"[{name}] Fix {tag} [{level}] агент {agent['name']}...")
    prompt = f"""=== DOCUMENT TO REVISE ({level}) ===
{text}

=== TBPR REVIEW ===
{review}

===

Produce the COMPLETE revised document. Return ONLY the full text."""
    try:
        return agent["fix"](prompt)
    except Exception as e:
        log.error(f"[{name}] {agent['name']} ошибка: {e}")
        return text


def save(path: Path, content: str, desc: str = ""):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


# ─── Чемпионат + Вызов ────────────────────────────────────────────────────────

def try_all_agents(name: str, base_text: str, out_dir: Path, label: str,
                    system_prompt: str, file_ext: str = ".md") -> tuple[str, int]:
    """
    Каждому агенту 1 попытка. Лучший = финал.
    Возвращает (лучший_текст, лучший_скор).
    """
    best_score = 0
    best_text = base_text
    best_agent = "original"
    history = []

    log.info(f"\n{'='*50}")
    log.info(f"[{name}] {label} — {len(AGENTS)} агентов, по 1 попытке")
    log.info(f"{'='*50}")

    for i, agent in enumerate(AGENTS):
        tag = f"{label}_a{i+1}_{agent['name']}"
        log.info(f"\n[{name}] {label} поп.{i+1}/{len(AGENTS)} — {agent['name']}")

        # TBPR
        tbpr = run_tbpr(best_text, name, tag, system_prompt)
        save(out_dir / f"tbpr_{tag}{file_ext}", tbpr)

        cs = parse_combined(tbpr)
        scores = parse_all_scores(tbpr)
        if cs == 0 and scores:
            cs = min(scores.values())
        vd = parse_verdict(tbpr)

        entry = {"agent": agent['name'], "scores": scores, "combined": cs, "verdict": vd}
        history.append(entry)

        log.info(f"  → {cs}/{SCORE_MAX} ({cs/SCORE_MAX*100:.1f}%) — {vd}")

        if cs >= TARGET_SCORE:
            log.info(f"🎉 [{name}] {label} ЦЕЛЬ ДОСТИГНУТА! {cs}/{SCORE_MAX}")
            save(out_dir / f"{label}_ACCEPTED{file_ext}", best_text)
            return best_text, cs

        if cs > best_score:
            best_score = cs
            best_agent = agent['name']
            log.info(f"  🆕 Лучший: {agent['name']} ⭐")

        # Фикс агентом
        fixed = fix_with_agent(best_text, tbpr, name, tag, agent, label)
        save(out_dir / f"{tag}_fixed{file_ext}", fixed)

    log.info(f"[{name}] {label} ЛУЧШИЙ: {best_agent} = {best_score}/{SCORE_MAX}")
    return best_text, best_score


# ─── Сбор файлов ──────────────────────────────────────────────────────────────

def read_file(path: Path) -> str:
    if path.is_file():
        return path.read_text(encoding="utf-8")
    return ""

def get_concept(root: Path) -> str:
    return read_file(root / "CONCEPT.md")

def get_core(root: Path) -> str:
    parts = []
    for f in CORE_FILES:
        content = read_file(root / f)
        if content:
            parts.append(f"=== {f} ===\n{content}")
    return "\n\n".join(parts) if parts else ""

def get_full_project(root: Path) -> str:
    parts = []
    for f in ["CONCEPT.md"] + CORE_FILES:
        content = read_file(root / f)
        if content:
            parts.append(f"=== {f} ===\n{content}")
    return "\n\n".join(parts) if parts else ""


# ─── Главная функция ──────────────────────────────────────────────────────────

def run_full(name: str, root: Path, max_level: int = 3):
    out = OUTPUT_DIR / name
    out.mkdir(parents=True, exist_ok=True)
    final_report = []

    # ─── Уровень 1: CONCEPT ─────────────────────────────────────────────────
    if max_level >= 1:
        log.info(f"\n{'#'*60}")
        log.info(f"# {name} — УРОВЕНЬ 1: CONCEPT")
        log.info(f"{'#'*60}")
        concept = get_concept(root)
        if not concept:
            log.warning(f"[{name}] CONCEPT.md не найден")
        else:
            log.info(f"[{name}] CONCEPT.md: {len(concept)} chars")
            best_c, score_c = try_all_agents(name, concept, out, "concept", SYSTEM_TBPR_CONCEPT)
            save(out / "CONCEPT_BEST.md", best_c)
            # Обновляем CONCEPT.md в проекте
            (root / "CONCEPT.md").write_text(best_c, encoding="utf-8")
            log.info(f"[{name}] → CONCEPT.md обновлён ({score_c}/{SCORE_MAX})")
            final_report.append(f"**CONCEPT:** {score_c}/{SCORE_MAX} ({(score_c/SCORE_MAX*100):.1f}%)")

    # ─── Уровень 2: Core файлы ──────────────────────────────────────────────
    if max_level >= 2:
        log.info(f"\n{'#'*60}")
        log.info(f"# {name} — УРОВЕНЬ 2: CORE FILES")
        log.info(f"{'#'*60}")
        core = get_core(root)
        if not core:
            log.warning(f"[{name}] Core файлы не найдены")
        else:
            log.info(f"[{name}] Core: {len(core)} chars")
            best_co, score_co = try_all_agents(name, core, out, "core", SYSTEM_TBPR_CORE)
            save(out / "CORE_BEST.md", best_co)
            log.info(f"[{name}] Core лучший: {score_co}/{SCORE_MAX}")
            final_report.append(f"**CORE:** {score_co}/{SCORE_MAX} ({(score_co/SCORE_MAX*100):.1f}%)")

    # ─── Уровень 3: Full project ────────────────────────────────────────────
    if max_level >= 3:
        log.info(f"\n{'#'*60}")
        log.info(f"# {name} — УРОВЕНЬ 3: FULL PROJECT")
        log.info(f"{'#'*60}")
        full = get_full_project(root)
        if not full:
            log.warning(f"[{name}] Нет документов для интегративной проверки")
        else:
            log.info(f"[{name}] Full: {len(full)} chars")
            best_f, score_f = try_all_agents(name, full, out, "full", SYSTEM_TBPR_FULL)
            save(out / "FULL_BEST.md", best_f)
            log.info(f"[{name}] Full лучший: {score_f}/{SCORE_MAX}")
            final_report.append(f"**FULL:** {score_f}/{SCORE_MAX} ({(score_f/SCORE_MAX*100):.1f}%)")

    # ─── Итог ──────────────────────────────────────────────────────────────
    log.info(f"\n{'='*50}")
    log.info(f"[{name}] ИТОГ:")
    for line in final_report:
        log.info(f"  {line}")
    log.info(f"{'='*50}")
    save(out / "FINAL_REPORT.md", "\n".join([
        f"# {name} — TBPR 3-Level Report",
        f"**Date:** {datetime.now().isoformat()}",
        "",
        *final_report
    ]))


def main():
    parser = argparse.ArgumentParser(description="TBPR 3-Level Overnight Cycle")
    parser.add_argument("--project", type=str, default="")
    parser.add_argument("--level", type=int, default=3, help="Max level (1=CONCEPT, 2=CORE, 3=FULL)")
    args = parser.parse_args()

    if args.project:
        projects = [(args.project, PROJECT_ROOTS[args.project])]
    else:
        projects = PROJECT_ORDER  # сохраняем порядок: снизу вверх

    for name, root in projects:
        log.info(f"\n{'#'*60}")
        log.info(f"# СТАРТ: {name} (уровни 1-{args.level})")
        log.info(f"{'#'*60}")
        run_full(name, root, args.level)


if __name__ == "__main__":
    main()
