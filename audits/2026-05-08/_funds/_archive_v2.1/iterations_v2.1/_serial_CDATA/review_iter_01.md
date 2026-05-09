# Review of CDATA

## Verdict
**FUND_AS_IS**

## Scores (1-5)
- **Premise:** 5  
  Логически необходимый вывод (¬R-аргумент) – сильная, дедуктивно обоснованная посылка, выдерживающая сравнение с альтернативами.

- **Method:** 4  
  Математическая модель (ODE, MCMC, Sobol), симулятор Cell-DT. Минус за LOO-CV mean = –0.093 (признак переобучения) и неразрешённый Sobol-парадокс, хотя он честно раскрыт.

- **Evidence:** 3  
  C2 подтверждена для нейральных прогениторов и Т-клеток, но ключевое звено (C1+C2 для HSC) отсутствует. Много косвенных данных, fabrications исправлены, но остаётся осадок.

- **Falsif:** 5  
  Десять количественных предсказаний с порогами (r, p, N), pre-registration с placeholder OSF, power-анализ. Образцово.

- **Deliv:** 4  
  Экспериментальный план реалистичен, но технически сложен (измерение PTM на единичной центриоли). Бюджет $75K для ключевого барьера указан, но не детализирован.

- **Novelty:** 5  
  Первая механистическая теория, объединяющая центриоль, ресничку и асимметричное наследование в контексте старения стволовых клеток. Потенциальный roadmap для новых геропротекторов.

- **Risk:** 4  
  Основной риск (отсутствие C1+C2 у HSC) признан, mitigation план есть. Риск фальсификации высок, но это особенность сильной теории.

## Checklist (✓/✗ each + explanation)
1. **Operationalised falsifiability (numeric thresholds)** – ✓  
   Прямые количественные пороги: r_spearman > 0.6, p < 0.01, N для каждого теста, effect size.

2. **Pre-registration plan (OSF placeholder + date)** – ✓  
   Пять тестов, все с osf.io/TBD и конкретными датами (2026-09-01 и т.д.).

3. **Sample size calc (power analysis)** – ✓  
   Для каждого теста: α=0.05, power=0.80, effect size → N (19, 25, 30, 20, 15).

4. **Risk matrix ≥5 rows** – ✓  
   5 строк: probability, impact, mitigation – все присутствуют.

5. **Limitations section** – ✓  
   Явный раздел в EVIDENCE.md (6 пунктов), включая главный пробел по HSC.

6. **Consortium / collaboration plan** – ✓  
   Четыре потенциальных партнёра с указанием институтов и ролей; placeholder для писем поддержки.

7. **References PubMed/Crossref-verified** – ✓  
   Все ссылки в EVIDENCE.md имеют PMID/DOI, дата верификации указана. Fabrications исправлены и помечены в CONCEPT.md.

8. **No fabrication markers** – ✓  
   Активных [REF_NEEDED] или [PMID_REMOVED] в тексте нет. Исправления задокументированы в комментариях.

## Top 5 text-level fixes (необязательно, т.к. FUND_AS_IS)
1. `CONCEPT.md` – Уточнить, что "in-sample R²(MCAI)=0.745" достигнут на литературных данных, а не на синтетике; уже есть, но можно вынести в executive summary.
2. `OPEN_PROBLEMS.md` – Добавить в Risk matrix строку "Failure of P8 (EPICLOCK correlation)" – это важно для NIH.
3. `EVIDENCE.md` – Чётче разграничить прямые и косвенные доказательства в таблицах (например, столбец "Type").
4. `PARAMETERS.md` – Унифицировать обозначения: `beta_HSC` имеет две формы (multiplicative dead/additive) – запутывает читателя.
5. `THEORY.md` – §4.1 (Sobol-парадокс): добавить ссылку на `PARAMS_RECONCILIATION_ANALYSIS_2026-04-21.md`, если он будет опубликован.

## PACKET
- **Fund as is, with minor stylistic improvements listed above.**  
- **Рекомендуется** добавить явный план по верификации CHIP-связи (MCOA Test 2) и конкретные сроки для UK Biobank продольного анализа.