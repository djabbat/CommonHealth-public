#!/usr/bin/env python3
"""
TBPR Project Overnight Cycle v2 — с перебором моделей по кругу.

Логика:
  1. Первые 6 циклов — champion (deepseek-v4-pro, лучшая для reasoning)
  2. Затем перебор всех моделей из списка, каждой — 3 попытки
  3. Пока список не кончится

Usage:
    python3 tbpr_project_overnight_v2.py                    # все проекты
    python3 tbpr_project_overnight_v2.py --project MCOA    # один проект
"""

import os, sys, re, json, logging, argparse, subprocess, shlex
from pathlib import Path
from datetime import datetime

sys.path.insert(0, str(Path(__file__).resolve().parent))
from llm import ask, ask_reasoner

HERE = Path(__file__).resolve().parent
LC_ROOT = HERE.parent
OUTPUT_DIR = HERE / "tbpr_project_output"
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

TARGET_SCORE = 45
MAX_CYCLES = 999  # unlimited — пока список не кончится
SCORE_MAX = 55

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[
        logging.StreamHandler(),
        logging.FileHandler(str(OUTPUT_DIR / "tbpr_project_v2.log"), mode="a")
    ]
)
log = logging.getLogger("tbpr_project_v2")

PROJECTS = {
    "CDATA": {"concept": LC_ROOT / "CDATA" / "CONCEPT.md",
              "tbpr_dir": LC_ROOT / "CDATA" / "docs" / "tbpr"},
    "MCOA": {"concept": LC_ROOT / "MCOA" / "CONCEPT.md",
             "tbpr_dir": LC_ROOT / "MCOA" / "docs" / "tbpr"},
    "Ze": {"concept": LC_ROOT / "Ze" / "CONCEPT.md",
           "tbpr_dir": LC_ROOT / "Ze" / "docs" / "tbpr"},
    "AIM": {"concept": LC_ROOT / "AIM" / "CONCEPT.md",
            "tbpr_dir": LC_ROOT / "AIM" / "docs" / "tbpr"},
    "BioSense": {"concept": LC_ROOT / "BioSense" / "CONCEPT.md",
                 "tbpr_dir": LC_ROOT / "BioSense" / "docs" / "tbpr"},
}
EXPERIMENTS = {
    "MitoROS": {"concept": LC_ROOT / "CDATA" / "experiments" / "MitoROS" / "CONCEPT.md",
                "tbpr_dir": LC_ROOT / "CDATA" / "experiments" / "MitoROS" / "docs" / "tbpr"},
    "Telomere": {"concept": LC_ROOT / "CDATA" / "experiments" / "Telomere" / "CONCEPT.md",
                 "tbpr_dir": LC_ROOT / "CDATA" / "experiments" / "Telomere" / "docs" / "tbpr"},
    "Proteostasis": {"concept": LC_ROOT / "CDATA" / "experiments" / "Proteostasis" / "CONCEPT.md",
                     "tbpr_dir": LC_ROOT / "CDATA" / "experiments" / "Proteostasis" / "docs" / "tbpr"},
    "EpigeneticDrift": {"concept": LC_ROOT / "CDATA" / "experiments" / "EpigeneticDrift" / "CONCEPT.md",
                        "tbpr_dir": LC_ROOT / "CDATA" / "experiments" / "EpigeneticDrift" / "docs" / "tbpr"},
    "CellLineageTree": {"concept": LC_ROOT / "CDATA" / "experiments" / "CellLineageTree" / "CONCEPT.md",
                        "tbpr_dir": LC_ROOT / "CDATA" / "experiments" / "CellLineageTree" / "docs" / "tbpr"},
    "AutomatedMicroscopy": {"concept": LC_ROOT / "CDATA" / "experiments" / "AutomatedMicroscopy" / "CONCEPT.md",
                            "tbpr_dir": LC_ROOT / "CDATA" / "experiments" / "AutomatedMicroscopy" / "docs" / "tbpr"},
    "E0": {"concept": LC_ROOT / "CDATA" / "experiments" / "E0" / "CONCEPT.md",
           "tbpr_dir": LC_ROOT / "CDATA" / "experiments" / "E0" / "docs" / "tbpr"},
}
ALL_PROJECTS = {**PROJECTS, **EXPERIMENTS}


