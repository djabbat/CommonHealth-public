# Open Problems in Telomere Counter Formalization

## OP-T1: Количественное определение константы времени стресс-зависимой эрозии (`τ₂`)


### Risk Assessment for OP-T1

| Outcome | Probability | Impact on Model | Action (Model Update Protocol) |
|---------|-------------|-----------------|--------------------------------|
| 1. L_avg(t) linear, Var(t) linear | 0.40 (moderate) | Low – confirms current parameterization | Estimate τ₂ = β₂ / slope; update PARAMETERS.md |
| 2. L_avg(t) nonlinear, Var(t) jump | 0.30 (moderate) | High – requires reformulation | Replace β₂·(t/τ₂) with stochastic jump process; update CONCEPT.md §2 |
| 3. L_avg(t) unchanged, Var(t) jump, TIF+ | 0.20 (low) | Critical – paradigm shift | Switch to subpopulation model; update CONCEPT.md §1 and §2 |
| 4. No significant change in L_avg or Var | 0.10 (low) | Critical – falsification | Reject β₂ contribution in vitro; revise CONCEPT.md §2; design in vivo test |

**Decision rule:** If outcome 2, 3, or 4 occurs, initiate Model Update Protocol (see DESIGN.md §5).


**Statistical criterion for 'no significant change' (Outcome 4):**
- **Frequentist:** p > 0.10 from a two-sided t-test comparing slope to zero, AND an equivalence test (TOST) with equivalence bounds of ±2 bp/PD shows p < 0.05 for equivalence.
- **Bayesian:** Bayes factor (BF01) > 3 in favor of the null model (no slope) over the alternative model (slope ≠ 0).

If both criteria are met, Outcome 4 is confirmed. If only one criterion is met, the result is considered inconclusive and additional data (N increased by 50%) will be collected.




**Power analysis for OP-T1:**



**Statistical Analysis Plan (SAP) for OP-T1:**
- **Primary endpoint:** Slope of L_avg(t) (bp/PD) under stress vs. control conditions.
- **Secondary endpoint:** Variance of telomere length distribution Var(t) at each time point.
- **Multiple comparison correction:** Bonferroni correction for two primary comparisons (slope and variance) → α_adj = 0.025 per test.
- **Missing data handling:** Last observation carried forward (LOCF) for cultures that senesce early; sensitivity analysis using complete-case only.
- **Controls:** Matched passage number, same donor, same culture medium (except stressor). Scrambled shRNA control for OP-T3 (if applicable).
- **Replication strategy:** Split each condition into two independent culture flasks (technical replicates); repeat entire experiment with a second donor line (biological replicate).

Effect size: expected slope difference of 5 bp/PD between stress and control conditions, SD = 4 bp/PD (from PMID:24374808). Two-sided t-test, α = 0.05, power = 0.80 → N ≥ 7 independent cultures per condition (calculated as: n = (1.96 + 0.84)² · 4² / 5² = 7.84 · 16 / 25 = 5.02 → 6, adjusted for 10% attrition → 7).



**Проблема:** В уравнении `D₂(n, t) = ... + β₂ · (t / τ₂)` параметр `τ₂` является наименее охарактеризованным. Мы интерпретируем его как характерное время для существенной стресс-зависимой потери теломер. Отсутствие прямой экспериментальной оценки `τ₂` делает член `β₂ · (t / τ₂)` плохо ограниченным — неясно, происходит ли эрозия равномерно по времени или скачкообразно с определённой частотой.

