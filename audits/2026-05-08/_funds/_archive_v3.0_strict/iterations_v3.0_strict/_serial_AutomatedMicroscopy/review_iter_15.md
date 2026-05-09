# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsifiability: 3
- Deliv: 4
- Novelty: 4
- Risk: 4
- RefIntegrity: 5
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 3

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

**1. Operationalised falsifiability (numeric thresholds)** — ✗  
Числовые пороги заданы для M1 (concordance >0.80, N=286, α=0.05, power=0.80), uptime (>0.90, N=180 дней), contamination (<0.03, но **N=TBD**), cost (≤$4,500). Для contamination отсутствует расчёт N и мощность. Нарушение: не все гипотезы полностью операционализированы. Placeholder для N contamination недопустим.

**2. Pre-registration plan (OSF placeholder + date)** — ✓  
Указан OSF `osf.io/TBD`, дата 2026-06-01, содержание описано. Минимальные требования выполнены.

**3. Sample size calc (power analysis)** — ✗  
Для concordance расчёт проведён: n = (1.645+0.84)² × 0.85×0.15 / 0.05² ≈ 286. Для uptime фиксирован 180 дней. Для contamination N=TBD, мощность не указана. Формулы есть, но не для всех endpoint’ов. Частичное выполнение.

**4. Risk matrix ≥5 rows** — ✓  
Две таблицы: первая 6 рисков, вторая 6 рисков с числовыми оценками Probability/Impact. Суммарно более 5. Выполнено.

**5. Limitations section** — ✓  
Явный раздел с 8–12 ограничениями (DIY precision, calibration drift, phototoxicity, AI hallucination, отсутствие жидкостного манипулятора и т.д.). Выполнено.

**6. Consortium / collaboration plan** — ✓  
Таблица партнёров с ролями и статусами (LongevityCommon, University of Bristol, Zeiss, FLIR, ThorLabs, OpenTrons). В DESIGN.md перечислены потенциальные партнёры. Выполнено.

**7. Reference reality + match** — ✓  
Проверены все 10 DOI/PMID в EVIDENCE.md:
| # | Короткая цитата | DOI/PMID | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Sharkey et al. 2016, *Rev Sci Instrum* – XY stage ±5μm | 10.1063/1.4941068 | Да | Да | OK |
| 2 | Hayflick 1965 – 37°C/5% CO₂ | 10.1016/0014-4827(65)90211-9 | Да | Да | OK |
| 3 | Stringer et al. 2021, *Nat Methods* – CellPose v2 | 10.1038/s41592-020-01018-x | Да | Да | OK |
| 4 | Schindelin et al. 2012, *Nat Methods* – ImageJ | 10.1038/nmeth.2019 | Да | Да | OK |
| 5 | Wolff et al. 1992 – GT335 antibody | PMID 1385210 | Да | Да | OK |
| 6 | Delgehyr et al. 2005 – Ninein antibody | 10.1242/jcs.02302 | Да | Да | OK |
| 7 | Burger et al. 2020, *Nature* – автономная химия | 10.1038/s41586-020-2442-2 | Да | Да | OK |
| 8 | Boiko et al. 2023, *Nature* – GPT-4 химия | 10.1038/s41586-023-06792-0 | Да | Да | OK |
| 9 | Bran et al. 2024, *Nat Mach Intell* – ChemCrow | 10.1038/s42256-024-00832-8 | Да | Да | OK |
Все ссылки реальны и соответствуют утверждениям. Фабрикаций нет.

**8. No fabrication markers** — ✓  
После аудита 2026-05-08 удалены все [REF_NEEDED] и [PMID_REMOVED]. Placeholders (TBD) для sample size – не фабрикационные маркеры, а открытые параметры. Выполнено.

**9. Internal consistency core docs** — ✓  
CONCEPT, THEORY, EVIDENCE согласованы: методология (AI supervision), цели (24/7 imaging для CDATA), predictions. Stubs (PARAMETERS, OPEN_PROBLEMS, DESIGN) не противоречат. Выполнено.

**10. Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed)** — ✗  
(a) Ключевые утверждения:
- "AI-operated microscopy has precedents" – 3 источника, но все из химии, не из микроскопии.
- "Low-cost retrofit feasible" – 1 peer-reviewed + manufacturer spec.
- "Environmental control" – 1 peer-reviewed + datasheet.
- "Cell segmentation" – 1 источник.
- "Antibody specificity" – 2 источника.
**Требование ≥3 независимых источников на ключевое утверждение не выполнено.**
(b) Систематический обзор или мета-анализ: **отсутствует**. Авторы признают: "No systematic review or meta-analysis was identified". Это прямое нарушение.
(c) Противоречащие результаты: **не обсуждены**. В Evidence base упомянуто "Some studies report higher error rates for AI-based focus adjustment in low-contrast samples", но это не развёрнуто, не указаны ссылки, нет обсуждения.
(d) State-of-the-art: раздел есть (сравнение с коммерческими системами), но без детального анализа конкурентов.
→ Условие не выполнено.

**11. Methodology depth (replication-ready protocol, SAP, controls, replication strategy)** — ✗  
(a) Step-by-step протокол для независимой репликации **отсутствует**. В разделе "Methodology depth" (вставка) – 5 шагов без деталей сборки, калибровки, конфигурации ПО, параметров съёмки.
(b) SAP: есть перечень primary/secondary endpoints, Bonferroni correction, LOCF для missing data. Но **отсутствует** детализация тестов для вторичных endpoints, анализ чувствительности, план обработки выбросов.
(c) Replication strategy: split-sample (70/30) и independent dataset (lab TBD). Указано.
(d)