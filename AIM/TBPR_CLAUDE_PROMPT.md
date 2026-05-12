# TBPR 3-Level Project Improvement — Claude Prompt

## Твоя задача

Улучшить все проекты **LongevityCommon** через трёхуровневый TBPR-цикл (Triple-Blind Peer Review), пробивая порог **81% (45/55)**.

---

## 1. Архитектура

### 3 уровня для каждого проекта

1. **Level 1 — CONCEPT.md** (концепция проекта)
2. **Level 2 — Core** (THEORY.md, DESIGN.md, PARAMETERS.md, EVIDENCE.md вместе)
3. **Level 3 — Full** (все документы вместе: интегративная проверка)

### Порядок проектов (снизу вверх)

Сначала подпроекты/эксперименты (самые глубокие), потом основные проекты:

```
E0 → AutomatedMicroscopy → CellLineageTree → EpigeneticDrift →
Proteostasis → Telomere → MitoROS → BioSense → AIM → Ze →
MCOA → CDATA
```

Каждый проект полностью (уровни 1-2-3) перед переходом к следующему.

### Агенты (модели × режимы мышления)

Всего **9 агентов**, каждому 1 попытка на уровень. Лучший результат = финал.

| # | Имя | Модель | Режим |
|---|-----|--------|-------|
| 1 | flash-nothink | deepseek-v4-flash | non-think |
| 2 | flash-low | deepseek-v4-flash | effort=low |
| 3 | flash-high | deepseek-v4-flash | effort=high |
| 4 | flash-max | deepseek-v4-flash | effort=max |
| 5 | chat-high | deepseek-chat | effort=high |
| 6 | reasoner | deepseek-reasoner | default |
| 7 | reasoner-high | deepseek-reasoner | effort=high |
| 8 | reasoner-max | deepseek-reasoner | effort=max |
| 9 | pro-nothink | deepseek-v4-pro | default |

Порядок: от быстрых/дешёвых к мощным/дорогим.

---

## 2. Алгоритм для каждого уровня каждого проекта

```
for each level (CONCEPT → Core → Full):
    1. Прочитать документ(ы) проекта
    2. Собрать всё в один текст
    3. for each agent (flash-nothink → pro-nothink):
        a. Запустить TBPR review (deepseek-reasoner, max_tokens=16384)
        b. Распарсить Score Sum: XX/55 от 3 рецензентов
        c. Combined Score = MIN(3 scores)
        d. Если Combined >= 45/55 → ЦЕЛЬ ДОСТИГНУТА, сохранить, перейти к уровню
        e. Если скор выше лучшего → запомнить как лучший ⭐
        f. Запустить фикс (текущий агент, model + reasoning_effort)
        g. Сохранить tbpr_review и fixed_document
    4. Сохранить лучший результат как финальный
    5. Обновить CONCEPT.md / core / full в проекте
```

Важно: **каждому агенту 1 попытка**. После 9 агентов — лучший результат принимается.

---

## 3. Технические детали

### LLM вызовы

Используй `/home/oem/Desktop/LongevityCommon/AIM/llm.py`:

```python
from llm import ask, ask_reasoner

# TBPR review — всегда deepseek-reasoner
review = ask_reasoner(prompt, max_tokens=16384, temperature=0.1)

# Fix — зависит от агента
# flash-nothink:
ask(prompt, model="deepseek-v4-flash", max_tokens=16384, temperature=0.3, timeout=120)
# flash-low: как выше + reasoning_effort="low" (через params)
# flash-high: + reasoning_effort="high"
# flash-max: + reasoning_effort="max"
# chat-high: model="deepseek-chat", + reasoning_effort="high"
# reasoner: ask_reasoner(prompt, ...)
# reasoner-high: ask_reasoner(prompt, ..., reasoning_effort="high")
# reasoner-max: ask_reasoner(prompt, ..., reasoning_effort="max")
# pro-nothink: ask(prompt, model="deepseek-v4-pro", ...)
```

**Важно**: `reasoning_effort` передаётся через extra body params. В `llm.py` нет прямой поддержки, поэтому используй `requests` напрямую:

```python
import requests, os
key = open(os.path.expanduser('~/.aim_env')).read().split('DEEPSEEK_API_KEY=')[1].split('\n')[0]
payload = {
    "model": model,
    "messages": [{"role": "user", "content": prompt}],
    "max_tokens": max_tokens,
    "temperature": temp,
    "reasoning_effort": effort,  # None/"low"/"high"/"max"
}
resp = requests.post(
    "https://api.deepseek.com/v1/chat/completions",
    headers={"Authorization": f"Bearer {key}", "Content-Type": "application/json"},
    json=payload,
    timeout=timeout,
)
text = resp.json()["choices"][0]["message"]["content"]
```

### Парсинг скора

Из TBPR review нужно извлечь **Score Sum: XX/55** для каждого из 3 рецензентов и Combined Score.

Регулярки:
```python
import re
# Score Sum
sums = re.findall(r'Score\s+Sum[^:]*?[:=]\s*(\d+)\s*/\s*55', text)
# Combined Score
combined = re.search(r'Combined\s+Score[^=]*=\s*(\d+)\s*/\s*55', text)
if combined: score = int(combined.group(1))
else: score = min(int(s) for s in sums[:3])  # fallback
```