**Фальсифицируемый тест (in silico / in vitro):**
* **Дизайн:** Длительное культивирование первичных человеческих фибробластов в условиях контролируемого окислительного стресса (например, стабильная низкая доза paraquat или колебания кислорода 5% ↔ 20%). Каждые 3-5 дней (или каждое деление) проводить single-cell Q-FISH для >100 клеток, получая не только среднюю длину, но и полное распределение длин теломер.
* **Измеряемые величины:** 1) Средняя длина теломер во времени `L_avg(t)`. 2) Дисперсия распределения длин `Var(t)`. 3) Доля клеток с TIF (Telomere Dysfunction-Induced Foci).
* **Прогноз модели с постоянным `τ₂`:** `L_avg(t)` будет линейно уменьшаться во времени при постоянном стрессе. `Var(t)` будет линейно возрастать, если `β₂` имеет стохастическую природу.
* **Четыре возможных исхода:**
 1. **`L_avg(t)` линейна, `Var(t)` растёт линейно:** Подтверждает текущую параметризацию. `τ₂` можно оценить как время, за которое `L_avg` уменьшается на `β₂` bp. (`τ₂ = β₂ / slope`) Приоритет: Низкий (подтверждение).
 2. **`L_avg(t)` нелинейна (например, ступенчато), `Var(t)` скачкообразно меняется:** Свидетельствует о том, что стресс-зависимая эрозия происходит не непрерывно, а дискретными событиями (например, кризис отдельных теломер). Требует переформулировки члена `β₂ · (t / τ₂)` в stochastic jump process. Приоритет: Высокий (модификация теории).
 3. **`L_avg(t)` не меняется, `Var(t)` резко возрастает, TIF+ клетки появляются:** Указывает, что стресс не укорачивает все теломеры равномерно, но вызывает катастрофическое укорочение отдельных теломер в подпопуляции клеток. Это потребует перехода от модели средней длины к модели субпопуляций. Приоритет: Критический (смена парадигмы).
 4. **Ни `L_avg(t)`, ни `Var(t)` значимо не меняются под стрессом:** Фальсифицирует гипотезу о значимом вкладе окислительного стресса (`β₂`) в укорочение теломер в данной клеточной системе in vitro. Ставит под вопрос аксиому T1 для этих условий. Приоритет: Критический (фальсификация).

**Приоритет:** Высокий. Без оценки `τ₂` количественные предсказания модели для in vivo старения ненадёжны.

## OP-T2: Экспериментальное разделение вкладов `α₂` и `β₂` in vivo


### Risk Assessment for OP-T2

| Outcome | Probability | Impact on Model | Action (Model Update Protocol) |
|---------|-------------|-----------------|--------------------------------|
| 1. α₂ and β₂ separable (distinct slopes) | 0.50 (moderate) | Low – confirms model | Update PARAMETERS.md with in vivo estimates |
| 2. α₂ and β₂ confounded (collinear) | 0.30 (moderate) | High – requires additional experiments | Design intervention study (e.g., antioxidant treatment) to break collinearity |
| 3. β₂ not detectable in vivo | 0.15 (low) | High – revise model | Remove β₂ term for in vivo predictions; update CONCEPT.md §2 |
| 4. α₂ not detectable in vivo | 0.05 (low) | Critical – paradigm shift | Reconsider role of division-dependent erosion in vivo; update CONCEPT.md §1 |

**Decision rule:** If outcome 2, 3, or 4 occurs, initiate Model Update Protocol (see DESIGN.md §5).



**Power analysis for OP-T2:**
Effect size: expected α₂ contribution vs β₂ contribution. The test requires two-photon imaging or equivalent equipment for in vivo visualization. contribution of 15 bp/PD vs β₂ contribution of 5 bp/year, SD = 8 bp/PD (from PMID:24374808). Two-sided t-test, α = 0.05, power = 0.80 → N ≥ 10 independent longitudinal cohorts per condition.

Effect size: expected α₂ = 5 bp/PD, SD = 4 bp/PD (from PMID:24374808). Two-sided t-test, α = 0.05, power = 0.80 → N ≥ 12 independent cell lines per condition.




**Проблема:** В организме скорость деления клеток (`dn/dt`) неизвестна с точностью. Поэтому измеряемая в лейкоцитах скорость укорочения (например, 30 bp/год) является суммой `(α₂ / n₂*) * (dn/dt) + (β₂ / τ₂)`. Невозможно определить, какая часть обусловлена делениями, а какая — стрессом, что затрудняет проверку предсказания P1 и калибровку модели.

