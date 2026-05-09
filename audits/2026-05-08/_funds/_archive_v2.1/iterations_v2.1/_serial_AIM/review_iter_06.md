# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsifiability: 3
- Deliverables: 2
- Novelty: 4
- Risk: 2
- Reference Integrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   Пороги указаны: PAM-13 Δ ≥ 5.4 points, α = 0.05 (primary), power 0.80, N ≥ 55 per group, stopping rule, interim analysis. Небольшая путаница с Bonferroni (в CONCEPT.md только для secondary, в THEORY.md без уточнения), но в целом условие выполнено.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Присутствует `osf.io/TBD` (placeholder) и дата 2026-09-01. Placeholder допустим.

3. **Sample size calc (power analysis)** — ✓  
   Формула, подстановка, sensitivity analysis (σ = 8, 10, 12) присутствуют. Дублирование фрагмента — технический дефект, но условие выполнено.

4. **Risk matrix ≥5 rows** — ✓  
   В разных разделах CONCEPT.md приведены матрицы с 5, 7 и 5 строками. Требование выполнено.

5. **Limitations section** — ✓  
   Отдельный раздел в CONCEPT.md (8 пунктов) и в THEORY.md (8 пунктов). Выполнено.

6. **Consortium / collaboration plan** — ✓  
   Перечислены роли (Lead PI, Co-I Clinical, Co-I Technical) и потенциальные партнёры (Insignia Health, Fraunhofer IGD, TSU, University of Copenhagen). Placeholder-имена допустимы.

7. **Reference reality + match** — **✗**  
   **Три ссылки имеют невалидные идентификаторы (DOI TBD / PMID неверен):**  
   - Tao et al. (2026) — `[pre-print; DOI TBD]` (нереальный идентификатор).  
   - Blumenthal & Lee (2024) — `[pre-print; DOI TBD]`.  
   - Tkemaladze (2026) — `[pre-print; DOI TBD]`.  
   - Hibbard et al. (2004) в одном месте указан PMID 15527447 (это статья 2005 года) — невалидный идентификатор для 2004 года.  
   - Jaba (2022) и Tkemaladze (2023) не имеют PMID/DOI.  
   **Это автоматический REJECT компонента** (п. 7а правил: «Невалидный идентификатор = автоматический REJECT»).

8. **No fabrication markers** — **✗**  
   В ссылках присутствуют `