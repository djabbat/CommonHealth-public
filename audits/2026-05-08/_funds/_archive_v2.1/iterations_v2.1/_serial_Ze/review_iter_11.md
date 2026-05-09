# Review of Ze

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- **Premise:** 3 — оригинальная теоретическая рамка, но слабая связь с данными и неопределённая эмпирическая база  
- **Method:** 2 — множество TBD, отсутствие конкретных численных порогов, незавершённый анализ мощности  
- **Evidence:** 2 — крайне мало данных (N≤196 с высокой гетерогенностью), нет независимых репликаций, все эффекты — исследовательские  
- **Falsif:** 2 — численные пороги не операционализированы, нет формальных правил отвержения гипотез  
- **Deliv:** 2 — рабочие пакеты, вехи и конкретные результаты не описаны; проект находится на стадии концепции  
- **Novelty:** 4 — Ze Vectors Theory — новая интерпретативная рамка, сочетающая квантовые и информационные идеи  
- **Risk:** 2 — высокий риск из-за отсутствия валидированных инструментов, малой выборки и неопределённости в ключевых параметрах  

## Checklist (✓/✗ each + explanation)

1. **✗ Operationalised falsifiability (numeric thresholds)**  
   В таблице гипотез все effect size, α, power, N — TBD. Нет пороговых значений для отклонения гипотез (например, «p > 0.05 → reject»). Без конкретных чисел условие не выполнено.

2. **✓ Pre-registration plan (OSF placeholder + date)**  
   Указаны placeholder `osf.io/TBD` и даты (2026-09-01, 2026-12-01). Формально есть.

3. **✗ Sample size calc (power analysis)**  
   Формула и α=0.05, power=0.80 приведены, но effect size и σ = TBD, calculated N = TBD. Расчёт не завершён, конкретное N не обосновано.

4. **✓ Risk matrix ≥5 rows**  
   Матрица содержит 6 строк с оценками Probability, Impact, Mitigation. Выполнено.

5. **✓ Limitations section**  
   Раздел «2.0.1. Limitations» включает 7 пунктов. Выполнено.

6. **✓ Consortium / collaboration plan**  
   Описаны текущий PI и три planned партнёра с пометкой «letters of support pending». Placeholder-список присутствует.

7. **✗ All references PubMed/Crossref-verified or marked as pre-print**  
   Есть PMID 27330520, есть ссылки на bioRxiv (pre‑print). Однако в KNOWLEDGE.md присутствуют строки «[Reference removed — citation pending verification]» — часть ссылок не верифицирована. Условие не выполнено.

8. **✗ No fabrication markers ([REF_NEEDED] / [PMID_REMOVED])**  
   KNOWLEDGE.md содержит маркеры «[Reference removed — citation pending verification. Will be replaced with valid PMID or removed entirely.]». Fabrication marker присутствует, хотя проект проводит чистку. Условие не выполнено.

## Top 5 text-level fixes (добавить/изменить)

1. **CONCEPT.md: «Operational falsifiability for each hypothesis»**  
   Заменить все TBD на конкретные числа. Пример: для гипотезы `v*_active > v*_passive`: Cohen’s d ≥ 0.3, α = 0.05, power = 0.80, N per group = 64. Добавить строку: «Hypothesis rejected if p > 0.05 or effect size < 0.2 (two-sided).»

2. **CONCEPT.md: «Sample size calculation (power analysis)»**  
   Предоставить расчёт N на основе предполагаемого effect size из пилотных данных (или литературы) с указанием источника σ. Пример: «δ = 0.3 (based on pilot N=12: Δv = 0.15, SD = 0.5), σ = 0.5 → n = (1.96+0.84)²·0.5²/0.3² ≈ 68 per group. Planned N = 70 per group.»

3. **KNOWLEDGE.md: Удалить все fabrication markers**  
   Строки вида «[Reference removed — citation pending verification...]» заменить корректными ссылками (PMID/DOI) или, если верификация невозможна, удалить совсем. Для pre-print указать статус: «[Pre-print, not peer-reviewed]».

4. **CONCEPT.md: «Risk matrix» — добавить строку для «Non‑reproducibility due to measurement error»**  
   Probability 0.6, Impact 0.8, Mitigation: «Use dual‑system recording (BioSense + Emotiv), pre‑register analysis code, share processed data on OSF.»

5. **CONCEPT.md: «Limitations section» — добавить план устранения каждого ограничения с конкретными сроками**  
   Например: «Limitation 3 (Sample size): WP2 will collect N = 100 per group by Q2 2027 (power analysis confirms N = 64 required). Limitation 4 (Independent replication): collaboration with TBD lab planned, protocol shared by Q3 2026.»

## PACKET

Ze