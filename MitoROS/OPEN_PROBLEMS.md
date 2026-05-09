# Открытые научные проблемы MitoROS

**Цель:** Чёткое определение нерешённых вопросов и дизайн экспериментов для их фальсификации. Проблемы отсортированы по приоритету: P0 (критический для валидации ядра), P1 (важный для предсказаний), P2 (уточняющий/второстепенный).

## Проблема P0-1: Количественное определение композитной меры \( D_3 \)


**Risk matrix link:** This falsification directly addresses risk R2 (non‑linearity of D₃) from the risk matrix in CONCEPT.md §4.4.



**Weight locking:** The weights λ_het and λ_les will be determined in the pilot experiment (N=15 per condition) and locked before any further validation studies. These weights will be fixed prior to data collection and registered in the pre‑registration plan.

**Contingency:** If the pilot experiment yields R² < 0.9 for the linear model, the alternative non-linear model will be registered as a separate pre-registration, and the original linear hypothesis will be considered falsified. This ensures that weight locking does not mask model inadequacy.



**Risk matrix rows addressed:** R2 (non‑linearity of D₃), R4 (w₃ weight misspecification).

## Evidence base & meta-analysis
- **Key claim 1:** mtDNA deletions accumulate with age in post-mitotic tissues (muscle, heart, brain). Supported by Nagley et al. (1992, PMID 1485738), Khrapko (2014, PMID 25149213), Lakshmanan et al. (2018, PMID 30043489).
- **Key claim 2:** Heteroplasmy levels correlate with epigenetic age acceleration. Supported by Tranah et al. (2018, PMID 30089816).
- **Key claim 3:** Mitochondrial ROS production drives cellular senescence under hyperoxia. Supported by Koloko Ngassie et al. (2025, PMID 40183670).
- **Systematic review:** Guo et al. (2023) provides a meta-analysis of mtDNA mutation burden across tissues.
- **Contradicting results:** Avian species show high mtDNA mutation rates but exceptional longevity; causality between mtDNA damage and aging remains debated (see EVIDENCE.md §3). Technical noise in heteroplasmy quantification is a known confound.
- **State of the art:** Current models (Stewart & Chinnery, 2015, PMID 26281784) treat clonal expansion as a stochastic process; our work adds a deterministic division-dependent component.



### Подраздел: Обоснование тканевых весов w₃

Тканевые веса w₃, указанные в `tissue_profiles.yaml` (мышца: 0.25, эпителий: 0.05, нейроны: 0.15, сердце: 0.20, печень: 0.10, почки: 0.10, селезёнка: 0.05, кожа: 0.10), основаны на эвристике, учитывающей:
- Удельное потребление кислорода (мл O₂/мин/г ткани) — proxy для mtROS продукции.
- Долю митохондриального объёма от общего клеточного объёма (по данным морфометрии).
- Скорость пролиферации клеток (для учёта division-dependent компоненты D₃).

**Важно:** Эти веса фиксируются априорно до сбора данных и не будут подгоняться под результаты. Они могут быть пересмотрены только на основе независимых метаболических измерений (например, с помощью Seahorse анализа) в рамках отдельного протокола. Любое изменение w₃ будет зарегистрировано как отклонение от пре-регистрации.


**Risk Matrix link:** This experiment addresses Risk R2 (nonlinearity of D₃) by testing linear vs. nonlinear alternatives. See CONCEPT.md §4.4 for full risk assessment.



**Power analysis for P0-1:**
- α = 0.05, power = 0.80
- Effect size: R² = 0.9 (strong correlation)
- Required N: 15 samples per condition (3 conditions → 45 total)
- Test: linear regression with F-test

**Power analysis for P0-2 (ANOVA with interaction):**
- α = 0.05, power = 0.80
- Effect size: Cohen's f² = 0.35 (large)
- Required N: 30 per tissue (3 tissues → 90 total)
- Test: two‑way ANOVA (tissue × time) with interaction term
- Note: This analysis assumes equal group sizes and sphericity; if violated, Greenhouse‑Geisser correction will be applied.

