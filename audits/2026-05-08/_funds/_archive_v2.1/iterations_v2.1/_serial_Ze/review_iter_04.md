# Review of Ze

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 1
- Falsif: 2
- Deliv: 2
- Novelty: 4
- Risk: 5

## Checklist (✓/✗ each + explanation)

1. **✗ Operationalised falsifiability (numeric thresholds)** — Таблица присутствует, но все effect sizes и required N указаны как `TBD`. «Operationalised» требует конкретных чисел (d=0.5, N=64 и т.д.), а не плейсхолдеров. Без этого пункт не засчитывается.

2. **✓ Pre-registration plan (OSF placeholder + date)** — Есть `osf.io/TBD` с датами 2026-09-01 и 2026-12-01. Формально выполнено, хотя идентификатор — плейсхолдер.

3. **✗ Sample size calc (power analysis)** — Формула приведена, но σ, δ, calculated N везде `TBD`. Реальный расчёт отсутствует. Power analysis без числовых значений не является расчётом.

4. **✓ Risk matrix ≥5 rows** — 6 строк с probability, impact, mitigation. Выполнено.

5. **✓ Limitations section** — 7 явных пунктов. Выполнено.

6. **✓ Consortium / collaboration plan** — Перечислены планируемые роли (PI, neurophysiology, biostatistics, clinical, data management), хотя все партнёры `TBD`. Формально план есть.

7. **✗ References PubMed/Crossref-verified** — В тексте нет списка литературы. Упоминания (BrainYears, Wearable Aging Clock) даны без PMID/DOI-верификации. Не выполнено.

8. **✗ No fabrication markers** — В `KNOWLEDGE.md` и `CONCEPT.md` присутствуют строки: `[Reference pending — placeholder; will be replaced...]` и `[reference removed — to be replaced...]`. Это прямые маркеры незавершённой очистки. Даже если процесс идёт, на момент ревью маркеры есть → пункт не пройден.

**Итог:** 5/8 пунктов не выполнены или выполнены частично → **REVISE_MAJOR**.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

1. **`CONCEPT.md: §2.3.0 (Operational falsifiability table)`** — Заменить все `TBD` на конкретные числа, включая effect size (хотя бы 0.5 по Cohen), α=0.05, power=0.80, N per group. Если данных нет — указать «to be determined in pilot phase» и дать обоснование, почему chosen effect size разумен.

2. **`CONCEPT.md: §2.3.0 (Sample size calculation)`** — Привести полный расчёт: σ и δ из литературы или собственных пилотов (хотя бы N=12). Указать, что при отсутствии данных используется консервативная оценка. Ссылка на PMID для σ.

3. **`KNOWLEDGE.md / CONCEPT.md`** — Удалить все строки вида `[Reference pending — placeholder]`, `[reference removed — to be replaced...]`. Заменить на реальные PMID или DOI. Если ссылка ещё не найдена — убрать утверждение, а не оставлять маркер.

4. **`CONCEPT.md: §8 (v*_active)`** — Привести статистику per dataset (Cuban, Dortmund, MPI-LEMON) с 95% CI и Cochran Q. Чётко указать, что pooled estimate некорректен из-за I²=90.3%. Дать план будущего анализа (bootstrap, permutation test).

5. **`CONCEPT.md: §7 (Limitations)`** — Добавить explicit признание, что все pilot данные имеют N<200, не pre-registered, и ни одна гипотеза не реплицирована независимо. Указать, что любое клиническое утверждение — исследовательское и не должно цитироваться как результат.

## PACKET

**Файлы для доработки:**  
- `Ze/CONCEPT.md` — заполнить TBD, убрать fabrication markers, добавить верифицированные ссылки.  
- `Ze/KNOWLEDGE.md` — удалить placeholder-маркеры, заменить валидными PMID.  

**Срок:** 60 дней для повторной подачи. При следующем ревью невыполнение пунктов 1,3,7,8 → автоматический REJECT.