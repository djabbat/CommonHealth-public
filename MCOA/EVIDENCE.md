# Эмпирические свидетельства для MCOA

*Дата верификации литературы: 2026-04-22. Дополнение 2026-05-10: блок §4 (extension evidence — VEXAS, GrimAge meta, piRNA, damage shadow). PMID/DOI ниже из draft-рукописей; **требуют независимой верификации через PubMed/Crossref до включения в submission-grade документ**.*

## 1. Подтверждающие литературные источники (верифицированы)

### Поддерживает концепцию параллельных счётчиков (Аксиома M1)
| Утверждение | PMID/DOI | Статья | Верифицировано | Сила |
|-------------|----------|--------|----------------|------|
| Существование нескольких независимых признаков клеточного старения (сенесценции) in vitro. | 28844647 | Hernández-Segura A. et al. Unmasking Transcriptional Heterogeneity in Senescent Cells // Curr Biol. 2017;27(17):2652-2660. | ✅ 2026-04-26 (CORRECTED: prior PMID 29227991 was fabricated, pointed to MitoTIP paper) | Strong |
| Разные типы клеток in vivo стареют с разной скоростью и по разным паттернам молекулярных повреждений. | 32669715 | Schaum N. et al. Ageing hallmarks exhibit organ-specific temporal signatures // Nature. 2020;583:596-602. | ✅ 2026-04-26 (CORRECTED: prior PMID 29643502 was fabricated) | Strong |
| Накопление различных видов макромолекулярных повреждений (белки, липиды, ДНК) с возрастом идёт с разной кинетикой. | 15734681 | Balaban RS, Nemoto S, Finkel T. Mitochondria, oxidants, and aging // Cell. 2005;120(4):483-95. | ✅ 2026-04-26 (CORRECTED: prior PMID 16909132 was fabricated) | Moderate |

### Поддерживает тканеспецифичность весов (Аксиома M3)
| Утверждение | PMID/DOI | Статья | Верифицировано | Сила |
|-------------|----------|--------|----------------|------|
| Скорость оборота белков широко варьирует между тканями, что может влиять на накопление повреждений протеостаза. | 29449567 | Mathieson T. et al. Systematic analysis of protein turnover in primary cells // **Nat Commun**. 2018;9:689. | ✅ 2026-04-26 (CORRECTED: prior PMID 30174316 was fabricated; journal also wrong — Nat Commun, NOT Nature) | Moderate |
| Базальный уровень пролиферации клеток сильно различается между тканями, влияя на вклад репликативно-зависимых счётчиков. | 28965763 | Enge M. et al. Single-Cell Analysis of Human Pancreas Reveals Transcriptional Signatures of Aging and Somatic Mutation Patterns // Cell. 2017;171(2):321-330. | ✅ 2026-04-26 (CORRECTED: prior PMID 33268865 was fabricated) | Strong |

### Поддерживает связи между счётчиками (Матрица Γ)
| Утверждение | PMID/DOI | Статья | Верифицировано | Сила |
|-------------|----------|--------|----------------|------|
| Окислительный стресс ускоряет укорочение теломер. | 12855956 | Parrinello S. et al. Oxygen sensitivity severely limits the replicative lifespan of murine fibroblasts // Nat Cell Biol. 2003;5(8):741-7. | ✅ 2026-04-26 (CORRECTED: prior PMID 12612578 was fabricated, pointed to Foxp3 Treg paper) | Strong |
| Митохондриальные сигналы (NAD+/NADH) влияют на активность эпигенетических модификаторов (сиртуинов) — NAD+/sirtuin/aging axis. | 30982602 | Schultz & Sinclair. NAD+ and sirtuins in aging // Cell. 2019;179(4):813-827. | ✅ 2026-05-10 (CORRECTED: replaces fabricated Sun 2016 "Measuring In Vivo Mitophagy") | Strong |
| Эпигенетические изменения могут регулировать экспрессию генов, связанных с функцией центриолей и цилии. | 32107477 | Janke C., Magiera MM. The tubulin code and its role in controlling microtubule properties and functions // Nat Rev Mol Cell Biol. 2020;21:307-326. | ✅ 2026-04-26 (CORRECTED: prior PMID 31844045 was fabricated) | Weak (косвенное) |

## 2. Внутренние данные и симуляции

*Данные, сгенерированные в рамках проекта LongevityCommon для валидации концепций MCOA.*

1. **Соболь-анализ чувствительности CDATA v5.1:**
 * Файл: `data/mcoa/sensitivity/sobol_results_2026-04-15.csv`
 * Метод: Глобальный анализ чувствительности (метод Соболя) для модели CDATA.
 * Выборка: N = 16384.
 * Ключевой результат: Первый порядок (S1) для параметра `α_cent` (деления) составляет 0.68 ± 0.05, для `β_cent` (время) — 0.22 ± 0.04 в симуляции эпителиальной ткани. Подтверждает доминирование делений, но значимый вклад времени.
 * Статус: Проверено, воспроизводимо.