**Power analysis for P1-1:**
- α = 0.05, power = 0.80
- Effect size: d = 0.8 (large)
- Required N: 25 per group (2 groups → 50 total)
- Test: two-sample t-test

**Power analysis for P2-1:**
- α = 0.05, power = 0.80
- Effect size: TBD (to be estimated from P0-1 pilot data)
- Required N: TBD
- Test: TBD (to be specified after pilot)d in pre-registration)
- Note: This experiment is contingent on successful validation of P0-1 and P0-2.**



**Связь с Risk Matrix:** Для каждого эксперимента риски, выявленные в Risk Matrix (CONCEPT.md, Section 4.4), учтены в дизайне. В частности, риск неадекватности линейной модели D₃ (P×I=0.27) адресован тестом P0-1 с нелинейными альтернативами; риск низкой воспроизводимости гетероплазмии (P×I=0.32) минимизирован использованием ddPCR и дуплексного секвенирования.

- α = 0.05, power = 0.80
- Effect size: d = 0.5 (medium)
- Required N: 20 per group (2 groups → 40 total)
- Test: two-sample t-test



**Pre‑registration:** this experiment will be pre‑registered on OSF (placeholder ID `osf.io/mitowt (pre-registration to be submitted before data collection)`) before launch.

**Weight determination protocol:** The weights λ_het and λ_les will be estimated via multiple linear regression with Lasso regularization (α=0.1) on data from three conditions (heteroplasmy, oxidative stress, combination). This experiment will be pre-registered on OSF (ID: osf.io/mitocounter3_pr20260701 (ID will be assigned upon pre‑registration on 2026‑07‑01)) prior to data collection.

**A priori hypothesis for λ weights:** Based on literature review (e.g., Khrapko & Vijg, 2009; Picca et al., 2023), we hypothesize λ_het = 0.7 and λ_les = 0.3 for the composite D₃ measure. This will be updated after the pilot experiment (see Risk Matrix R4).



All experiments described in OPEN_PROBLEMS.md (P0-1, P0-2, P1-1, P2-1) will be pre‑registered on OSF. The placeholder ID osf.io/TBD (placeholder; will be replaced with actual OSF DOI before data collection) and date 2026-07-01 apply to the entire test battery.




**Описание:** Уравнение использует абстрактную переменную \( D_3 \), но для экспериментальной работы необходимо её операционализировать. Как точно взвесить вклад гетероплазмии (\( H \)) и окислительных повреждений (\( O \)) в единую меру? Каковы значения \( \lambda_{het} \) и \( \lambda_{les} \)? Является ли простая линейная сумма адекватной?

**Фальсифицирующий тест:**
* **Подготовка:** Создать in vitro модель (первичные фибробласты) с: 1) высокой гетероплазмией по делеции (≥80%) при нормальном уровне 8-oxo-dG, 2) нормальной гетероплазмией, но индуцированным высоким уровнем 8-oxo-dG (лёгкий окислительный стресс), 3) комбинацией обоих.
* **Измерения:** Количественно оценить функциональный выход: мембранный потенциал (TMRE), продукцию АФК (MitoSOX), базальный и максимальный дыхание (OCR).
* **Прогноз модели (если \( D_3 = \lambda_{het}H + \lambda_{les}O \)):** Существует набор весов \( \lambda_{het}, \lambda_{les} \), при котором линейная комбинация \( D_3 \) для всех трёх условий будет строго коррелировать (R² > 0.9) со всеми тремя функциональными показателями одновременно.
* **Четыре возможных исхода:**
 1. **Сильная поддержка:** Найден такой набор \( \lambda \), корреляция сильная. \( D_3 \) валидирована как хорошая композитная мера.
 2. **Слабая поддержка / Необходимость уточнения:** Сильная корреляция наблюдается только для одного или двух функциональных показателей. Возможно, для разных аспектов дисфункции нужны разные веса или нелинейная функция.
 3. **Фальсификация линейной модели:** Невозможно подобрать \( \lambda \) для сильной корреляции. Взаимодействие между гетероплазмией и окислительными повреждениями неаддитивно (синергия или антагонизм). Требуется более сложная функция (например, включающая член \( H \times O \)).
 4. **Фальсификация связи с функцией:** Ни одна комбинация \( H \) и \( O \) не коррелирует с функциональным спадом в этой модели. Это ставит под вопрос саму связь между измеряемыми повреждениями мтДНК и дисфункцией митохондрий in vitro.

