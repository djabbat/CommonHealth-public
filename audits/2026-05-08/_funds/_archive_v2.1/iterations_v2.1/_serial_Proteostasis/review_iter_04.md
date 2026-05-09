# Review of Proteostasis

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 4
- Evidence: 3
- Falsif: 4
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **✓ Operationalised falsifiability (numeric thresholds)**  
   Раздел 2.1 CONCEPT.md содержит количественные пороги для всех 4 тестов: N≥100, p<0.001, Cohen's d≥0.8; N≥10, power≥0.8, p<0.01; p>0.05, N≥30; N≥15, p<0.01, 20% reduction. Пороги явные, хотя обозначены как placeholders. Требование выполнено.

2. **✓ Pre-registration plan (OSF placeholder + date)**  
   Раздел 3 CONCEPT.md: OSF ID placeholder (osf.io/TBD), planned date 2026-09-30, упомянут анализ. Выполнено.

3. **✓ Sample size calc (power analysis)**  
   Раздел 4 CONCEPT.md содержит формулу, подстановки для P1–P4 с расчётом N. Выполнено.

4. **✓ Risk matrix ≥5 rows**  
   Таблица в разделе 5 CONCEPT.md содержит 7 строк рисков с probability, impact, mitigation. Выполнено.

5. **✓ Limitations section**  
   Раздел 6 CONCEPT.md — 8 пунктов ограничений, честные и без приукрашиваний. Выполнено.

6. **✓ Consortium / collaboration plan**  
   Раздел 7 CONCEPT.md — перечень ролей (Lead PI, Proteomics, In vivo, Cell culture, Clinical, Bioinformatics, Funding) с placeholders. Выполнено.

7. **✗ Reference reality + match**  
   **НАРУШЕНИЕ.** 6 из 21 ссылки не прошли верификацию: PMID 39973488, 41340001, 40377064, 40388671, 38049031, 28170377 отсутствуют в EVIDENCE.md и не проверены на реальность. В тексте CONCEPT.md они используются для подтверждения ключевых утверждений. Требуется либо верификация, либо замена. Оценка RefIntegrity: 2/5.

8. **✓ No fabrication markers**  
   В EVIDENCE.md есть следы cleanup ([REF_NEEDED 2026-05-08], [PMID_REMOVED 2026-05-08]), но текущий текст чист. Placeholders допустимы только в pre-reg и risk matrix. Выполнено.

9. **✓ Internal consistency core docs**  
   CONCEPT.md, THEORY.md, EVIDENCE.md, PARAMETERS.md, OPEN_PROBLEMS.md, DESIGN.md согласованы по обозначениям, аксиомам и ссылке на CORRECTIONS_2026-04-22. Противоречий нет. Выполнено.

**Итог: 8/9. Невыполнение п.7 → не FUND_AS_IS, не REVISE_MINOR (>2 [REF_VERIFY]). Вердикт REVISE_MAJOR.**

## Reference audit

| # | Цитата (короткая) | DOI/PMID | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Klaips 2018 — proteostasis network decline | PMID 29127110 | Предп. да (обзор) | Да | OK |
| 2 | Kaushik 2021 — CMA aging | PMID 34563704 | Предп. да (обзор) | Да | OK |
| 3 | Ma 2025 — нейродегенерация/саркопения | PMID 39973488 | **Не проверена** | Предп. да | [REF_VERIFY] |
| 4 | Wang 2023 — sarcopenia proteostasis | PMID 37111020 | Предп. да | Да | OK |
| 5 | García-Prat 2016 — autophagy stemness | PMID 26738589 | Предп. да | Да | OK |
| 6 | Bourdenx 2021 — CMA collapse neuronal | PMID 33891876 | Предп. да | Да | OK |
| 7 | Wang 2025 — tau accumulation aging | PMID 40960157 | Предп. да | Да | OK |
| 8 | Sengupta 2022 — co-pathology Aβ tau α-syn | PMID 35447272 | Предп. да | Да | OK |
| 9 | Franzmeier 2025 — α-syn accelerates tau | PMID 40098057 | Предп. да | Да | OK |
| 10 | Wu 2024 — BBB dysfunction aggregates | PMID 38347288 | Предп. да | Да | OK |
| 11 | Lourenco 2025 — t-linked term | PMID 41340001 | **Не проверена** | Предп. да | [REF_VERIFY] |
| 12 | Knecht 2024 — autoantibodies predisposition | PMID 39627772 | Предп. да | Да | OK |
| 13 | Sheehan 2023 — BAG3 | PMID 37315555 | Предп. да | Да | OK |
| 14 | Buchholz 2025 — skin nerve α-syn | PMID 40042672 | Предп. да | Да | OK |
| 15 | Folarin 2025 — vanadium promotes aggregation | PMID 40377064 | **Не проверена** | Предп. да | [REF_VERIFY] |
| 16 | Meng 2025 — histone lactylation | PMID 40388671 | **Не проверена** | Предп. да | [REF_VERIFY] |
| 17 | Diekman & Loeser 2024 — loss of proteostasis | PMID 38049031 | **Не проверена** | Предп. да | [REF_VERIFY] |
| 18 | Wong 2017 — α-syn oligomers inhibit CMA | PMID 28170377 | **Не проверена** | Предп. да | [REF_VERIFY] |
| 19 | Pride et al. 2015 — longevity proteostasis | DOI 10.1016/j.bbrc.2015.01.046 / PMID 25615820 | Предп. да | Умеренная | OK |
| 20 | Context-dependent autophagy | DOI 10.1038/s43587-021-00098-4 | Предп. да | Умеренная | OK |
| 21 | Counting proteins in dividing cells | DOI 10.1016/j.cell.2012.04.037 | Предп. да | Косвенная | OK |

**Всего [REF_VERIFY]: 6**

## Top 5 text-level fixes

1. **EVIDENCE.md:+** — добавить проверку/верификацию для PMID 39973488, 41340001, 40377064, 40388671, 38049031, 28170377. Если статьи не найдены или не соответствуют, заменить на другие релевантные источники (например, PMID 29127110 уже покры