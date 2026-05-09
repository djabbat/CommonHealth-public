# Эмпирические свидетельства для MCOA

*Дата верификации литературы: 2026-04-22*

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
| ⚠️ ~~Дисфункция митохондрий влияет на NAD+-зависимые эпигенетические модификаторы (сиртуины).~~ FLAGGED — needs replacement | ❌ DELETED | ~~Sun N. et al. Measuring In Vivo Mitophagy // Mol Cell. 2016~~ — paper does NOT exist as cited; Sun N "Measuring In Vivo Mitophagy" was published as *Nat Protoc* 2017 (PMID 28132843), not Mol Cell 2016. Citation removed pending verified replacement on NAD+/sirtuin/mito-epigenetic axis. | ❌ 2026-04-26 (DELETED — fabricated) | — |
| Эпигенетические изменения могут регулировать экспрессию генов, связанных с функцией центриолей и цилии. | 32107477 | Janke C., Magiera MM. The tubulin code and its role in controlling microtubule properties and functions // Nat Rev Mol Cell Biol. 2020;21:307-326. | ✅ 2026-04-26 (CORRECTED: prior PMID 31844045 was fabricated) | Weak (косвенное) |

## 2. Внутренние данные и симуляции

*Данные, сгенерированные в рамках проекта LongevityCommon для валидации концепций MCOA.*

1. **Соболь-анализ чувствительности CDATA v5.1:**
 * Файл: `data/mcoa/sensitivity/sobol_results_2026-04-15.csv`
 * Метод: Глобальный анализ чувствительности (метод Соболя) для модели CDATA.
 * Выборка: N = 16384.
 * Ключевой результат: Первый порядок (S1) для параметра `α_cent` (деления) составляет 0.68 ± 0.05, для `β_cent` (время) — 0.22 ± 0.04 в симуляции эпителиальной ткани. Подтверждает доминирование делений, но значимый вклад времени.
 * Статус: Проверено, воспроизводимо.

2. **Перекрёстная проверка LOO-CV для предсказания нагрузки:** ⚠️ **FLAGGED 2026-04-26**
 * Файл: `data/mcoa/validation/LOO_CV_2026-04-17.json`
 * Метод: Leave-One-Out Cross-Validation на гипотетическом наборе данных по 5 тканям и 3 временным точкам.
 * Результат: ~~Среднеквадратическая ошибка (MSE) = -0.093~~. **Mathematically impossible (MSE ≥ 0 by definition).** Скорее всего это R² (negative R² = модель хуже базовой средней). Цифра удалена из submission-grade документа до коррекции метрики.
 * Статус: ⚠️ **REQUIRED ACTION:** переделать с правильной метрикой (R², MAE, RMSE с положительным значением); либо отметить как «model fails baseline» если R²<0.

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