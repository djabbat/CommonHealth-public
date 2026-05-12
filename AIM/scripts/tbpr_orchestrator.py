#!/usr/bin/env python3
"""TBPR 3-Level orchestrator (Claude prompt 2026-05-10).

12 проектов × 3 уровня × 9 агентов через DeepSeek API.
Цель: Combined Score >= 45/55 (81%).

Использование:
    python tbpr_orchestrator.py --project E0
    python tbpr_orchestrator.py --project E0 --level concept --only-agent 1
    python tbpr_orchestrator.py --all
    python tbpr_orchestrator.py --resume   # продолжить с чекпоинта
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
import time
import traceback
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

import requests

ROOT = Path("/home/oem/Desktop/LongevityCommon")
AIM = ROOT / "AIM"
OUT_ROOT = AIM / "tbpr_claude_output"
LOG_FILE = OUT_ROOT / "orchestrator.log"
DEBUG_FILE = OUT_ROOT / "debug.log"
STATE_FILE = OUT_ROOT / "orchestrator_state.json"

PROJECTS_ORDER = [
    ("E0", ROOT / "CDATA/experiments/E0"),
    ("AutomatedMicroscopy", ROOT / "CDATA/experiments/AutomatedMicroscopy"),
    ("CellLineageTree", ROOT / "CDATA/experiments/CellLineageTree"),
    ("EpigeneticDrift", ROOT / "CDATA/experiments/EpigeneticDrift"),
    ("Proteostasis", ROOT / "CDATA/experiments/Proteostasis"),
    ("Telomere", ROOT / "CDATA/experiments/Telomere"),
    ("MitoROS", ROOT / "CDATA/experiments/MitoROS"),
    ("BioSense", ROOT / "BioSense"),
    ("AIM", ROOT / "AIM"),
    ("Ze", ROOT / "Ze"),
    ("MCOA", ROOT / "MCOA"),
    ("CDATA", ROOT / "CDATA"),
]

# AGENTS загружается из FIXER_MODEL_LIST в provider_router
# чтобы избежать дублирования между tbpr_orchestrator.py и tbpr_overnight.py
_FIXER_LIST_CACHE: list[tuple[str, str]] | None = None

def _get_fixer_list() -> list[tuple[str, str]]:
    """Загрузить FIXER_MODEL_LIST из provider_router (единственный источник)."""
    global _FIXER_LIST_CACHE
    if _FIXER_LIST_CACHE is not None:
        return _FIXER_LIST_CACHE
    try:
        sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent.parent / "Services" / "tbpr" / "scripts"))
        from provider_router import FIXER_MODEL_LIST
        _FIXER_LIST_CACHE = FIXER_MODEL_LIST
        return FIXER_MODEL_LIST
    except ImportError:
        # Fallback, если provider_router не найден
        _FIXER_LIST_CACHE = [
            ("deepseek", "deepseek-chat"),
            ("deepseek", "deepseek-reasoner"),
            ("deepseek", "deepseek-v4-flash"),
            ("deepseek", "deepseek-v4-pro"),
        ]
        return _FIXER_LIST_CACHE

def _agent_name(provider: str, model: str, idx: int) -> str:
    """Сгенерировать имя агента из FIXER_MODEL_LIST."""
    short = model.replace("deepseek-", "").replace("gemini-", "")
    return f"{short}-{provider}"

# Динамические AGENTS: читаются из FIXER_MODEL_LIST
# Доступны как AGENTS (для обратной совместимости) и через get_agents()
AGENTS: list[dict] = []  # будет заполнено при первом вызове get_agents()

def get_agents() -> list[dict]:
    """Вернуть список всех доступных агентов из FIXER_MODEL_LIST."""
    fixer_list = _get_fixer_list()
    agents = []
    for idx, (provider, model) in enumerate(fixer_list):
        agents.append({
            "name": _agent_name(provider, model, idx),
            "provider": provider,
            "model": model,
            "effort": None,
        })
    return agents

# Инициализируем AGENTS
AGENTS = get_agents()

REVIEW_MODEL = "deepseek-reasoner"
GOAL = 45
MAX_TOKENS_REVIEW = 16384
MAX_TOKENS_FIX = 16384
TIMEOUT = 600

LEVEL_FILES = {
    "concept": ["CONCEPT.md"],
    "core":    ["THEORY.md", "DESIGN.md", "PARAMETERS.md", "EVIDENCE.md"],
    "full":    ["CONCEPT.md", "THEORY.md", "DESIGN.md", "PARAMETERS.md", "EVIDENCE.md",
                "KNOWLEDGE.md", "MAP.md", "LINKS.md"],
}

LEVELS = ["concept", "core", "full"]


# ---------- DeepSeek API ----------

def _api_key() -> str:
    env_path = Path.home() / ".aim_env"
    if env_path.is_file():
        for line in env_path.read_text().splitlines():
            line = line.strip()
            if line.startswith("DEEPSEEK_API_KEY="):
                return line.split("=", 1)[1].strip()
    key = os.environ.get("DEEPSEEK_API_KEY")
    if key:
        return key
    raise RuntimeError("DEEPSEEK_API_KEY not found")


def call_deepseek(prompt: str, model: str, effort: Optional[str], max_tokens: int,
                  temperature: float = 0.3, retries: int = 2) -> str:
    """Один вызов DeepSeek с retry."""
    payload = {
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": max_tokens,
        "temperature": temperature,
    }
    if effort is not None:
        payload["reasoning_effort"] = effort
    last_err = None
    for attempt in range(retries + 1):
        try:
            r = requests.post(
                "https://api.deepseek.com/v1/chat/completions",
                headers={
                    "Authorization": f"Bearer {_api_key()}",
                    "Content-Type": "application/json",
                },
                json=payload,
                timeout=TIMEOUT,
            )
            if r.status_code == 400 and effort and "reasoning_effort" in r.text:
                # модель не поддерживает reasoning_effort — повтор без него
                payload.pop("reasoning_effort", None)
                continue
            r.raise_for_status()
            return r.json()["choices"][0]["message"]["content"]
        except Exception as e:
            last_err = e
            log(f"    API error (attempt {attempt + 1}): {e}")
            time.sleep(5 + 5 * attempt)
    raise RuntimeError(f"DeepSeek failed after {retries + 1} attempts: {last_err}")


# ---------- Prompt templates ----------

REVIEW_SYS_CONCEPT = """You are a grant review panel conducting a TRIPLE-BLIND PEER REVIEW (TBPR)
of a PROJECT CONCEPT DOCUMENT. Evaluate it as a grant proposal.

