# Review of AutomatedMicroscopy

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 1
- Falsif: 3
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 1
- EvidenceDepth: 1
- MethodDepth: 2
- Reproducibility: 1

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   Числовые пороги есть: concordance >0.80 (N=286, α=0.05, power=0.80), uptime >0.90 (N=180), contamination <0.03 (N=TBD). Формулы приведены.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Указан OSF ID `osf.io/TBD`, `osf.io/automicroscopy_cdata`, дата 2026-06-01. Плейсхолдер допустим при условии, что будет реально зарегистрирован до сбора данных.

3. **Sample size calc (power analysis)** — ✓  
   Для concordance: n = (1.645+0.84)² × 0.85×0.15 / 0.05² ≈ 286. Для contamination: placeholder (TBD). Минимально приемлемо, но TBD ослабляет.

4. **Risk matrix ≥5 rows** — ✓  
   Несколько таблиц, в сумме ≥6 строк с probability, impact, mitigation.

5. **Limitations section** — ✓  
   Есть явные разделы в CONCEPT и EVIDENCE, перечислены 8+ ограничений.

6. **Consortium / collaboration plan** — ✓  
   Таблица с 5+ партнёрами (LongevityCommon, Univ. Bristol, Zeiss, FLIR, ThorLabs, OpenTrons), роли и статус указаны.

7. **Reference reality + match** — **✗**  
   В EVIDENCE.md часть ссылок имеет реальные DOI/PMID (Burger, Boiko, Bran, Stringer, Wolff и др.) и соответствует тексту. Однако:  
   - В разделе «Evidence base & meta-analysis» (конец EVIDENCE.md и CONCEPT) ссылки даны как `[Author(s), Year, Journal, DOI TBD]` — невалидные, не подтверждённые.  
   - В той же части CONCEPT: «[Placeholder: e.g., OpenTrons, µManager…]» — не ссылка, а плейсхолдер.  
   - Часть источников не имеет DOI/PMID (manufacturer spec, community documentation, standard practice) — для крупного гранта это недостаточно.  

   **Требование проверки КАЖДОЙ цитируемой работы нарушено** — наличие TBD-ссылок автоматически ведёт к REJECT.

8. **No fabrication markers** — **✗**  
   В документах присутствуют следующие маркеры:  
   - `[Author(s), Year, Journal, DOI TBD]` (EVIDENCE.md, раздел «Evidence base & meta-analysis»)  
   - `[Placeholder: e.g., …]` (CONCEPT.md, Evidence base & meta-analysis)  
   - `Repository: TBD` (EVIDENCE.md, Reproducibility)  
   - `Data deposit plan: TBD` (там же)  
   - `protocols.io link TBD` (там же)  
   - `osf.io/TBD` (формально допустимо для pre-reg, но в паре с остальными — fabricational pattern)  

   Это прямое нарушение условия 8. REJECT.

9. **Internal consistency core docs** — ✓ (с натяжкой)  
   Методы в EVIDENCE согласуются с CONCEPT, цели — с THEORY. Однако PARAMETERS, OPEN_PROBLEMS, DESIGN — stubs, что снижает доверие к целостности, но не создаёт явных противоречий.

10. **Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed)** — **✗**  
    - Утверждение «Low-cost microscope retrofit is feasible» подтверждено только одним peer-reviewed источником (OpenFlexure).  
    - Утверждение «Environmental control for long-term imaging» — только Hayflick (1965) и datasheet, без независимого воспроизведения.  
    - Ни один систематический обзор или мета-анализ по теме не процитирован (упоминается, что «не найден»).  
    - Противоречивые результаты не приведены, хотя автор упоминает, что «some studies report higher error rates for AI-based focus adjustment» — без конкретных ссылок.  
    - Overall: 10a, 10b, 10c, 10d не выполнены.

