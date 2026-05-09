# Review of BioSense

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsif: 3
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Операционализированная фальсифицируемость**  
   ✗ — Числовые пороги заданы (δ=0.05, α=0.001, d≥0.5, мощность≥0.95, AUC≥0.75), но итоговое N в разделе Falsifiability (N≥161) противоречит Sample size calculation (N=161 на группу для ВСР и N=110 на группу для ЭЭГ). Требуется согласование.

2. **Pre-registration plan**  
   ✓ — Указана платформа OSF, placeholder osf.io/ze3x7, дата до 2026-12-31. План описан.

3. **Sample size calculation**  
   ✗ — Приведены формулы и подстановки, но итоговое N = TBD. TBD недопустим в этом разделе (п.8). Также противоречие с N в Falsifiability.

4. **Risk matrix ≥5 rows**  
   ✓ — 6 строк + 1 дополнительная. Выполнено.

5. **Limitations section**  
   ✓ — Отдельный раздел с 9 пунктами. Учтены основные ограничения.

6. **Consortium / collaboration plan**  
   ✓ — Список партнёров с ролями (placeholder). Выполнено.

7. **Reference reality + match**  
   ✗ — Две ссылки не имеют реального идентификатора:
   - Tkemaladze J. "Ze EEG paper" [Manuscript under review, 2026] — нет DOI/PMID/arXiv.
   - PhysioNet EEG-MMI — указано только physionet.org, нет конкретного DOI.
   Остальные 5 проверенных ссылок реальны и соответствуют тексту.

8. **Отсутствие фабрикационных маркеров**  
   ✗ — В Sample size calculation стоит "Итоговый N = TBD". В CONCEPT.md также "N = TBD". Placeholder разрешён только в pre-registration и risk matrix. TBD здесь является фабрикационным маркером.

9. **Внутренняя согласованность core-документов**  
   ✗ — Выявлены противоречия:
   - CONCEPT.md: χ_Ze(Cuban, young 20-30) = 0.87 ± 0.04, old 60+ = 0.71 ± 0.06.  
     KNOWLEDGE.md: χ_Ze(young 18-35) = 0.5287 ± 0.036, old 60-80 = 0.4895 ± 0.036. Расхождение ~0.3.
   - CON