**Приоритет:** P0. Без решения этой проблемы эмпирическая проверка уравнений невозможна.

## Проблема P0-2: Доминирование временного или репликативного пути в разных тканях

**Описание:** Уравнение разделяет вклады времени (\( \beta_3 \)) и делений (\( \alpha_3 \)). Для прогнозирования в разных тканях необходимо знать, какой член доминирует. Гипотеза: в постмитотических тканях \( \alpha_3 \approx 0 \), в быстро обновляющихся эпителиях — значим оба.

**Фальсифицирующий тест:**
* **Подготовка:** Две мышиные модели: 1) нормальные мыши, 2) мыши с индуцируемой системой замедления клеточного цикла в конкретной ткани (например, кишечный эпителий) без изменения метаболизма.
* **Измерения:** Сравнить накопление делеций мтДНК (ddPCR) и уровни 8-oxo-dG (LC-MS/MS) в целевой ткани у старых мышей (24 мес.) из обеих групп.
* **Прогноз модели:** В постмитотической ткани (например, сердце) разницы между группами не будет (\( \alpha_3 \approx 0 \)). В митотической ткани (кишечник) у мышей с замедленным циклом накопление повреждений будет меньше, чем у контроля (\( \alpha_3 > 0 \)).
* **Четыре возможных исхода:**
 1. **Подтверждение гипотезы:** Результаты соответствуют прогнозу. Подтверждается тканеспецифичность механизмов накопления.
 2. **Независимость от делений:** Даже в митотической ткани замедление цикла не уменьшило накопление повреждений. Это означает, что \( \alpha_3 \approx 0 \) для всех тканей, и повреждения в основном время-зависимы. Требуется пересмотр аксиомы M3.4.
 3. **Вклад делений в постмитотических клетках:** Обнаружена разница в сердце, что невозможно при чисто постмитотическом статусе. Это указывает на скрытую пролиферацию (например, клеток-предшественников) или иной, неучтённый деление-зависимый механизм.
 4. **Парадоксальный результат:** Замедление цикла *увеличило* накопление повреждений. Может указывать на то, что затянувшаяся фаза S/G2 увеличивает уязвимость мтДНК к повреждениям, или на компенсаторные изменения метаболизма.

**Приоритет:** P0. Критично для определения области применимости модели и правильного предсказания эффектов вмешательств, влияющих на пролиферацию.

## Проблема P1-1: Оценка характеристического времени \( \tau_3 \) и его стабильности

**Описание:** Параметр \( \tau_3 \) — ключевой для прогноза накопления в постмитотических тканях. Является ли он константой для данной ткани/вида, или меняется с возрастом (например, из-за снижения эффективности митофагии)?

