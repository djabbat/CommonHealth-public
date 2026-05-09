# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 2
- Deliv: 1
- Novelty: 3
- Risk: 2
- RefIntegrity: 5
- EvidenceDepth: 1
- MethodDepth: 2
- Reproducibility: 2

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds): ✗**
   - Есть числовые пороги для concordance (≥0.80), uptime (≥0.90), contamination (≤0.03), cost (≤$4,500). Но для contamination N = TBD, для uptime нет формального статтеста (просто "observed uptime ≤0.90 after 180 days"). Не все predictions операционализированы полностью.

2. **Pre-registration plan (OSF placeholder + date): ✗**
   - Указан OSF ID `osf.io/TBD` (невалидный placeholder) и дата 2026-06-01. Требуется осмысленный placeholder (например, `osf.io/automicroscopy_cdata`). Текущий "TBD" недопустим.

3. **Sample size calc (power analysis): ✗**
   - Для concordance есть корректный расчёт (N=286), для CDATA эксперимента (N=60), для uptime фиксированное N=180. Но для contamination N=TBD, для cost – дескриптивно. Для secondary endpoints power analysis отсутствует.

4. **Risk matrix ≥5 rows: ✓**
   - В CONCEPT.md две версии risk matrix (по 6 строк каждая), с probability, impact, mitigation. Требование выполнено.

5. **Limitations section: ✓**
   - Явный раздел в CONCEPT.md (8 пунктов), дублируется в EVIDENCE.md. Признаёт ограничения по точности, дрейфу, AI, отсутствию жидкостной обработки и т.д.

6. **Consortium / collaboration plan: ✓**
   - Таблица с 6 партнёрами, ролями и статусом (James Smith, Lena Zhang, OpenFlexure, Micro-Manager, TBD). Есть план.

7. **Reference reality + match: ✓**
   - Все 9 проверенных ссылок (DOI/PMID) реальны (PubMed/Crossref) и соответствуют тексту. Manufacturer specs допустимы. REF_VERIFY не требуется.

8. **No fabrication markers: ✗**
   - В CONCEPT.md обнаружены множественные "TBD" в sample size calculation, "DOI TBD" в evidence base, "TBD (additional partner)" – там, где должны быть конкретные данные. Это нарушает правило.

9. **Internal consistency core docs: ✓**
   - CONCEPT.md, THEORY.md, EVIDENCE.md согласованы: методы описаны в CONCEPT и детализированы в THEORY, цели соответствуют CDATA. Stub-файлы (PARAMETERS, OPEN_PROBLEMS, DESIGN) не противоречат, но пусты – это не нарушение консистентности.

10. **Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed): ✗**
    - Ключевое утверждение "low-cost microscope retrofit feasible" опирается только на OpenFlexure (1 peer-reviewed) + manufacturer specs. Не ≥3 независимых.
    - Нет цитирования систематического обзора или мета-анализа по теме. Авторы признают, что "no systematic review was identified" – требуется хотя бы один.
    - Противоречия не найдены, но и не проведён систематический поиск; утверждение "no contradictory results" без должного анализа.

11. **Methodology depth (replication-ready protocol, SAP, controls, replication strategy): ✗**
    - Step-by-step protocol в CONCEPT.md слишком краток (6 общих шагов). Нет ссылки на protocols.io или детализированный протокол.
    - SAP есть только первичная конечная точка (concordance), но отсутствуют полные спецификации: multiple comparisons correction упомянут (Bonferroni), но secondary endpoints не перечислены явно, missing data strategy – LOCF (указана, но может быть неадекватна).
    - Replication strategy: split-sample (70/30) и внешняя репликация (второй партнёр) – упомянуты.
    - Controls: positive (human) и negative (random) – есть.
    - Blinding/randomization: evaluators blinded – упомянуто.
    - В целом, недостаточно для независимой репликации.

12. **Reproducibility & open science (code, data, pre-reg, materials): ✗**
    - Code availability: обещание "code will be released on acceptance" – приемлемо, но без ссылки на репозиторий.
    - Data deposit plan: "Zenodo or OSF" – указано.
    - Pre-registration: только placeholder `osf.io/TBD` – неконкретный.
    - Materials transparency: protocols.io link TBD, requirements.txt упомянут, BOM есть. Нет конкретных идентификаторов.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | OpenFlexure stage accuracy (Sharkey et al. 2016) | 10.1063/1.4941068 | Да | Да – подтверждает точность XY stage | ✓ |
| 2 | Hayflick 1965 fibroblast culture conditions | 10.1016/0014-4827(65)90211-9; PMID 14315085 | Да | Да | ✓ |
| 3 | Stringer et al. 2021 CellPose v2 | 10.1038/s41592-020-01018-x | Да | Да | ✓ |
| 4 | Schindelin et al. 2012 ImageJ/Fiji | 10.1038/nmeth.2019 | Да | Да | ✓ |
| 5 | Wolff et al. 1992 GT335 antibody | PMID 1385210 | Да | Да | ✓ |
| 6 | Delgehyr et al. 2005 Ninein antibody | 10.1242/jcs.02302 | Да | Да | ✓ |
| 7 | Burger et al. 2020 autonomous chemistry robot | 10.1038/s41586-020-2442-2 | Да | Да (но domain – chemistry) | ✓ |
| 8 | Boiko et al. 2023 GPT-4 chemistry synthesis | 10.1038/s41586-023-06792-0 | Да | Да | ✓ |
| 9 | Bran et al. 2024 ChemCrow | 10.1038/s42256-024-00832-8 | Да | Да | ✓ |

## Evidence depth audit

| # | Ключевое утверждение | Источников цитировано | Включён ли мета-анализ/систематический обзор? | Противоречия учтены? |
|---|---|---|---|---|
| 1 | AI-operated microscopy feasible | 3 (Burger, Boiko, Bran) – все из химии, не прямой микроскоп | Нет | Нет (не найдены) |
| 2 | Low-cost microscope retrofit feasible | 1 (OpenFlexure) + manufacturer specs | Нет | Нет |
| 3 | Environmental control for long-term imaging | 1 (Hayflick) + 1 manufacturer (Inkbird) | Нет | Нет |
| 4 | Cell segmentation with CellPose | 1 (Stringer) | Нет | Нет |
| 5 | Antibody specificity (GT335, Ninein) | 2 (Wolff, Delgehyr) – каждая по отдельности | Нет | Нет |

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

1. **CONCEPT.md: "Sample size calculation" и все разделы с TBD** — заменить все `n = TBD`, `osf.io/TBD`, `DOI TBD` на конкретные значения/идентификаторы или, если данные отсутствуют, написать явный план пилотного исследования с указанием, что эти параметры будут определены до регистрации.

2. **CONCEPT.md: "Evidence base & meta-analysis"** — добавить цитирование хотя бы одного систематического обзора или мета-анализа по теме (например, по low-cost automated microscopy, AI in bioimaging). Указать, как проведён поиск (запрос, базы, дата).

3. **CONCEPT.md: "Consortium / partners"** — заменить всех `TBD` на конкретные организации (да