Produce three independent reviewers, each scoring 1-5 on these 11 criteria:
Impact, Approach, Innovation, Preliminary Data, PI & Team, Feasibility,
Experimental Design, Budget, Clarity, Ethics, Overall.

For each reviewer, output a clear line: **Score Sum: XX/55**.

REVIEWER A — domain expert.
REVIEWER B — fluff/impact auditor.
REVIEWER C — red team.

After all three: a Combined Verdict block with this exact line:
**Combined Score: MIN = XX/55**
Then a Recommendation (Accept / Major Revisions / Reject) and Top 3 Actions.
"""

REVIEW_SYS_CORE = """You are a TECHNICAL PEER REVIEW panel evaluating CORE SCIENTIFIC DOCUMENTS.
The bundle includes (some may be missing): THEORY.md, DESIGN.md, PARAMETERS.md, EVIDENCE.md.

Produce three independent reviewers scoring 1-5 on these 11 criteria:
TheorySoundness, DesignCoherence, ParameterJustification, EvidenceQuality,
Reproducibility, InternalConsistency, Completeness, Clarity, Novelty, Risks, Overall.

For each reviewer, output a clear line: **Score Sum: XX/55**.

REVIEWER A — domain expert.
REVIEWER B — cynic.
REVIEWER C — red team.

After all three: a Combined Verdict block with this exact line:
**Combined Score: MIN = XX/55**
Plus Top 3 Actions.
"""

REVIEW_SYS_FULL = """You are an INTEGRATIVE PEER REVIEW panel evaluating an ENTIRE PROJECT
across all its core documents (CONCEPT, THEORY, DESIGN, PARAMETERS, EVIDENCE,
KNOWLEDGE, MAP, LINKS — some may be missing).

Produce three independent reviewers scoring 1-5 on these 11 criteria:
CrossDocConsistency, Feasibility, ScientificMerit, Completeness, Reproducibility,
Impact, Innovation, Clarity, Ethics, Risks, Overall.

For each reviewer, output a clear line: **Score Sum: XX/55**.

REVIEWER A — domain expert.
REVIEWER B — cynic.
REVIEWER C — red team.