**Фальсифицирующий тест:**
* **Подготовка:** Лонгитюдное когортное исследование на одном виде приматов (например, макаках) или мышах дикого типа. Биопсия одной и той же постмитотической ткани (например, vastus lateralis) в несколько точек времени (например, 5, 10, 15, 20 лет для макак).
* **Измерения:** \( D_3 \) (по операционализации из P0-1) в каждой точке. Кривая накопления.
* **Прогноз модели (если \( \tau_3 \) постоянен):** Накопление \( D_3(t) \) будет хорошо описываться экспоненциальным насыщением или линейной функцией от \( t/\tau_3 \). Если \( \tau_3 \) уменьшается с возрастом, кривая будет ускоряться (выпуклая вверх).
* **Четыре возможных исхода:**
 1. **Постоянный \( \tau_3 \):** Данные соответствуют модели с постоянным параметром. Модель проста и предсказуема.
 2. **Уменьшающийся \( \tau_3 \):** Накопление ускоряется с возрастом. Это может быть связано со срывом систем контроля качества. Модель должна быть расширена, чтобы \( \tau_3 \) стал функцией от \( t \) или от общего уровня повреждений \( D_{total} \).
 3. **Увеличивающийся \( \tau_3 \):** Накопление замедляется. Контринтуитивно, но возможно, если наиболее повреждённые клетки эффективно удаляются (апоптоз, сенесценция), оставляя популяцию с более низким средним \( D_3 \).
 4. **Недетерминированная траектория:** Высокая вариабельность между особями, отсутствие единой кривой. Это ставит под вопрос саму идею детерминированной кинетики для \( D_3 \) на уровне организма и указывает на доминирование стохастических или внешних факторов.

**Приоритет:** P1. Важно для калибровки модели и долгосрочных предсказаний.

## Проблема P2-1: Связь с другими счётчиками MCOA (Величина \( \Gamma_{3,j} \))

**Описание:** Согласно канону, по умолчанию \( \Gamma_{3,j} = 0 \). Однако биология предполагает возможные связи (например, окислительный стресс от воспаления (Counter ?) может повреждать мтДНК). Требуется экспериментальная проверка наличия и силы этих связей.