**Фальсифицируемый тест (in vivo, мышиная модель):**
* **Дизайн:** Использовать двухрепортерную систему в трансгенных мышах: 1) Гистон H2B-GFP для отслеживания делений (метод dilution), 2) Теломерный зонд для Q-FISH. Провести продольный отбор проб из медленно (нейроны, кардиомиоциты) и быстро (крипты кишечника, клетки крови) обновляющихся тканей у одной и той же особи в возрасте 2, 6, 12, 18 месяцев.
* **Измеряемые величины:** 1) Средняя длина теломер на клетку в каждой ткани (`L_tissue(age)`). 2) Среднее число делений, пройденных клеточной линией в каждой ткани (`n_tissue(age)`), по разведению GFP.
* **Прогноз модели:** Для быстрообновляющихся тканей: `L_tissue(age)` сильно коррелирует с `n_tissue(age)` (доминирует `α₂`). Для медленнообновляющихся: `L_tissue(age)` коррелирует с `age`, но не с `n_tissue(age)` (доминирует `β₂`). Общий тренд: `L_tissue(age) = D₂,₀ + α₂ * (n_tissue(age)/n₂*) + β₂ * (age/τ₂)`.
* **Четыре возможных исхода:**
 1. **Данные хорошо описываются уравнением, с разными коэффициентами для разных тканей:** Подтверждает модель, позволяет методом multiple linear regression оценить `α₂/n₂*` и `β₂/τ₂` in vivo. Приоритет: Низкий (подтверждение).
 2. **`L_tissue(age)` коррелирует ТОЛЬКО с `age` во всех тканях, независимо от `n`:** Фальсифицирует значимость division-dependent компонента (`α₂`) in vivo. Указывает, что in vivo укорочение в основном определяется стрессом/временем. Приоритет: Критический (фальсификация ключевого компонента).
 3. **`L_tissue(age)` коррелирует ТОЛЬКО с `n_tissue(age)`, даже в постмитотических тканях (`n≈const`):** Невозможно в рамках модели, так как предсказывает неизменную длину. Если длина меняется, это немедленно фальсифицирует модель. Более вероятно, что метод измерения `n` ошибочен. Приоритет: Средний (проверка методологии).
 4. **Данные не описываются линейной суммой `n` и `age`, а следуют сложной нелинейной траектории:** Указывает на недостаточность модели или на сильные нелинейные связи (Γ матрица). Например, если при старении `dn/dt` само меняется, или `β₂` зависит от `D₂`. Требует усложнения модели. Приоритет: Высокий (развитие теории).

**Приоритет:** Высокий. Критичен для валидации MCOA, так как проверяет саму идею разделения счётчиков по драйверам (деления vs. время).

## OP-T3: Верификация связи Γ_{2,5} (Протеостаз → Теломеры) через RIOK2-TRiC

**Проблема:** Есть данные (PMID: 39164231), что RIOK2 регулирует сборку теломеразы через транскрипцию комплексов TRiC (CCT) и дискерина. Это предполагает связь между счётчиком протеостаза (#5) и эффективностью поддержания теломер. Однако количественная связь между уровнем повреждения протеостаза (`D₅`) и скоростью теломерной эрозии не установлена.

**Фальсифицируемый тест (in vitro, генетическое вмешательство):**
* **Дизайн:** В клеточной линии с низкой базальной теломеразной активностью (например, фибробласты) создать: 1) Knockdown RIOK2 (shRNA), 2) Knockdown отдельной субъединицы CCT (например, CCT5), 3) Контроль (scramble). Во всех линиях дополнительно индуцировать лёгкий протеостатический стресс (например, сублетальная доза MG132 или тепловой шок). Контроль без стресса.
* **Измеряемые величины:** 1) Скорость укорочения теломер на деление (TRF или Q-FISH) в условиях стресса и без. 2) Активность теломеразы (TRAP assay). 3) Уровень агрегации/неправильного свёртывания белков (флуоресцентные репортёры).
* **Прогноз модели:** При нарушении протеостаза (knockdown + стресс) скорость укорочения теломер (`α₂_effective` или `β₂_effective`) увеличится сильнее, чем в контроле с таким же стрессом. Эффект knockdown RIOK2 или CCT имитирует/усиливает эффект протеостатического повреждения.
* **Четыре возможных исхода:**
 1. **Скорость укорочения увеличивается пропорционально тяжести нарушения протеостаза, коррелируя с падением теломеразной активности:** Подтверждает количественную связь Γ_{2,5}. Позволяет оценить коэффициент связи. Приоритет: Низкий (подтверждение).
 2. **Скорость укорочения увеличивается, но теломеразная активность не меняется:** Указывает на альтернативный механизм связи (например, через стабильность шелтерина), не связанный с теломеразой. Требует пересмотра механизма Γ_{2,5}. Приоритет: Средний.
 3. **Нарушение протеостаза не влияет на скорость укорочения, но резко повышает долю TIF+ клеток:** Указывает, что связь идёт не через скорость эрозии, а через эффективность capping (функция шелтерина). Это потребует переопределения `g_5(D_5)` в матрице Γ как влияющей на порог сенесценции, а не на `dD₂/dt`. Приоритет: Высокий.
 4. **Никакого эффекта на теломеры не наблюдается:** Фальсифицирует гипотезу о значимой связи протеостаза с теломерным maintenance в данной клеточной системе. Приоритет: Критический (фальсификация связи).

