# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 3
- Deliv: 2
- Novelty: 3
- Risk: 3
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)
1. **Operationalised falsifiability (numeric thresholds)** ✓  
   Числовые пороги есть (PAM-13 Δ ≥ 5.4, α=0.05, power=0.80, N≥55 per group). Однако присутствуют внутренние расхождения (α=0.05 vs. Bonferroni 0.025; secondary outcomes количество не совпадает с correction). Тем не менее базовые числа указаны.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   Указан OSF ID `osf.io/TBD` как placeholder, дата 2026-09-01, описан зарегистрированный дизайн. Условие выполнено.

3. **Sample size calc (power analysis)** ✓  
   Приведена формула, подстановка значений, расчёт N=55 per group, sensitivity analysis для σ=8,12, justification σ=10. Есть учёт dropout (20%).

4. **Risk matrix ≥5 rows** ✓  
   Представлено несколько матриц, минимальное количество строк ≥5 (например, 7 строк в CONCEPT.md). Условие выполнено.

5. **Limitations section** ✓  
   Явный раздел с перечислением 8 ограничений. Назван «Limitations», содержание адекватное.

6. **Consortium / collaboration plan** ✓  
   Указаны Lead PI, Co-I, потенциальные партнёры (Insignia Health, Fraunhofer IGD, TSU, University of Copenhagen) с ролями. Отмечено, что письма поддержки в процессе.

7. **Reference reality + match** ✗  
   — **Tao et al. (2026) *Nature Medicine*: DOI TBD** — невалидный идентификатор (не разрешается).  
   — **Blumenthal D., Lee J. (2024) *JAMA*: DOI TBD** — невалидный.  
   — **Tkemaladze J. (2026) *Longevity Horizon*: DOI TBD** — невалидный.  
   — **Hibbard et al. (2004)** в одном из экземпляров Sample size calc указан PMID 15527447 вместо правильного 15333167, что является неверным идентификатором (статья Hibbard et al. 2004 имеет PMID 15333167; 15527447 относится к Hibbard et al. 2005).  
   — Несмотря на наличие примечания об опечатке, в самом тексте остаётся ошибочный PMID.  
   Соответствие тексту для pre-prints (Tao, Blumenthal, Tkemaladze) невозможно проверить из-за отсутствия разрешаемого идентификатора.  
   **Итого: 3 из 6 цитируемых работ имеют невалидные/отсутствующие идентификаторы; одна работа содержит ошибочный PMID. Условие не выполнено.**

8. **No fabrication markers** ✗  
   В ссылках присутствуют «DOI TBD» — это прямой пример «TBD» там, где должны стоять конкретные данные. Согласно правилу, placeholder допустим только в pre-reg плане и risk matrix. Использование «TBD» в библиографических ссылках является фабрикационным маркером.

9. **Internal consistency core docs** ✗  
   — **Falsifiability:** в одном месте α=0.05 с Bonferroni 0.025 для ≤2 вторичных исходов, в другом — просто α=0.05. Количество secondary outcomes (3) не соответствует коррекции (≤2).  
   — **Sample size:** в одном месте N≥55 per group, в другом Total N = 132 (с учётом dropout 20%). Несоответствие между «N≥55 per group» (чисто аналитическая выборка) и общим числом с dropout ведёт к путанице: целевая randomised выборка 110, но из текста неясно, должен ли N=132 включать dropout или это отдельный параметр.  
   — **Списки secondary outcomes:** в Pre-registration plan указаны MMAS-8, EQ-5D-5L, число госпитализаций; в другом месте список иной (physician time per visit, medication adherence, adverse events).  
   — **Ссылки на провайдеров:** README.md содержит устаревшую информацию (KIMI, Qwen как rejected, но помечены как «not implemented»).  
   — **Дублирование секций (Falsifiability, Pre-registration, Sample size)** создаёт риск разночтений.  
   Условие внутренней согласованности не выполнено.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Hibbard et al. (2004) PAM development, Health Serv Res 39(4 Pt 1):1005–26 | PMID 15333167 | Да (реальная статья