# ═══════════════════════════════════════════════════════════════════════════════
# МОДЕЛИ — полный список из pi --list-models
# ═══════════════════════════════════════════════════════════════════════════════

ALL_MODELS = [
    # provider|model
    "deepseek|deepseek-v4-flash",
    "deepseek|deepseek-v4-pro",
    "google|gemini-1.5-flash",
    "google|gemini-1.5-flash-8b",
    "google|gemini-1.5-pro",
    "google|gemini-2.0-flash",
    "google|gemini-2.0-flash-lite",
    "google|gemini-2.5-flash",
    "google|gemini-2.5-flash-lite",
    "google|gemini-2.5-pro",
    "google|gemini-3-flash-preview",
    "google|gemini-3-pro-preview",
    "google|gemini-3.1-flash-lite-preview",
    "google|gemini-3.1-pro-preview",
    "google|gemma-3-27b-it",
    "google|gemma-4-26b-a4b-it",
    "google|gemma-4-31b-it",
    "groq|deepseek-r1-distill-llama-70b",
    "groq|gemma2-9b-it",
    "groq|groq/compound",
    "groq|groq/compound-mini",
    "groq|llama-3.1-8b-instant",
    "groq|llama-3.3-70b-versatile",
    "groq|llama3-70b-8192",
    "groq|llama3-8b-8192",
    "groq|meta-llama/llama-4-maverick-17b-128e-instruct",
    "groq|meta-llama/llama-4-scout-17b-16e-instruct",
    "groq|mistral-saba-24b",
    "groq|moonshotai/kimi-k2-instruct",
    "groq|moonshotai/kimi-k2-instruct-0905",
    "groq|openai/gpt-oss-120b",
    "groq|openai/gpt-oss-20b",
    "groq|openai/gpt-oss-safeguard-20b",
    "groq|qwen/qwen3-32b",
    "groq|qwen-qwq-32b",
]

CHAMPION = "deepseek|deepseek-v4-pro"  # лучшая для reasoning
CHAMPION_CYCLES = 6   # первых 6 циклов — champion
ATTEMPTS_PER_MODEL = 3  # каждой модели — 3 попытки

# ═══════════════════════════════════════════════════════════════════════════════
# Model Router
# ═══════════════════════════════════════════════════════════════════════════════

class ModelRouter:
    """Определяет, какую модель использовать на каждом цикле."""
    
    def __init__(self, champion_cycles=6, attempts_per_model=3):
        self.cycle = 0
        self.model_index = 0
        self.model_attempt = 0
        self.phase = "champion"
        self.current_model = CHAMPION
        self.history = []
        self.champion_cycles = champion_cycles
        self.attempts = attempts_per_model
    
    def next_model(self) -> str:
        """Возвращает модель для следующего цикла."""
        self.cycle += 1
        self.model_attempt += 1
        
        if self.cycle <= self.champion_cycles:
            self.phase = "champion"
            self.current_model = CHAMPION
        else:
            # Фаза cycling
            self.phase = "cycling"
            
            if self.model_attempt > self.attempts:
                self.model_attempt = 1
                self.model_index += 1
            
            # Если вышли за границы списка — останавливаемся
            if self.model_index >= len(ALL_MODELS):
                self.current_model = None
                self.history.append((self.cycle, None, "exhausted"))
                return None
            
            self.current_model = ALL_MODELS[self.model_index]
        
        self.history.append((self.cycle, self.current_model, self.phase))
        return self.current_model
    
    def get_current_model_name(self) -> str:
        if self.current_model:
            return self.current_model.split("|")[1]
        return "NONE"
    
    def get_current_provider(self) -> str:
        if self.current_model:
            return self.current_model.split("|")[0]
        return "NONE"
    
    def exhausted(self) -> bool:
        return self.current_model is None and self.cycle > self.champion_cycles


# ─── System prompts (unchanged) ───────────────────────────────────────────────

