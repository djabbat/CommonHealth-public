# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 3
- Evidence: 2
- Falsifiability: 2
- Deliv: 3
- Novelty: 3
- Risk: 3
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✗  
   Пороги указаны (PAM-13 Δ ≥ 5.4, α = 0.05, power = 0.80, N ≥ 55 per group), но в THEORY.md дополнительно используется Bonferroni-коррекция α = 0.025, а в CONCEPT.md эта коррекция только для secondary, не для primary. Есть неоднозначность. Кроме того, в CONCEPT.md дважды приведён противоречивый sample size (один с σ = TBD, другой с σ = 10). Нарушена строгая операционализация.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Указан OSF ID placeholder (`osf.io/TBD`) и дата 2026-09-01. Допустимый placeholder.

3. **Sample size calc (power analysis)** — ✗  
   Формула и подстановка есть, но значение σ = 10 противоречит соседнему указанию «σ: TBD (placeholder)». Отсутствует единый согласованный расчёт. Условие не выполнено.

4. **Risk matrix ≥5 rows** — ✓  
   В CONCEPT.md приведены две таблицы: одна текстовых (7 rows), другая числовых (7 rows). ≥5 rows есть.

5. **Limitations section** — ✓  
   Явный раздел есть в обоих core-документах (CONCEPT.md 8 пунктов, THEORY.md 8 пунктов). Хотя содержание различается, сам раздел присутствует.

6. **Consortium / collaboration plan** — ✓  
   Указаны lead PI (TBD), Co-Is (TBD), потенциальные партнёры (Insignia Health, Fraunhofer, TSU, Copenhagen) с ролями. Достаточно для предварительного плана.

7. **Reference reality + match** — ✗  
   **Critical:** из 6 цитируемых научных работ только 2 имеют валидные разрешаемые идентификаторы (Hibbard 2004 PMID 15333167, Hibbard 2005 PMID 15527447). Остальные 4 (Hibbard 2009, Tao et al. 2026, Blumenthal-Lee 2024, Tkemaladze 2026) имеют