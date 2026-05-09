# Review of Telomere

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 4
- Falsif: 4
- Deliv: 2
- Novelty: 3
- Risk: 2

## Checklist

1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   CONCEPT.md §6: α₂ ≤ 10 bp/PD, β₂ ≤ 5 bp/year. Power analysis приведены. Пороги обоснованы detection limits Q-FISH.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   CONCEPT.md: `https://osf.io/abcde`, дата 2026-09-30.

3. **Sample size calc (power analysis)** — ✓  
   CONCEPT.md (FalsCond1: N≥10, FalsCond2: N≥12). OPEN_PROBLEMS.md (OP-T1: N≥12, OP-T2: N≥10).

4. **Risk matrix ≥5 rows** — ✗  
   **Отсутствует.** Нет ни одной таблицы с probability × impact × mitigation. OPEN_PROBLEMS.md — это научные гипотезы, а не risk matrix (нет вероятностей, нет стратегий смягчения). **Критическое нарушение.**

5. **Limitations section** — ✓  
   CONCEPT.md §7 «Open Questions and Limitations» (6 пунктов). EVIDENCE.md «Refuting Evidence».

6. **Consortium / collaboration plan** — ✗  
   **Отсутствует.** Ни одного упоминания партнёров, даже placeholder. Нужен минимум: перечень необходимых компетенций (биология теломер, биоинформатика, клинические когорты) и примерные институты.

7. **References PubMed/Crossref-verified** — ✓  
   Все PMID верифицированы (CONCEPT.md – скрипт, EVIDENCE.md – дата проверки). Нет непроверенных pre-print.

8. **No fabrication markers** — ✓  
   Нет [REF_NEEDED] или [PMID_REMOVED].

**Итого: 2/8 пунктов не выполнены → REVISE_MAJOR**

---

## Top 5 text-level fixes

1. **`CONCEPT.md` – добавить Risk Matrix**  
   Таблица ≥5 строк: риск (неопределённость τ₂; неразделимость α/β in vivo; игнорирование теломеразы в стволовых клетках; артефакты qPCR; неучёт гетерогенности) – вероятность (Low/Medium/High) – impact (3-5) – mitigation (доп. эксперименты, сценарии чувствительности, контрольные линии).

2. **`CONCEPT.md` – добавить Consortium / Collaboration Plan**  
   Placeholder: «Планируемое сотрудничество с лабораториями по теломерной биологии (например, группа [PMID:24374808] – измерение α₂; группа [PMID:39837827] – окислительный стресс; клинические когорты для in vivo валидации). Детали – в WP1 EIC Pathfinder.»

3. **`CONCEPT.md` §6 – явно указать statistical tests**  
   Для каждого Falsification Condition: «FalsCond1: one-sided t-test (H₀: α₂ ≤ 10 bp/PD), α=0.05, power=0.80. FalsCond2: linear mixed model с random intercept, test slope β₂ > 0…» Сейчас тесты только в power analysis, но не в самом протоколе.

4. **`OPEN_PROBLEMS.md` – для OP-T1/T2/T3 добавить risk assessment**  
   Каждый Open Problem должен содержать оценку: вероятность исхода, влияние на модель, план действий (→ Model Update Protocol). Сейчас есть decision tree, но нет вероятностей.

5. **`PARAMETERS.md` – добавить suggested measurement protocols**  
   Для каждого параметра с пометкой «To Be Calibrated» или «Hypothesized» – краткое описание эксперимента, которым его можно измерить (ссылка на OPEN_PROBLEMS). Сейчас uncertainty есть, но protocols отсутствуют.

---

## Дополнительные замечания (не обязательные, но желательные)

- **`CONCEPT.md` §3 – уточнить, что β₂·(t/τ₂) – это линейное приближение. В OPEN_PROBLEMS признаётся, что может быть стохастический процесс. Это хорошо, но в основном уравнении стоит линейный член – добавьте оговорку.**
- **`EVIDENCE.md` – для PMID:30472697 указано «ошибка в заголовке». Это нормально, но стоит явно написать, что проверен title статьи, а не заголовок в цитате.**
- **`DESIGN.md` – coupling_inputs в update() захардкожено как placeholder. Добавьте хотя бы прототип функции `_compute_beta_effective` с учётом матрицы Γ.**