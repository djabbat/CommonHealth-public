# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 2
- Deliv: 1
- Novelty: 4
- Risk: 3
- RefIntegrity: 3
- EvidenceDepth: 2
- MethodDepth: 1
- Reproducibility: 1

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

### 1. Operationalised falsifiability (numeric thresholds)
✗ — Формальные пороги есть (concordance >0.80, uptime >0.90, contamination <0.03), но они противоречат THEORY.md (±20% deviation, uptime <80%, contamination >10%). Несогласованность между документами. Также отсутствует числовой порог для **каждого** ключевого утверждения (например, stage accuracy, LED bleaching). Требуются единые, непротиворечивые пороги.

### 2. Pre-registration plan (OSF placeholder + date)
✓ — Указан OSF ID "osf.io/TBD" и дата 2026-06-01. Однако ID невалидный; формально placeholder допустим, но отсутствует указание платформы (OSF) в явном виде в CONCEPT.md (есть в другом разделе). Слабый плюс.

### 3. Sample size calc (power analysis)
✗ — В CONCEPT.md есть два конкурирующих расчёта: один даёт n = TBD (с TBD для σ² и δ), другой — n=30/группу. Противоречие. В THEORY.md расчёт n=286 для concordance. Отсутствует единый, финальный, согласованный расчёт. Не выполнено.

### 4. Risk matrix ≥5 rows
✓ — Присутствует две матрицы (6+6 строк). Однако формат probability/impact не унифицирован (текстовые Medium/Low и числовые 1-5). Нарушение формального требования "probability × impact × mitigation", но количественно ≥5 строк есть.

### 5. Limitations section
✓ — Присутствует в нескольких вариантах (в CONCEPT.md, EVIDENCE.md). Несмотря на дублирование, раздел есть и охватывает основные ограничения.

### 6. Consortium / collaboration plan
✓ — Таблица с партнёрами, ролями и статусом. Некоторые партнёры TBD, но план представлен.

### 7. Reference reality + match
✗ — Проверены все 9 ссылок из EVIDENCE.md — все реальны и соответствуют тексту. **Но** в CONCEPT.md есть утверждения без ссылок (например, про stage accuracy ±2 µm, LED bleaching, низкий уровень AI hallucination). Также есть маркеры [Placeholder: ...] и [Reference needed] (удалённые, но следы остались). Для **каждого** ключевого утверждения ссылка отсутствует — нарушение.

### 8. No fabrication markers
✗ — Множественные "TBD", "placeholder", "σ² = TBD", "δ = TBD", "n = TBD", OSF ID "TBD", "/* [Reference needed] */" (хотя удалён, но в заголовке EVIDENCE.md осталась строка об удалении). Fabrication markers присутствуют в образцах расчёта мощности, OSF ID, consortium status.

### 9. Internal consistency core docs
✗ — Критические противоречия:
- CONCEPT.md: uptime ≥95%, contamination ≤3%; THEORY.md: uptime <80%, contamination >10% — различаются.
- Sample size: CONCEPT.md: n=TBD и n=30; THEORY.md: n=286 — несогласованы.
- Risk matrix форматы различаются (текстовые vs числовые).
- Limitations списки дублируются с разными пунктами.
- Evidence base: в CONCEPT.md утверждается, что мета-анализа нет, но в EVIDENCE.md не упоминается поиск.

### 10. Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed)
✗ — 
- AI-operated microscopy: 3 источника (Burger, Boiko, Bran) — OK.
- Low-cost retrofit: 2 источника (OpenFlexure, Zeiss manual) — <3.
- Environmental control: 1 источник (Hayflick) + manufacturer spec — <3.
- Cell segmentation: 1 источник (Stringer) — <3.
- Antibodies: по 1 источнику — <3.
- Нет цитирования ни одного систематического обзора или мета-анализа.
- Противоречия не упомянуты (сказано, что не найдены, но это само по себе проблемно — явное игнорирование известных данных об ошибках AI-фокусировки в low-contrast образцах, например).

