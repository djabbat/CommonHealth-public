# Review of EpigeneticDrift

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 4
- Evidence: 4
- Falsifiability: 4
- Deliverables: 3
- Novelty: 4
- Risk: 3

## Checklist (✓/✗ each + explanation)
1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   Falsifiability Protocol (Section 6) содержит явные пороги: 95% CI β₄ ≠ 0, p<0.05, effect size d≥0.3, power 80% f²=0.15, правила для исходов A–D.

2. **Pre-registration plan (OSF placeholder + date)** — ✗  
   Упоминается «sample size justification… along with the pre-registration», но **OSF identifier и planned date отсутствуют**. Это критическое требование для любого экспериментального проекта.

3. **Sample size calc (power analysis)** — ✓  
   Представлен расчёт: t-test N=64/group (Δ=0.2, α=0.05, power=0.80), регрессия N=92 (f²=0.15). Соответствует стандартам.

4. **Risk matrix ≥5 rows** — ✓  
   5 строк с Probability, Impact, Mitigation (ABL-2 confound, separation β₄/α₄, universality, coupling, tissue diversity).

5. **Limitations section** — ✓  
   Раздел «Опровергающие свидетельства» в EVIDENCE.md выполняет функцию limitations (честное освещение слабых мест).

6. **Consortium / collaboration plan** — ✓  
   Три группы указаны (Primary PI, potential partners Horvath/UCLA, Brunet/Stanford), предусмотрены letters of intent.

7. **References PubMed/Crossref-verified** — ✓  
   Все ссылки в EVIDENCE.md проверены через API с датой верификации. PMID указаны.

8. **No fabrication markers** — ✓  
   Маркеры [REF_NEEDED] или [PMID_REMOVED] отсутствуют.

## Top 5 text-level fixes (обязательно для перехода к REVIEW_MINOR)
1. **CONCEPT.md** (или новый файл **PREREG.md**):  
   - Вставить OSF placeholder: `OSF registration: https://osf.io/XXXXXXXX (planned for 2026-09-01)`.  
   - Указать, что все основные тесты (ABL-2 modulation, ex vivo clone tracing, factor analysis) будут зарегистрированы до начала сбора данных.

2. **OPEN_PROBLEMS.md** (Тест 1 ABL-2):  
   - Для каждого исхода A–D добавить **явное количественное правило принятия решения**, например: «Исход A: |ΔD₄| > 0.15, p < 0.01, power ≥ 0.80». Сейчас есть «предсказуемый и синхронный» — нужно конкретные метрики.

3. **EVIDENCE.md** (подраздел 3 «Опровергающие свидетельства»):  
   - Дополнить **количественной оценкой** для каждого парадокса: насколько сильно они могут изменить параметры модели (например, «If ABL-2 is primary, bias in β₄ may exceed 50%»).

4. **PARAMETERS.md** (параметры связи γ₄ⱼ):  
   - Для всех Hypothetical параметров добавить **план валидации** (какой эксперимент, ожидаемая мощность, сроки). Пример: «γ₄₃: planned measurement via rotenone treatment in HSCs, N=12 per dose, expected effect size f=0.25, power 0.80».

5. **Создать файл STATISTICAL_ANALYSIS_PLAN.md**:  
   - Детально описать первичные и вторичные исходы, методы множественного тестирования (Benjamini-Hochberg), критерии останова (interim analysis). Это требование NIH R01 / ERC AdG.

## PACKET
Ответ включает полный пакет с исправлениями. После внесения указанных изменений проект может быть оценён как REVISE_MINOR или FUND_AS_IS.