**Приоритет:** Средний. Важно для построения сети связей MCOA, но не является краеугольным камнем самой модели теломер.

## OP-T4: Разрешение парадокса стабильности/удлинения теломер у долгоживущих видов

**Проблема:** У некоторых видов (голый землекоп, некоторые киты) или в конкретных нишах (кишечные стволовые клетки) длина теломер не уменьшается с возрастом или демонстрирует неожиданную стабильность. Это противоречит простой экстраполяции нашей модели, если только не предположить `α₂ ≈ 0` и `β₂ ≈ 0`, что биологически маловероятно.

**Фальсифицируемый тест (сравнительная биология / анализ данных):**
* **Дизайн:** Систематический сбор опубликованных данных по длине теломер и возрасту для видов с аномальными паттернами (голый землекоп, bowhead whale, летучая мышь *Myotis*). Анализ с использованием расширенной модели, включающей: 1) Базальную активность теломеразы/ALT (`η`), как член `-η·t` в уравнении. 2) Возрастзависимую регуляцию параметров (например, `β₂(age)` может снижаться из-за усиленных антиоксидантных систем).
* **Измеряемые величины:** 1) Наклон регрессии длина/возраст в публичных данных. 2) Наличие/активность теломеразы в соматических тканях. 3) Уровень маркеров окислительного стресса.
* **Прогноз расширенной модели:** Паттерн "стабильности" может быть объяснён балансом: `dD₂/dt = (α₂ / n₂*)·(dn/dt) + (β₂ / τ₂) - η`. Для долгоживущих видов `η ≈ (α₂ / n₂*)·(dn/dt) + (β₂ / τ₂)`. Альтернативно, `β₂` у них может быть много ниже из-за superior stress resistance.
* **Четыре возможных исхода:**
 1. **Данные хорошо описываются моделью с ненулевым `η` или пониженным `β₂`:** Подтверждает обобщаемость модели после включения механизмов maintenance. Приоритет: Низкий.
 2. **Данные показывают периодические или скачкообразные изменения длины, а не тренд:** Указывает на сильную регуляцию или эпизодическую активацию ALT, что не укладывается в непрерывную модель. Требует перехода к модели с дискретными событиями рекомбинации/удлинения. Приоритет: Высокий.
 3. **Длина теломер положительно коррелирует с возрастом в некоторых тканях:** Радикально противоречит базовой аксиоме о накоплении дефицита. Может указывать на селективное преимущество клеток с длинными теломерами с возрастом или на артефакт измерения. Требует тщательной методологической проверки. Приоритет: Критический.
 4. **Нет корреляции между длиной теломер и максимальной продолжительностью жизни вида:** Ставит под вопрос универсальную роль теломер как лимитирующего счётчика старения за пределами конкретных моделей (человек, мышь). Приоритет: Средний (ограничивает область применимости MCOA).