SYSTEM_TBPR_PROJECT = """You are a grant review panel conducting a TRIPLE-BLIND PEER REVIEW (TBPR) of a project concept document.

Evaluate this document as a scientific research project proposal (EIC Pathfinder / ERC / NIH format).

Produce EXACTLY the following structure:

# Triple-Blind Peer Review: PROJECT [Name]

## Evaluation Framework: [EIC Pathfinder / ERC AdG / NIH R01] (choose best fit)

---

# REVIEWER A: [Role e.g. Senior Scientist / Domain Expert]

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
[Strengths, Critical Weaknesses, Other Concerns]

---

# REVIEWER B: [Role]

## Scores
[Same table]
**Score Sum: XX/55**

## Fluff & Padding Audit
[Review of signal-to-noise ratio, repetition, filler content]

---

# REVIEWER C: [Role]

## Scores
[Same table]
**Score Sum: XX/55**

## Counter-argument & Bias Audit
[Devil's advocate review, bias detection]

---

# Combined Verdict

## Overall Assessment
[Summary paragraph]

## Recommendation
**[ACCEPT / REVISE_MAJOR / REVISE_MINOR / REJECT]**

## Combined Score: **XX/55**

## Top 3 Actions for Author
1. ...
2. ...
3. ...

IMPORTANT: Score each of 11 criteria 1-5 for each reviewer. Sum = Score Sum/55. Combined Score = MIN of 3 Score Sums."""


SYSTEM_FIX_PROJECT = """You are revising a PROJECT CONCEPT DOCUMENT in response to a TBPR peer review for grant funding.

You are given:
1. The ORIGINAL CONCEPT document
2. The TBPR REVIEW with specific recommendations

Your task: Apply ALL fixable recommendations to produce an improved version that would score higher.

Rules:
- Preserve the project's core idea and vision.
- Address all "Fixable issues" and "Top 3 actions".
- For blocking issues: fix those that can be fixed (add PI track record, add budget, fix references, add falsifiability criteria). 
- For truly unfixable blocking issues (e.g., "not a research project"), explain how to reframe.
- Add missing sections: preliminary data, methodology details, risk matrix, budget estimates, PI qualifications.
- Replace vague claims with specific, verifiable statements.
- Return the COMPLETE revised document, no commentary."""


# ═══════════════════════════════════════════════════════════════════════════════
# API call через pi CLI
# ═══════════════════════════════════════════════════════════════════════════════

def call_pi(prompt: str, provider: str, model: str, system_prompt: str = "",
            temperature: float = 0.1, max_tokens: int = 16384, timeout: int = 300) -> str:
    """Вызывает модель через pi CLI."""
    
    cmd = ["pi", "--provider", provider, "--model", model, "-p", "--print",
           "--temperature", str(temperature), "--max-tokens", str(max_tokens),
           "--system-prompt", system_prompt]
    
    log.info(f"  🖥 pi --provider {provider} --model {model}")
    
    try:
        result = subprocess.run(
            cmd,
            input=prompt,
            capture_output=True,
            text=True,
            timeout=timeout
        )
        if result.returncode != 0:
            log.warning(f"  ⚠ pi вернул код {result.returncode}: {result.stderr[:200]}")
            # fallback на DeepSeek
            log.info("  → fallback на ask_reasoner")
            return ask_reasoner(prompt, max_tokens=max_tokens, temperature=temperature)
        
        output = result.stdout.strip()
        if not output or len(output) < 10:
            log.warning(f"  ⚠ pi вернул пустой ответ")
            return ask_reasoner(prompt, max_tokens=max_tokens, temperature=temperature)
        
        return output
    
    except subprocess.TimeoutExpired:
        log.warning(f"  ⚠ pi timeout {timeout}s, fallback на ask_reasoner")
        return ask_reasoner(prompt, max_tokens=max_tokens, temperature=temperature)
    except Exception as e:
        log.warning(f"  ⚠ pi error: {e}, fallback на ask_reasoner")
        return ask_reasoner(prompt, max_tokens=max_tokens, temperature=temperature)


def call_model(prompt: str, provider: str, model: str,
               temperature: float = 0.1, max_tokens: int = 16384,
               timeout: int = 300, system_prompt: str = "") -> str:
    """Универсальный вызов с fallback."""
    
    # Для DeepSeek используем прямой API (надёжнее)
    if provider == "deepseek":
        ds_model = "deepseek-reasoner" if "pro" in model else "deepseek-chat"
        return ask_reasoner(prompt, max_tokens=max_tokens, temperature=temperature)
    
    # Для остальных — через pi CLI
    return call_pi(prompt, provider, model, system_prompt=system_prompt,
                   temperature=temperature, max_tokens=max_tokens, timeout=timeout)


