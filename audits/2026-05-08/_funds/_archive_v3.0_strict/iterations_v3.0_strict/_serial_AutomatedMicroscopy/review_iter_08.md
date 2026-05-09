# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsifiability: 4
- Deliv: 4
- Novelty: 4
- Risk: 4
- RefIntegrity: 3
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 3

## Checklist (✓/✗ + объяснение)

1. **Operationalised falsifiability (numeric thresholds)** — ✓ Частично. Для M1 указаны α=0.05, power=0.80, N=286, δ=0.05. Для uptime и contamination есть критерии, но sample size для contamination — TBD (placeholder). Строго: фальсифицируемость операционализирована для главной гипотезы, но не для всех вторичных. Считаем выполнено с замечанием.

2. **Pre-registration plan (OSF placeholder + date)** — ✗ Есть placeholder `osf.io/TBD` и дата 2026-06-01. Формально план есть, но идентификатор не указан. Это borderline. В сочетании с другими TBD — скорее невыполнение, т.к. требуется конкретный placeholder, а не `TBD`.

3. **Sample size calc (power analysis)** — ✓ Частично. Есть power analysis для CDATA (n=30/группа, Cohen's d=0.75, α=0.05, power=0.80) и для M1 (N=286). Для contamination — N=TBD. Пункт выполнен не полностью.

4. **Risk matrix ≥5 rows** — ✓ В CONCEPT.md 6 строк, в EVIDENCE.md тоже ≥5. Выполнено.

5. **Limitations section** — ✓ Явный раздел в CONCEPT.md (8 пунктов) и в EVIDENCE.md. Выполнено.

6. **Consortium / collaboration plan** — ✓ Частично. Таблица партнёров с ролями есть, но статусы многих "TBD" или "placeholder". Формально план есть.

7. **Reference reality + match** — ✗ Все указанные DOI/PMID реальны (проверено). Однако в CONCEPT.md раздел "Evidence base & meta-analysis" содержит плейсхолдеры `[Placeholder: e.g., OpenTrons, µManager, or similar open-source microscopy automation projects]` — это не ссылка, а fabrication marker. Также в том же разделе `[Author(s), Year, Journal, DOI TBD]`. Это нарушает пункт 7 (ссылка должна быть реальной). Вывод: нарушено.

8. **No fabrication markers** — ✗ Множество TBD, placeholder, osf.io/TBD, "N = TBD", "design effect: DE = TBD", "Required N: TBD", "sample size: n = TBD", "σ² = TBD", "δ = TBD", "Repository: TBD", "DOI: TBD". Это фабрикационные маркеры. Пункт не выполнен.

9. **Internal consistency core docs** — ✓ В целом согласовано: CONCEPT, THEORY, EVIDENCE не противоречат друг другу. Небольшие расхождения (например, в EVIDENCE есть таблица ссылок, а в CONCEPT сказано "No systematic review performed") — не противоречие. Выполнено.

10. **Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed)** — ✗
    (a) Ключевое утверждение M1 (AI-operated microscopy ≥80% quality) опирается только на 3 работы по AI в химии (Burger, Boiko, Bran) — это не про микроскопию и не три независимых источника для данного утверждения. Нет работ, демонстрирующих LLM-управление микроскопом.
    (b) Систематический обзор или мета-анализ не проведены и не процитированы. В CONCEPT прямо указано "No systematic review or meta-analysis was performed".
    (c) Cochrane/PRISMA отсутствует.
    (d) Противоречия упомянуты лишь в общих чертах ("Some studies report higher error rates for AI-based focus adjustment") без конкретных ссылок. Недостаточно.
    (e) State-of-the-art описан. Пункт не выполнен.

11. **Methodology depth (replication-ready protocol, SAP, controls, replication strategy)** — ✗
    (a) Протокол описан в 5 шагов, но не содержит конкретных команд, параметров, не воспроизводим независимо. Ссылка на `AUTOMATED_MICROSCOPY_SETUP.md` — но этот файл не предоставлен. Репликация невозможна.
    (b) SAP есть (endpoints, correction, missing data). Но LOCF — устаревшая практика.
    (c) Replication strategy (split-sample + external) указана.
    (d) Controls (positive/negative) указаны.
    (e) Blinding/randomisation указаны.
    Из-за (a) пункт не выполнен.

12. **Reproducibility & open science (code, data, pre-reg, materials)** — ✗
    (a) Code: обещание "будет выложен на acceptance", репозиторий TBD. Это не конкретная ссылка.
    (b) Data: Zenodo/OSF, но DOI TBD.
    (c) Pre-reg: osf.io/TBD, 2026-06-01.
    (d) Materials: protocols.io DOI TBD, requirements.txt "to be included".
    Формально обещания есть, но все с TBD. Учитывая строгость требований (ссылка на репозиторий или явное обещание), — это нарушение.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | OpenFlexure (Sharkey et al. 2016) | 10.1063/1.4941068 | Да | Да (описывает DIY stage) | OK |
| 2 | Micro-Manager | open-source (нет DOI) | N/A (software) | Да | OK |
| 3 | Hayflick 1965 | 10.1016/