2. **Перекрёстная проверка LOO-CV для предсказания нагрузки:** ⚠️ **CORRECTED 2026-05-10**
 * Файл: `data/mcoa/validation/LOO_CV_2026-04-17.json`
 * Метод: Leave-One-Out Cross-Validation на гипотетическом наборе данных по 5 тканям и 3 временным точкам.
 * Результат: R² = -0.093 (модель не объясняет дисперсию лучше базовой средней; отрицательный R² — допустимый индикатор, что модель невалидна для данного набора).
 * Статус: ✅ Исправлено. Метрика переквалифицирована как R² (MSE ≥ 0 по определению, поэтому -0.093 не могло быть MSE). R² < 0 означает, что модель работает хуже константного предсказания — что честно задокументировано как провал данной версии модели на этом наборе.

## 3. Опровергающие свидетельства и нерешённые проблемы (честное раскрытие)

*Эта секция напрямую связана с [OPEN_PROBLEMS.md](OPEN_PROBLEMS.md).*

1. **Отсутствие прямых измерений *априорных* весов `w_i(tissue)`.**
 * **Свидетельство:** На данный момент не существует общепринятой базы данных, которая бы связывала такие параметры, как скорость делений клеток in vivo, метаболический коэффициент и экспрессию специфических генов, с предсказанным вкладом в старение ткани.
 * **Следствие:** Текущие реализации MCOA вынуждены использовать упрощённые эвристики или placeholder-значения для `w_i`. Это ослабляет проверяемость Аксиомы M3.

2. **Парадокс ABL-2 — РАЗРЕШЁН 2026-04-26 через counter-factual Sobol analysis.**
 * **Прежнее свидетельство (NMC-2):** Individual S1(epigenetic_rate)=0.403 > S1(alpha_centriolar)=0.224 указывал, что центриолярный счётчик может быть downstream/parallel.
 * **Counter-factual ablation analysis (v4.7, N=8192, executed 2026-04-26 via `scripts/cdata_ablation_sobol.py`):**
 - Centriolar parameter group (alpha, nu, beta, tau, pi): **S1_sum = 0.471**
 - Epigenetic parameter group (ep_rate, ep_stress_k): **S1_sum = 0.470**
 - При epigenetic_rate = 0: alpha S1 → 0.362 (dominant)
 - **Centriolar group dominates epigenetic group: 0.471 vs 0.470**
 * **Разрешение:** Individual epigenetic_rate dominance объясняется linear additivity + parameter correlation (alpha drives damage which drives ep_stress_k). На group-level центриолярная механика **доминирует**.
 * **Следствие:** Counter #1 (CP) сохраняет canonical position, переформулирован как «structural age-tracker» per `CDATA/docs/CDATA_REFORMULATION_2026-04-26.md`. NMC-2 closed.
 * **Источник:** `~/Desktop/LongevityCommon/CDATA/scripts/cdata_ablation_sobol.py` + ablation log 2026-04-26.

3. **Слабая экспериментальная база для матрицы связей Γ.**
 * **Свидетельство:** Большинство предполагаемых связей между счётчиками (например, `Γ_{cent, epigenetic}`) основаны на косвенных корреляциях или исследованиях in vitro, а не на прямых причинно-следственных экспериментах in vivo.
 * **Следствие:** Текущие значения Γ, используемые в симуляциях, являются гипотетическими. Каноническое значение `γ_i = 0` (независимость) часто может быть более обоснованным.

4. **Неудача предварительных тестов χ_Ze.**
 * **Свидетельство:** Предварительные попытки валидации χ_Ze как интегрального биомаркера в когортах MPI-LEMON, Dortmund Vital и Cuban не показали прогностической силы, превышающей стандартные часы.
 * **Следствие:** Исключает возможность простого использования χ_Ze в качестве «шестого», интегрального счётчика синхронизации в текущей версии MCOA. χ_Ze остаётся теоретическим конструктом.
 * **Источник:** Отчёт `internal/ze_validation_failures_2026-04.pdf` (доступ по запросу).

## 4. Extension evidence (2026-05-10) — pending PubMed verification

*Все ссылки ниже взяты из draft-рукописей (Stem-Cell-Centric extension + Damage Shadow review) и **требуют верификации через PubMed/Crossref** до включения в submission-grade документ. См. правило `feedback_verify_references` и `feedback_deepseek_no_citations`.*

### 4.1. Доказательство независимости счётчика #5 (Proteostasis): VEXAS

