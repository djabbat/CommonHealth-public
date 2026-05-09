# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 4
- Falsif: 4
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 5

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **✓ Operationalised falsifiability (numeric thresholds)**  
   Для M1 (concordance) указаны α=0.05, power 0.80, N=286, MDE=0.05. Для uptime — H₀: ≤0.90, 180 дней. Для contamination — H₀: ≥0.03, но N=TBD (placeholder). Основные предсказания операционализированы числами. Допустимо для REVISE_MAJOR, но contamination требует конкретного N к финалу.

2. **✓ Pre-registration plan (OSF placeholder + date)**  
   Указан OSF ID `osf.io/automicroscopy_cdata` (placeholder), дата 2026-06-01. Выполнено.

3. **✓ Sample size calc (power analysis)**  
   Для CDATA эксперимента приведён расчёт: Cohen’s d=0.75, α=0.05, power=0.80, n=30/group, формула с подстановкой. Есть также расчёт для concordance (n=286) и uptime (180 дней). Выполнено.

4. **✓ Risk matrix ≥5 rows**  
   В CONCEPT.md таблица из 6 строк (AI misinterpretation, chamber failure, camera degradation, network outage, contamination, stepper drift). Количество строк выполнено.

5. **✓ Limitations section**  
   В CONCEPT.md раздел Limitations из 8 пунктов. В EVIDENCE.md есть другой список Limitations & Known Biases. Наличие раздела — да, но см. пункт 9.

6. **✓ Consortium / collaboration plan**  
   В CONCEPT.md таблица партнёров (LongevityCommon, Bristol, Zeiss, FLIR, ThorLabs, OpenTrons) с ролями и статусом. В DESIGN.md упомянуты James Smith, Lena Zhang. План есть.

7. **✓ Reference reality + match**  
   Аудит всех 9 ссылок из EVIDENCE.md: каждая ведёт на реальную публикацию (PubMed/Crossref/arXiv) и содержание соответствует утверждению. См. таблицу ниже. Score 5.

8. **✓ No fabrication markers**  
   Все [REF_NEEDED] и [PMID_REMOVED] удалены; текущий текст не содержит явных маркеров фабрикации. Placeholder в pre-reg и risk matrix допустимы.

9. **✗ Internal consistency core docs**  
   **Критическое нарушение:** Risk matrix в CONCEPT.md (6 строк) и Risk matrix в EVIDENCE.md (7 строк) — разные списки с разными рисками и оценками. Раздел Limitations в CONCEPT.md (8 пунктов) и Limitations & Known Biases в EVIDENCE.md (6+5 пунктов) не согласованы. Это прямое противоречие между core-файлами. Требуется унификация.

## Reference audit

| # | Цитата (короткая) | DOI/PMID | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------|----------|----------------------|---------|
| 1 | OpenFlexure microscope stage (Sharkey et al. 2016) | DOI: 10.1063/1.4941068 | Да | Да — описывает Arduino-based motorized stage с ±5μm | OK |
| 2 | Hayflick 1965 — 37°C + 5% CO₂ для фибробластов | PMID: 14315085 | Да | Да — стандартные условия культивирования | OK |
| 3 | Stringer et al. 2021 — CellPose v2 | DOI: 10.1038/s41592-020-01018-x, PMID: 33318659 | Да | Да — generalist model для сегментации клеток | OK |
| 4 | Schindelin et al. 2012 — ImageJ/Fiji | DOI: 10.1038/nmeth.2019, PMID: 22743772 | Да | Да — batch processing в центросомных исследованиях | OK |
| 5 | Wolff et al. 1992 — GT335 antibody | PMID: 1385210 | Да | Да — распознаёт полиглутамилированный тубулин | OK |
| 6 | Delgehyr et al. 2005 — Ninein antibody | DOI: 10.1242/jcs.02302, PMID: 15784680 | Да | Да — маркер дистального appendage материнской центриоли | OK |
| 7 | Burger et al. 2020 — autonomous chemistry robot | DOI: 10.1038/s41586-020-2442-2, PMID: 32641813 | Да | Да — прецедент автономной лаборатории | OK |
| 8 | Boiko et al. 2023 — GPT-4 химический синтез | DOI: 10.1038/s41586-023-06792-0, PMID: 38123806 | Да | Да — LLM-driven chemical synthesis | OK |
| 9 | Bran et al. 2024 — ChemCrow | DOI: 10.1038/s42256-024-00832-8 | Да | Да — LLM с chemistry tools | OK |

## Top 5 text-level fixes (для REVISE_MAJOR)

1. **CONCEPT.md + EVIDENCE.md: Унифицировать Risk matrix**  
   — Выбрать единый список рисков (6–7 строк) с согласованными вероятностями и воздействиями. Убрать дублирование. Оба файла должны ссылаться на один master-список.

2. **CONCEPT.md + EVIDENCE.md: Унифицировать Limitations**  
   — Объединить два разных списка (8 пунктов в CONCEPT, 6+5 в EVIDENCE) в единый, непротиворечивый раздел. Убрать повторы и противоречия (например, про stage accuracy: в CONCEPT "±1–2 µm", в EVIDENCE "±2 µm" — следует выбрать одно).

3. **CONCEPT.md: Устранить дублирование Sample size calculation**  
   — Раздел встречается дважды: после "Primary use case" и после "Pre-registration plan". Оставить один блок, дополнив расчёты для contamination (заменить TBD на конкретное N после пилота).

4. **DESIGN.md, PARAMETERS.md, OPEN_PROBLEMS.md — Заполнить содержимое**  
   — Три core-файла являются заглушками ("Stub"). Для оценки готовности компонента требуется хотя бы минимальное наполнение (архитектура, параметры, открытые проблемы). Без этого deliverability ставится под сомнение.

5. **CONCEPT.md: Привести Falsification conditions к единому стилю с THEORY.md**  
   — В THEORY.md фальсификация M1–M4 описана в одном формате, в CONCEPT.md — в другом (с H₀/H₁). Согласовать формулировки, убрать расхождение в uptime (CONCEPT: 95%, THEORY: 80%? — проверьте, в THEORY §8 сказано "uptime <80%", в CONCEPT "uptime ≥95%" — это противоречие).