After all three: a Combined Verdict block with this exact line:
**Combined Score: MIN = XX/55**
Plus Top 3 Actions.
"""

REVIEW_SYS = {
    "concept": REVIEW_SYS_CONCEPT,
    "core": REVIEW_SYS_CORE,
    "full": REVIEW_SYS_FULL,
}


def make_review_prompt(level: str, project: str, doc_text: str) -> str:
    return (
        f"{REVIEW_SYS[level]}\n\n"
        f"=== PROJECT: {project} | LEVEL: {level} ===\n\n"
        f"{doc_text}\n\n"
        "=== END DOCUMENT(S) ===\n\n"
        "Now produce the three reviewer reports and the Combined Verdict."
    )


def make_fix_prompt(level: str, project: str, doc_text: str, review_text: str) -> str:
    if level == "concept":
        kind = "CONCEPT document (single CONCEPT.md)"
        instruction = (
            "Produce the COMPLETE revised CONCEPT.md incorporating ALL fixable "
            "recommendations from the review. Return ONLY the full revised "
            "Markdown document, no commentary, no fences."
        )
    elif level == "core":
        kind = "CORE bundle (THEORY/DESIGN/PARAMETERS/EVIDENCE)"
        instruction = (
            "Produce the COMPLETE revised core bundle. Use clear section headers "
            "for each file, formatted exactly as:\n"
            "===== FILE: THEORY.md =====\n<content>\n"
            "===== FILE: DESIGN.md =====\n<content>\n"
            "===== FILE: PARAMETERS.md =====\n<content>\n"
            "===== FILE: EVIDENCE.md =====\n<content>\n"
            "Include ONLY files that already existed in the input. "
            "Return raw Markdown only, no commentary, no fences."
        )
    else:
        kind = "FULL project bundle"
        instruction = (
            "Produce the COMPLETE revised full bundle. Use clear section headers "
            "for each file, formatted exactly as:\n"
            "===== FILE: <NAME>.md =====\n<content>\n"
            "Include ONLY files that already existed in the input. "
            "Return raw Markdown only, no commentary, no fences."
        )
    return (
        f"=== ORIGINAL {kind} — Project: {project} ===\n\n"
        f"{doc_text}\n\n"
        f"=== TBPR REVIEW ===\n\n{review_text}\n\n"
        f"===\n\n{instruction}"
    )


# ---------- Score parsing ----------

def parse_score(text: str) -> Optional[int]:
    m = re.search(r"Combined\s+Score[^=\n]*=\s*(\d+)\s*/\s*55", text, re.IGNORECASE)
    if m:
        return int(m.group(1))
    m = re.search(r"Combined\s+Score[^:\n]*:\s*MIN\s*=?\s*(\d+)\s*/\s*55",
                  text, re.IGNORECASE)
    if m:
        return int(m.group(1))
    sums = re.findall(r"Score\s+Sum[^:=\n]*[:=]\s*(\d+)\s*/\s*55", text, re.IGNORECASE)
    if len(sums) >= 3:
        return min(int(s) for s in sums[:3])
    if sums:
        return min(int(s) for s in sums)
    return None


# ---------- Core file generation from CONCEPT ----------

CORE_FILES = ["THEORY.md", "DESIGN.md", "PARAMETERS.md", "EVIDENCE.md"]


def build_ensure_core_prompt(project: str, concept_text: str,
                             missing: list[str]) -> str:
    rules = """1. THEORY.md — научная теория
   - Формальные определения, аксиомы, постулаты
   - Математический формализм (уравнения, модели)
   - Связь с существующими теориями
   - Фальсифицируемые предсказания
   - Структура: Formal Framework → Core Axioms → Derived Properties → Predictions
2. DESIGN.md — архитектура и реализация
   - Компоненты и их взаимодействие
   - Технические детали (языки, фреймворки, протоколы)
   - API, интерфейсы, схемы данных
   - Потоки данных и управления
   - Структура: Architecture Overview → Components → Data Flow → Interfaces
3. PARAMETERS.md — параметры
   - Все константы, переменные, коэффициенты
   - Единицы измерения, диапазоны, обоснования
   - Параметры моделей, пороги, лимиты
   - Таблицы со значениями и источниками
   - Структура: Physical Constants → Model Parameters → Thresholds → Calibration
4. EVIDENCE.md — доказательства
   - Ссылки на литературу с PMID/DOI
   - Экспериментальные данные и их интерпретация
   - Сильные и слабые стороны доказательств
   - Пробелы в знаниях и план их заполнения
   - Структура: Direct Evidence → Supporting Evidence → Gaps → Validation Plan