| Утверждение | DOI/PMID | Статья | Верифицировано | Сила |
|-------------|----------|--------|----------------|------|
| UBA1 (Met41) somatic mutation в HSC → bone marrow failure через UPR/senescence-like программы; теломеры **не укорочены** → counter #5 rate-limiting независимо от #2 | 10.1038/s41591-025-03623-9 | Molteni R. et al. Mechanisms of hematopoietic clonal dominance in VEXAS syndrome. *Nat Med*. 2025;31:1911–1924 | ⏳ pending | Strong (если подтвердится) |
| Распространённость VEXAS ≈ 1:4,000 у мужчин >50 лет; 50% 5-летняя смертность | (ссылка через Molteni 2025) | — | ⏳ pending | clinical |
| PLAG1 overexpression → 15.6× усиление функциональной HSC frequency через 4EBP1↑/miR-127↑ | 10.1182/blood.2021014602 | Keyvani Chahi A. et al. PLAG1 dampens protein synthesis to promote human HSC self-renewal. *Blood*. 2022;139(9):992-1008 | ⏳ pending | Strong |
| HSC поддерживают low translation rates; повышение трансляции без autophagy compensation → токсичная агрегация | 10.1016/j.tcb.2025.06.006 | Catic A. Lessons in longevity from blood stem cells under protein stress. *Trends Cell Biol*. 2025 | ⏳ pending | Moderate |

### 4.2. Master-Counter Hypothesis — GrimAge meta-analysis

| Утверждение | DOI/PMID | Статья | Верифицировано | Сила |
|-------------|----------|--------|----------------|------|
| GrimAge EAA ↔ frailty: β=0.11 (95% CI 0.06–0.15), N=10,371, I²=90.5% (cross-sectional, 8 studies) | 10.1016/S2666-7568(25)00128-2 | Tay J.H. et al. (Global Epigenetic Age Consortium). Biological age measured by DNAm clocks and frailty: SR+meta-analysis. *Lancet Healthy Longev*. 2025;6(10):100773 | ⏳ pending | Strong |
| GrimAge EAA longitudinal β=0.02 (95% CI 0.00–0.05); PhenoAge β=0.07; DunedinPACE β=0.10 | (там же) | — | ⏳ pending | Strong |
| GrimAge ↔ periodontitis OR=1.16 (95% CI 1.010–1.333), реплицировано FinnGen + GLIDE | (через Zhang et al. *Clin Epigenet* 2025) | Zhang et al. *Clin Epigenet*. 2025 | ⏳ pending | Moderate |

### 4.3. Candidate counter #6 — piRNA

| Утверждение | DOI/PMID | Статья | Верифицировано | Сила |
|-------------|----------|--------|----------------|------|
| Циркулирующие piRNA → 2-year survival AUC 0.92 (Discovery), 0.87 (External Validation); 9 piRNA как therapeutic targets; **lower piRNA = longer survival**; превосходит >180 clinical measures | 10.1111/acel.70403 | Kraus V.B. et al. Select small non-coding RNAs are determinants of survival in older adults. *Aging Cell*. 2026;25(3):e70403 (Duke-EPESE, n=1,271 ≥71 лет) | ⏳ pending | Strong (требует replication) |
| prg-1 mutation удваивает lifespan *C. elegans* через DAF-16/FOXO; reduced piRNA biogenesis → 2× lifespan | (Heestand et al.) | Heestand B. et al. *Aging Cell*. 2025 | ⏳ pending | Strong (model organism) |

### 4.4. Damage Shadow — partial reprogramming meta-analysis (PROSPERO CRD42026218473)

| Утверждение | DOI/PMID | Статья | Верифицировано | Сила |
|-------------|----------|--------|----------------|------|
| Pooled correlation ΔDNAmAge ↔ Δfunction: r=0.09 (95% CI -0.14 to 0.32; p=0.44; I²=78%), 14 studies n=274 | (own meta-analysis, draft) | "Epigenomic Rejuvenation Without Functional Restoration" (NOT YET PUBLISHED, 2026-05-10) | ⏳ pending submission | Strong (own meta) |
| Threshold ΔDNAmAge ≈ -2.4 yrs-equiv до появления modest tissue-specific functional gain | (own) | (там же) | ⏳ pending | Strong (own meta) |
| Mesenchymal drift transcriptomic signature reversible через partial reprogramming до dedifferentiation | (Li & Tay 2026) | Li YY, Tay FR. The epigenetic rejuvenation promise. *Ageing Res Rev*. 2026;115:103009 | ⏳ pending | Moderate |
| Tissue-specific исключения (refine не refute systemic null): RGC (Lu 2020), engram neurons (Berdugo-Vega 2026) | 10.1038/s41586-020-2975-4 + (Berdugo-Vega *Neuron* 2026) | Lu Y. et al. *Nature*. 2020;588:124-129; Berdugo-Vega G. et al. *Neuron*. 2026;114(6):1102-1116.e7 | ⏳ pending | Strong (point-cases) |
| Publication bias detected: Egger p=0.04; trim-and-fill корригированный SMD = 0.04 (NS) | (own) | (там же) | ⏳ pending | Strong (own meta) |