---

## 4. Системные промпты

### Для TBPR-review (Level 1 — CONCEPT)

```
You are a grant review panel conducting a TRIPLE-BLIND PEER REVIEW (TBPR)
of a PROJECT CONCEPT DOCUMENT. Evaluate as a grant proposal.

Produce:
- REVIEWER A: domain expert. Scores (1-5): Impact, Approach, Innovation,
  Preliminary Data, PI & Team, Feasibility, Experimental Design, Budget,
  Clarity, Ethics, Overall. **Score Sum: XX/55**
- REVIEWER B: fluff/impact auditor. Same scores. **Score Sum: XX/55**
- REVIEWER C: red team. Same scores. **Score Sum: XX/55**
- Combined Verdict: Combined Score = MIN = XX/55 + Recommendation + Top 3 Actions
```

### Для TBPR-review (Level 2 — Core)

```
You are a TECHNICAL PEER REVIEW panel evaluating CORE SCIENTIFIC DOCUMENTS.
Review: THEORY.md (theory), DESIGN.md (design), PARAMETERS.md (parameters),
EVIDENCE.md (evidence).

Produce:
- REVIEWER A: Scores (1-5): TheorySoundness, DesignCoherence,
  ParameterJustification, EvidenceQuality, Reproducibility,
  InternalConsistency, Completeness, Clarity, Novelty, Overall.
  **Score Sum: XX/55**
- REVIEWER B: cynic. Same scores. **Score Sum: XX/55**
- REVIEWER C: red team. Same scores. **Score Sum: XX/55**
- Combined: **Combined Score: MIN = XX/55** + Top 3 Actions
```

### Для TBPR-review (Level 3 — Full)

```
You are an INTEGRATIVE PEER REVIEW panel evaluating an ENTIRE PROJECT.
Review all docs together: CONCEPT.md, THEORY.md, DESIGN.md, PARAMETERS.md,
EVIDENCE.md. Evaluate cross-document consistency, feasibility, completeness.

Produce:
- REVIEWER A: Scores (1-5): CrossDocConsistency, Feasibility, ScientificMerit,
  Completeness, Reproducibility, Impact, Innovation, Clarity, Ethics, Overall.
  **Score Sum: XX/55**
- REVIEWER B: cynic. Same scores. **Score Sum: XX/55**
- REVIEWER C: red team. Same scores. **Score Sum: XX/55**
- Combined: **Combined Score: MIN = XX/55** + Top 3 Actions
```

### Для Fix (все уровни)

```
=== ORIGINAL CONCEPT [Project] (version [N]) ===
[документ]

=== TBPR REVIEW ===
[review text]

===

Produce the COMPLETE revised concept document incorporating ALL fixable
recommendations. Return ONLY the full revised document, no commentary.
```

---

## 5. Структура файлов проектов

Все проекты в `/home/oem/Desktop/LongevityCommon/`:

| Проект | Путь |
|--------|------|
| E0 | `CDATA/experiments/E0/` |
| AutomatedMicroscopy | `CDATA/experiments/AutomatedMicroscopy/` |
| CellLineageTree | `CDATA/experiments/CellLineageTree/` |
| EpigeneticDrift | `CDATA/experiments/EpigeneticDrift/` |
| Proteostasis | `CDATA/experiments/Proteostasis/` |
| Telomere | `CDATA/experiments/Telomere/` |
| MitoROS | `CDATA/experiments/MitoROS/` |
| BioSense | `BioSense/` |
| AIM | `AIM/` |
| Ze | `Ze/` |
| MCOA | `MCOA/` |
| CDATA | `CDATA/` |

В каждом: `CONCEPT.md`, `THEORY.md`, `DESIGN.md`, `PARAMETERS.md`, `EVIDENCE.md` (не во всех есть все файлы).

---

## 6. Сохранение результатов

Для каждого проекта создать папку `AIM/tbpr_claude_output/<Project>/`:

```
tbpr_claude_output/
  E0/
    concept/
      tbpr_a1_flash-nothink.md
      concept_a1_flash-nothink_fixed.md
      tbpr_a2_flash-low.md
      concept_a2_flash-low_fixed.md
      ...
      CONCEPT_BEST.md
    core/
      tbpr_a1_flash-nothink.md
      core_a1_flash-nothink_fixed.md
      ...
      CORE_BEST.md
    full/
      ...
      FULL_BEST.md
    FINAL_REPORT.md
  AutomatedMicroscopy/
    ...
```

После каждого уровня обновлять соответствующий файл в проекте (CONCEPT.md, THEORY.md и т.д.).

---

## 7. Цель

Для каждого проекта: **пробить Combined Score >= 45/55 (81%)** хотя бы на одном из 3 уровней.

Если проект достигает цели — помечать `🎉 PASS` в итоговом отчёте.
Если после 9 агентов на всех 3 уровнях цель не достигнута — `❌ FAIL (best: X/55)`.

---

## 8. Старт

Начни с **E0** (самый маленький, 56 строк), уровень 1 (CONCEPT), агент 1 (flash-nothink).

После завершения всех 12 проектов напиши итоговый отчёт в `AIM/tbpr_claude_output/OVERALL_REPORT.md`.