# ═══════════════════════════════════════════════════════════════════════════════
# Parsing (copied from original)
# ═══════════════════════════════════════════════════════════════════════════════

def parse_all_scores(tbpr_text: str) -> dict:
    scores = {}
    combined_block = re.search(r'##\s*Combined\s+Score.*?(?=\n##|\Z)', tbpr_text, re.DOTALL)
    if combined_block:
        block = combined_block.group(0)
        abcs = re.findall(r'(Reviewer\s+[ABC])\s+Score\s+Sum:\s*(\d+)/55', block)
        for name, val in abcs:
            scores[name.strip()] = int(val)
    if scores:
        return scores
    m = re.findall(r'\*\*Score\s+Sum\*\*[^|]*\|\s*\*\*(\d+)\s*/\s*55\*\*', tbpr_text)
    if m:
        for i, val in enumerate(m[:3]):
            scores[f'R{i+1}'] = int(val)
        return scores
    m = re.findall(r'Score\s+Sum[^:]*?[:=]\s*(\d+)\s*/\s*55', tbpr_text)
    if m:
        for i, val in enumerate(m[:3]):
            scores[f'R{i+1}'] = int(val)
        return scores
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
        r'worst of 3[^)]*?\)\s*=\s*(\d+)/55',
        r'Combined\s+Score[^:]*:\s*(\d+)\s*/\s*55',
    ]
    for p in patterns:
        m = re.search(p, tbpr_text)
        if m:
            try: return int(m.group(1))
            except: continue
    scores = parse_all_scores(tbpr_text)
    return min(scores.values()) if scores else 0

def parse_verdict(tbpr_text: str) -> str:
    patterns = [
        r'Overall\s+Recommendation:\s*\*\*([^*]+)\*\*',
        r'##\s*Combined\s+verdict[^#]*\n\*\*([^*]+)\*\*',
        r'Recommendation:\s*\*\*([^*]+)\*\*',
        r'##\s*Overall\s+[Aa]ssessment[^#]*?\n[^*]*?(ACCEPT|REJECT|REVISE)',
    ]
    for p in patterns:
        m = re.search(p, tbpr_text)
        if m: return m.group(1).strip()
    return "UNKNOWN"


# ═══════════════════════════════════════════════════════════════════════════════
# Core
# ═══════════════════════════════════════════════════════════════════════════════

def run_tbpr(concept_text: str, project_name: str, version: int,
             provider: str, model: str) -> str:
    log.info(f"[{project_name}] === TBPR-project Cycle v{version} [{model}] ===")
    prompt = f"""=== PROJECT CONCEPT for TBPR PROJECT REVIEW (version {version}) ===
Project: {project_name}

{concept_text}

===

Produce a complete Triple-Blind Peer Review (TBPR) in the specified format.

3 REVIEWERS: A (domain expert — fact-check, methodology, PI), B (fluff/impact auditor), C (red team — counter-arguments, bias).

Each reviewer scores 11 criteria 1-5 (table format), provides **Score Sum: XX/55**.

Combined Score = MIN of 3 Score Sums.
Follow the exact template structure as specified in the system prompt."""
    return call_model(prompt, provider, model, temperature=0.1, system_prompt=SYSTEM_TBPR_PROJECT)


def fix_concept(concept_text: str, tbpr_review: str, project_name: str, version: int,
                provider: str, model: str) -> str:
    log.info(f"[{project_name}] Fixing document based on TBPR review [{model}]...")
    prompt = f"""=== ORIGINAL CONCEPT {project_name} (version {version}) ===
{concept_text}

=== TBPR PROJECT REVIEW ===
{tbpr_review}

===

Produce the COMPLETE revised concept document incorporating all fixable recommendations.
Return ONLY the full revised document, no commentary.
"""
    return call_model(prompt, provider, model, temperature=0.3, timeout=600, system_prompt=SYSTEM_FIX_PROJECT)


