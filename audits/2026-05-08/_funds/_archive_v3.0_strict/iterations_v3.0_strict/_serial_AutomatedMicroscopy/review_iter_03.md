# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- **Premise:** 3 (интересная концепция, но отсутствие систематического обзора и мета-анализа подрывает доказательную базу)
- **Method:** 2 (описано на уровне концепции, но недостаточно деталей для репликации; множество TBD и плейсхолдеров)
- **Evidence:** 2 (часть ссылок реальна, но ключевые утверждения в CONCEPT.md не подкреплены; нет 3+ источников на одно утверждение)
- **Falsif:** 4 (хорошая формализация с числовыми порогами, но contamination расчёт TBD)
- **Deliv:** 2 (много неопределённостей: hardware precision, AI performance, open science compliance)
- **Novelty:** 4 (Claude Code в качестве оператора микроскопа для aging biology — genuinely novel)
- **Risk:** 3 (риски описаны, но не все количественно; mitigation местами поверхностные)
- **RefIntegrity:** 3 (в EVIDENCE.md все ссылки реальны и соответствуют; в CONCEPT.md — плейсхолдеры, что нарушает п.7)
- **EvidenceDepth:** 1 (нет систематического обзора/мета-анализа; противоречия не учтены; утверждения не подтверждены 3+ независимыми источниками)
- **MethodDepth:** 2 (протокол слишком краткий; SAP неполный; controls есть, но replication strategy не детализирована)
- **Reproducibility:** 1 (code, data, pre-reg, materials — всё TBD; ни одной конкретной платформы или репозитория)

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. ✗ **Operationalised falsifiability (numeric thresholds)**  
   Числовые пороги есть (concordance >0.80, uptime >0.90, contamination <0.03, BOM ≤$4,500), и для concordance проведён power analysis. Однако **contamination sample size TBD**; это неполная операционализация. Необходимо завершить.

2. ✗ **Pre-registration plan (OSF placeholder + date)**  
   Указан OSF ID `osf.io/TBD` и дата 2026-06-01. **Идентификатор TBD — невалидный placeholder**. Требуется конкретный ID (например, osf.io/abcde).

3. ✗ **Sample size calc (power analysis)**  
   Для concordance расчёт есть (N=286). Для contamination — TBD. В CONCEPT.md также есть расчёт для CDATA cell division (n=30/group). Однако **расчёт для contamination не завершён**, что нарушает полноту.

4. ✓ **Risk matrix ≥5 rows**  
   Есть две таблицы: в CONCEPT.md (6 строк) и в EVIDENCE.md (7 строк). Условие выполнено.

5. ✓ **Limitations section**  
   Есть в CONCEPT.md (8 пунктов) и в EVIDENCE.md (8 пунктов). Выполнено.

6. ✓ **Consortium / collaboration plan**  
   В CONCEPT.md таблица с ролями и статусами (6 партнёров). В DESIGN.md ещё 4. Есть contacts, пусть и с TBD. Формально условие выполнено.

7. ✗ **Reference reality + match**  
   **Критическое нарушение.** В EVIDENCE.md все проверенные ссылки реальны (DOI/PMID валидны). Однако в CONCEPT.md в разделе "Evidence base & meta-analysis" стоят `[Placeholder: e.g., OpenTrons...]` и `[Author(s), Year, Journal, DOI TBD]`. Это **не реальные ссылки** — они не ведут на существующие записи. Также в THEORY.md нет ссылок вообще. Требуется заменить на реальные.

8. ✗ **No fabrication markers**  
   В CONCEPT.md присутствуют множественные `TBD` и `[Placeholder...]`, а также “Placeholder” в OSF ID, в sample size contamination, в DE. Согласно условию, TBD в местах, где должны быть конкретные данные, — маркер недопустим. Нарушение.

9. ✓ **Internal consistency core docs**  
   Между CONCEPT.md, THEORY.md, EVIDENCE.md нет явных противоречий. EVIDENCE.md честно указывает на отсутствие мета-анализа. Согласованно.

10. ✗ **Evidence base depth**  
    - Нет систематического обзора / мета-анализа. В CONCEPT.md сказано: “No systematic review or meta-analysis was performed”. Противоречия в литературе не упомянуты. State-of-the-art раздел слишком краткий.  
    - Ключевое утверждение “AI-operated microscopy achieves ≥80% trained-technician quality” опирается на три работы по AI в химии, но **ни одна не про микроскопию**. Нет трёх независимых источников именно по AI-operated microscopy.  
    - Условие не выполнено.

11. ✗ **Methodology depth**  
    - Replication-ready protocol: приведён список из 5 шагов без конкретных деталей (как собрать, откалибровать, настроить ПО). Ссылка на `AUTOMATED_MICROSCOPY_SETUP.md` — не входит в core-пакет.  
    - Statistical Analysis Plan: есть primary endpoint (agreement rate), secondary endpoints, Bonferroni, missing data strategy. Но **нет sensitivity analyses**, **нет указания теста для secondary endpoints**.  
    - Replication strategy: split-sample + independent dataset, но второй сайт TBD.  
    - Controls: positive (human) и negative (AI без PROMPT) — есть.  
    - Blinding/randomisation: описано.  
    - **Недостаточная детализация для независимой репликации** — протокол не может быть воспроизведён без `AUTOMATED_MICROSCOPY_SETUP.md`.

12. ✗ **Reproducibility & open science**  
    - Code: “deposited in public GitHub repository upon acceptance (URL: TBD)” — нет ссылки.  
    - Data: “Zenodo (DOI: TBD) or OSF (osf.io/TBD)” — все TBD.  
    - Pre-registration: osf.io/TBD.  
    - Materials: “protocols.io (DOI: TBD)” — TBD.  
    - Ни один из элементов не предоставлен конкретно. Условие полностью не выполнено.

## Reference audit (обязательная таблица — все ссылки компонента)

Примечание: проверены все явные DOI/PMID из EVIDENCE.md и CONCEPT.md. Ссылки-плейсхолдеры из CONCEPT.md отмечены как нереальные.

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|------------------------|---------|
| 1 | OpenFlexure Microscope project (Sharkey et al.