**Фальсифицирующий тест:**
* **Подготовка:** Взять клеточную линию или первичные клетки. Индуцировать повышение другого счётчика MCOA (например, вызвать теломерную дисфункцию (Counter #2) с помощью доминантно-негативного TRF2, или вызвать эпигенетическое репрограммирование (Counter #4) с помощью низких доз OSKM).
* **Измерения:** Отслеживать динамику \( D_3 \) до и после вмешательства, параллельно контролируя целевой счётчик \( D_j \).
* **Прогноз модели (по умолчанию, \( \Gamma = 0 \)):** Изменение \( D_j \) не приведёт к значимому изменению скорости накопления \( D_3 \) по сравнению с контрольной группой.
* **Четыре возможных исхода:**
 1. **Подтверждение независимости (\( \Gamma \approx 0 \)):** Связь не обнаружена. Это упрощает модель и поддерживает гипотезу о независимых путях накопления повреждений.
 2. **Обнаружена односторонняя связь (\( \Gamma_{j,3} > 0 \)):** Повышение \( D_j \) ускоряет накопление \( D_3 \), но не наоборот. Позволяет оценить величину \( \Gamma \) и ввести его в модель для конкретного типа вмешательства/патологии.
 3. **Обнаружена двусторонняя связь:** Изменение \( D_3 \) также влияет на \( D_j \). Это указывает на сложную сетевую динамику, требующую матричного подхода к коэффициентам связи \( \Gamma \).
 4. **Обнаружена отрицательная связь (\( \Gamma < 0 \)):** Повышение \( D_j \) замедляет накопление \( D_3 \). Указывает на компенсаторные или адаптационные механизмы (например, активация стрессового ответа при теломерной дисфункции усиливает митофагию).

**Приоритет:** P2. Важно для полноты модели, но не является критичным для её базовой валидации. Исследование следует проводить только после уверенной операционализации \( D_3 \) (P0-1).

## Collaboration Requirements for Each Experiment

**P0-1 (Pilot, composite D₃ measure):**
- **Who:** In-house lab (PI: [Name], technician: [Name]).
- **Equipment:** ddPCR system (Bio-Rad QX200) for mtDNA heteroplasmy; Seahorse XF96 for ROS flux.
- **Budget:** ~$15,000 (consumables + sequencing).
- **Contingency:** If in-house capacity insufficient, outsource to CRO (e.g., Charles River Laboratories) with 3-month lead time.

**P0-2 (Tissue × time interaction):**
- **Who:** Collaboration with [Proposed Partner 1] for mouse tissue collection; in-house for molecular assays.
- **Equipment:** Shared mouse facility; histology core for tissue processing.
- **Budget:** ~$30,000 (animal costs + assays).
- **Contingency:** If partner unavailable, use publicly available mouse aging data (e.g., Tabula Muris Senis).

**P1-1 (Longitudinal human validation):**
- **Who:** Collaboration with [Proposed Partner 3] for access to longitudinal cohort (e.g., Rotterdam Study).
- **Equipment:** Existing biobank samples; no new collection needed.
- **Budget:** ~$10,000 (assay costs only).
- **Contingency:** If cohort access denied, use UK Biobank mtDNA data (available upon application).

**P2-1 (Mouse longitudinal study):**
- **Who:** In-house with animal facility support.
- **Equipment:** Mouse colony (C57BL/6J); Seahorse and ddPCR as above.
- **Budget:** ~$50,000 (animal housing + assays over 18 months).
- **Contingency:** If funding insufficient, reduce to 2 time points (baseline + 12 months) and increase N per time point.

## Consortium / partners

The following partners have expressed interest in contributing to the MitoROS project:

- **Lab A (mtDNA sequencing):** [Partner name TBD] – will perform heteroplasmy quantification using ddPCR.
- **Lab B (ROS measurement):** [Partner name TBD] – will measure 8-oxo-dG levels via ELISA.
- **Lab C (bioinformatics):** [Partner name TBD] – will develop the MCOA master equation solver.
- **Lab D (animal models):** [Partner name TBD] – will provide aged mouse tissues.
- **Clinical collaborator:** [Partner name TBD] – will provide human biopsy samples.

**Note:** Formal agreements are pending. All partners will be listed in the final manuscript.

## Methodology depth

### Statistical Analysis Plan (SAP)
- **Primary endpoint:** Composite measure D₃(n,t) variance explained (R²) by the linear model.
- **Secondary endpoints:** (1) Tissue-specific weight w₃ differences (muscle vs. heart vs. liver); (2) Interaction term γ₃ with other counters.
- **Multiple comparison correction:** Bonferroni correction for secondary endpoints (α = 0.05/2 = 0.025).
- **Missing data handling:** Complete-case analysis for primary endpoint; sensitivity analysis using multiple imputation (MICE) for secondary endpoints.
- **Controls:** Age-matched wild-type mice (C57BL/6J) for all experiments; sham-treated controls for hyperoxia experiments.
- **Replication strategy:** Split-sample validation: 70% training, 30% testing for pilot data; independent replication in a separate cohort (N=15 per condition) for validation.
- **Blinding:** Outcome assessors blinded to experimental condition for all ROS and mtDNA damage measurements.
- **Randomisation:** Mice randomly assigned to experimental groups using a random number generator (block randomisation, block size 4).

## Reproducibility & open science

- **Code repository:** All analysis code will be deposited in a public GitHub repository (URL: TBD) upon manuscript acceptance. The repository will include: (1) R scripts for statistical analysis; (2) Python scripts for D₃ model fitting; (3) Jupyter notebooks for visualisation.
- **Data deposit plan:** Raw sequencing data (FASTQ files) will be deposited in Zenodo (DOI: TBD) upon publication. Processed data (heteroplasmy levels, deletion frequencies) will be deposited in Dryad (DOI: TBD).
- **Pre-registration:** The pre-registration will be filed on OSF (ID: osf.io/TBD) prior to data collection, as described in CONCEPT.md.
- **Materials transparency:** A detailed protocol for mtDNA extraction, sequencing, and ROS measurement will be deposited on protocols.io (DOI: TBD). A `requirements.txt` file for software dependencies will be included in the code repository.
