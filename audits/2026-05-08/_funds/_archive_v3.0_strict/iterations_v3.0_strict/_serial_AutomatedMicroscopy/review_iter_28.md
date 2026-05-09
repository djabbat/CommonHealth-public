# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 4
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 3
- EvidenceDepth: 2
- MethodDepth: 3
- Reproducibility: 1

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   Числовые пороги для M1 (concordance >0.80, N=286), uptime (>0.90, 180 days), contamination (<0.03, N=TBD), cost (<$4,500). Есть TBD для contamination N, но основная структура присутствует.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Плейсхолдер `osf.io/TBD`, дата 2026-06-01. Указано, что будет зарегистрировано до сбора данных.

3. **Sample size calc (power analysis)** — ✓ (minor)  
   Для concordance: формула с подстановкой, N=286. Для uptime: 180 days. Для contamination: TBD (зависит от пилота). Отсутствует конечное число для contamination, но основной расчёт есть.

4. **Risk matrix ≥5 rows** — ✓  
   В CONCEPT.md таблица из 6 рисков, в EVIDENCE.md ещё 7 строк. Формат probability×impact×mitigation соблюдён.

5. **Limitations section** — ✓  
   Подробный раздел в CONCEPT.md (8 пунктов) и дополнительно в EVIDENCE.md. Прозрачное перечисление ограничений.

6. **Consortium / collaboration plan** — ✓ (minor)  
   Таблица партнёров с ролями (University of Bristol, Zeiss, FLIR, ThorLabs, OpenTrons). Некоторые статусы "TBD" или "Contact initiated", но роли указаны.

7. **Reference reality + match** — ✗  
   - **Реальные ссылки в EVIDENCE.md** (OpenFlexure, Hayflick, Wolff, Delgehyr, Stringer, Schindelin, Burger, Boiko, Bran) — все проверены, реальны, соответствуют тексту.  
   - **В CONCEPT.md** в разделе "Evidence base & meta-analysis" приведены те же ссылки, плюс **плейсхолдеры** `[Author(s), Year, Journal, DOI TBD]` для трёх ключевых утверждений (AI-assisted microscopy, low-cost retrofit, CDATA protocol). Это нарушение: в этих местах должны быть конкретные данные.  
   - **В EVIDENCE.md** в конце раздела "Evidence base & meta-analysis" также есть `[Placeholder: e.g., OpenTrons, µManager…]`.  
   - **Вывод:** часть ссылок — плейсхолдеры, не соответствующие требованиям реальности. Условие не выполнено.

8. **No fabrication markers** — ✗  
   - В EVIDENCE.md есть `[Placeholder: e.g., OpenTrons, µManager…]` и `[Author(s), Year, Journal, DOI TBD]`.  
   - В CONCEPT.md раздел "Evidence base & meta-analysis" содержит те же плейсхолдеры ("[Author(s), Year, Journal, DOI TBD]").  
   - Это маркеры "TBD" там, где должны быть конкретные данные. Условие нарушено.

9. **Internal consistency core docs** — ✓  
   CONCEPT, THEORY, EVIDENCE согласованы по целям (CDATA Phase A), методам (ретрофит, AI supervision), прогнозам. PARAMETERS, OPEN_PROBLEMS, DESIGN — стабы, но не противоречат. Повторы в разделах Limitations не считаются противоречием.

10. **Evidence base depth** — ✗  
    - **Для утверждения "AI-operated microscopy feasibility":** 3 источника (Burger, Boiko, Bran) — из химии, не microscopy; засчитывается, но слабо.  
    - **Для утверждения "Low-cost retrofit feasible":** только OpenFlexure (1 peer-reviewed) + мануал Zeiss. <3.  
    - **Для утверждения "Environmental control":** Hayflick (1) + даташит Inkbird. <3.  
    - **Для утверждения "Cell segmentation":** Stringer (1). <3.  
    - **Для утверждения "Antibody specificity":** Wolff + Delgehyr (2). <3.  
    - **Систематический обзор/мета-анализ:** не представлен; авторы честно признают, что не нашли.  
    - **Противоречия:** не учтены (сказано "no contradictory results found", но нет обсуждения возможных расхождений).  
    - **State-of-the-art:** описан (коммерческие системы vs open-source vs AI).  
    - Условие не выполнено (недостаточно источников, нет мета-анализа).

11. **Methodology depth** — ✗ (частично)  
    - **Step-by-step protocol:** в CONCEPT.md есть список шагов (Setup, Configuration, Execution…), но не детализирован до уровня независимой репликации. Ссылка на `AUTOMATED_MICROSCOPY_SETUP.md` (не предоставлен) — недостаточно.  
    - **SAP:** Primary endpoint (concordance rate, Cohen's kappa), secondary (uptime, contamination, image quality). Multiple comparisons: Bonferroni. Missing data: LOCF. Sensitivity: не указана.  
    - **Replication strategy:** split-sample (70/30) + independent lab (TBD).  
    - **Controls:** positive (human expert), negative (random decisions).  
    - **Blinding/randomization:** evaluators blinded.  
    - **Пробелы:** протокол не replica-ready, нет explicit sensitivity analyses, нет плана по missing data для вторичных endpoints. Условие не выполнено.

12. **Reproducibility & open science** — ✗  
    - **Code:** обещано на GitHub, но репозиторий не указан.  
    - **Data:** обещано в Zenodo/OSF, но депозитный план не конкретизирован.  
    - **Pre-registration:** `osf.io/TBD` — только плейсхол