### 11. Methodology depth (replication-ready protocol, SAP, controls, replication strategy)
✗ — 
- Step-by-step protocol описан крайне поверхностно (5 строк в EVIDENCE.md) — недостаточно для репликации.
- SAP есть (concordance, Cohen's kappa, Bonferroni, LOCF), но не детализирован (например, нет плана по missed data — только LOCF, который часто смещён; нет sensitivity analysis).
- Replication strategy: split-sample (70/30) + внешняя (TBD) — указано, но внешняя не специфицирована.
- Controls: positive (human expert) и negative (random decisions) — OK.
- Blinding: заявлено, но не описано, как достигается (двойной? тройной?).
- Нет протокола на protocols.io или эквиваленте.
- Отсутствует анализ статистической мощности для вторичных конечных точек.

### 12. Reproducibility & open science (code, data, pre-reg, materials)
✗ — 
- Code: обещание "будет выложен по принятии" — на момент ревью нет репозитория, нет ссылки.
- Data: обещание на Zenodo/OSF, ссылки TBD.
- Pre-reg: osf.io/TBD (невалидный, нет содержания).
- Materials: protocols.io TBD, requirements.txt обещан, но не предоставлен.
- Hardware BOM не включён в пакет (только упоминание в AUTOMATED_MICROSCOPY_SETUP.md, который не представлен).

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|-----------------------|---------|
| 1 | Sharkey et al. 2016 "OpenFlexure Microscope" | 10.1063/1.4941068 | Да | Да (утверждение про XY stage accuracy ±5µm) | OK |
| 2 | Stringer et al. 2021 "CellPose v2" | 10.1038/s41592-020-01018-x | Да | Да | OK |
| 3 | Hayflick 1965 "fibroblast culture conditions" | PMID 14315085 | Да | Да | OK |
| 4 | Wolff et al. 1992 "GT335 antibody" | PMID 1385210 | Да | Да | OK |
| 5 | Delgehyr et al. 2005 "Ninein antibody" | 10.1242/jcs.02302 | Да | Да | OK |
| 6 | Burger et al. 2020 "autonomous chemistry robot" | 10.1038/s41586-020-2442-2 | Да | Да | OK |
| 7 | Boiko et al. 2023 "GPT-4 chemical synthesis" | 10.1038/s41586-023-06792-0 | Да | Да | OK |
| 8 | Bran et al. 2024 "ChemCrow" | 10.1038/s42256-024-00832-8 | Да | Да | OK |
| 9 | Schindelin et al. 2012 "ImageJ/Fiji" | 10.1038/nmeth.2019 | Да | Да | OK |

**Дополнительно:** В тексте CONCEPT.md есть ссылки на "Zeiss product manual 1985" и "FLIR product datasheet" — это производители, не DOIs, но они реальны. Утверждение "Inkbird ITC-100 controller specification claims ±0.3°C stability" — без DOI, но это datasheet. Проблема: некоторые утверждения (stage accuracy, LED bleaching) не подкреплены ссылками. Ссылка на OpenFlexure используется для XY accuracy, но затем в тексте используется ±2 µm (не из OpenFlexure). Несоответствие.

## Evidence depth audit (новое v3.0)

| # | Ключевое утверждение | Источников цитировано | Включён ли мета-анализ/систематический обзор? | Противоречия учтены? |
|---|----------------------|-----------------------|-----------------------------------------------|-----------------------|
| 1 | AI-operated microscopy feasible | 3 (Burger, Boiko, Bran) | Нет (сказано, что не найден) | Нет (утверждается, что противоречий нет, но известно об ошибках AI в low-contrast) |
| 2 | Low-cost retrofit feasible | 2 (OpenFlexure, Zeiss manual) | Нет | Нет |
| 3 | Environmental control ±0.5°C/CO₂ | 1 (Hayflick) + 1 datasheet | Нет | Нет |
| 4 | CellPose segments senescence cells | 1 (Stringer) | Нет | Нет (не обсуждается применимость к стареющим клеткам) |
| 5 | GT335 antibody specificity | 1 (Wolff) | Нет | Нет |
| 6 | Ninein antibody specificity | 1 (Delgehyr) | Нет | Нет |
| 7 | Stage accuracy ±2 µm (placeholder) | 0 | Нет | Нет |
| 8 | LED bleaching negligible | 0 | Нет | Нет |

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

### file: `CONCEPT.md`
- **Раздел "Sample size calculation"** — удалить дублирующий расчёт с TBD; оставить единый расчёт (n=30/группу) и явно указать, что pilot оценит σ и δ, но финальный N будет рассчитан до пре-регистрации.
- **Раздел "Risk matrix"** — унифицировать формат (числовые probabilities и impacts 1-5), обеспечить пересечение с THEORY.md.
- **Раздел "Limitations"** — объединить в один список, исключить дублирование.
- **Раздел "Evidence base & meta-analysis"** — добавить ссылки на систематические обзоры (например, по AI microscopy) или явно указать, что не найдено, но цитировать более широкие обзоры по автоматизации. Для каждого утверждения ≥3 источника или объяснить, почему это невозможно.
- **Pre-registration** — заменить `osf.io/TBD` на конкретный зарегистрированный ID (хотя бы draft) или указать дату