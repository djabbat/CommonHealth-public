# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

---

## Scores (1-5)
- Premise: 4  
- Method: 3  
- Evidence: 3  
- Falsif: 3  
- Deliv: 3  
- Novelty: 4  
- Risk: 4  
- RefIntegrity: 5  

---

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)**  
   ✗ **Частично не выполнено.** Пороги и статистические тесты есть (concordance >0.80, uptime >0.90, contamination <3%, cost ≤4500). Однако для contamination sample size = TBD, для uptime H₀: ≤0.90, хотя в predictions указано ≥95% – числовые пороги не везде согласованы и не полностью операционализированы. Нарушение пункта 1.

2. **Pre-registration plan (OSF placeholder + date)**  
   ✓ Есть OSF ID `osf.io/automicroscopy_cdata` и дата 2026-06-01. Формально выполнено (placeholder допустим в pre-reg).

3. **Sample size calc (power analysis)**  
   ✗ **Не выполнено.** Для concordance расчёт есть (N=286), для uptime – 180 дней фиксировано. Однако в разделе Sample size calculation CONCEPT.md первая формула содержит σ² = TBD, δ = TBD, n = TBD. Для contamination N = TBD. Также design effect = TBD. Наличие TBD в sample size calculation недопустимо (см. п.8). Также внутренняя противоречивость: два разных расчёта для CDATA experiment (один с TBD, другой конкретный).

4. **Risk matrix ≥5 rows**  
   ✓ Присутствует две матрицы: одна с 6 рисками (text), другая с 6 рисками (числа). Формально ≥5 строк выполнено.

5. **Limitations section**  
   ✓ Есть явные разделы в CONCEPT.md (8 пунктов) и в EVIDENCE.md. Выполнено.

6. **Consortium / collaboration plan**  
   ✓ Таблица партнёров с ролями и статусом в CONCEPT.md (6 партнёров). Есть также список в DESIGN.md. Выполнено.

7. **Reference reality + match**  
   ✓ Все 10 внешних ссылок в EVIDENCE.md проверены: DOI/PMID разрешаются, содержание соответствует утверждениям. Ссылки без DOI (Zeiss manual, FLIR datasheet, Micro-Manager, inkbird) – реальные источники (manufacturer spec, open-source). Нарушений нет. **RefIntegrity score = 5.**

8. **No fabrication markers**  
   ✗ **Не выполнено.** В sample size calculation (CONCEPT.md) присутствуют TBD для σ², δ, n, design effect, contamination N. По правилу placeholder допустим **только** в pre-reg плане и risk matrix. Здесь TBD в sample size – нарушение. Также в falsifiability для contamination «Required N: TBD (placeholder)». Это fabrication markers по определению.

9. **Internal consistency core docs**  
   ✗ **Не выполнено.** Множественные противоречия и дублирования:  
   - Uptime target: predictions говорят ≥95%, falsifiability H₀: uptime ≤0.90.  
   - Sample size для CDATA: первый блок с σ²=TBD, второй – конкретные числа (Cohen's d=0.75).  
   - Risk matrix дублируется дважды (текстовый и числовой).  
   - Limitations дублируются (CONCEPT.md и EVIDENCE.md).  
   - Pre-reg план описан в двух разных местах с разной детализацией.  
   - DESIGN.md, PARAMETERS.md, OPEN_PROBLEMS.md – заглушки с минимальным содержанием, что нарушает согласованность core-документов (требование п.9).

**Итого:** выполнены пункты 2,4,5,6,7 (5 из 9). Пункты 1,3,8,9 не выполнены. ➔ **REVISE_MAJOR.**

---

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|------------------------|---------|
| 1 | OpenFlexure XY stage accuracy | 10.1063/1.4941068 | Yes (DOI → Rev Sci Instrum 2016) | Yes | OK |
| 2 | 37°C + 5% CO₂ for BJ-hTERT | 10.1016/0014-4827(65)90211‑9 (+ PMID 14315085) | Yes (Hayflick 1965) | Yes | OK |
| 3 | CellPose v2 segmentation | 10.1038/s41592-020-01018-x | Yes (Nat Methods 2021)