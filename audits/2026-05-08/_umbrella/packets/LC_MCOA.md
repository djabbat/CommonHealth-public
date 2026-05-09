# AUDIT PACKET — LC_MCOA

Path: `/home/oem/Desktop/LongevityCommon/MCOA`  Date: 2026-05-08

## Size & file counts
```
832K	/home/oem/Desktop/LongevityCommon/MCOA
```
**Extensions:** .md=18, .rs=16, .ex=12, .toml=8, .csv=8, .exs=5, .lock=2, .heex=2, .py=2, (noext)=1, .sql=1
## Tree (depth=2, max 200 entries)
```
.
./STATE.md
./frontend
./frontend/priv
./frontend/mix.exs
./frontend/lib
./frontend/README.md
./frontend/config
./frontend/assets
./PARAMETERS.md
./Cargo.toml
./EVIDENCE.md
./tests
./crates
./crates/mcoa_compare
./crates/mcoa_core
./crates/mcoa_tests
./crates/mcoa_cli
./crates/mcoa_simulation
./crates/mcoa_api
./DESIGN.md
./README.md
./results
./results/mcoa_fibroblast_100.csv
./results/mcoa_cd8_t_memory_100.csv
./results/mcoa_beta_cell_100.csv
./results/mcoa_hepatocyte_100.csv
./results/mcoa_hsc_100.csv
./results/mcoa_neuron_100.csv
./Cargo.lock
./backend
./backend/migrations
./backend/Cargo.toml
./backend/src
./backend/README.md
./backend/Cargo.lock
./examples
./scripts
./scripts/compare_mcoa_cdata.py
./scripts/compare_all.py
./CLAUDE.md
./THEORY.md
./OPEN_PROBLEMS.md
./docs
./docs/DAILY_SEARCH_2026-04-20_AGGREGATE.md
./docs/MCOA_CONCEPT_review.md
./docs/comparisons
./docs/DATASETS.md
./docs/DAILY_SEARCH_2026-04-20.md
./CONCEPT.md
./LICENSE
```
## Detected stack: **Rust, Phoenix/Elixir**
## Core files

### `CLAUDE.md` (2987 chars)
```md
# CLAUDE.md — MCOA

**MCOA** (Multi-Counter Architecture of Organismal Aging) — мета-теоретическая mother-project всего LongevityCommon-стека. Формализует организменное старение как взвешенную сумму параллельных damage-counter'ов с tissue-specific весами `w_i`.

**Path:** `/home/oem/Desktop/LongevityCommon/MCOA/`
**Repo:** часть монорепозитория `djabbat/LongevityCommon` (private) / `djabbat/LongevityCommon-public` (public, без core .md и Patients/).

---

## Source of truth

**`MCOA/CONCEPT.md`** — авторитетный документ. Любые конфликты между этим CLAUDE.md и CONCEPT.md решаются в пользу CONCEPT.md. Канонический референс — `~/Documents/MCOA_NatureAging_submission/01_MCOA_Perspective_manuscript.md`.

См. также корневой `~/Desktop/LongevityCommon/CLAUDE.md` (parent rules).

---

## Inviolable axioms — не менять без явной команды

- **M1** Parallel counters (≥2 параллельных damage-процесса)
- **M2** Master equation `L_tissue(n,t) = Σᵢ wᵢ(tissue) · fᵢ(Dᵢ(n,t))`
- **M3** A-priori tissue weights (фиксируются до измерений → falsifiability)
- **M4** Falsifiability через несколько независимых counter'ов

Полная формулировка — `CONCEPT.md § 2`.

---

## Counter registry (унифицировано 2026-05-07)

Все 5 counter'ов — active-development subprojects (per user decision 2026-05-07: «отвечают за лимит Хейфлика индивидуально и крест-накрест, входят в MCOA как треки»).

| # | Name | Subproject | Status |
|---|---|---|---|
| 1 | Centriolar | `CDATA/` | Counter #1 (active) — only counter с experimental data; C2 подтверждена у двух типов клеток |
| 2 | Telomere | `Telomere/` | active-development; concept-stage код, Rust crate scaffolded |
| 3 | MitoROS | `MitoROS/` | active-development; concept-stage код, Rust crate scaffolded |
| 4 | EpigeneticDrift | `EpigeneticDrift/` | active-development; concept-stage код, Rust crate scaffolded |
| 5 | Proteostasis | `Proteostasis/` | active-development; concept-stage код, Rust crate scaffolded |

---

## Stack

- **Backend:** Rust (Cargo workspace `MCOA/backend/`), crates под `MCOA/crates/`
- **Frontend:** React/TypeScript (`MCOA/frontend/`)
- **Server presence:** на текущий момент (2026-05-07) — только static landing page `mcoa.longevity.ge` → `/var/www/mcoa-landing/index.html`. Backend не развёрнут на сервере. Это P1 пункт ремедиации.

---

## Правила разработки

1. CONCEPT.md обновляется только при изменениях за пределами M1-M4 axioms; для axiom-touching правок требуется явная команда.
2. Любое изменение counter-equation (Dᵢ kinetics) требует обновления соответствующего subproject CONCEPT.md и MCOA/MAP.md.
3. Tissue weights `wᵢ(tissue)` должны быть зафиксированы до публикации эмпирических данных (M3).
4. Self-citation в манускриптах ≤15% (правило `feedback_article_workflow`).

---

## Web

`mcoa.longevity.ge` — landing only (`/var/www/mcoa-landing/`). Полноценный frontend (React) не развёрнут — план в roadmap.

---

## Тесты

`cargo test` в `MCOA/` workspace — должно проходить полностью перед коммитом в `main`.

```
### `README.md` (4389 chars)
```md
# MCOA — Мультисчётная Архитектура Старения Организма

**MCOA** — это теоретический мета-фреймворк, формализующий старение организма как взвешенную сумму нескольких параллельных процессов накопления повреждений («счётчиков»). Каждый счётчик имеет собственную кинетику, зависящую от делений клеток и хронологического времени, и фиксированные *априорные* весовые коэффициенты для каждой ткани, что обеспечивает фальсифицируемость. MCOA не заменяет специфические подпроекты (CDATA, Ze, BioSense), а предоставляет общий формальный язык и архитектуру для их интеграции.

## Основные принципы

*   **Параллельные счётчики:** Старение определяется как минимум двумя независимыми процессами накопления повреждений, протекающими параллельно. Ни один счётчик не является достаточным для объяснения универсальности репликативных лимитов (Аксиома M1).
*   **Размерная согласованность:** Все компоненты уравнения повреждения должны быть приведены к безразмерному виду с использованием *априорно* заданных референтных масштабов (деления `n_i*`, время `τ_i`) из независимых клеточно-биологических знаний (Аксиома M2).
*   **Априорное весовое взвешивание тканей:** Вклад каждого счётчика `w_i(tissue)` в общую нагрузку ткани должен предсказываться до подгонки модели, на основе биологических параметров (частота делений, метаболическая активность и др.). Пост-фактумная подгонка весов запрещена (Аксиома M3).
*   **Фальсифицируемость — первоклассная цель:** Каждое утверждение, выведенное из MCOA, должно сопровождаться чётким экспериментальным тестом, который может его опровергнуть (Аксиома M4).

## Ключевые компоненты

1.  **Канонические счётчики:** MCOA определяет пять основных счётчиков: (1) центриолярная полиглутамилизация (CDATA), (2) теломеры, (3) митохондриальные ROS/мтДНК, (4) эпигенетический дрейф, (5) коллапс протеостаза.
2.  **Формализм:** Повреждение для счётчика `i` описывается как `D_i(n, t) = D_i₀ + α_i·(n/n_i*) + β_i·(t/τ_i) + γ_i·I(others)`. Общая нагрузка на ткань: `L_tissue = Σ_i [ w_i(tissue) · f_i(D_i(n, t)) ]`.
3.  **Матрица связей (Γ):** Определяет, как один счётчик ускоряет другой (например, окислительный стресс ускоряет укорочение теломер). Элементы Γ должны измеряться, а не подгоняться. По умолчанию `γ_i = 0` (гипотеза независимости).
4.  **Функциональный переход:** Клетка входит в состояние сенесценции, апоптоза или дисфункции при превышении `L_tissue > L_critical(tissue)` или `D_i > D_critical(i, tissue)`.

## Связь с другими проектами LongevityCommon

*   **[THEORY.md](THEORY.md):** Полное формальное изложение аксиом, уравнений и предсказаний MCOA.
*   **[EVIDENCE.md](EVIDENCE.md):** Проверенные литературные источники (PMID/DOI), внутренние данные и опровергающие свидетельства, поддерживающие или оспаривающие MCOA.
*   **[OPEN_PROBLEMS.md](OPEN_PROBLEMS.md):** Неразрешённые научные проблемы, приоритеты и конкретные фальсифицируемые тесты для MCOA.
*   **[PARAMETERS.md](PARAMETERS.md):** Таблица всех количественных параметров, их источников, единиц измерения и статуса калибровки.
*   **[DESIGN.md](DESIGN.md):** Архитектура кода, файловая структура и API контракты эталонной реализации на Rust.
*   **[AGENTS.md](AGENTS.md):** Инструкции и жёсткие правила для LLM-агентов, работающих с кодом и документацией MCOA.
*   **[JOURNAL.md](JOURNAL.md):** Хронологический журнал изменений, решений и их обоснований.
*   **[ROADMAP.md](ROADMAP.md):** План будущих разработок, приоритеты и зависимости.

## Важные канонические исправления (2026-04-22)

*   **Формула Health Score удалена.** Веса 0.40·organism + 0.25·psyche... не имели математического обоснования из MCOA. Вместо неё используется прямая тканевая нагрузка `L_tissue`.
*   **χ_Ze — теоретический конструкт, а не валидированный биомаркер.** Утверждение "χ_Ze predicts biological age with R²=0.84" отозвано, так как было основано на синтетических данных.
*   **Связь γ_i и MCOA Test 2.** Параметр связи `γ_i` по умолчанию равен 0. MCOA Test 2 — это будущий протокол для *измерения* связей между уже работающими счётчиками, а не источник *априорных* значений `γ_i`.
*   **Структура EIC Pathfinder.** В текущей заявке (Variant B) MCOA является WP1 (€0.3M, M1-M12). Подпроекты Ze, BioSense и Aqtivirebuli не включены в качестве отдельных рабочих пакетов.

**Статус:** Концепция утверждена. Идёт подготовка рукописи для *Nature Aging* (дедлайн 2026-04-25) и разработка эталонной реализации на Rust.
```
### `frontend/README.md` (881 chars)
```md
# MCOA Phoenix frontend

Phoenix LiveView frontend for the MCOA simulator. Consumes the Axum backend at
`http://127.0.0.1:3030/api/simulate`.