"""
    instructions = (
        "Правила:\n"
        "- Все 4 файла согласованы друг с другом\n"
        "- Не выдумывай несуществующие ссылки/данные\n"
        "- Если чего-то нет в CONCEPT — так и напиши: \"не указано\"\n"
        "- Используй язык CONCEPT (русский/английский)\n"
        "- Каждый файл — полноценный markdown документ\n"
        "- Формат вывода:\n\n"
        "```\n"
        "=== THEORY.md ===\n[содержимое]\n\n"
        "=== DESIGN.md ===\n[содержимое]\n\n"
        "=== PARAMETERS.md ===\n[содержимое]\n\n"
        "=== EVIDENCE.md ===\n[содержимое]\n"
        "```\n"
    )
    missing_note = (
        f"\nВ проекте `{project}` отсутствуют файлы: {', '.join(missing)}.\n"
        "Сгенерируй ВСЕ четыре файла (THEORY, DESIGN, PARAMETERS, EVIDENCE) "
        "согласованно — но в проект будут записаны только отсутствующие.\n"
    )
    return (
        f"Сгенерируй core-файлы для проекта `{project}` из его CONCEPT.md.\n\n"
        f"CONCEPT.md проекта:\n\n```\n{concept_text}\n```\n\n"
        f"Правила генерации (4 файла):\n\n{rules}\n"
        f"{instructions}{missing_note}"
    )


def parse_ensure_core_output(text: str) -> dict[str, str]:
    """Parse `=== FILENAME.md ===` sections."""
    pattern = re.compile(r"^={3,}\s*(\S+?\.md)\s*={3,}\s*$", re.MULTILINE)
    matches = list(pattern.finditer(text))
    out = {}
    for i, m in enumerate(matches):
        fn = m.group(1).strip()
        start = m.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        body = text[start:end].strip()
        # strip surrounding ``` fences if present
        body = re.sub(r"^```[a-zA-Z]*\s*\n", "", body)
        body = re.sub(r"\n```\s*$", "", body)
        out[fn] = body
    return out


def ensure_core_files(project: str, project_dir: Path) -> dict:
    missing = [f for f in CORE_FILES if not (project_dir / f).is_file()]
    if not missing:
        return {"missing": [], "generated": []}
    concept_path = project_dir / "CONCEPT.md"
    if not concept_path.is_file():
        log(f"[{project}] ensure_core skip — no CONCEPT.md")
        dlog(EVENT="ensure_core_skip", PROJECT=project, REASON="no_concept")
        return {"missing": missing, "generated": [], "error": "no_concept"}
    concept = concept_path.read_text(encoding="utf-8", errors="replace")
    log(f"[{project}] ensure_core: missing={missing}, generating via reasoner")
    dlog(EVENT="ensure_core_start", PROJECT=project, MISSING=",".join(missing),
         CONCEPT_CHARS=len(concept))
    prompt = build_ensure_core_prompt(project, concept, missing)
    t0 = time.time()
    try:
        text = call_deepseek(prompt, "deepseek-reasoner", None, 32768,
                             temperature=0.2)
    except Exception as e:
        log(f"[{project}] ensure_core FAIL: {e}")
        dlog(EVENT="ensure_core_err", PROJECT=project, ERR=str(e)[:120])
        return {"missing": missing, "generated": [], "error": str(e)}
    elapsed = time.time() - t0
    bundle = parse_ensure_core_output(text)
    written = []
    header = ("<!-- AUTO-GENERATED from CONCEPT.md by TBPR orchestrator "
              "2026-05-10 ensure_core (DeepSeek-reasoner). "
              "Review and edit as needed. -->\n\n")
    for fn in missing:
        if fn in bundle:
            (project_dir / fn).write_text(header + bundle[fn], encoding="utf-8")
            written.append(fn)
    raw_path = OUT_ROOT / project / "ensure_core_raw.md"
    raw_path.parent.mkdir(parents=True, exist_ok=True)
    raw_path.write_text(text, encoding="utf-8")
    log(f"[{project}] ensure_core OK ({elapsed:.0f}s): wrote {written}")
    dlog(EVENT="ensure_core_ok", PROJECT=project,
         MISSING=",".join(missing), WRITTEN=",".join(written),
         DURATION=f"{elapsed:.0f}s")
    ddecision(
        f"ensure_core for {project}: generated {written} from CONCEPT.md "
        f"(missing={missing}); files written with AUTO-GENERATED header"
    )
    return {"missing": missing, "generated": written}


# ---------- Document loading ----------

def load_docs(level: str, project_dir: Path) -> tuple[str, list[str]]:
    files = LEVEL_FILES[level]
    parts = []
    present = []
    for fn in files:
        p = project_dir / fn
        if p.is_file():
            content = p.read_text(encoding="utf-8", errors="replace")
            parts.append(f"===== FILE: {fn} =====\n{content}")
            present.append(fn)
    if not parts:
        return "", []
    return "\n\n".join(parts), present


# ---------- Output writing ----------

def split_fixed_bundle(text: str) -> dict[str, str]:
    """Парсит вывод fix для core/full → {filename: content}."""
    result = {}
    pattern = re.compile(r"^=====\s*FILE:\s*(\S+?\.md)\s*=====\s*$", re.MULTILINE)
    matches = list(pattern.finditer(text))
    if not matches:
        return result
    for i, m in enumerate(matches):
        fn = m.group(1).strip()
        start = m.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        result[fn] = text[start:end].strip()
    return result


def fixed_to_doctext(level: str, fixed_text: str, present: list[str]) -> str:
    """Превращает вывод fix обратно в doc_text для следующего агента."""
    if level == "concept":
        return f"===== FILE: CONCEPT.md =====\n{fixed_text.strip()}"
    bundle = split_fixed_bundle(fixed_text)
    if not bundle:
        return fixed_text  # неизвестный формат — пихаем как есть
    # сохраняем порядок present_files
    parts = []
    for fn in present:
        if fn in bundle:
            parts.append(f"===== FILE: {fn} =====\n{bundle[fn].strip()}")
    # на случай неожиданных файлов в bundle
    for fn, content in bundle.items():
        if fn not in present:
            parts.append(f"===== FILE: {fn} =====\n{content.strip()}")
    return "\n\n".join(parts) if parts else fixed_text


def save_fixed(level: str, project_dir: Path, out_dir: Path, agent_idx: int,
               agent_name: str, fixed_text: str, present_files: list[str]) -> Path:
    if level == "concept":
        fp = out_dir / f"concept_a{agent_idx}_{agent_name}_fixed.md"
        fp.write_text(fixed_text, encoding="utf-8")
        return fp
    bundle = split_fixed_bundle(fixed_text)
    base = out_dir / f"a{agent_idx}_{agent_name}_fixed"
    base.mkdir(exist_ok=True)
    if not bundle:
        # fallback: сохранить целиком
        fp = base / "raw_output.md"
        fp.write_text(fixed_text, encoding="utf-8")
        return base
    for fn, content in bundle.items():
        if fn in present_files:
            (base / fn).write_text(content, encoding="utf-8")
    return base


# ---------- Logging / state ----------

def log(msg: str):
    OUT_ROOT.mkdir(parents=True, exist_ok=True)
    ts = time.strftime("%Y-%m-%d %H:%M:%S")
    line = f"[{ts}] {msg}"
    print(line, flush=True)
    with LOG_FILE.open("a", encoding="utf-8") as f:
        f.write(line + "\n")


def dlog(**fields):
    """Write a structured line to debug.log per CLAUDE_README format."""
    OUT_ROOT.mkdir(parents=True, exist_ok=True)
    ts = time.strftime("%Y-%m-%d %H:%M:%S")
    parts = [f"[{ts}]"]
    for k, v in fields.items():
        if v is None:
            v = "-"
        parts.append(f"{k}={v}")
    line = " ".join(parts)
    with DEBUG_FILE.open("a", encoding="utf-8") as f:
        f.write(line + "\n")


def ddecision(msg: str, **fields):
    """Free-form DECISION line."""
    OUT_ROOT.mkdir(parents=True, exist_ok=True)
    ts = time.strftime("%Y-%m-%d %H:%M:%S")
    extra = ""
    if fields:
        extra = " " + " ".join(f"{k}={v}" for k, v in fields.items())
    line = f"[{ts}] DECISION:{extra} msg=\"{msg}\""
    with DEBUG_FILE.open("a", encoding="utf-8") as f:
        f.write(line + "\n")


def load_state() -> dict:
    if STATE_FILE.is_file():
        try:
            return json.loads(STATE_FILE.read_text())
        except Exception:
            pass
    return {"completed": {}, "best": {}}


def save_state(state: dict):
    STATE_FILE.parent.mkdir(parents=True, exist_ok=True)
    STATE_FILE.write_text(json.dumps(state, indent=2, ensure_ascii=False))


# ---------- Main TBPR loop ----------

@dataclass
class LevelResult:
    project: str
    level: str
    best_score: int = 0
    best_agent: Optional[str] = None
    best_fixed_path: Optional[str] = None
    attempts: list[dict] = field(default_factory=list)
    goal_reached: bool = False
    skipped: bool = False
    skip_reason: str = ""


def run_level(project: str, project_dir: Path, level: str,
              only_agent: Optional[int], state: dict) -> LevelResult:
    out_dir = OUT_ROOT / project / level
    out_dir.mkdir(parents=True, exist_ok=True)

    res = LevelResult(project=project, level=level)
    key = f"{project}/{level}"

    if key in state.get("best", {}):
        prev = state["best"][key]
        log(f"[{project}/{level}] resume skip — already attempted (best {prev.get('score')}/55 by {prev.get('agent')})")
        dlog(EVENT="level_resume_skip", PROJECT=project, LEVEL=level,
             BEST=f"{prev.get('score')}/55", BEST_AGENT=prev.get('agent'))
        res.best_score = prev.get("score") or 0
        res.best_agent = prev.get("agent")
        res.best_fixed_path = prev.get("fixed_path")
        res.goal_reached = (res.best_score >= GOAL)
        return res

    doc_text, present = load_docs(level, project_dir)
    if not doc_text:
        res.skipped = True
        res.skip_reason = f"no files present for level {level}"
        log(f"[{project}/{level}] SKIP — {res.skip_reason}")
        return res

    log(f"[{project}/{level}] start — files: {present}")

    # Кумулятивный режим: doc_text обновляется fixed_text каждого следующего агента.
    current_doc = doc_text

    for idx, agent in enumerate(AGENTS, start=1):
        if only_agent is not None and idx != only_agent:
            continue
        name = agent["name"]
        log(f"[{project}/{level}] agent {idx}/{len(AGENTS)} = {name}")

        # Review (текущая редакция документа)
        review_prompt = make_review_prompt(level, project, current_doc)
        dlog(MODEL=REVIEW_MODEL, EFFORT="-", ACTION="review", STATUS="start",
             PROJECT=project, LEVEL=level, AGENT=name,
             PROMPT_CHARS=len(review_prompt))
        try:
            t0 = time.time()
            review_text = call_deepseek(
                review_prompt, REVIEW_MODEL, None,
                MAX_TOKENS_REVIEW, temperature=0.1,
            )
            t1 = time.time()
            log(f"  review ok ({t1 - t0:.0f}s, {len(review_text)} chars)")
        except Exception as e:
            log(f"  review FAIL: {e}")
            dlog(MODEL=REVIEW_MODEL, EFFORT="-", ACTION="review", STATUS="err",
                 PROJECT=project, LEVEL=level, AGENT=name,
                 ERR=str(e).replace(" ", "_")[:120])
            res.attempts.append({"agent": name, "error": f"review: {e}"})
            continue

        review_path = out_dir / f"tbpr_a{idx}_{name}.md"
        review_path.write_text(review_text, encoding="utf-8")
        score = parse_score(review_text)
        log(f"  score = {score}/55  (best so far {res.best_score})")
        dlog(MODEL=REVIEW_MODEL, EFFORT="-", ACTION="review", STATUS="ok",
             PROJECT=project, LEVEL=level, AGENT=name,
             SCORE=f"{score}/55" if score is not None else "?/55",
             DURATION=f"{t1 - t0:.0f}s", RESP_CHARS=len(review_text))

        attempt = {"agent": name, "score": score, "review_path": str(review_path)}

        if score is not None and score > res.best_score:
            prev_best = res.best_score
            res.best_score = score
            res.best_agent = name
            ddecision(
                f"new champion at {project}/{level}: agent {idx} ({name}) "
                f"score {score}/55, prev best {prev_best}/55 → keep as champion"
            )

        # Если уже достигли цели — фикс не нужен (но сохраним пометку).
        if score is not None and score >= GOAL:
            res.goal_reached = True
            attempt["goal_reached"] = True
            res.attempts.append(attempt)
            best_path = out_dir / f"{level.upper()}_BEST.md"
            best_path.write_text(
                f"# {project}/{level} — BEST (goal reached)\n\n"
                f"**Score: {score}/55** by agent **{name}**\n\n"
                f"Review: {review_path.name}\n\n"
                f"Original document used as final (no fix needed).\n",
                encoding="utf-8",
            )
            res.best_fixed_path = str(best_path)
            log(f"  GOAL REACHED at agent {name} ({score}/55) — stop level")
            state["best"][key] = {
                "score": score, "agent": name, "fixed_path": str(best_path),
            }
            save_state(state)
            return res

        # Fix
        fix_prompt = make_fix_prompt(level, project, current_doc, review_text)
        dlog(MODEL=agent["model"], EFFORT=agent["effort"] or "-",
             ACTION="fix", STATUS="start",
             PROJECT=project, LEVEL=level, AGENT=name,
             PROMPT_CHARS=len(fix_prompt))
        try:
            t0 = time.time()
            fixed_text = call_deepseek(
                fix_prompt, agent["model"], agent["effort"],
                MAX_TOKENS_FIX, temperature=0.3,
            )
            t1 = time.time()
            log(f"  fix ok ({t1 - t0:.0f}s, {len(fixed_text)} chars)")
            fixed_path = save_fixed(level, project_dir, out_dir, idx, name,
                                    fixed_text, present)
            dlog(MODEL=agent["model"], EFFORT=agent["effort"] or "-",
                 ACTION="fix", STATUS="ok",
                 PROJECT=project, LEVEL=level, AGENT=name,
                 CHARS_IN=len(current_doc), CHARS_OUT=len(fixed_text),
                 DELTA=f"{len(fixed_text) - len(current_doc):+d}",
                 DURATION=f"{t1 - t0:.0f}s",
                 SAVED=Path(fixed_path).name)
            attempt["fixed_path"] = str(fixed_path)
            if score is not None and score == res.best_score and name == res.best_agent:
                res.best_fixed_path = str(fixed_path)
            # Кумулятив: следующий агент будет review-ить эту fixed-версию
            current_doc = fixed_to_doctext(level, fixed_text, present)
        except Exception as e:
            log(f"  fix FAIL: {e}")
            dlog(MODEL=agent["model"], EFFORT=agent["effort"] or "-",
                 ACTION="fix", STATUS="err",
                 PROJECT=project, LEVEL=level, AGENT=name,
                 ERR=str(e).replace(" ", "_")[:120])
            attempt["fix_error"] = str(e)

        res.attempts.append(attempt)
        state.setdefault("attempts", {}).setdefault(key, []).append(attempt)
        if res.best_score:
            state["best"][key] = {
                "score": res.best_score, "agent": res.best_agent,
                "fixed_path": res.best_fixed_path,
            }
        save_state(state)

    # Финал уровня
    best_path = out_dir / f"{level.upper()}_BEST.md"
    summary = (
        f"# {project}/{level} — BEST\n\n"
        f"**Score: {res.best_score}/55**  "
        f"({'GOAL' if res.goal_reached else 'no-goal'})\n"
        f"Agent: **{res.best_agent}**\n\n"
        f"Fixed artifact: {res.best_fixed_path}\n"
    )
    best_path.write_text(summary, encoding="utf-8")
    log(f"[{project}/{level}] DONE — best {res.best_score}/55 ({res.best_agent}); "
        f"goal={res.goal_reached}")
    dlog(VERDICT="level_verdict", PROJECT=project, LEVEL=level,
         BEST=f"{res.best_score}/55", BEST_AGENT=res.best_agent,
         GOAL=res.goal_reached)
    return res


def run_project(project: str, project_dir: Path, levels: list[str],
                only_agent: Optional[int], state: dict) -> dict:
    log(f"### PROJECT {project} @ {project_dir} ###")
    dlog(EVENT="project_start", PROJECT=project, DIR=str(project_dir),
         LEVELS=",".join(levels))
    if not project_dir.is_dir():
        log(f"  project dir missing — skip")
        dlog(EVENT="project_skip", PROJECT=project, REASON="dir_missing")
        return {"project": project, "skipped": True}
    pres = {"project": project, "levels": {}}
    any_pass = False
    # Перед уровнем core убедиться, что все 4 core-файла существуют.
    if "core" in levels:
        ensure_core_files(project, project_dir)
    for lvl in levels:
        lr = run_level(project, project_dir, lvl, only_agent, state)
        pres["levels"][lvl] = {
            "best_score": lr.best_score,
            "best_agent": lr.best_agent,
            "goal_reached": lr.goal_reached,
            "skipped": lr.skipped,
            "skip_reason": lr.skip_reason,
        }
        if lr.goal_reached:
            any_pass = True
    pres["any_pass"] = any_pass
    write_project_report(project, pres)
    state["completed"][project] = pres
    save_state(state)
    scores = {l: pres["levels"].get(l, {}).get("best_score", 0) for l in LEVELS}
    dlog(VERDICT="project_verdict", PROJECT=project,
         CONCEPT=f"{scores['concept']}/55",
         CORE=f"{scores['core']}/55",
         FULL=f"{scores['full']}/55",
         STATUS="PASS" if any_pass else "FAIL")
    return pres


def write_project_report(project: str, pres: dict):
    out = OUT_ROOT / project
    out.mkdir(parents=True, exist_ok=True)
    lines = [f"# {project} — TBPR FINAL REPORT", ""]
    lines.append("| Level | Best | Agent | Goal | Skipped |")
    lines.append("|-------|------|-------|------|---------|")
    for lvl in LEVELS:
        lvl_data = pres["levels"].get(lvl, {})
        lines.append(
            f"| {lvl} | {lvl_data.get('best_score', 0)}/55 | "
            f"{lvl_data.get('best_agent', '-')} | "
            f"{'PASS' if lvl_data.get('goal_reached') else '-'} | "
            f"{'yes' if lvl_data.get('skipped') else '-'} |"
        )
    lines.append("")
    lines.append(f"**Overall: {'PASS' if pres['any_pass'] else 'FAIL'}**")
    (out / "FINAL_REPORT.md").write_text("\n".join(lines), encoding="utf-8")


def write_overall_report(state: dict):
    lines = ["# TBPR OVERALL REPORT — All Projects", ""]
    lines.append("| Project | Concept | Core | Full | Status |")
    lines.append("|---------|---------|------|------|--------|")
    for proj, _ in PROJECTS_ORDER:
        comp = state.get("completed", {}).get(proj)
        if not comp:
            lines.append(f"| {proj} | - | - | - | not run |")
            continue
        c = comp["levels"].get("concept", {}).get("best_score", 0)
        co = comp["levels"].get("core", {}).get("best_score", 0)
        fu = comp["levels"].get("full", {}).get("best_score", 0)
        st = "PASS" if comp["any_pass"] else f"FAIL (best {max(c, co, fu)}/55)"
        lines.append(f"| {proj} | {c}/55 | {co}/55 | {fu}/55 | {st} |")
    (OUT_ROOT / "OVERALL_REPORT.md").write_text("\n".join(lines), encoding="utf-8")


# ---------- CLI ----------

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--project", help="run a single project by short name")
    ap.add_argument("--all", action="store_true", help="run all 12 projects")
    ap.add_argument("--level", choices=LEVELS, help="restrict to one level")
    ap.add_argument("--only-agent", type=int, help="run only agent N (1..9)")
    ap.add_argument("--resume", action="store_true",
                    help="skip projects already completed in state file")
    args = ap.parse_args()

    OUT_ROOT.mkdir(parents=True, exist_ok=True)
    state = load_state()

    if args.project:
        targets = [(n, p) for n, p in PROJECTS_ORDER if n == args.project]
        if not targets:
            log(f"unknown project: {args.project}")
            sys.exit(2)
    elif args.all:
        targets = list(PROJECTS_ORDER)
    else:
        log("specify --project or --all")
        sys.exit(2)

    levels = [args.level] if args.level else LEVELS
    log(f"=== orchestrator start: targets={[n for n, _ in targets]}, "
        f"levels={levels}, only_agent={args.only_agent} ===")
    dlog(EVENT="run_start", PID=os.getpid(),
         TARGETS=",".join(n for n, _ in targets),
         LEVELS=",".join(levels),
         ONLY_AGENT=str(args.only_agent or "-"),
         RESUME=str(args.resume))

    for name, pdir in targets:
        if args.resume and state.get("completed", {}).get(name, {}).get("any_pass"):
            log(f"resume: skip {name} (already PASS)")
            continue
        try:
            run_project(name, pdir, levels, args.only_agent, state)
        except Exception as e:
            log(f"PROJECT {name} CRASH: {e}\n{traceback.format_exc()}")

    write_overall_report(state)
    log("=== orchestrator done ===")
    dlog(EVENT="run_end", PID=os.getpid(), STATUS="ok")


if __name__ == "__main__":
    main()