**Приоритет:** Средний. Проблема важна для эволюционной геронтологии и общности MCOA, но не блокирует прогресс в основной человеко-ориентированной модели.

## Model Update Protocol (Decision Tree)

Based on the outcomes of OP-T1 and OP-T2, the following decision tree governs model revision:

1. **If OP-T1 outcome = 1 (linear L_avg, linear Var):**
   - Action: Keep current parameterization. Estimate τ₂ = β₂ / slope. Update CONCEPT.md with empirical τ₂ value.
   - Priority: Low (confirmation).

2. **If OP-T1 outcome = 2 (non-linear L_avg, stepwise Var):**
   - Action: Replace continuous β₂·(t/τ₂) term with a stochastic jump process (e.g., Poisson-distributed erosion events). Revise Equation 1 in CONCEPT.md.
   - Priority: High (theory modification).

3. **If OP-T1 outcome = 3 (L_avg unchanged, Var spikes, TIF+ cells appear):**
   - Action: Transition from mean-field model to subpopulation model. Introduce a fraction of cells with catastrophic telomere loss. Revise D₂ definition to include distribution moments.
   - Priority: Critical (paradigm shift).

4. **If OP-T1 outcome = 4 (no significant change in L_avg or Var):**
   - Action: Falsify hypothesis that oxidative stress (β₂) contributes significantly to telomere erosion in this system. Remove β₂ term or set β₂ = 0 for in vitro conditions. Re-evaluate in vivo relevance.
   - Priority: Critical (falsification).

5. **If OP-T2 fails to separate α₂ and β₂ in vivo:**
   - Action: Adopt a reduced model with a single effective erosion rate (α₂ + β₂/τ₂) for in vivo applications. Document loss of mechanistic resolution.
   - Priority: High (model simplification).

## Consolidated Risk Matrix (OP-T1 through OP-T4)



| Problem | Outcome | Probability | Impact on Model | Mitigation / Action |
|---------|---------|-------------|-----------------|---------------------|
| OP-T1: τ₂ quantification | L_avg(t) linear, Var(t) linear | 0.40 | Low – confirms current parameterization | Estimate τ₂ = β₂ / slope; update PARAMETERS.md |
| OP-T1: τ₂ quantification | L_avg(t) nonlinear, Var(t) jump | 0.30 | High – requires reformulation | Replace β₂·(t/τ₂) with stochastic jump process; update CONCEPT.md §2 |
| OP-T1: τ₂ quantification | L_avg(t) unchanged, Var(t) jump, TIF+ | 0.20 | Critical – paradigm shift | Switch to subpopulation model; update CONCEPT.md §1 and §2 |
| OP-T1: τ₂ quantification | No significant change in L_avg or Var | 0.10 | Critical – falsification | Reject β₂ contribution in vitro; revise CONCEPT.md §2; design in vivo test |
| OP-T2: β₂ in vivo | β₂ confirmed within expected range | 0.50 | Low – validates model | Update PARAMETERS.md with refined estimate |
| OP-T2: β₂ in vivo | β₂ significantly lower than expected | 0.30 | Medium – requires reparameterization | Revise β₂ bounds; investigate tissue-specific shielding |
| OP-T2: β₂ in vivo | β₂ significantly higher than expected | 0.15 | High – suggests additional stress factors | Incorporate stress-modulation term into D₂ equation |
| OP-T2: β₂ in vivo | No detectable erosion | 0.05 | Critical – falsification | Reject β₂ contribution in vivo; revise CONCEPT.md §2 |
| OP-T3: Proteostasis coupling failure | Γ₂,₅ outside predicted range | 0.25 | Medium – coupling model revision | Update Γ matrix; revise CONCEPT.md §5 |
| OP-T4: Paradox resolution failure | No consistent pattern across cohorts | 0.20 | High – paradox framework invalid | Revisit CONCEPT.md §7; consider alternative models |

**Decision rule:** If any outcome with Impact ≥ High occurs, initiate Model Update Protocol (see DESIGN.md §5).