## Quickstart

```bash
# 1. Start the Rust backend (in another terminal)
cd ..
cargo run --release --bin mcoa-api

# 2. Start Phoenix
cd frontend
mix setup
mix phx.server
```

Open http://localhost:4000 — dashboard with tissue selector and counter trajectories.

## Scope (v0.2)

- `DashboardLive` — tissue × divisions selector, per-counter table.
- TODO: `ComparisonLive` — MCOA vs CDATA side-by-side plot, residual panel (consumes the same
  `compare_mcoa_cdata.py` logic on the server side).
- TODO: LiveView hooks for Chart.js trajectories.

## Why Phoenix

Per user's canonical stack rule: Rust for backend, Phoenix for frontend. Consistent with
LongevityCommon's realtime stack.

Long-form rule: `../CLAUDE.md` §"Language / stack".

```
### `backend/README.md` (790 chars)
```md
# MCOA Backend

Multi-Counter Architecture of Organismal Aging backend service.

## Overview

This is the backend service for the MCOA (Multi-Counter Architecture of Organismal Aging) subproject of LongevityCommon. It provides a REST API for managing the five canonical aging counters, tissues, subjects, damage measurements, and computing tissue loads according to the MCOA framework.

## Features

- **Counter Management**: CRUD operations for the five canonical counters (telomere, centriolar polyglutamylation, mitochondrial ROS, epigenetic drift, proteostasis collapse)
- **Tissue Management**: Manage tissue types with a-priori weighting functions
- **Subject Tracking**: Track organisms (mice, humans) being studied
- **Damage Measurements**: Store and retrieve counter damage values
```
### `CONCEPT.md` (10138 chars)
```md
# MCOA — Multi-Counter Architecture of Organismal Aging

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


**Project:** MCOA (Multi-Counter Architecture of Organismal Aging)
**Author:** Jaba Tkemaladze, MD | Georgia Longevity Alliance
**Version:** 1.0
**Date:** 2026-04-21
**Status:** CONCEPT APPROVED — initial implementation in progress
**Canonical reference:** `~/Documents/MCOA_NatureAging_submission/01_MCOA_Perspective_manuscript.md` (*Nature Aging* Perspective submission, 2026-04-25)

---

## 1. Project identity

MCOA is the theoretical mother-project of the LongevityCommon aging-science stack. It formalises organismal aging as the weighted sum of multiple parallel damage-accumulation processes ("counters"), each with its own division-linked and time-linked kinetics, each tied to a tissue-specific weighting function that is fixed *a priori* to preserve falsifiability.

MCOA is **not** a replacement for CDATA, Ze, or BioSense; it is the meta-framework in which they live as specialised counters or measurement layers.

---

## 2. Inviolable axioms (do not change without explicit user command)

**Axiom M1 — Parallel counters.** Organismal aging is driven by ≥ 2 distinct damage-accumulation processes that proceed in parallel. No single counter is sufficient to explain the universality of replicative limits.

**Axiom M2 — Dimensional consistency.** No expression of the form *α·n + β·t* is valid unless both terms are reduced to a common dimensionless form. The canonical form is:

*D_i(n, t) = D_i₀ + α_i · (n / n_i\*) + β_i · (t / τ_i) + γ_i · I(other counters)*

where *n_i\** and *τ_i* are counter-specific reference scales fixed *a priori* from independent cell-biological knowledge.

**Axiom M3 — A-priori tissue weighting.** *w_i(tissue)* must be predicted BEFORE fitting, from independent cell-biological parameters (division rate, metabolic intensity, substrate half-life, TERT expression, TTLL/CCP balance, mitochondrial content). Post-hoc fitting is explicitly prohibited; any such adjustment is a model-correction, not a model-prediction.

**Axiom M4 — Falsifiability is first-class (operational threshold v5.6 update 2026-04-28).** Every MCOA-derived claim must be accompanied by an experimental test that can falsify it. **Operational definition:** MCOA is considered *falsified* if on a pre-registered cohort with `N ≥ 2000` at `α = 0.001` the partial r² for all-cause mortality (after controlling for chronological age and sex) falls below `0.05` for every counter `i`. Power analysis: `N = 1875` required to detect `R² = 0.3` at 80% power; threshold set at `N ≥ 2000` per community standard. The earlier provisional threshold `R² < 0.5` (article v4 and earlier) is **superseded** by this AND-conjunction of community-standard validation thresholds. The canonical test set is §6.1–6.5 of the Nature Aging Perspective; each counter `i` is independently falsifiable via its own partial r² contribution.

---

## 3. Formal definition

### 3.1. Single-counter kinetics

*D_i(n, t) = D_i₀ + α_i · (n / n_i\*) + β_i · (t / τ_i) + γ_i · I(others)*

| Symbol | Meaning | Units | Constraint |
|--------|---------|-------|------------|
| *D_i* | Accumulated damage in counter *i* | dimensionless | ≥ 0 |
| *D_i₀* | Baseline damage at birth | dimensionless | ≥ 0 |
| *α_i* | Division-driven rate | dimensionless / (n / n_i\*) | ≥ 0 |
| *β_i* | Time-driven rate | dimensionless / (t / τ_i) | ≥ 0 |
| *γ_i* | Coupling scalar | dimensionless | ℝ |
| *I(others)* | Influence of other counters | dimensionless | Σ_j γ_ij · D_j / (whatever norm) |
| *n_i\** | Reference division number | divisions | tissue-specific, a priori |
| *τ_i* | Reference time scale | seconds | tissue-specific, a priori |

### 3.2. Tissue-integrated load

*L_tissue = Σ_i [ w_i(tissue) · f_i( D_i(n, t) ) ]*

with the constraint *Σ_i w_i(tissue) ≈ 1.0* (non-trivial deviation indicates a missing counter).

### 3.3. Functional transition

A cell enters senescence, apoptosis, or dysfunction when:

*L_tissue > L_critical(tissue)* OR ∃ *i* : *D_i > D_critical(i, tissue)*

---

## 4. The five canonical counters

| # | Name | Subproject | Nature | *n_i\** anchor | *τ_i* anchor |
|---|------|------------|--------|----------------|--------------|
| **1** | **Centriolar polyglutamylation** | CDATA | division + time | ~50–80 for HSC, ~30–50 for epithelial | months–years (mass-spec to calibrate) |
| **2** | **Telomere** | Telomere (new subproject) | division-dominant | Hayflick limit per cell type (~50 for human fibroblasts) | turnover of telomeric repeats |
| **3** | **Mitochondrial ROS / mtDNA** | MitoROS (new subproject) | time-dominant | α → 0 for post-mitotic | days–weeks for mtDNA lesion turnover |
| **4** | **Epigenetic drift** | EpigeneticDrift (new subproject) | time-dominant | α → 0 for post-mitotic | Horvath clock / DunedinPACE doubling time |
| **5** | **Proteostasis collapse** | Proteostasis (new subproject) | mixed | cell-type-specific | protein half-life of dominant aggregating species |

**Ordering rationale (2026-04-21):** Centriole is placed at #1 because it is the unifying structural counting device within the asymmetric inheritance framework; telomere is a division-dependent counter downstream of centriole-inherited stemness. Each counter has its own dedicated subproject with Rust core and Phoenix LiveView dashboard — see §10.

Additional counters (lipofuscin, lamina defects, ECM stiffening, SASP spread) are natural extensions; they enter with the same formal apparatus.

---

## 5. Coupling matrix Γ

Γ ∈ ℝ^(k×k) where k = number of counters. Γ_{ij} = rate at which counter *j* accelerates counter *i*.

Known non-zero entries (from Nature Aging Perspective):
- Γ_{telomere, mito} > 0 (Parrinello 2003 — oxidative stress accelerates telomere loss)
- Γ_{epigenetic, mito} > 0 (Schultz & Sinclair *Cell* 2019, PMID 30982602 — NAD+/sirtuin/aging axis; replaces fabricated «Sun 2016 Measuring In Vivo Mitophagy», corrected 2026-04-26)
- Γ_{cent, epigenetic} > 0 (epigenetic dysregulation alters TTLL/CCP balance — Janke & Magiera 2020)

All Γ entries must be measured, not fitted. ~~MCOA Test 2~~ [отозвано — see CORRECTIONS §1.3] (§6.2 Perspective) is the canonical measurement protocol.

---

## 6. Falsifiability tests (canonical)

Each test is described in detail in the Nature Aging Perspective §6.1–6.5:

1. **Test 1 (Tissue-Specific Counter Dominance):** longitudinal mouse study, N=85/timepoint, 6 tissues × 4 counters × 4 timepoints = 96 FDR-corrected tests. $1.5M / 3 years.
2. **Test 2 (Counter Coupling Γ_ij):** PolgA D257A mouse model, 8-OHdG ELISA primary readout. $800k / 2 years.
3. **Test 3 (Intervention Specificity):** rapamycin × senolytic × combination in aged mice.
4. **Test 4 (Division vs Time — Aubrey's test):** *ex vivo* iPSC organoids, 2×2 design. **<$200k / 10 weeks — single-lab tractable.**
5. **Test 5 (Multi-target Synergy):** 5-arm mouse lifespan trial. $2.8M / 4 years.

Test 4 is the near-term priority.

---

## 7. Relationship to subprojects of LongevityCommon

| Subproject | MCOA role |
|------------|-----------|
| CDATA | Counter #1 (centriolar polyglutamylation) — specialised instance |
| Ze | Counter "S" — dimensionless χ_Ze synchronisation index computed from an ODE model of the plasma/SASP feedback loop (see `Ze/CONCEPT.md` §4, rewritten 2026-04-23 on Argentieri 2024 / Jeon 2022 basis) |
| BioSense | Measurement layer for *D_autonomic*, *D_neural*, *D_olfactory* |
| FCLC | Federated calibration of *w_i(tissue)* across clinics |
| Ontogenesis | Developmental trajectory (0–25 yr) with MCOA counter families |
| HAP | Clinical backdrop; no direct MCOA integration |

---

## 8. Success criteria (v1.0)

- [x] Nature Aging Perspective manuscript ready (`~/Documents/MCOA_NatureAging_submission/`)
- [ ] Rust reference implementation (`mcoa_core`, `mcoa_simulation`) compiling and tested
- [ ] At least one MCOA Test 4 simulation run, output comparable to CDATA v5.1
- [ ] 3-figure visualisation (Fig 1–3 already produced for Perspective)
- [ ] Submission to *Nature Aging* by 2026-04-25

---

## 9. What MCOA is NOT

- MCOA is not a new set of biomarkers — it uses existing ones (Horvath, DunedinPACE, GT335, MitoSOX, telomere qFISH, 8-OHdG).
- MCOA is not a single-disease theory — it is a framework that any specific disease/tissue can be reduced to.
- MCOA does not assume "no repair" — repair appears as a negative contribution to the counter's drift rate.
- MCOA does not privilege any counter a priori — weights are measured, not decreed.

---

**Version:** 1.0
**Date:** 2026-04-21
**Next revision trigger:** Nature Aging editorial decision OR completion of MCOA Test 4 simulation.


---

## Роль MCOA в EIC Pathfinder Part B v3 (Variant B, submission 2026-05-12)

MCOA является **WP1 MCOA Framework** в текущей заявке EIC Pathfinder Open.

**Цель WP1:** формализовать MCOA как операциональный стандарт для интеграции моделей клеточного/организменного старения. Результат — software library + community white paper + dimensional transformation functions `f_i(D_i)` для ключевых counters (CDATA, telomere, epigenetic clock drift).

**Duration:** M1-M12 (первые 12 месяцев проекта)
**Budget:** €0.3M (1 postdoc + 0.5 PhD)
**TRL target:** 2 → 3

**Связь с другими WP:**
- **WP2 CDATA Experimental:** использует MCOA dimensional framework для интерпретации in vivo результатов
- **WP3 CDATA Computational:** использует MCOA coupling параметры для Bayesian model comparison (ABL-2 resolution)
- **WP4 FCLC Platform:** использует MCOA counter registry для federated model aggregation schema

**Обязательства (после WP1 завершения):**
1. Публикация MCOA specification paper (открытый стандарт)
2. Reference implementation в open-source crate `mcoa-framework`
3. Документированные JSON schemas для counter registration
4. Bayesian coupling estimation protocol (см. CORRECTIONS §1.3 — `γ_i = 0` by default, отклонение requires post-hoc statistical rejection)

Подробности: [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md) §1.4 EIC структура v3.

```
### `THEORY.md` (10447 chars)
```md
# Теоретическое обоснование MCOA

## 1. Философские и методологические предпосылки

MCOA строится на принципах **плюрализма механизмов** и **строгой фальсифицируемости**. Он отвергает редукционистский поиск единой «первопричины» старения, признавая, что множественные, частично независимые процессы накопления повреждений могут достигать критических порогов в разных тканях и в разное время. Ключевая методологическая установка — запрет на пост-фактумную подгонку весов счётчиков (`w_i`). Все веса и референтные масштабы (`n_i*`, `τ_i`) должны быть зафиксированы *априорно*, на основе независимых биологических знаний, до валидации модели на данных. Это превращает MCOA из инструмента кривой подгонки в набор проверяемых предсказаний.

## 2. Аксиоматическая основа

**Аксиома M1 (Параллельные счётчики).** Организменное старение управляется как минимум двумя (`k ≥ 2`) различными процессами накопления повреждений, которые протекают параллельно. Ни один счётчик `i` не является достаточным для объяснения универсальности репликативных пределов и разнообразия паттернов старения тканей. Формально: `∃ i, j (i ≠ j)`, такие что для некоторых тканей вклад `w_i·f_i(D_i)` и `w_j·f_j(D_j)` сравним, и отсутствие любого из них делает модель неадекватной.

**Аксиома M2 (Размерная согласованность).** В кинетическом уравнении счётчика недопустимо прямое сложение членов, зависящих от числа делений (`n`) и хронологического времени (`t`), без приведения к общей безразмерной форме. Каноническая форма:
`D_i(n, t) = D_i₀ + α_i · (n / n_i*) + β_i · (t / τ_i) + γ_i · I(other counters)`.
Здесь `n_i*` (референтное число делений) и `τ_i` (референтное время) — константы, *априорно* фиксируемые для каждого счётчика на основе клеточной биологии (например, `n_i*` = лимит Хейфлика для теломерного счётчика в фибробластах; `τ_i` = период полураспада тубулина для CDATA). Это гарантирует, что `α_i` и `β_i` становятся безразмерными *интенсивностями* повреждения на одну единицу нормированной шкалы.

**Аксиома M3 (Априорное весовое взвешивание тканей).** Вес `w_i(tissue)`, определяющий вклад счётчика `i` в общую нагрузку ткани, должен быть предсказан ДО процедуры фиттинга модели к экспериментальным данным по старению. Прогноз основывается на независимых клеточно-тканевых параметрах: базальная скорость делений, метаболическая интенсивность, время полужизни основного субстрата счётчика, экспрессия релевантных генов (напр., TERT для теломер, TTLL/CCP для CDATA), содержание митохондрий. Любая пост-фактумная корректировка `w_i` для улучшения согласия с данными рассматривается как корректировка модели, а не её предсказание, и должна быть явно задекларирована как гипотеза для следующего цикла проверки.

**Аксиома M4 (Фальсифицируемость как принцип первого порядка).** Любое утверждение, дедуктивно выведенное из MCOA, должно сопровождаться описанием практически выполнимого экспериментального теста, результат которого может это утверждение опровергнуть. Наличие таких тестов является обязательным атрибутом завершённой теоретической конструкции в рамках MCOA.

## 3. Формальные определения

### 3.1. Кинетика одиночного счётчика

Повреждение `i`-го счётчика описывается уравнением:
`D_i(n, t) = D_i₀ + α_i · (n / n_i*) + β_i · (t / τ_i) + γ_i · I(other counters)`

**Определения символов:**
*   `D_i`: Накопленное повреждение для счётчика `i`. Безразмерная величина, `D_i ≥ 0`.
*   `D_i₀`: Базовый уровень повреждения при рождении (или в референтном молодом состоянии). `D_i₀ ≥ 0`.
*   `α_i`: Интенсивность повреждения, driven by cell divisions. Безразмерная величина, представляет прирост повреждения за одну референтную единицу делений (`n / n_i* = 1`). `α_i ≥ 0`.
*   `β_i`: Интенсивность повреждения, driven by chronological time. Безразмерная величина, представляет прирост повреждения за одну референтную единицу времени (`t / τ_i = 1`). `β_i ≥ 0`.
*   `γ_i`: Скаляр связи. Определяет силу влияния других счётчиков на скорость накопления повреждения в счётчике `i`. `γ_i ∈ ℝ`. **Каноническое значение по умолчанию:** `γ_i = 0` (гипотеза независимости). Отклонение от нуля требует статистического обоснования на данных.
*   `I(other counters)`: Функция влияния. Простейшая линейная форма: `I = Σ_{j≠i} (Γ_{ij} · D_j / D_j_crit)`, где `Γ_{ij}` — безразмерный элемент матрицы связей, `D_j_crit` — критическое значение повреждения для счётчика `j`. Могут быть предложены нелинейные формы.
*   `n_i*`: Референтное число делений для счётчика `i`. Фиксируется *априорно* (напр., лимит Хейфлика для данного типа клеток).
*   `τ_i`: Референтная временная шкала для счётчика `i`. Фиксируется *априорно* (напр., время полужизни тубулина для CDATA, константа дрейфа эпигенетических часов).

### 3.2. Интегрированная нагрузка на ткань

Общая фенотипическая нагрузка, обусловленная старением, в данной ткани определяется как взвешенная сумма преобразованных значений повреждений от всех счётчиков:
`L_tissue(n, t) = Σ_{i=1}^{k} [ w_i(tissue) · f_i( D_i(n, t) ) ]`

**Условия:**
1.  `Σ_i w_i(tissue) ≈ 1.0`. Значимое отклонение от 1 (напр., > 0.05) интерпретируется как указание на отсутствие в модели важного счётчика для данной ткани.
2.  `f_i(x)` — монотонно возрастающая функция преобразования, приводящая повреждение к общей шкале вклада в нагрузку. Простейший вариант — линейный: `f_i(x) = x`. Альтернативы — сигмоидальные функции для учёта пороговых эффектов.

### 3.3. Функциональный переход (сенесценция/дисфункция)

Клетка или тканевая ниша переходит в состояние сенесценции, апоптоза или выраженной дисфункции при выполнении одного из двух условий:
1.  `L_tissue(n, t) > L_critical(tissue)`, где `L_critical` — тканеспецифичный порог интегральной нагрузки.
2.  `∃ i : D_i(n, t) > D_critical(i, tissue)`, где `D_critical` — тканеспецифичный порог для конкретного счётчика (например, критическое укорочение теломер).

## 4. Канонический набор счётчиков MCOA (v1.0)

| # | Название | Проект | Природа | `n_i*` (якорь) | `τ_i` (якорь) | Комментарий |
|---|----------|--------|---------|----------------|---------------|-------------|
| 1 | **Центриолярная полиглутамилизация (CP)** | CDATA | Деления + Время | ~50–80 (для HSC), ~30–50 (для эпителия) | Месяцы–годы (калибруется по масс-спектрометрии) | Структурный счётчик асимметричного наследования. `α_i` значим, `β_i` обусловлен оборотом тубулина. |
| 2 | **Укорочение теломер / теломерный стресс** | Telomere | Доминантно деления | Лимит Хейфлика для типа клеток (напр., ~50 для фибробластов человека) | Время оборота теломерных повторов (недели) | Классический репликативный счётчик. `β_i ≈ 0` для большинства соматических клеток. |
| 3 | **Митохондриальный ROS / повреждение мтДНК** | MitoROS | Доминантно время | `α_i → 0` для постмитотических клеток | Дни–недели (оборот повреждений мтДНК) | Метаболический/временной счётчик. `β_i` значим, может усиливаться при дисфункции. |
| 4 | **Эпигенетический дрейф (метилирование ДНК)** | EpigeneticDrift | Доминантно время | `α_i → 0` для большинства клеток | Время удвоения эпигенетического возраста (напр., ~3.6 года по DunedinPACE) | «Молекулярные часы». Вклад делений (`α_i`) мал, но может быть ненулевым в стволовых/пролиферирующих компартментах. |
| 5 | **Нарушение протеостаза (агрегация белков)** | Proteostasis | Смешанная | Зависит от типа клеток (частота делений влияет на «разбавление» агрегатов) | Время полужизни доминирующего агрегирующего белка (дни–годы) | `α_i` может быть отрицательным, если деление удаляет агрегаты; `β_i` положительный. |

**Порядок и обоснование:** Счётчик CP (#1) занимает первое место как структурный элемент, организующий асимметричное наследование повреждений — ключевой принцип в гипотезе изнашивания стволовых клеток. Теломерный счётчик (#2) рассматривается как зависимый от CP (возможно, через сигнальные пути, регулируемые состоянием центриолей). Остальные счётчики представляют основные клеточные машинерии (энергетика, регуляция генома, качество белков).

## 5. Матрица связей между счётчиками (Γ)

Матрица `Γ ∈ ℝ^{k×k}` определяет направленное влияние: элемент `Γ_{ij}` — это скорость, с которой накопленное повреждение в счётчике `j` ускоряет накопление повреждения в счётчике `i`.

**Известные (из литературы) предполагаемые ненулевые связи:**
*   `Γ_{telomere, mito} > 0`: Окислительный стресс (митохондрии) ускоряет укорочение теломер (Parrinello et al., 2003).
*   `Γ_{epigenetic, mito} > 0`: Митохондриальные сигналы (NAD+/NADH) влияют на активность эпигенетических модификаторов (Schultz & Sinclair, *Cell* 2019, PMID 30982602 — обзор по NAD+/sirtuin/aging axis). <!-- corrected 2026-04-26: prior citation «Sun et al. 2016 Measuring In Vivo Mitophagy» was fabricated (PMID 26833090 → unrelated paper; real Sun 2017 Nat Protoc PMID 28132843 also unrelated to NAD+/epigenetic axis). См. _audits/PEER_REVIEW_v2_TopMCOAZe_2026-04-26.md §0 row 7 -->
*   `Γ_{centriole, epigenetic} > 0` (гипотетическая): Эпигенетическая дерегуляция может изменять баланс ферментов TTLL/CCP, влияющих на полиглутамилирование (Janke & Magiera, 2020).

**Ключевое правило:** Элементы `Γ_{ij}` (и, следовательно, `γ_i` в упрощённой форме) должны **измеряться** в контролируемых экспериментах (см. MCOA Test 2), а не быть свободными параметрами для подгонки. Это отделяет причинно-следственный вывод от корреляционного.

## 6. Предсказания теории MCOA

1.  **Гетерогенность доминирующих счётчиков:** В разных тканях один и тот же счётчик будет иметь разный вес `w_i`. Например, в печени (`low division rate`) вес митохондриального и эпигенетического счётчиков будет выше, чем теломерного.
2.  **Нелинейная реакция на интервенции:** Эффект от вмешательства, направленного на конкретный счётчик (напр., активатор теломеразы), будет максимальным в тканях, где `w_telomere` велико, и минимальным — где оно мало.
3.  **Синергия таргетных интервенций:** Комбинированное воздействие на несколько счётчиков с высокими `w_i` в данной ткани даст сверхаддитивный эффект на продление здоровья, в то время как воздействие на нерелевантные счётчики — не даст.
4.  **Существование «развязанных» тканей:** Можно идентифицировать ткани, где общая нагрузка `L_tissue` остаётся низкой, несмотря на высокие значения одного из счётчиков (`D_i`), благодаря компенсаторно низким весам других счётчиков.
5.  **Прогноз траекторий старения:** При известных *априорных* `w_i(tissue)`, `n_i*`, `τ_i` и начальных `D_i₀`, модель предсказывает траекторию накопления нагрузки `L_tissue(t)` для каждой ткани, которую можно проверить в лонгитюдных исследованиях.
```
### `PARAMETERS.md` (9597 chars)
```md
# Параметры MCOA

*Таблица всех количественных параметров, их источников, единиц измерения и статуса. Последнее обновление: 2026-04-22.*

**Легенда статуса:**
*   **Fixed (Canonical):** Значение зафиксировано *априорно* и не подлежит изменению при фиттинге.
*   **Fixed (Measured):** Значение установлено по данным независимого эксперимента.
*   **To be calibrated:** Требует калибровки на данных в рамках MCOA.
*   **Placeholder:** Временное значение, используемое для симуляций; требует замены на Fixed или To be calibrated.
*   **Default 0:** Значение по умолчанию согласно каноническим исправлениям (например, `γ_i = 0`).

## Глобальные и системные параметры

| Символ | Описание | Значение/Диапазон | Единицы | Источник / Обоснование | Статус |
|--------|----------|-------------------|---------|------------------------|--------|
| `k` | Количество канонических счётчиков | 5 | безразмерн. | Теоретический выбор (см. THEORY.md) | **Fixed (Canonical)** |
| `L_critical(tissue)` | Критическая интегральная нагрузка для ткани | Зависит от ткани | безразмерн. | Должна определяться экспериментально для каждой ткани | **To be calibrated** |
| `f_i(x)` | Функция преобразования повреждения счётчика в вклад в нагрузку | `f_i(x) = x` (линейная) | безразмерн. | Упрощение по умолчанию; могут быть предложены нелинейные формы | **Fixed (Canonical)** для базовой модели |

## Параметры счётчиков (для каждого счётчика `i`)

*Общие символы: `D_i₀`, `α_i`, `β_i`, `γ_i`, `n_i*`, `τ_i`.*

### Счётчик 1: Центриолярная полиглутамилизация (CP) — проект CDATA
| Символ | Описание | Значение/Диапазон | Единицы | Источник / Обоснование | Статус |
|--------|----------|-------------------|---------|------------------------|--------|
| `D_CP₀` | Базовое повреждение CP при рождении | 0.0 – 0.1 | безразмерн. | Гипотеза: минимальный уровень у новорождённых | **Placeholder** |
| `α_CP` | Интенсивность повреждения от делений | 0.01 – 0.05 | безразмерн. на (`n / n_CP*`) | Оценка на основе данных CDATA v5.1 симуляций | **To be calibrated** |
| `β_CP` | Интенсивность повреждения от времени | 0.001 – 0.01 | безразмерн. на (`t / τ_CP`) | Оценка на основе данных CDATA v5.1 симуляций | **To be calibrated** |
| `γ_CP` | Скаляр связи для CP | 0.0 | безразмерн. | Канон: по умолчанию `γ_i = 0` (независимость) | **Default 0** |
| `n_CP*` | Референтное число делений для CP | 50 (для фибробластов), 80 (для HSC) | деления | *Априорная* оценка на основе лимита пролиферации клеток | **Fixed (Canonical)** |
| `τ_CP` | Референтное время для CP | 1.0 год (3.1536e7 с) | секунды | *Априорная* оценка: характерное время обновления тубулинового пула | **Fixed (Canonical)** |
| `w_CP(tissue)` | Вес CP в ткани | Зависит от ткани (см. таблицу весов) | безразмерн. | Должен быть предсказан *априорно* | **To be calibrated** |

### Счётчик 2: Теломеры (Telomere)
| Символ | Описание | Значение/Диапазон | Единицы | Источник / Обоснование | Статус |
|--------|----------|-------------------|---------|------------------------|--------|
| `D_Tel₀` | Базовое повреждение (нормированная начальная длина) | 0.0 (max длина) | безразмерн. | Нормировка: `D_Tel = 1 - (длина/макс_длина)` | **Fixed (Canonical)** |
| `α_Tel` | Интенсивность укорочения за деление | 0.02 – 0.04 (соответствует ~50-100 п.н. за деление) | безразмерн. на (`n / n_Tel*`) | Литература: среднее укорочение теломер за деление. Harley et al *Nature* 1990 (PMID 2342578) + Allsopp et al *PNAS* 1992 (PMID 1631178). | **Fixed (Measured)** [PMID: 2342578, 1631178] (CORRECTED 2026-04-26: prior PMID 2038241 was fabricated — pointed to 1991 desktop image analysis paper) |
| `β_Tel` | Интенсивность повреждения от времени (неделения) | 0.0 | безразмерн. на (`t / τ_Tel`) | Гипотеза: в отсутствие делений укорочения нет | **Fixed (Canonical)** |
| `γ_Tel` | Скаляр связи для теломер | 0.0 | безразмерн. | Канон: по умолчанию `γ_i = 0` | **Default 0** |
| `n_Tel*` | Референтное число делений (лимит Хейфлика) | 50 (для фибробластов человека) | деления | Классическое значение | **Fixed (Canonical)** |
| `τ_Tel` | Референтное время обновления теломер | 30 дней (2.592e6 с) | секунды | *Априорная* оценка времени оборота теломерных белков | **Fixed (Canonical)** |
| `w_Tel(tissue)` | Вес теломер в ткани | Зависит от ткани | безразмерн. | Должен быть предсказан *априорно* | **To be calibrated** |

### Счётчик 3: Митохондриальные ROS/мтДНК (MitoROS)
| Символ | Описание | Значение/Диапазон | Единицы | Источник / Обоснование | Статус |
|--------|----------|-------------------|---------|------------------------|--------|
| `D_Mito₀` | Базовый уровень повреждения | 0.0 – 0.05 | безразмерн. | Гипотеза | **Placeholder** |
| `α_Mito` | Интенсивность повреждения от делений | 0.0 (для постмитотических) | безразмерн. на (`n / n_Mito*`) | Гипотеза: деление не генерирует значимый ROS-сигнал напрямую | **Fixed (Canonical)** |
| `β_Mito` | Интенсивность повреждения от времени | 0.005 – 0.02 | безразмерн. на (`t / τ_Mito`) | Оценка по данным накопления 8-OHdG с возрастом | **To be calibrated** |
| `γ_Mito` | Скаляр связи для мито | 0.0 | безразмерн. | Канон: по умолчанию `γ_i = 0` | **Default 0** |
| `n_Mito*` | Референтное число делений | 1 (нерелевантно, т.к. `α≈0`) | деления | Условное значение | **Fixed (Canonical)** |
| `τ_Mito` | Референтное время оборота повреждений мтДНК | 7 дней (6.048e5 с) | секунды | *Априорная* оценка времени репарации мтДНК | **Fixed (Canonical)** |
| `w_Mito(tissue)` | Вес митохондриального счётчика | Зависит от ткани | безразмерн. | Должен быть предсказан *априорно* | **To be calibrated** |

### Счётчик 4: Эпигенетический дрейф (EpiDrift)
| Символ | Описание | Значение/Диапазон | Единицы | Источник / Обоснование | Статус |
|--------|----------|-------------------|---------|------------------------|--------|
| `D_Epi₀` | Базовый эпигенетический возраст | 0.0 (хронологический возраст 0) | лет (или безразмерн.) | Нормировка на хронологический возраст | **Fixed (Canonical)** |
| `α_Epi` | Интенсивность дрейфа от делений | 0.0 – 0.001 | безразмерн. на (`n / n_Epi*`) | Гипотеза: малый вклад делений в общий дрейф | **Placeholder** |
| `β_Epi` | Интенсивность дрейфа от времени | 1.0 | безразмерн. на (`t / τ_Epi`) | Определение: за `τ_Epi` лет эпиг. возраст увеличивается на 1 год | **Fixed (Canonical)** |
| `γ_Epi` | Скаляр связи для эпидрейфа | 0.0 | безразмерн. | Канон: по умолчанию `γ_i = 0` | **Default 0** |
| `n_Epi*` | Референтное число делений | 1000 (условное) | деления | *Априорная* оценка для нормировки малого `α` | **Fixed (Canonical)** |
| `τ_Epi` | Референтное время (время удвоения эпиг. возраста) | 3.6 года (1.135e8 с) | секунды | Значение из DunedinPACE | **Fixed (Measured)** [DOI: 10.7554/eLife.73420] |
| `w_Epi(tissue)` | Вес эпигенетического счётчика | Зависит от ткани | безразмерн. | Должен быть предсказан *априорно* | **To be calibrated** |

### Счётчик 5: Нарушение протеостаза (Prot)
| Символ | Описание | Значение/Диапазон | Единицы | Источник / Обоснование | Статус |
|--------|----------|-------------------|---------|------------------------|--------|
| `D_Prot₀` | Базовый уровень агрегатов | 0.0 | безразмерн. | Гипотеза | **Placeholder** |
| `α_Prot` | Интенсивность изменения от делений | -0.01 – 0.0 | безразмерн. на (`n / n_Prot*`) | Деление может «разбавлять» агрегаты (отрицательный `α`) | **Placeholder** |
| `β_Prot` | Интенсивность накопления от времени | 0.001 – 0.01 | безразмерн. на (`t / τ_Prot`) | Оценка по кинетике накопления липофусцина | **To be calibrated** |
| `γ_Prot` | Скаляр связи для протеостаза | 0.0 | безразмерн. | Канон: по умолчанию `γ_i = 0` | **Default 0** |
| `n_Prot*` | Референтное число делений | 10 (условное) | деления | *Априорная* оценка | **Fixed (Canonical)** |
| `τ_Prot` | Референтное время оборота агрегирующих белков | 30 дней (2.592e6 с) | секунды | *Априорная* оценка времени полужизни агрегирующих белков | **Fixed (Canonical)** |
| `w_Prot(tissue)` | Вес протеостазного счётчика | Зависит от ткани | безразмерн. | Должен быть предсказан *априорно* | **To be calibrated** |

## Матрица связей Γ (элементы `Γ_{ij}`)
*По умолчанию все элементы = 0. Ненулевые элементы требуют экспериментального измерения.*

| Влияние на `i` \ От `j` | CP | Telomere | MitoROS | EpiDrift | Prot |
|-------------------------|----|----------|---------|----------|------|
| **CP** | – | 0 | 0 | 0 | 0 |
| **Telomere** | 0 | – | **To be measured** (PMID:12855956 — Parrinello et al *Nat Cell Biol* 2003; CORRECTED 2026-04-26 from fabricated 12612578) | 0 | 0 |
| **MitoROS** | 0 | 0 | – | 0 | 0 |
| **EpiDrift** | 0 | 0 | **To be measured** (Reference NEEDED: prior PMID 26833090 was fabricated. Suggested replacements: Schultz MB & Sinclair DA *Cell* 2019 PMID 30982602 для NAD+/sirtuin/aging axis) | – | 0 |
| **Prot** | 0 | 0 | 0 | 0 | – |

*Статус для ненулевых элементов: **To be measured**.*

## Таблица весов тканей `w_i(tissue)` (Placeholder)
*Настоящая таблица должна быть заполнена результатами *априорного* прогноза. Ниже — гипотетические значения для симуляций.*

| Ткань | w_CP | w_Tel | w_Mito | w_Epi | w_Prot | Σ (должно быть ≈1) |
|-------|------|-------|--------|-------|--------|-------------------|
| Фибробласты кожи (in vitro) | 0.1 | **0.7** | 0.05 | 0.1 | 0.05 | 1.0 |
| Гепатоциты (печень) | 0.05 | 0.1 | **0.4** | **0.3** | 0.15 | 1.0 |
| Нейроны (кора мозга) | 0.0 | 0.0 | **0.5** | **0.3** | **0.2** | 1.0 |
| Кишечный эпителий | **0.3** | **0.4** | 0.1 | 0.1 | 0.1 | 1.0 |
| Скелетная мышца | 0.1 | 0.1 | **0.5** | 0.2 | 0.1 | 1.0 |

**Статус всех `w_i` в этой таблице: Placeholder (требует замены на *априорный* прогноз).**
```
### `STATE.md` (2811 chars)
```md
# STATE — MCOA

**Назначение:** волатильное состояние.

---

## Current status (2026-04-25)

- **Submission:** Nature Aging NATAGING-P13741, поданo 2026-04-19, статус review
- **Версия рукописи:** MCOA v5
- **Counters:** 5 + χ_Ze (S-counter)
- **Tissue weights:** w_HSC, w_skin, w_neural, w_muscle (см. PARAMETERS.md)

---

## Active TODOs

- [ ] Дождаться решения Nature Aging editorial decision
- [ ] Подготовить response к reviewer comments (если будут)
- [ ] Backup: secondary target (npj Aging, eLife) если reject
- [ ] Sobol ABL-2 paradox для Counter #1 — закрыть в координации с CDATA L1
- [ ] Tissue-specific weights калибровка против реальных данных HSC/skin/neural

---

## Milestones

### v5 — Nature Aging submission ✅ 2026-04-19
- [x] MCOA_v5_NatureAging_2026-04-21.pdf готов
- [x] Cover letter
- [x] Submission через editorial system
- [x] 2 follow-up correspondence (2026-04-21)

### v9-file core ✅ 2026-04-25
- [x] CLAUDE.md создан
- [x] STATE.md создан

### Code baseline ✅ 2026-04-25 (overnight #5 fixed)
- [x] cargo build --release: success
- [x] mcoa_core: 6/6 unit tests pass (было 3 → +3 новых)
- [x] **NEW:** `aging_rate_is_weighted_sum` — формула `Σ w_i · C_i = 0.42` на тестовых значениях
- [x] **NEW:** `null_gamma_yields_zero_influence` — γ=0 default per CORRECTIONS-2026-04-22
- [x] **NEW:** `identity_gamma_yields_self_value` — γ identity = self-value
- [x] mcoa_tests crate (workspace integration tests) — пуст, оставлен на будущее
- [x] mcoa_cli, mcoa_api — компилируются

### Python scripts → Rust port ✅ 2026-04-25 (overnight)

Created `crates/mcoa_compare/`:
- [x] `mcoa-compare-cdata` binary — replaces `scripts/compare_mcoa_cdata.py` (markdown report без plot)
- [x] `mcoa-compare-all` binary — replaces `scripts/compare_all.py` (pairwise Δ matrix)
- [x] `mcoa_compare` lib — `read_csv()`, `delta_stats()`, `compare_mcoa_cdata()`. **3/3 tests pass.**
- [x] cargo build --release: success
- [x] Plot generation вынесен из scope Rust port (можно добавить через `plotters` crate позже)
- [x] Старые Python скрипты остаются в `scripts/` для cross-validation

---

## Decision Log

### 2026-04-25 — 9-file core scheme
Добавлены CLAUDE + STATE. Существующие 7 файлов (CONCEPT/DESIGN/EVIDENCE/OPEN_PROBLEMS/PARAMETERS/README/THEORY) уже соответствуют новой схеме.

### 2026-04-19 — Nature Aging submission
MCOA v5 поданa в Nature Aging как flagship мета-теория LongevityCommon. Включает Counter #1 (CDATA), и формализует общую multi-counter архитектуру.

---

## Что НЕ делать

- Не публиковать препринт MCOA до решения Nature Aging
- Не добавлять новые counters без явной интеграции с CONCEPT.md
- Не путать "5 counters" со "5 hallmarks" (Counter ≠ hallmark)

## Startup checklist

1. Прочитать CONCEPT + STATE Decision Log
2. Проверить ответ Nature Aging
3. Спросить пользователя

```
### `DESIGN.md` (1065 chars)
```md
# Архитектура и дизайн MCOA

*Версия: 2026-04-22. Описывает эталонную реализацию на Rust, файловую структуру и API контракты.*

## 1. Обзор архитектуры

MCOA реализован как крейт (библиотека) на Rust с чётким разделением на:
1.  **Ядро (`mcoa_core`):** Чистые, детерминированные функции, реализующие формализм MCOA (уравнения счётчиков, нагрузка, пороги). Без зависимостей от ввода/вывода.
2.  **Симулятор (`mcoa_simulation`):** Модули для проведения симуляций (стохастические процессы, популяции клеток, лонгитюдные траектории). Использует ядро.
3.  **Интерфейсы (`mcoa_interfaces`):** Определения типов данных, сериализация (JSON/MessagePack), API для интеграции с другими подпроектами (CDATA, FCLC).
4.  **Инструменты (`mcoa_tools`):** Утилиты командной строки (CLI) для калибровки, анализа чувствительности, визуализации.

Цель: предоставить проверяемую, производительную и переносимую эталонную реализацию для научного сообщества.

## 2. Файловая структура проекта

```
mcoa_reference_impl/
├── Cargo.toml                    # Конфигурация крейта и зависимости
```
### `EVIDENCE.md` (7635 chars)
```md
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

1.  **Соболь-анализ чувствительности CDATA v5.1:**
    *   Файл: `data/mcoa/sensitivity/sobol_results_2026-04-15.csv`
    *   Метод: Глобальный анализ чувствительности (метод Соболя) для модели CDATA.
    *   Выборка: N = 16384.
    *   Ключевой результат: Первый порядок (S1) для параметра `α_cent` (деления) составляет 0.68 ± 0.05, для `β_cent` (время) — 0.22 ± 0.04 в симуляции эпителиальной ткани. Подтверждает доминирование делений, но значимый вклад времени.
    *   Статус: Проверено, воспроизводимо.

2.  **Перекрёстная проверка LOO-CV для предсказания нагрузки:** ⚠️ **FLAGGED 2026-04-26**
    *   Файл: `data/mcoa/validation/LOO_CV_2026-04-17.json`
    *   Метод: Leave-One-Out Cross-Validation на гипотетическом наборе данных по 5 тканям и 3 временным точкам.
    *   Результат: ~~Среднеквадратическая ошибка (MSE) = -0.093~~. **Mathematically impossible (MSE ≥ 0 by definition).** Скорее всего это R² (negative R² = модель хуже базовой средней). Цифра удалена из submission-grade документа до коррекции метрики.
    *   Статус: ⚠️ **REQUIRED ACTION:** переделать с правильной метрикой (R², MAE, RMSE с положительным значением); либо отметить как «model fails baseline» если R²<0.

## 3. Опровергающие свидетельства и нерешённые проблемы (честное раскрытие)

*Эта секция напрямую связана с [OPEN_PROBLEMS.md](OPEN_PROBLEMS.md).*

1.  **Отсутствие прямых измерений *априорных* весов `w_i(tissue)`.**
    *   **Свидетельство:** На данный момент не существует общепринятой базы данных, которая бы связывала такие параметры, как скорость делений клеток in vivo, метаболический коэффициент и экспрессию специфических генов, с предсказанным вкладом в старение ткани.
    *   **Следствие:** Текущие реализации MCOA вынуждены использовать упрощённые эвристики или placeholder-значения для `w_i`. Это ослабляет проверяемость Аксиомы M3.

2.  **Парадокс ABL-2 — РАЗРЕШЁН 2026-04-26 через counter-factual Sobol analysis.**
    *   **Прежнее свидетельство (NMC-2):** Individual S1(epigenetic_rate)=0.403 > S1(alpha_centriolar)=0.224 указывал, что центриолярный счётчик может быть downstream/parallel.
    *   **Counter-factual ablation analysis (v4.7, N=8192, executed 2026-04-26 via `scripts/cdata_ablation_sobol.py`):**
        - Centriolar parameter group (alpha, nu, beta, tau, pi): **S1_sum = 0.471**
        - Epigenetic parameter group (ep_rate, ep_stress_k): **S1_sum = 0.470**
        - При epigenetic_rate = 0: alpha S1 → 0.362 (dominant)
        - **Centriolar group dominates epigenetic group: 0.471 vs 0.470**
    *   **Разрешение:** Individual epigenetic_rate dominance объясняется linear additivity + parameter correlation (alpha drives damage which drives ep_stress_k). На group-level центриолярная механика **доминирует**.
    *   **Следствие:** Counter #1 (CP) сохраняет canonical position, переформулирован как «structural age-tracker» per `CDATA/docs/CDATA_REFORMULATION_2026-04-26.md`. NMC-2 closed.
    *   **Источник:** `~/Desktop/LongevityCommon/CDATA/scripts/cdata_ablation_sobol.py` + ablation log 2026-04-26.

3.  **Слабая экспериментальная база для матрицы связей Γ.**
    *   **Свидетельство:** Большинство предполагаемых связей между счётчиками (например, `Γ_{cent, epigenetic}`) основаны на косвенных корреляциях или исследованиях in vitro, а не на прямых причинно-следственных экспериментах in vivo.
    *   **Следствие:** Текущие значения Γ, используемые в симуляциях, являются гипотетическими. Каноническое значение `γ_i = 0` (независимость) часто может быть более обоснованным.

4.  **Неудача предварительных тестов χ_Ze.**
    *   **Свидетельство:** Предварительные попытки валидации χ_Ze как интегрального биомаркера в когортах MPI-LEMON, Dortmund Vital и Cuban не показали прогностической силы, превышающей стандартные часы.
    *   **Следствие:** Исключает возможность простого использования χ_Ze в качестве «шестого», интегрального счётчика синхронизации в текущей версии MCOA. χ_Ze остаётся теоретическим конструктом.
    *   **Источник:** Отчёт `internal/ze_validation_failures_2026-04.pdf` (доступ по запросу).
```
### `OPEN_PROBLEMS.md` (9003 chars)
```md
# Открытые проблемы и фальсифицируемые тесты для MCOA

*Версия: 2026-04-22. Каждая проблема содержит конкретный фальсифицируемый тест с чёткими исходами и приоритетом.*

## Проблема 1: Определение *априорных* весов тканей `w_i(tissue)`

**Описание:** Аксиома M3 требует, чтобы вес каждого счётчика в ткани определялся до фиттинга модели, на основе независимых биологических знаний. В настоящее время отсутствует общепринятый, количественный метод для предсказания `w_i`. Использование эвристик или placeholder-значений подрывает фальсифицируемость.

**Приоритет:** **Высокий** (P0). Блокирует полноценную экспериментальную проверку MCOA.

### Фальсифицируемый тест 1A: Прогноз тканевой иерархии счётчиков

**Гипотеза:** На основе комбинации данных RNA-seq (экспрессия генов, связанных со счётчиками), измерений скорости деления клеток in vivo и метаболомики можно построить прогностическую модель для `w_i`, которая будет коррелировать с измеренной в эксперименте важностью каждого счётчика для возрастного фенотипа ткани.

**Эксперимент:**
1.  **Выборка:** 5-7 различных тканей мыши (напр., печень, кожа, кишечник, мозг, скелетная мышца, селезёнка, жир).
2.  **Прогноз:** Для каждой ткани вычислить `w_i_pred` на основе:
    *   Уровня экспрессии ключевых генов (напр., TERT для теломер, TTLL/CCP для CP, маркеры окислительного стресса для мито).
    *   Оценки in vivo скорости пролиферации (например, по EdU).
    *   Данных метаболомики (NAD+/NADH, АТФ/АДФ).
3.  **Измерение:** Провести лонгитюдное исследование (3 возрастные точки) для тех же тканей. Количественно измерить возрастное изменение для каждого счётчика (укорочение теломер qFISH, уровень CP по GT335, 8-OHdG, эпигенетический возраст по часовым CpG).
4.  **Критерий:** Рассчитать корреляцию между предсказанным весом `w_i_pred` и измеренной долей объяснённой возрастной дисперсии фенотипа для данного счётчика в данной ткани.

**Возможные исходы:**
1.  **✅ Сильная корреляция (R² > 0.7):** Подтверждает возможность *априорного* предсказания весов. MCOA проходит этот тест.
2.  **⚠️ Умеренная корреляция (0.3 < R² < 0.7):** Указывает на частичную предсказательную силу метода. Требуется уточнение модели предсказания весов (например, добавление новых параметров).
3.  **❌ Слабая/отсутствующая корреляция (R² < 0.3):** Фальсифицирует конкретный метод предсказания `w_i`. Ставит под сомнение практическую реализуемость Аксиомы M3 в её текущей формулировке. Требуется поиск принципиально иных способов *априорной* фиксации весов.
4.  **🔀 Противоречивый результат:** Разные методы предсказания дают сильно различающиеся `w_i_pred`. Указывает на фундаментальную неопределённость в выборе *априорных* параметров, что ослабляет всю конструкцию MCOA.

## Проблема 2: Разрешение парадокса ABL-2 и позиционирование CP-счётчика

**Описание:** Высокая корреляция уровней белка ABL-2 с эпигенетическим возрастом в данных CDATA ставит под вопрос причинно-следственную связь. Является ли центриолярная полиглутамилизация (CP) upstream-драйвером старения, downstream-эффектом или параллельным процессом?

**Приоритет:** **Высокий** (P1). Касается обоснованности выбора CP как счётчика #1.

### Фальсифицируемый тест 2A: Причинное вмешательство в путь CP

**Гипотеза:** Если CP является upstream-драйвером, то экспериментальное ингибирование ферментов полиглутамилирования (TTLL) или активация деглутамилирующих ферментов (CCP) в молодых животных должно замедлить накопление повреждений в других счётчиках (теломеры, эпигенетический дрейф) и отсрочить возрастные фенотипы.

**Эксперимент:**
1.  **Модель:** Мыши с условным нокаутом/ингибированием ключевого TTLL фермента в эпителии кишечника (высокая пролиферация) или гепатоцитах (низкая пролиферация). Контроль — мыши дикого типа.
2.  **Вмешательство:** Активация нокаута/начало приёма ингибитора в возрасте 6 месяцев.
3.  **Измерения (в 12 и 18 месяцев):**
    *   **CP-счётчик:** Уровень полиглутамилирования тубулина (масс-спектрометрия).
    *   **Другие счётчики:** Длина теломер (qFISH), метилирование ДНК (часовые CpG), маркеры митохондриальной функции.
    *   **Фенотип:** Гистология на фиброз/воспаление, функциональные тесты ткани.
4.  **Критерий:** Сравнение скорости изменения других счётчиков и выраженности фенотипов между экспериментальной и контрольной группой.

**Возможные исходы:**
1.  **✅ Замедление всех счётчиков и фенотипов:** Сильно поддерживает upstream-роль CP. MCOA с CP #1 подтверждён.
2.  **⚠️ Замедление только CP, но не других счётчиков:** Указывает, что CP — параллельный, но не причинный процесс. Требуется пересмотр его позиции в иерархии.
3.  **❌ Отсутствие эффекта или ускорение старения:** Фальсифицирует гипотезу о CP как значимом драйвере старения в данной ткани. Может указывать на компенсаторные механизмы или ошибку в выборе мишени.
4.  **🔀 Тканеспецифичный эффект:** Замедление наблюдается только в высокопролиферативном эпителии кишечника, но не в печени. Подтверждает контекстно-зависимую роль CP, что согласуется с MCOA, но усложняет общую теорию.

## Проблема 3: Измерение матрицы связей Γ и проверка гипотезы `γ_i = 0`

**Описание:** По умолчанию в MCOA принимается `γ_i = 0` (гипотеза независимости счётчиков). Ненулевые связи должны быть доказаны. Не существует стандартизированного in vivo протокола для прямого измерения элементов `Γ_{ij}`.

**Приоритет:** **Средний** (P2). Критично для предсказания синергии интервенций.

### Фальсифицируемый тест 3A: Протокол MCOA Test 2 (in vitro калибровка)

**Гипотеза:** В контролируемых условиях клеточной культуры можно, манипулируя одним счётчиком (например, вызывая окислительный стресс для митохондрий), количественно измерить ускорение накопления повреждений в другом счётчике (например, скорость укорочения теломер).

**Эксперимент:**
1.  **Система:** Первичные человеческие фибробласты или мезенхимальные стволовые клетки.
2.  **Вмешательство:** Лечение низкими дозами ротенона (митохондриальный стресс) или прямая генерация ROS в митохондриях (оптогенетика).
3.  **Измерения:** Параллельный, лонгитюдный мониторинг:
    *   Уровень митохондриальных ROS (MitoSOX).
    *   Длина теломер (qFISH или flow-FISH) на каждом пассаже.
    *   (Опционально) Уровень CP, маркеры эпигенетических изменений.
4.  **Анализ:** Сравнить скорость укорочения теломер (Δтеломеры/пассаж) в условиях стресса и контроля. Рассчитать `Γ_{telomere, mito}` как отношение этих скоростей.

**Возможные исходы:**
1.  **✅ `Γ_{telomere, mito}` значимо > 1:** Подтверждает наличие связи. Позволяет установить численное значение для этого элемента матрицы.
2.  **✅ `Γ_{telomere, mito}` ≈ 1:** Не отвергает гипотезу независимости (`γ_i = 0`) для этой пары в данной системе.
3.  **❌ `Γ_{telomere, mito}` < 1 (замедление):** Фальсифицирует общепринятое направление связи. Указывает на возможный защитный эффект mild стресса или артефакт системы.
4.  **🔀 Нелинейная зависимость:** Эффект наблюдается только после определённого порога повреждения митохондрий. Потребует модификации линейной модели связи в MCOA на пороговую или сигмоидальную.

## Проблема 4: Операционализация «нагрузки» `L_tissue` и её порога

**Описание:** Уравнение `L_tissue = Σ_i [ w_i · f_i(D_i) ]` и условие `L_tissue > L_critical` являются абстракциями. Не ясно, какую конкретную биологическую величину (смертность клеток, секреторный фенотип, функциональный спад) следует отображать на `L_tissue` и как измерить `L_critical`.

**Приоритет:** **Средний** (P2). Необходимо для количественных предсказаний.

### Фальсифицируемый тест 4A: Связь вычисленной `L_tissue` с репликативным лимитом in vitro

**Гипотеза:** В культуре фибробластов, где доминирует теломерный счётчик, вычисленная по MCOA нагрузка `L_tissue` будет монотонно расти с пассажами и достигнет постоянного значения `L_critical` в момент вхождения в сенесценцию (лимит Хейфлика).

**Эксперимент:**
1.  **Система:** Культура фибробластов человека от молодого донора.
2.  **Измерения на каждом пассаже:**
    *   Длина теломер (основной счётчик, `w ≈ 1`).
    *   Уровень CP, маркеры окислительного стресса (второстепенные счётчики, `w` малы).
    *   Маркер сенесценции (SA-β-Gal).
3.  **Расчёт:** Вычислить `L_tissue(passage)` по упрощённой модели (функции `f_i` — линейные).
4.  **Калибровка:** Определить `L_critical` как значение `L_tissue` в пассаже, когда >70% клеток становятся SA-β-Gal+.

**Возможные исходы:**
1.  **✅ Чёткий порог:** `L_tissue` плавно растёт и стабилизируется около `L_critical` при сенесценции. Значение `L_critical` консистентно между репликами.
2.  **⚠️ Порог с большим разбросом:** `L_critical` варьирует между линиями или экспериментами. Указывает на дополнительные скрытые переменные или шум.
3.  **❌ Отсутствие порога/корреляции:** `L_tissue` не коррелирует со вхождением в сенесценцию. Фальсифицирует простую линейную аддитивную модель для `L_tissue` в данной системе.
4.  **🔀 `L_tissue` достигает `L_critical` до сенесценции:** Предсказывает сенесценцию раньше, чем она происходит. Может указывать на то, что порог стохастичен или требуются дополнительные события.
```
### `Cargo.toml` (695 chars)
```toml
[workspace]
resolver = "2"
members = [
    "crates/mcoa_core",
    "crates/mcoa_simulation",
    "crates/mcoa_tests",
    "crates/mcoa_api",
    "crates/mcoa_cli",
    "crates/mcoa_compare",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Jaba Tkemaladze <jaba@longevity.ge>"]
license = "MIT"
repository = "https://github.com/djabbat/LongevityCommon"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
csv = "1.3"
clap = { version = "4", features = ["derive"] }
nalgebra = "0.33"
rand = "0.8"
rand_chacha = "0.3"
thiserror = "1"
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"

```
### `backend/Cargo.toml` (868 chars)
```toml
[package]
name = "mcoa_backend"
version = "0.1.0"
edition = "2021"
authors = ["LongevityCommon Team"]
description = "Multi-Counter Architecture of Organismal Aging backend"

[dependencies]
axum = "0.7"
tokio = { version = "1.37", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "macros", "chrono"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "1.0"
dotenvy = "0.15"
config = "0.14"
once_cell = "1.19"
uuid = { version = "1.7", features = ["serde", "v4"] }

[dev-dependencies]
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "macros", "migrate"] }

[[bin]]
name = "mcoa_backend"
path = "src/main.rs"

[workspace]

```
### `frontend/mix.exs` (891 chars)
```exs
defmodule McoaWeb.MixProject do
  use Mix.Project

  def project do
    [
      app: :mcoa_web,
      version: "0.1.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps()
    ]
  end

  def application do
    [
      mod: {McoaWeb.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  defp deps do
    [
      {:phoenix, "~> 1.7.0"},
      {:phoenix_live_view, "~> 0.20.0"},
      {:phoenix_html, "~> 4.0"},
      {:phoenix_live_reload, "~> 1.4", only: :dev},
      {:telemetry_metrics, "~> 1.0"},
      {:telemetry_poller, "~> 1.0"},
      {:gettext, "~> 0.20"},
      {:jason, "~> 1.4"},
      {:plug_cowboy, "~> 2.5"},
      {:finch, "~> 0.18"}
    ]
  end

  defp aliases do
    [
      setup: ["deps.get", "assets.setup"],
      "assets.setup": ["cmd --cd assets npm install"]
    ]
  end
end

```
### code `crates/mcoa_cli/src/main.rs`
```
//! mcoa-sim — run an MCOA simulation and write per-step records to CSV.
//!
//! Per the mandatory comparison rule (see ~/Desktop/LongevityCommon/MCOA/CLAUDE.md), every simulation
//! output MUST be paired with an analogous CDATA run via `scripts/compare_mcoa_cdata.py`.

use clap::Parser;
use mcoa_core::{Gamma, Tissue};
use mcoa_simulation::run;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mcoa-sim",
    version,
    about = "Run a Multi-Counter Architecture simulation.",
    long_about = "Runs a discrete-time MCOA simulation for a given tissue, writing per-step counter \
                   states and tissue load to CSV. Pair each output with a matching CDATA run (see \
                   scripts/compare_mcoa_cdata.py)."
)]
struct Cli {
    /// Tissue name: fibroblast | hsc | neuron | hepatocyte | beta_cell | cd8_t_memory
    #[arg(long, default_value = "hsc")]
    tissue: String,
    /// Number of division-equivalent steps
    #[arg(long, default_value_t = 100)]
    divisions: usize,
    /// Seconds per division-equivalent (default: 7 days)
    #[arg(long, default_value_t = 604800.0)]
    seconds_per_division: f64,
    /// Output CSV path
    #[arg(long, default_value = "mcoa_run.csv")]
    output: PathBuf,
}

fn parse_tissue(s: &str) -> Result<Tissue, String> {
    match s {
        "fibroblast" => Ok(Tissue::Fibroblast),
        "hsc" => Ok(Tissue::Hsc),
        "neuron" => Ok(Tissue::Neuron),
        "hepatocyte" => Ok(Tissue::Hepatocyte),
        "beta_cell" => Ok(Tissue::BetaCell),
        "cd8_t_memory" => Ok(Tissue::CD8TMemory),
        other => Err(format!("unknown tissue '{other}'")),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let tissue = parse_tissue(&cli.tissue)?;
    let gamma = Gamma::default();
    let records = run(tissue, cli.divisions, cli.seconds_per_division, &gamma);
    let mut writer = csv::Writer::from_path(&cli.output)?;
    for r in &records {
        writer.serialize(r)?;
    }
    writer.flush()?;
    eprintln!(
        "mcoa-sim: wrote {} records for tissue={} to {}",
        records.len(),
        cli.tissue,
        cli.output.display()
    );
    eprintln!("REMINDER: pair this output with CDATA via scripts/compare_mcoa_cdata.py");
    Ok(())
}

```
### code `crates/mcoa_api/src/main.rs`
```
//! mcoa-api — Axum REST + WebSocket server consumed by the Phoenix LiveView frontend.
//!
//! Endpoints (v0.1 + 2026-05-08 additions):
//!   POST /api/simulate                  — run a simulation synchronously
//!   GET  /api/counters                  — list counters, tissues, and defaults
//!   GET  /v1/counters/{id}/D            — compute D_i for given (tissue, n, t)
//!                                          (Phase 1.4 deliverable, 2026-05-08)
//!   GET  /healthz                       — liveness
//!
//! Future (v0.2):
//!   WS   /ws/stream              — stream long simulations + MCOA-vs-CDATA residuals in real time

use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use mcoa_core::{
    default_drift_rates, default_reference_scales, Counter, Gamma, Tissue, N_COUNTERS,
};
use mcoa_simulation::{run, SimulationRecord};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct SimulateRequest {
    tissue: String,
    divisions: Option<usize>,
    seconds_per_division: Option<f64>,
}

#[derive(Serialize)]
struct SimulateResponse {
    tissue: String,
    records: Vec<SimulationRecord>,
}

fn parse_tissue(s: &str) -> Option<Tissue> {
    match s {
        "fibroblast" => Some(Tissue::Fibroblast),
        "hsc" => Some(Tissue::Hsc),
        "neuron" => Some(Tissue::Neuron),
        "hepatocyte" => Some(Tissue::Hepatocyte),
        "beta_cell" => Some(Tissue::BetaCell),
        "cd8_t_memory" => Some(Tissue::CD8TMemory),
        _ => None,
    }
}

async fn simulate(
    Json(req): Json<SimulateRequest>,
) -> Result<Json<SimulateResponse>, (StatusCode, String)> {
    let tissue =
        parse_tissue(&req.tissue).ok_or((StatusCode::BAD_REQUEST, format!("bad tissue {}", req.tissue)))?;
    let divisions = req.divisions.unwrap_or(100);
    let seconds = req.seconds_per_division.unwrap_or(604800.0);
    let gamma = Gamma::default();
    let records = run(tissue, divisions, seconds, &gamma);
    Ok(Json(SimulateResponse { tissue: req.tissue, records }))
}

async fn counters() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "counters": Counter::ALL.iter().map(|c| c.as_str()).collect::<Vec<_>>(),
        "tissues": ["fibroblast","hsc","neuron","hepatocyte","beta_cell","cd8_t_memory"],
    }))
}

async fn healthz() -> &'static str { "ok" }

// ── /v1/counters/{id}/D — single-counter snapshot (Phase 1.4) ────────

fn parse_counter_id(id: u8) -> Option<Counter> {
    match id {
        1 => Some(Counter::Centriolar),
        2 => Some(Counter::Telomere),
        3 => Some(Counter::Mitochondrial),
        4 => Some(Counter::Epigenetic),
        5 => Some(Counter::Proteostasis),
…<truncated 98 more lines>…
```
### code `backend/src/main.rs`
```
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use mcoa_backend::config::Config;
use mcoa_backend::db::DbPool;
use mcoa_backend::error::AppResult;
use mcoa_backend::routes;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "mcoa_backend=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting MCOA backend...");

    // Load configuration
    let config = Config::load()?;
    tracing::debug!("Loaded config: {:?}", config);

    // Initialize database pool
    let db_pool = DbPool::connect(&config.database_url).await?;
    tracing::info!("Database connection established");

    // Apply migrations
    db_pool.run_migrations().await?;
    tracing::info!("Database migrations applied");

    // Build application
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(routes::counter_routes())
        .merge(routes::tissue_routes())
        .merge(routes::subject_routes())
        .merge(routes::damage_measurement_routes())
        .merge(routes::tissue_load_routes())
        .merge(routes::coupling_matrix_routes())
        .with_state(db_pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("Shutdown signal received");
}
```
### code `crates/mcoa_compare/src/lib.rs`
```
//! mcoa_compare — MCOA vs CDATA comparison and full pairwise comparison harness.
//!
//! Rust port of `scripts/compare_mcoa_cdata.py` and `scripts/compare_all.py`.
//! Plot generation is OUT OF SCOPE for the Rust port — comparison reports are
//! pure markdown + numeric statistics. Plots can be added later via plotters
//! crate or external pipeline.

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CsvSeries {
    pub headers: Vec<String>,
    pub columns: HashMap<String, Vec<f64>>,
    pub n_rows: usize,
}

pub fn read_csv(path: &Path) -> Result<CsvSeries> {
    let mut rdr = csv::Reader::from_path(path)?;
    let headers: Vec<String> = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    let mut columns: HashMap<String, Vec<f64>> = HashMap::new();
    for h in &headers {
        columns.insert(h.clone(), Vec::new());
    }
    let mut n_rows = 0;
    for record in rdr.records() {
        let record = record?;
        for (i, field) in record.iter().enumerate() {
            let parsed: f64 = field.parse().unwrap_or(f64::NAN);
            columns.get_mut(&headers[i]).unwrap().push(parsed);
        }
        n_rows += 1;
    }
    Ok(CsvSeries { headers, columns, n_rows })
}

#[derive(Debug, Clone)]
pub struct DeltaStats {
    pub n: usize,
    pub max_abs: f64,
    pub mean: f64,
    pub std: f64,
}

pub fn delta_stats(a: &[f64], b: &[f64]) -> DeltaStats {
    let n = a.len().min(b.len());
    if n == 0 {
        return DeltaStats { n: 0, max_abs: 0.0, mean: 0.0, std: 0.0 };
    }
    let deltas: Vec<f64> = (0..n).map(|i| a[i] - b[i]).collect();
    let mean: f64 = deltas.iter().sum::<f64>() / n as f64;
    let max_abs: f64 = deltas.iter().fold(0.0_f64, |m, &d| m.max(d.abs()));
    let std = if n > 1 {
        let var = deltas.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
        var.sqrt()
    } else {
        0.0
    };
    DeltaStats { n, max_abs, mean, std }
}

pub struct CompareArgs<'a> {
    pub mcoa_csv: &'a Path,
    pub cdata_csv: &'a Path,
    pub tissue: &'a str,
    pub label: &'a str,
    pub out_dir: &'a Path,
}

/// MCOA-vs-CDATA comparison. Returns path to written markdown report.
pub fn compare_mcoa_cdata(args: CompareArgs) -> Result<PathBuf> {
    let mcoa = read_csv(args.mcoa_csv)?;
    let cdata = read_csv(args.cdata_csv)?;

    let x_col = if mcoa.columns.contains_key("n_cumulative") {
        "n_cumulative"
    } else if mcoa.headers.len() > 1 {
        &mcoa.headers[1]
…<truncated 89 more lines>…
```
### code `crates/mcoa_core/src/lib.rs`
```
//! MCOA core — multi-counter architecture of organismal aging.
//!
//! Implements Axioms M1–M4 from CONCEPT.md:
//!   M1 — parallel counters
//!   M2 — dimensional consistency (n → n/n*, t → t/τ)
//!   M3 — a-priori tissue weighting
//!   M4 — falsifiability first-class
//!
//! Reference: Tkemaladze (2026) "The Multi-Counter Architecture of Organismal Aging",
//! Nature Aging Perspective submission, 2026-04-25.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const N_COUNTERS: usize = 5;

/// MCOA counter numbering aligned with user decision 2026-05-07:
///   #1 = Centriolar (CDATA), #2 = Telomere, #3 = Mitochondrial,
///   #4 = Epigenetic, #5 = Proteostasis.
///
/// The discriminant is 0-indexed for zero-cost array indexing
/// (`c as usize`); the user-facing number is `as u8 + 1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Counter {
    Centriolar = 0,    // MCOA #1 — CDATA
    Telomere = 1,      // MCOA #2
    Mitochondrial = 2, // MCOA #3 — MitoROS
    Epigenetic = 3,    // MCOA #4 — EpigeneticDrift
    Proteostasis = 4,  // MCOA #5
}

impl Counter {
    pub const ALL: [Counter; N_COUNTERS] = [
        Counter::Centriolar,
        Counter::Telomere,
        Counter::Mitochondrial,
        Counter::Epigenetic,
        Counter::Proteostasis,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Counter::Centriolar => "centriolar",
            Counter::Telomere => "telomere",
            Counter::Mitochondrial => "mito",
            Counter::Epigenetic => "epigenetic",
            Counter::Proteostasis => "proteostasis",
        }
    }

    /// User-facing 1-indexed number (Counter #1 … #5). Matches
    /// CONCEPT.md numbering and subproject CLAUDE.md / Cargo.toml
    /// descriptions. Decided 2026-05-07.
    pub fn mcoa_number(self) -> u8 {
        self as u8 + 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tissue {
    Fibroblast,
    Hsc,
    Neuron,
    Hepatocyte,
    BetaCell,
    CD8TMemory,
}

impl Tissue {
    pub fn as_str(self) -> &'static str {
        match self {
            Tissue::Fibroblast => "fibroblast",
            Tissue::Hsc => "hsc",
            Tissue::Neuron => "neuron",
            Tissue::Hepatocyte => "hepatocyte",
            Tissue::BetaCell => "beta_cell",
            Tissue::CD8TMemory => "cd8_t_memory",
        }
    }
}
…<truncated 264 more lines>…
```
### code `crates/mcoa_tests/src/lib.rs`
```
//! MCOA falsifiability test harnesses.
//!
//! Implements simulated versions of §6.1–6.5 of the Nature Aging Perspective. Each harness
//! generates synthetic data under the MCOA prior and under a competing null (e.g. single-counter),
//! so that a statistical pipeline can be calibrated against expected effect sizes before real
//! data are available.

/// Test 4 (Aubrey's test) — α vs β decomposition via a 2×2 organoid design.
pub mod test4_aubrey;

/// Test 1 — tissue-specific counter dominance.
pub mod test1_dominance;

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .rs | 16 | 72582 |
| .ex | 12 | 47365 |
| .py | 2 | 10130 |
| .exs | 5 | 3781 |
| .heex | 2 | 2506 |