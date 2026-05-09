# Review of BioSense

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsif: 5
- Deliv: 4
- Novelty: 4
- Risk: 3
- RefIntegrity: 3

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — **✓**  
   Для всех трёх модулей заданы H₀/H₁ с числовыми порогами (δ, α, d, r, AUC, мощность). Приведён итоговый N ≥ 161. Полный набор чисел.

2. **Pre-registration plan (OSF placeholder + date)** — **✓**  
   Указан placeholder `osf.io/ze3x7` и две противоречивые даты: «2026-06-01» и «до 2026-12-31». Наличие формально есть, но **требуется согласование дат** (см. fixes). Не снимает галочку, но снижает score.

3. **Sample size calc (power analysis)** — **✗**  
   Приведены расчёты для каждого модуля с формулой и подстановкой, но **итоговое N указано как «TBD»**, что является нарушением требования конкретного числа. В falsifiability сказано N ≥ 161, но в самом расчёте — TBD. Несоответствие. Условие не выполнено.

4. **Risk matrix ≥5 rows** — **✓**  
   Таблица содержит 6 строк (5 основных + 1 дополнительная). Все строки с probability, impact, mitigation.

5. **Limitations section** — **✓**  
   Отдельный раздел с 9 пунктами, включая признание необщепринятости Ze-теории, null result MPI-LEMON, отсутствие данных для ВСР и ольфакции и т.д.

6. **Consortium / collaboration plan** — **✓**  
   Placeholder список из 6 организаций с указанием ролей (University of Tbilisi, Charité, STMicroelectronics, MPI, University of Dortmund, OSF). Достаточно для ревью.

7. **Reference reality + match** — **✓ (с оговорками)**  
   Для трёх статей указаны DOI (Valdés-Sosa, Turin, Babayan) — реальны и соответствуют тексту.  
   Для двух PMID (Tkemaladze 2023, Lezhava 2011) — предполагаем реальность.  
   Для датасетов (Zenodo 3875159, 4244765, ds005385) указаны только номера, а не полные DOI (должен быть `10.5281/zenodo.XXXXX`). Это **[REF_VERIFY]** флаги.  
   Препринт «Ze EEG paper» помечен как under review – допустимо.  
   В целом соответствие тексту не нарушено, но требуется дооформление ссылок на датасеты.

8. **No fabrication markers** — **✗**  
   В разделе **Sample size calculation** присутствует **«TBD»** для итогового N. Согласно условию, placeholder допустим только в pre-reg