def save_artifact(path: Path, content: str, desc: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    log.info(f"  → {desc}: {path}")


def log_score_history(history: list, project_name: str):
    log.info(f"[{project_name}] === Score History ===")
    for h in history:
        model_name = h.get('model', '?')
        s = "; ".join(f"{k}={v}" for k, v in h.get('scores', {}).items())
        log.info(f"  v{h['version']} [{model_name}]: {s} → {h['combined']}/{SCORE_MAX} ({h['verdict']})")


def run_project_cycle(project_name: str, project_info: dict, router: ModelRouter, target: int):
    """Прогоняет TBPR-project цикл, перебирая модели через router."""
    concept_path = project_info["concept"]
    if not concept_path.is_file():
        log.warning(f"[{project_name}] CONCEPT.md not found: {concept_path}")
        return False

    concept_text = concept_path.read_text(encoding="utf-8")
    log.info(f"[{project_name}] Started. CONCEPT.md: {len(concept_text)} chars, "
             f"{len(concept_text.splitlines())} lines")

    proj_out = OUTPUT_DIR / project_name
    proj_out.mkdir(parents=True, exist_ok=True)

    history = []
    current_text = concept_text
    best_score = 0
    best_text = concept_text
    best_version = 0
    cycle = 0

    while True:
        # Берём следующую модель
        model_entry = router.next_model()
        if model_entry is None:
            log.warning(f"[{project_name}] ⛔ Список моделей исчерпан!")
            break

        provider, model_name = model_entry.split("|")
        cycle += 1

        log.info(f"\n[{project_name}] {'='*50}")
        log.info(f"[{project_name}] CYCLE {cycle} — {provider}/{model_name}")
        log.info(f"[{project_name}] {'='*50}")

        # Откат к лучшей
        if current_text is not best_text:
            log.info(f"[{project_name}]  ⬅ Откат к лучшей версии (v{best_version}: {best_score}/{SCORE_MAX})")
            current_text = best_text

        # TBPR
        tbpr_text = run_tbpr(current_text, project_name, cycle, provider, model_name)
        save_artifact(proj_out / f"tbpr_cycle_{cycle:02d}_review.md", tbpr_text, "TBPR Review")

        # Parse
        scores = parse_all_scores(tbpr_text)
        combined_score = parse_combined_score(tbpr_text)
        verdict = parse_verdict(tbpr_text)

        if combined_score == 0 and scores:
            combined_score = min(scores.values())

        is_best = combined_score > best_score
        if is_best:
            best_score = combined_score
            best_text = current_text
            best_version = cycle

        star = " ⭐" if is_best else ""

        entry = {
            "version": cycle,
            "model": f"{provider}/{model_name}",
            "scores": scores,
            "combined": combined_score,
            "verdict": verdict,
            "best": is_best,
        }
        history.append(entry)

        score_str = "; ".join(f"{k}={v}/55" for k, v in scores.items())
        log.info(f"[{project_name}]  Scores: {score_str}")
        log.info(f"[{project_name}]  → Combined: {combined_score}/{SCORE_MAX} "
                 f"({combined_score/SCORE_MAX*100:.1f}%) — {verdict}{star}")
        if is_best:
            log.info(f"[{project_name}]  🆕 Лучший результат! Модель: {provider}/{model_name}")

        # Goal?
        if combined_score >= target:
            log.info(f"\n[{project_name}] 🎉 ЦЕЛЬ ДОСТИГНУТА! {combined_score}/{SCORE_MAX}")
            save_artifact(proj_out / f"CONCEPT_v{cycle}_ACCEPTED.md", current_text, "Accepted CONCEPT")
            concept_path.write_text(current_text, encoding="utf-8")
            log.info(f"[{project_name}]  → CONCEPT.md обновлён!")
            log_score_history(history, project_name)
            return True

        # Если не лучший — откат
        if not is_best and best_score > 0:
            log.info(f"[{project_name}]  ⬇ Score {combined_score} < best {best_score}. Откат к v{best_version} без фикса.")
            current_text = best_text
            continue

        # Fix
        log.info(f"[{project_name}]  Исправляю...")
        fixed_text = fix_concept(current_text, tbpr_text, project_name, cycle, provider, model_name)
        save_artifact(proj_out / f"CONCEPT_v{cycle}_fixed.md", fixed_text, "Fixed CONCEPT")
        current_text = fixed_text

    # End
    if best_text is not concept_text:
        save_artifact(proj_out / f"CONCEPT_BEST_v{best_version}_{best_score}.md", best_text,
                      f"Best CONCEPT (v{best_version}, {best_score}/{SCORE_MAX})")
        concept_path.write_text(best_text, encoding="utf-8")
        log.info(f"[{project_name}]  → CONCEPT.md обновлён до лучшей версии (v{best_version}: {best_score}/{SCORE_MAX})!")
    log_score_history(history, project_name)
    
    # Итог по моделям
    log.info(f"\n[{project_name}] === Model Performance ===")
    model_scores = {}
    for h in history:
        m = h['model']
        if m not in model_scores or h['combined'] > model_scores[m]['best']:
            model_scores[m] = {'best': h['combined'], 'last': h['combined'], 'count': 1}
        else:
            model_scores[m]['last'] = h['combined']
            model_scores[m]['count'] += 1
    for m, data in sorted(model_scores.items(), key=lambda x: -x[1]['best']):
        log.info(f"  {m}: best={data['best']}/55, count={data['count']}")
    
    return False


def main():
    parser = argparse.ArgumentParser(description="TBPR Project v2 — с перебором моделей")
    parser.add_argument("--project", type=str, default="",
                        help="Project name (CDATA, MCOA, Ze, AIM, BioSense)")
    parser.add_argument("--target", type=int, default=TARGET_SCORE)
    parser.add_argument("--champion-cycles", type=int, default=CHAMPION_CYCLES)
    parser.add_argument("--attempts", type=int, default=ATTEMPTS_PER_MODEL)
    args = parser.parse_args()

    if args.project:
        projects = {args.project: ALL_PROJECTS[args.project]}
    else:
        projects = ALL_PROJECTS

    router = ModelRouter(champion_cycles=args.champion_cycles, attempts_per_model=args.attempts)

    log.info(f"\n{'='*60}")
    log.info(f"TBPR v2 — Model Cycling")
    log.info(f"{'='*60}")
    log.info(f"Champion: {CHAMPION} × {args.champion_cycles} циклов")
    log.info(f"Моделей в списке: {len(ALL_MODELS)}")
    log.info(f"Попыток на модель: {args.attempts}")
    log.info(f"Проектов: {', '.join(projects.keys())}")
    log.info(f"{'='*60}\n")

    results = {}
    for name, info in projects.items():
        log.info(f"\n{'#'*60}")
        log.info(f"# ЗАПУСК: {name}")
        log.info(f"{'#'*60}")
        ok = run_project_cycle(name, info, router, args.target)
        results[name] = "PASS" if ok else "FAIL"

    # Итоговый отчёт
    log.info(f"\n{'='*60}")
    log.info("ИТОГОВЫЙ ОТЧЁТ TBPR v2")
    log.info(f"{'='*60}")
    for name, status in results.items():
        log.info(f"  {name}: {status}")
    log.info(f"\nModel usage ({len(router.history)} циклов):")
    for c, m, ph in router.history:
        mname = m.split("|")[1] if m else "exhausted"
        log.info(f"  #{c}: {ph} → {mname}")

    report_path = OUTPUT_DIR / "FINAL_REPORT_v2.md"
    report = [
        "# TBPR-Project v2 — Final Report",
        f"**Date:** {datetime.now().isoformat()}",
        f"**Champion:** {CHAMPION} × {CHAMPION_CYCLES}",
        f"**Models in pool:** {len(ALL_MODELS)}",
        f"**Attempts per model:** {ATTEMPTS_PER_MODEL}",
        f"**Target:** >= {args.target}/{SCORE_MAX}",
        "",
        "## Results",
    ]
    for name, status in results.items():
        report.append(f"- **{name}:** {status}")
    report += [
        "",
        "## Model Cycle History",
    ]
    for c, m, ph in router.history:
        mname = m.split("|")[1] if m else "exhausted"
        report.append(f"- Cycle #{c}: {ph} — {mname}")
    
    (OUTPUT_DIR / "FINAL_REPORT_v2.md").write_text("\n".join(report) + "\n", encoding="utf-8")
    log.info(f"Report: {report_path}")


if __name__ == "__main__":
    main()