11. **Methodology depth (replication-ready protocol, SAP, controls, replication strategy)** — **✗**  
    - Step-by-step protocol в EVIDENCE.md крайне общий (1. Setup, 2. Configuration…), недостаточно деталей для независимой репликации.  
    - SAP есть: primary endpoint (concordance), secondary (uptime, contamination, image quality), Bonferroni, LOCF — но LOCF сомнительно при 84 time points.  
    - Replication strategy: split-sample (70/30) и independent dataset (TBD) — вторая часть не указана.  
    - Controls: positive (human expert), negative (random AI) — отрицательный контроль не детализирован.  
    - Blinding: указано, evaluators blinded — OK.  
    - Пункт не пройден из-за неконкретности протокола и отсутствия независимого репликационного плана.

12. **Reproducibility & open science (code, data, pre-reg, materials)** — **✗**  
    - Code availability: «Repository: TBD».  
    - Data deposit: «TBD» (платформа не указана).  
    - Pre-registration: `osf.io/TBD` — формально есть, но дата и содержимое только планируются.  
    - Materials: protocols.io link TBD.  
    - Все пункты 12a–12d либо TBD, либо отсутствуют.  
    - Open science compliance нулевой.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Zeiss IM 35 manual (manufacturer spec) | нет DOI/PMID | Да (документация) | Да | OK, но не научная статья |
| 2 | FLIR Blackfly S product datasheet | нет | Да | Да | OK |
| 3 | OpenFlexure (Sharkey et al., 2016) | 10.1063/1.4941068 | Да | Да | OK |
| 4 | Micro-Manager 2.0 | micro-manager.org (нет DOI) | Да | Да | OK |
| 5 | Hayflick (1965) | 10.1016/0014-4827(65)90211-9 / PMID 14315085 | Да | Да | OK |
| 6 | Inkbird ITC-100 spec (manufacturer) | нет | Да | Да | OK |
| 7 | CellPose v2 (Stringer et al., 2021) | 10.1038/s41592-020-01018-x / PMID 33318659 | Да | Да | OK |
| 8 | ImageJ/Fiji (Schindelin et al., 2012) | 10.1038/nmeth.2019 / PMID 22743772 | Да | Да | OK |
| 9 | GT335 antibody (Wolff et al., 1992) | PMID 1385210 | Да | Да | OK |
| 10 | Ninein antibody (Delgehyr et al., 2005) | 10.1242/jcs.02302 / PMID 15784680 | Да | Да | OK |
| 11 | Burger et al. (2020) Nature | 10.1038/s41586-020-2442-2 | Да | Да (химия, не микроскопия) | OK, но перенос контекста |
| 12 | Boiko et al. (2023) Nature | 10.1038/s41586-023-06792-0 | Да | Да | OK |
| 13 | Bran et al. (2024) Nat Mach Intell | 10.1038/s42256-024-00832-8 | Да | Да | OK |
| 14 | [Author(s), Year, Journal, DOI TBD] — AI-assisted microscopy | TBD | Нет | Нет | **FABRICATION MARKER** |
| 15 | [Author(s), Year, Journal, DOI TBD] — Low-cost retrofit | TBD | Нет | Нет | **FABRICATION MARKER** |
| 16 | [Author(s), Year, Journal, DOI TBD] — CDATA protocol | TBD | Нет | Нет | **FABRICATION MARKER** |
| 17 | Placeholder: OpenTrons, µManager | — | Нет | Нет | **PLACEHOLDER** |

## Evidence depth audit (новое v3.0)

| # | Ключевое утверждение | Источников цитировано | Включён ли мета-анализ/систематический обзор? | Противоречия учтены? |
|---|---|---|---|---|
| 1 | AI-operated microscopy has precedents | 3 (Burger, Boiko, Bran) — все из химии | Нет (упомянуто, что не найден) | Нет (не упомянуты возможные неудачи LLM в микроскопии) |
| 2 | Low-cost microscope retrofit is feasible | 1 (OpenFlexure) + manufacturer spec | Нет | Нет |
| 3 | Environmental control for long-term imaging | 2 (Hayflick + Inkbird datasheet) | Нет | Нет |
| 4 | Cell segmentation