# AUDIT PACKET — LC_CDATA

Path: `/home/oem/Desktop/LongevityCommon/CDATA`  Date: 2026-05-08

## Size & file counts
```
7,8M	/home/oem/Desktop/LongevityCommon/CDATA
```
**Extensions:** .rs=50, .toml=13, .md=10, .ex=9, .png=9, .exs=5, .py=5, .pdf=4, (noext)=3, .heex=2, .json=2, .sh=1, .yml=1, .example=1, .sql=1
## Tree (depth=2, max 200 entries)
```
.
./STATE.md
./frontend
./frontend/mix.exs
./frontend/lib
./frontend/config
./PARAMETERS.md
./Cargo.toml
./run.sh
./EVIDENCE.md
./crates
./crates/cell_dt_validation
./crates/cell_dt_modules
./crates/cell_dt_python
./crates/cell_dt_cli
./crates/cell_dt_gui
./crates/cell_dt_core
./DESIGN.md
./gui
./gui/cdata_gui.py
./README.md
./backend
./backend/migrations
./backend/Cargo.toml
./backend/src
./backend/README.md
./backend/Dockerfile
./scripts
./scripts/cdata_loocv.py
./scripts/cdata_sobol_ci.py
./scripts/cdata_ablation_sobol.py
./CLAUDE.md
./THEORY.md
./OPEN_PROBLEMS.md
./articles
./articles/First Direct Structural Evidence for Age-Dependent Polyglutamylation Asymmetry in Murine Hematopoietic Stem Cells.pdf
./articles/cdata_abstract_v2_improved.png
./articles/cdata_abstract_v8.png
./articles/cdata_abstract_v6.png
./articles/The Relapse Prediction.pdf
./articles/cdata_abstract_v7.png
./articles/cdata_abstract_v1_original.png
./articles/cdata_abstract_v5.png
./articles/The Centriolar Imperative.pdf
./articles/cdata_abstract_v3.png
./articles/cdata_abstract_v9.png
./articles/Asymmetric Inheritance of the Aged Mother Centriole by Stem Cells Is an Established Causal Fact.pdf
./articles/cdata_abstract_v4.png
./CONCEPT.md
./_archive
./_archive/PARAMS_RECONCILIATION_ANALYSIS_2026-04-21.md
./_archive/COUNTER_PARAMS_ADDITIVE_ANALYSIS_2026-04-21.md
./_archive/CONCEPT_CODE_AUDIT_2026-04-21.md
./LICENSE
```
## Detected stack: **Rust, Phoenix/Elixir**
## Core files

### `CLAUDE.md` (2646 chars)
```md
# CLAUDE.md — CDATA

**CDATA** (Centriolar Damage Accumulation Theory of Aging) — Counter в архитектуре MCOA. Постулирует накопление polyGlu-PTM на материнской центриоли как одну из канонических осей старения.

**Path:** `/home/oem/Desktop/LongevityCommon/CDATA/`
**Repo:** часть `djabbat/LongevityCommon` (private). Не отдельный repo.

---

## Source of truth

**`CDATA/CONCEPT.md`** v5.2 — авторитетный документ.
См. родительский `~/Desktop/LongevityCommon/CLAUDE.md` и `~/Desktop/LongevityCommon/MCOA/CONCEPT.md`.

---

## Counter numbering (унифицировано 2026-05-07)

CDATA = **Counter #1** (Centriolar) per user decision 2026-05-07. Все subscripts формул используют `_1` (`α₁`, `β₁`, `γ₁`, `n₁`, `τ₁`). Прежняя двойная маркировка #1/#2 в CONCEPT.md устранена.

---

## ⚠ Inviolable axioms

CDATA имеет три аксиомы (см. `CONCEPT.md § "АКСИОМЫ CDATA"`). **НЕ изменять без специальной команды.** Должны присутствовать во всех LOI/грантах/манускриптах.

---

## Status (2026-05-07)

- **C2 подтверждена** у двух типов клеток млекопитающих
- **Блокирующие барьеры:** C1 и C2 у HSC (нужны эксперименты)
- **In-sample R²(MCAI) = 0.745**; LOO-CV mean = -0.093 (требует исправления ROS-уравнения)
- **Sobol p = 0.12** — статистически inconclusive (см. корневой CONCEPT v5.6)
- ABL-2 Sobol S1 paradox — extended global sensitivity analysis в плане (target Aug 2026)

---

## Stack

- **Backend:** Rust workspace (`CDATA/backend/`, `CDATA/crates/`)
- **Frontend:** TypeScript (`CDATA/frontend/`)
- **GUI:** native (`CDATA/gui/`)
- **Articles:** `CDATA/articles/` (manuscripts, peer reviews)

---

## Server presence

`cdata.longevity.ge` → static landing `/var/www/cdata-landing/`. Backend на сервере не развёрнут. Roadmap план: либо развернуть рабочий backend, либо удалить nginx-конфиг.

---

## Co-PI / experimental partners (см. memory)

- **Geiger (Ulm)** — Phase B Co-PI, LoS signed 2026-04-23 (€100K, 18mo, conditional Phase A Go)
- **Janke (Curie)** — declined personally (CoI), introducing co-PI as real partner lab
- **HSC experiment** — Impetus LOI $75K в работе

---

## Правила разработки

1. Self-citation ≤15% во всех publications (`feedback_article_workflow`).
2. Bradford-Hill критерии обязательны для causal claims (`feedback_bradford_hill_rule`).
3. Все references проходят PubMed/Crossref верификацию (`feedback_verify_references`); **DeepSeek НЕ используется для lit-search** (галлюцинирует DOI/PMID).
4. Sync с MCOA: любое изменение D₁ kinetics → обновление MCOA/CONCEPT.md `Counter #1 row`.

---

## Тесты

`cargo test` в `CDATA/backend/` workspace, `pytest` для analysis scripts. Зелёный gate перед публикацией LOI/манускрипта.

```
### `README.md` (4597 chars)
```md
# CDATA — Centriolar Damage Accumulation Theory of Aging

**Статус:** Активная теория (Counter #1 в MCOA — согласовано с `cell_dt_cli::COUNTER_NUMBER = 1` и THEORY.md §6). TRL 3→4.
**Последнее обновление канона:** 2026-04-22 (CORRECTIONS).

## Краткое изложение

CDATA (Centriolar Damage Accumulation Theory of Aging) — это механистическая теория, объясняющая, почему стволовые клетки стареют и перестают функционировать даже при наличии активной теломеразы и низкого окислительного стресса. Теория постулирует, что ключевым ограничивающим фактором является не генетический или эпигенетический ущерб, а физическое накопление посттрансляционных модификаций (PTM), в первую очередь полиглутамилирования, на материнской центриоли.

Материнская центриоль — стабильная клеточная структура, преимущественно наследуемая стволовой дочерней клеткой и служащая основой для первичной реснички. Накопление PTM нарушает сигнализацию через ресничку (Hedgehog, Wnt), что ведёт к двум последствиям: 1) увеличению доли дифференцировочных, а не самообновляющих, делений; 2) замедлению самого темпа делений. Совокупный эффект — истощение пула функциональных стволовых клеток, что проявляется как репликативное старение (предел Хейфлика) *in vivo* и соответствует наблюдаемому ограничению в 4-5 генераций при серийной трансплантации гемопоэтических стволовых клеток (HSC).

CDATA формализована как Counter #2 в рамках мета-теории MCOA (Multi-Counter Architecture), где старение организма моделируется как сумма независимых, параллельно накапливающих повреждения "счётчиков". CDATA предоставляет первую количественную молекулярную основу для официального признака старения стволовых клеток — misorientation центросомы (Rando et al., Cell Stem Cell 2025).

## Ключевые утверждения (Аксиомы)

Теория стоит на трёх незыблемых аксиомах. Их изменение или удаление требует явной команды и пересмотра всего проекта.
1.  **Аксиома 1 (Hayflick в гипоксии с теломеразой):** Стволовые клетки в гипоксии с активной теломеразой всё равно достигают предела Хейфлика. Это указывает на необходимость структурного, счётно-делительного механизма.
2.  **Аксиома 2 (Дефектный сигналинг реснички):** PTM-нагруженная материнская центриоль ухудшает сигнализацию через первичную ресничку, нарушая восприятие нишевых сигналов самообновления.
3.  **Аксиома 3 (Замедление деления):** Темп деления стволовых клеток со старыми центриолями снижается.

## Навигация по документации

*   **Формальная теория:** Полная математическая формализация, деривация, предсказания — [`THEORY.md`](THEORY.md).
*   **Эмпирические основания:** Верифицированные литературные ссылки (PMID/DOI), внутренние данные и опровергающие свидетельства — [`EVIDENCE.md`](EVIDENCE.md).
*   **Неразрешённые вопросы:** Список открытых проблем с тестами на фальсификацию и приоритетами — [`OPEN_PROBLEMS.md`](OPEN_PROBLEMS.md).
*   **Количественные параметры:** Таблица всех 32 параметров модели с источниками, единицами и статусом — [`PARAMETERS.md`](PARAMETERS.md).
*   **Архитектура реализации:** Структура кода (Cell-DT), API и деревья файлов — [`DESIGN.md`](DESIGN.md).
*   **Инструкции для ИИ-агентов:** Жёсткие правила, ограничения безопасности и контекст для LLM — [`AGENTS.md`](AGENTS.md).
*   **Журнал изменений:** Хронологическая история решений и их обоснование — [`JOURNAL.md`](JOURNAL.md).
*   **Дорожная карта:** Планы по развитию, приоритеты и зависимости — [`ROADMAP.md`](ROADMAP.md).

## Корректировки (Канон 2026-04-22)

Все документы CDATA следуют единому источнику истины: **CORRECTIONS_2026-04-22**. Ключевые изменения:
*   **Формула Health Score удалена.** Веса не имели математического обоснования из MCOA.
*   **χ_Ze — теоретический конструкт, а не валидированный биомаркер.** Утверждение R²=0.84 получено на синтетических данных и отозвано.
*   **MCOA Test 2 не является источником параметра γ_i.** По умолчанию γ_i = 0 (гипотеза независимости).
*   **Скорость `v` в Ze-теории канонизирована как `v = N_S / (N − 1)`.**

## Текущий статус и ближайшие шаги

Теория прошла несколько раундов ревью. Механизм асимметричного наследования центриоли (C2) подтверждён на двух типах клеток млекопитающих. Главный текущий барьер — отсутствие прямых данных о накоплении PTM пропорционально числу делений (C1) и асимметрии наследования у гемопоэтических стволовых клеток (HSC). Эти пробелы являются предметом активной экспериментальной валидации в рамках дорожной карты.

CDATA позиционируется как основа для подачи заявки в EIC Pathfinder (дедлайн 2026-05-12) и является ядром для разработки первого класса геропротекторов, нацеленных на деглутамилазную активность.
```
### `backend/README.md` (3185 chars)
```md
# CDATA Backend

Centriolar Damage Accumulation Theory of Aging backend implementation for LongevityCommon project.

## Overview

This is the production-grade Axum backend for the CDATA subproject, implementing:
- Full CRUD operations for all domain entities
- PostgreSQL database with SQLx
- RESTful API endpoints
- Proper error handling and tracing
- Docker containerization

## Architecture

### Domain Entities

1. **Parameter** - Quantitative parameters from PARAMETERS.md with γ_i = 0 default
2. **Counter** - MCOA counter registry (α_i, β_i, γ_i kinetics)
3. **CdataCounter** - CDATA-specific extension (Hayflick limit, D_crit, rescue half-life)
4. **Tissue** - Tissue types and weights for MCOA
5. **TransplantArm** - HSC transplant arm tracking
6. **SensitivityAnalysis** - Sobol sensitivity storage
7. **McoaComputation** - L_tissue computation results
8. **FclcData** - Privacy budget (ε) and secure aggregation
9. **BiosenseData** - Raw EEG/HRV upload (NO χ_Ze computation)
10. **ScaffoldCounter** - Telomere/MitoROS/EpigeneticDrift/Proteostasis time-series
11. **HapData** - Hepatic+affective joint biomarkers
12. **OntogenesisMilestone** - 0-25 year developmental milestones

### Database

PostgreSQL with:
- UUID primary keys
- Automatic timestamps (created_at, updated_at)
- Proper indices and constraints
- Enum types for status fields
- JSONB for flexible data storage

## Getting Started

### Prerequisites

- Rust 1.75+ (2021 edition)
- PostgreSQL 15+
- Docker (optional)

### Environment Setup

1. Copy `.env.example` to `.env`:
```bash
cp .env.example .env
```

2. Update `.env` with your configuration:
```bash
ENVIRONMENT=development
PORT=3003
DATABASE_URL=postgres://cn:cn@localhost/cdata_db
LOG_LEVEL=debug
```

### Database Setup

1. Create database:
```bash
createdb cdata_db
```

2. Run migrations:
```bash
sqlx database create
sqlx migrate run
```

### Running Locally

```bash
cargo run
```

Server will start at `http://localhost:3003`

### API Endpoints

- `GET /health` - Health check
- `GET /parameters` - List all parameters
- `POST /parameters` - Create new parameter
- `GET /parameters/:id` - Get parameter by ID
- `PUT /parameters/:id` - Update parameter
- `DELETE /parameters/:id` - Delete parameter

Similar endpoints for all other entities.

### Running with Docker

```bash
docker build -t cdata-backend .
docker run -p 3003:3003 --env-file .env cdata-backend
```

## Development

### Testing

```bash
cargo test
```

### Database Migrations

Create new migration:
```bash
sqlx migrate add -r descriptive_name
```

Run migrations:
```bash
sqlx migrate run
```

### Code Style

- Follow Rust 2021 edition conventions
- Use `tracing` for logging
- Proper error handling with `thiserror`
- Input validation with `validator`
- SQLx for type-safe database queries

## Deployment

### Production Considerations

1. Set `ENVIRONMENT=production`
2. Use proper database connection pooling
3. Configure CORS appropriately
4. Enable request/response logging
5. Set up monitoring and alerting

### Health Checks

- `GET /health` - Basic service health
- Database connectivity is validated at startup
- Graceful shutdown on SIGTERM/SIGINT

## License

MIT
```
### `CONCEPT.md` (135028 chars)
```md
# CDATA — Concept v5.2 — Counter #1 (Centriolar) in MCOA

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


## Родительская теория

**CDATA является одним из треков MCOA** — Multi-Counter Architecture of Organismal Aging (Tkemaladze J., 2026, Nature Aging submission). MCOA = flagship meta-теория экосистемы LongevityCommon; организменное старение формализуется как взвешенная сумма параллельных damage-accumulation counter'ов D_i, каждый с собственной division-зависимой (α_i) и time-зависимой (β_i) кинетикой.

**CDATA = MCOA Counter #1 (Centriolar):** накопление polyGlu PTM на материнской центриоли → D_centriole(n, t) = D_centriole,0 + α_1·(n/n₁\*) + β_1·(t/τ_1) + γ_1·I(others).

Этот документ описывает механистические детали Counter #1; для общей архитектуры см. `~/Desktop/LongevityCommon/MCOA/CONCEPT.md`.

## Centriolar Damage Accumulation Theory of Aging (механизм Counter #1)

**Версия:** 5.2 (Counter #1 framing, унифицировано 2026-05-07; numbering decision: CDATA = #1, Telomere = #2, MitoROS = #3, EpigeneticDrift = #4, Proteostasis = #5); механистическое содержание v5.1 (Обновлена 2026-04-15: ATF5-PGT-PCNT механизм (Madarampalli 2015) (PMID 26213385); Bobinnec 1998 — GT335 инъекция = полная потеря центриоли + восстановление через de novo синтез (поддерживает ¬R); Pan 2025 (Cells Tissues Organs) — CCP1-дефицит → укороченные цилии в BMSC → нарушение остеогенной дифференцировки → восстановление CCP1 обращает эффект; формализованная P11 N_relapse = (P_crit − P₀)/α; количественные предсказания CDATA: rescue half-life ~40–60 делений, full relapse ~80–120 делений; CellTrace Violet + TTLL6 siRNA/LDC10 как контроли P11; Asymmetry Index AI = MFI(Ninein+)/MFI(Ninein−); BHCA прогноз 22/27 при подтверждении всех 5 предсказаний Phase 0)
**Дата:** 2026-04-15
**Статус:** C2 подтверждена у млекопитающих (2 клеточных типа); блокирующий барьер — C1 и C2 у HSC; in-sample R²(MCAI)=0.745; LOO-CV mean=-0.093 (требует исправления ROS-уравнения); готов для Longevity Impetus LOI (дедлайн 2026-04-25) и EIC Pathfinder (2026-05-12)

---

## ⚠️ АКСИОМЫ CDATA — НЕ ИЗМЕНЯТЬ БЕЗ СПЕЦИАЛЬНОЙ КОМАНДЫ

> **Эти три утверждения являются фундаментом теории CDATA. Они не подлежат изменению, пересмотру или удалению без явной команды пользователя. Они должны присутствовать во всех LOI, грантах, статьях и публичных материалах CDATA.**

### АКСИОМА 1 — Hayflick в гипоксии с теломеразой
> **Стволовые клетки, находящиеся в гипоксической среде при активной теломеразе, всё равно достигают предела Хейфлика.**
>
> *Механизм:* Классические защитные механизмы (низкий ROS, активная теломераза) недостаточны для предотвращения репликативного истощения. Это устанавливает необходимость структурного, счётно-делительного механизма — каковым является накопление PTM на материнской центриоли.
>
> *Ключевые ссылки:* Harrison & Astle, JEM 1982; Allsopp et al., JEM 2003; Spencer et al., Cell Stem Cell 2020.

### АКСИОМА 2 — Дефектный сигналинг реснички из старой материнской центриоли
> **Материнская центриоль, преимущественно наследуемая стволевой дочерней клеткой, является базальным телом первичной реснички. Накопление PTM (прежде всего полиглутамилирование) на стареющей материнской центриоли ухудшает сигналинг через ресничку (Hedgehog, Wnt/PCP, Notch), нарушая восприятие нишевых сигналов самообновления.**
>
> *Механизм:* TTLL6 > CCP1 в гемопоэтической ткани → накопление polyGlu на материнской центриоли → нарушение IFT и структуры аксонемы → дефектная ресничка → снижение Shh/Wnt-сигналинга → сдвиг к симметричным дифференцировочным делениям.
>
> *Ключевые ссылки:* Whitfield et al., Cell Reports 2016; Gao et al., Nature 2009; Mukhopadhyay et al., Nat Cell Biol 2017; Pimenta-Marques et al., Science 2023.

### АКСИОМА 3 — Снижение темпа деления стволовых клеток со старой центриолью
> **Со временем темп деления стволовых клеток с преимущественно унаследованными старыми (PTM-нагруженными) центриолями снижается.** Hh/Wnt-сигналинг через ресничку поддерживает переход G1→S; нарушение цилиарного сигналинга удлиняет межделительный интервал. Замедление делений + увеличение доли симметричных дифференцировочных делений = суммарное истощение пула, соответствующее наблюдаемому пределу 4–5 генераций при серийной трансплантации.
>
> *Ключевые ссылки:* Wilson et al., Nature 2008 (2–3× снижение в старых HSC); Kowalczyk et al., Cell Stem Cell 2015 (углублённый покой, удлинённый цикл).

---

### Hallmark Recognition (открывающая цитата для всех рукописей и грантов — добавлено v4.8)

> **«Centrosome misorientation is an officially recognized hallmark of stem cell aging»** (Rando, Brunet & Goodell, *Cell Stem Cell* 2025). CDATA provides the first quantitative molecular mechanism for this hallmark: PTM accumulation in the mother centriole is the upstream driver of centrosome misorientation, and the only ¬R-candidate satisfying conditions C1+C2+C3. This positions CDATA as the mechanistic backbone of the newest stem cell aging hallmark.

### Impact Statement (включать в начало каждой рукописи и гранта)

> **Все существующие клинические инструменты биологического возраста (GrimAge, DunedinPACE, PhenoAge) — диагностические, не терапевтические. Они отвечают на вопрос «насколько быстро вы стареете?», но не на вопрос «что именно остановить?». CDATA — единственная теория, дающая ответ на второй вопрос через логическую необходимость: PTM-деградацию центриолей. Это превращает CDATA из академической теории в roadmap для разработки первого класса anti-aging drugs с конкретной молекулярной мишенью — деглутамилазами — которые не были бы идентифицированы никаким эмпирическим скринингом, поскольку не входят ни в один из 12 Hallmarks of Aging и не предсказываются ни одной существующей теорией старения.**

### Расширенное название теории

**Полное механистическое название** (для рукописей и грантов):
«**Asymmetric Centriolar Damage Accumulation Theory of Aging (ACDATA)**»

Аббревиатура CDATA сохраняется для совместимости с PMID 36583780. В тексте использовать:
«CDATA (Centriolar Damage Accumulation through Asymmetric Inheritance)» — это немедленно сигнализирует рецензенту, что механистическая уникальность — в асимметрии наследования, а не просто «накоплении».

---

### Этика и данные
> **Статус этики:** Cell-DT v3.0 — полностью *in silico* симулятор. **Реальных пациентов нет.** Все данные — публично доступные биомаркерные траектории из опубликованных когортных исследований (NHANES; Jaiswal 2017 PMID 28636844; Horvath 2013 PMID 24138928; Shay & Wright 2000; Dultz 2008 PMID 18316408). Этическое одобрение для симуляции не требуется. Для планируемого экспериментального теста CEP135 (Тест P6) потребуется одобрение IRB/IACUC перед началом работы с клеточными линиями или животными моделями.

---

## Executive Summary

CDATA (Centriolar Damage Accumulation Theory of Aging) — механистическая теория старения, объясняющая деградацию организма как неизбежное следствие накопления PTM-повреждений в материнских центриолях стволовых клеток.

После 7 раундов жёсткого peer review концепция достигла:
- **32 параметра** (редуцировано со 120 — см. Model Selection ниже)
- **8 ключевых механизмов** (с реальными PMID для 23 из 32 параметров)
- **R²(MCAI)=0.745, R²(CHIP)=0.611, R²(Telo)=0.465** (in-sample cross-sectional fit на реальных литературных данных); ⚠️ R²=0.84 было получено на **синтетических данных** (`null_model_r2.py`) и **изъято из всей грантовой документации** с 2026-04-13
- **TRL 3→4** позиционирование
- **10 фальсифицируемых предсказаний** (P1–P6 + P7–P10, добавлены v4.9 по итогам мета-анализа)

> ✅ **Статус C2 обновлён (v4.6):** C2 (асимметричное наследование материнской центросомы) прямо продемонстрирована в двух независимых системах млекопитающих: нейральные прогениторы человека (Royall et al. 2023, eLife, ~80% стволевых дочерей) и CD8+ Т-лимфоциты мыши (Barandun & Oxenius 2025, Cell Reports, >90% первых делений). Ninein идентифицирован как молекулярный медиатор направленного наследования в обеих системах.
>
> ⚠️ **Оставшийся блокирующий барьер для Aging Cell:** Отсутствие данных C1 (PTM ∝ числу делений) и C2 у HSC (кровь). PTM-статус наследуемой центриоли не измерен ни в одном исследовании. Барьер устраним за ~$75K.

---

## Advances beyond Tkemaladze 2023 (PMID: 36583780)

Статья Tkemaladze 2023 сформулировала центральный тезис CDATA качественно. CDATA v4.3 добавляет:

| Аспект | Tkemaladze 2023 | CDATA v4.3 |
|--------|-----------------|------------|
| Формализация | Качественная теория | ODE-система, 32 параметра, MCMC калибровка |
| Sobol analysis | Нет | nu (S1=0.416) и alpha (S1=0.193) как доминанты |
| Тканевая специфика | Упомянута | 4 ткани (HSC/ISC/Muscle/Neural) с отдельными ν, β, τ |
| Математическое доказательство | Нет | ¬R-аргумент + три достаточных условия C1–C3 |
| CHIP-дрейф | Нет | DNMT3A/TET2, 5-й компонент frailty (0.05 × chip_vaf) |
| Hypoxia module | Качественно | O₂-зависимый mito_shield + верификация Peters-Hall 2020 |
| Эксперим. предсказания | 1 общее | 10 конкретных предсказаний P1–P10 (P7–P10 добавлены v4.9) |
| Конкурентный анализ | Нет | ¬R матрица vs. NPC, lipofuscin, базальные тела |
| Код | Нет | Cell-DT v3.0 Rust (DOI: 10.5281/zenodo.19174506), 483 теста |

### Model Selection: от 120 к 32 параметрам

**Вопрос рецензента (Aging Cell v7):** "Как отбирались 32 параметра из 120?"

**Метод редукции (документировано для публикации):**

| Этап | Критерий | Удалено | Осталось |
|------|----------|---------|----------|
| 1. Идентифицируемость | Posterior SD > 50% posterior mean (MH-MCMC pilot=1000) | 51 | 69 |
| 2. Чувствительность | Sensitivity index S_i = Var(Y\|X_i)/Var(Y) < 0.01 (Morris screening) | 24 | 45 |
| 3. Биологическая обоснованность | Параметры без PMID-подтверждения удалены | 13 | 32 |
| **Итого** | | **88** | **32** |

**Sobol sensitivity analysis (N=16384, Saltelli quasi-MC, bootstrap 95% CI — 2026-04-13) ✅ S4 CLOSED:**

| Ранг | Параметр | S1 | 95% CI | ST | Вывод |
|------|----------|----|--------|----|-------|
| 1 | **epigenetic_rate** | **0.403** | [0.389–0.416] | 0.408 | DOMINANT |
| 2 | **alpha** (повреждение/деление) | **0.224** | [0.215–0.233] | 0.259 | DOMINANT |
| 3 | **nu_HSC** (скорость деления) | **0.155** | [0.145–0.164] | 0.184 | DOMINANT |
| 4 | epigenetic_stress_k | 0.071 | [0.065–0.078] | 0.087 | Moderate |
| 5 | tau_protection | 0.046 | [0.042–0.051] | 0.058 | Moderate |
| 6 | beta_HSC | 0.025 | [0.021–0.028] | 0.031 | Moderate |
| 7 | pi_base | 0.015 | [0.012–0.019] | 0.019 | Low |
| 8 | pi_0 | 0.013 | [0.010–0.015] | 0.017 | Low |
| 9–32 | 24 параметра | <0.001 | CI подтверждают <0.010 | <0.002 | Negligible ✓ |

**S1_sum (топ-3) = 0.782.** Доминируют: epigenetic clock rate, base damage per division, и HSC division rate.

**✅ S4 CLOSED (2026-04-13):** N=16384, bootstrap CI = SALib conf_level=0.95. Утверждения о negligible параметрах теперь статистически валидны: 23 параметра имеют S1<0.001 AND ST<0.010, подтверждённые 95% CI.

**Обновлённый ключевой вывод:** Три параметра (epigenetic_rate, alpha, nu_HSC) объясняют 78% дисперсии выходного D(50). alpha и nu_HSC **прямо поддерживают CDATA-тезис**: скорость делений × повреждение за деление = главные физические детерминанты накопления повреждений. Доминирование epigenetic_rate обусловлено прямой линейной зависимостью ep_age = ep_rate × T в аналитическом приближении; в полной Rust-ODE вес эпигенетики ожидается меньше.

**✅ Ablation Sobol (v4.7, N=8192, 2026-04-13) — РАЗРЕШАЕТ СОБОЛЬ-ПАРАДОКС (NMC-2):**

| Группа параметров | S1_sum (FULL model) |
|------------------|---------------------|
| **Centriolar** (alpha, nu, beta, tau, pi_0, pi_base) | **0.471** |
| **Epigenetic** (ep_rate, ep_stress_k) | **0.470** |

**При epigenetic_rate=0:** S1(alpha)=0.362 → DOMINANT. Centriolar параметры доминируют без линейного ep-слагаемого.

**Вывод:** Индивидуальное доминирование epigenetic_rate (S1=0.402) отражает линейную аддитивность в аналитическом приближении. Как группа — centriolar ≈ epigenetic (0.471 vs 0.470). CDATA-название «Centriolar Damage Accumulation» обосновано на групповом уровне.

**⚠️ Ablation R² (новая находка v4.7):**
- FULL model: R²=0.778 на MCAI-траектории
- ABL-1 (ep_rate=0, только центриоль): R²=0.579
- ABL-2 (alpha=0, только эпигенетика): **R²=0.833 > FULL**

Центриолярный компонент в текущей спецификации **снижает** предсказательную точность относительно чисто эпигенетической модели. Причина: ep_rate × T — линейная функция, идеально совпадающая с линейным MCAI-трендом; нелинейная D(t) вносит систематическое смещение.

**✅ P0b — Решение (механистическая интеграция, Cell-DT v4.0):**

Вместо аддитивного добавления epigenetic_rate как независимого параметра, спецификация должна быть:

```
ep_age(t) = ∫₀ᵗ [ep_rate_base + k_ep × D(τ)] dτ
           = ep_rate_base × t + k_ep × ∫₀ᵗ D(τ) dτ
```

где `ep_rate_base` — базовая скорость эпигенетического дрейфа (дивизион-независимая компонента), `k_ep` — коэффициент усиления D(t)→эпигенетика. Это устраняет ep_rate как независимый параметр (→ ep_rate_base + k_ep×D, не ep_rate), связывает ep-компоненту с alpha через D(t), и позволяет alpha однозначно доминировать в Sobol-анализе. ABL-2 парадокс исчезает: при alpha=0 → D(t)≈0 → ep_age ≈ ep_rate_base × t (минимальный дрейф). Реализация запланирована в Cell-DT v4.0 перед Aging Cell.

**⚠️ LOO-CV валидация (v4.7, реальные литературные данные):**

| Биомаркер | R² (full) | R²_loo |
|-----------|----------|--------|
| ROS (Franceschi 2000) | -0.512 | -0.682 |
| CHIP (Jaiswal 2017) | 0.611 | 0.611 |
| MCAI (Rockwood/Franceschi) | 0.745 | -0.765 |
| Telo (Lansdorp 2005) | 0.465 | 0.465 |
| **Mean** | **0.327** | **-0.093** |

**Значение:** In-sample R²=0.327 (not 0.84; см. ниже). LOO-CV mean=-0.093 → модель переобучена на 28 точках (4 биомаркера × 7 возрастных точек). R²(ROS)=-0.512 указывает на модельную спецификацию ROS-компонента при pi_0=0.99 (граница оптимизации → признак нeидентифицируемости). Требуется исправление ROS-уравнения и расширение калибровочного датасета.

**Разрыв R²=0.84 vs R²=0.327 — диагноз:**
`null_model_r2.py` (источник цифры 0.84) использует **SIMULATED data** («currently uses synthetic data calibrated to CDATA parameters» — явно в комментарии скрипта). R²=0.84 на синтетических данных нельзя цитировать как валидацию. Реальный in-sample fit на литературных данных: R²(MCAI)=0.745, R²(CHIP)=0.611, R²(Telo)=0.465; R²(ROS)=-0.512 (ошибка спецификации). **Для грантов использовать:** «in-sample cross-sectional fit: R²(MCAI)=0.745, R²(CHIP)=0.611, R²(Telo)=0.465; ROS component requires model revision (see §Limitations). Independent hold-out validation (UK Biobank/InCHIANTI) — planned as Aim 3.»

**Ограничение (для Aging Cell §Limitations):** «Sobol analysis employed a vectorized analytic model approximation; epigenetic_rate sensitivity (S1=0.403) may be overestimated relative to the full Cell-DT v3.0 ODE. Group ablation analysis (N=8,192) confirms centriolar parameter group dominance (S1_sum=0.471) approximately equal to epigenetic group (S1_sum=0.470), with centriolar parameters becoming clearly dominant when epigenetic_rate=0. Mechanistic integration of D(t)→epigenetic_rate is planned for Cell-DT v4.0.»

---

## Центральный тезис

**Старение организма — неизбежное следствие накопления повреждений в материнских центриолях стволовых клеток, скорость которого определяется произведением скорости деления и эффективности защитных механизмов молодости.**

### Дедуктивно ограниченный аргумент для первичности центриоли (deductively constrained argument — не стандартная эмпирическая гипотеза)

```
ПОСЫЛКА 1:
  Если теломерная теория старения полна →
  hTERT-сверхэкспрессия + гипоксия (1–3% O₂) = бесконечная пролиферация стволевых клеток

ПОСЫЛКА 2:
  Если ROS-теория старения полна →
  [O₂]=1–3% (минимальный ROS) + активная теломераза = бесконечная пролиферация

ЭКСПЕРИМЕНТАЛЬНЫЙ ФАКТ:
  Прогениторные клетки с hTERT в условиях 2% O₂ достигают предела ~200 PD
…<truncated 1340 more lines>…
```
### `THEORY.md` (8280 chars)
```md
# CDATA — Формальная теория

**Версия:** 5.2 (Counter #1 в MCOA — см. §6 и `cell_dt_cli::COUNTER_NUMBER`)
**Статус:** Активная, подготовка к публикации.
**Канон:** CORRECTIONS_2026-04-22

## 1. Родительская рамка: MCOA (Multi-Counter Architecture)

CDATA формализована как Counter #1 (Centriolar) в мета-теории MCOA (Tkemaladze J., 2026, *Nature Aging* submission). MCOA постулирует, что старение организма `L_organism(t)` есть взвешенная сумма `K` параллельных, тканеспецифичных счётчиков повреждений:

`L_organism(t) = Σ_{ткани} w_ткань · [ Σ_{i=1..K} w_i(ткань) · f_i( D_i(n_ткань, t) ) ]`

где:
*   `D_i(n, t)` — уровень повреждения i-го счётчика после `n` делений за время `t`.
*   `f_i()` — функция воздействия повреждения на фенотип (например, линейная, пороговая).
*   `w` — калибруемые веса, определяемые из данных, а не априорно.

CDATA предоставляет механистическую модель для `D_centriole(n, t)`.

## 2. Аксиоматическая основа

Теория построена на трёх аксиомах, выведенных из эмпирических наблюдений и имеющих статус логической необходимости.

### **Аксиома 1. Неустранимость предела Хейфлика**
`∀ S ∈ StemCells: (Environment(S) = Hypoxia) ∧ (Telomerase(S) = Active) ⇒ ∃ n_max: ReplicativeLimit(S, n_max)`

**Интерпретация:** Даже в оптимальных условиях (гипоксия, активная теломераз) стволовые клетки достигают предела делений. Это исключает теломерное и ROS-обусловленное истощение как единственные причины, требуя дополнительного, структурного механизма, зависящего от числа делений `n`.

### **Аксиома 2. PTM-зависимая дисфункция реснички**
`PTM(MotherCentriole) ∝ n` и `CiliarySignal(Shh, Wnt | PTM) ↘`

**Интерпретация:** Накопление полиглутамилирования (polyGlu) на материнской центриоли, являющейся базальным телом первичной реснички, ухудшает внутриресничный транспорт (IFT) и последующую передачу сигналов Hedgehog и Wnt/PCP. Эти пути критичны для принятия решения о самообновлении в нише.

### **Аксиома 3. Замедление клеточного цикла**
`CellCycleDuration(StemCell | PTM) ↗`

**Интерпретация:** Нарушенный цилиарный сигналинг удлиняет G1-фазу, увеличивая межделительный интервал. Это, в сочетании со сдвигом в сторону дифференцировочных делений, приводит к нетто-истощению пула.

## 3. Математическая модель Counter #1

Повреждение центриоли `D_c` для клетки, совершившей `n` делений за время `t`, моделируется как:

`D_c(n, t) = D_{c,0} + α · (n / n*) + β · (t / τ) + γ · I(other counters)`

где:
*   `D_{c,0}` — базальный уровень повреждения (например, при рождении).
*   `α` (alpha) — прирост повреждения за одно асимметричное деление. Единица: [damage/division]. Первичный параметр CDATA.
*   `n*` — масштабирующий коэффициент (опорное число делений).
*   `β` (beta) — прирост повреждения со временем, независимо от делений (например, спонтанное химическое повреждение). Единица: [damage/time].
*   `τ` (tau) — временная константа (напр., характерное время реакции).
*   `γ` (gamma) — сила связи с другими счётчиками MCOA. **По канону CORRECTIONS-2026-04-22: по умолчанию γ = 0 (гипотеза независимости).** Ненулевые значения возникают только при статистическом отвержении независимости на реальных данных.

### 3.1. Тканевая спецификация
Параметры `α, β, ν` (частота делений) специфичны для ткани. Для ткани `j`:
`D_c_j(n_j, t) = D_{c,0} + α_j · (n_j / n*) + β_j · (t / τ_j)`

### 3.2. Динамика популяции (Cell-DT модель)
Модель имитирует популяцию стволовых клеток:
1.  **Асимметричное наследование:** При делении стволовая клетка с вероятностью `p_async` передаёт старую (PTM+) материнскую центриоль стволовой дочери, а новую — дифференцирующейся.
2.  **Вероятность самообновления `P_self`:** Зависит от уровня сигналинга `S`, который, в свою очередь, обратно зависит от `D_c`:
    `S(D_c) = S_max / (1 + exp( (D_c - D_half) / k_s ))`
    `P_self(D_c) = π_0 + (π_base - π_0) · S(D_c) / S_max`
    где `π_0` — базовая вероятность при нулевом сигнале, `π_base` — при максимальном.
3.  **Время генерации `T_gen`:** Увеличивается с ростом `D_c`:
    `T_gen(D_c) = T_gen_0 · (1 + η · D_c)`
4.  **Исход:** Клетка может: а) самообновиться (делиться дальше), б) дифференцироваться (выход из пула), в) войти в сенесцентное состояние (при `D_c > D_senescence`).

## 4. Ключевые предсказания (P1-P10)

Теория генерирует фальсифицируемые количественные предсказания.

**P1 (Накопление PTM):** Уровень полиглутамилирования (GT335 сигнал) на материнской центриоли будет положительно коррелировать с числом делений клетки-предшественницы *in vivo* (`r_spearman > 0.6, p < 0.01`).

**P2 (Асимметрия у HSC):** При делении гемопоэтических стволовых клеток (HSC) старая материнская центриоль будет наследоваться стволовой дочерью с вероятностью >70% (измеряется по Ninein или CEP170).

**P3 (Сигналинг):** HSC с унаследованной старой центриолью будут показывать сниженную активацию нижележащих эффекторов Shh (Gli1) и Wnt (β-catenin) на 40-60% по сравнению с сиблингами, получившими новую центриоль.

**P4 (Исход деления):** Доля симметричных дифференцировочных делений будет положительно коррелировать с уровнем PTM на материнской центриоли в популяции HSC.

**P5 (Темп деления):** Межделительный интервал HSC, отслеживаемый *in vivo*, будет увеличиваться с возрастом животного, и этот рост будет коррелировать с накоплением центриолярного PTM.

**P6 (Генетическое вмешательство):** Нокаут гена деглутамилазы CCP1 в HSC мыши приведёт к ускоренному накоплению центриолярного polyGlu и преждевременному истощению пула HSC при серийной трансплантации (сокращение числа успешных генераций с 4-5 до 2-3).

**P7 (Фармакологическое вмешательство):** Ингибирование полиглутамилазы TTLL6 улучшит функцию старых HSC в трансплантационных assays.

**P8 (Корреляция с эпиклоком):** В когортах людей скорость эпигенетического старения (DunedinPACE) будет положительно коррелировать с уровнем сывороточного биомаркера, ассоциированного с цилиарной дисфункцией (например, уровень экстрацеллюлярных везикул, несущих дефектные компоненты ресничек).

**P9 (Восстановление):** Оверэкспрессия CCP1 в старых HSC восстановит цилиарный морфологию, улучшит сигналинг и частично восстановит функциональность в трансплантационных тестах.

**P10 (Специфичность):** Эффекты, описанные в P1-P5, не будут воспроизводиться при индукции сопоставимого уровня окислительного повреждения в цитоплазме, демонстрируя специфичность центриолярного механизма.

### 4.1. ABL-2 / Sobol-парадокс (честное disclosure)

Глобальный sensitivity analysis (Sobol, N=16384, аналитическое приближение) даёт `S1(epigenetic_rate) = 0.403 > S1(alpha) = 0.224`; ablation с обнулённым `epigenetic_rate` улучшает R². Это зафиксировано как центральная задача WP3 и описано в `OPEN_PROBLEMS.md` OP3 (тест FT3.1 — полный ODE Sobol на Cell-DT). Исход теста — одна из четырёх возможностей (Validated / Correlational / Downstream / Null), все публикабельны. Полная политика: `CORRECTIONS_2026-04-22.md` §1.6 и §2.2.

## 5. Доказательство ¬R (Не-Ремонт)

CDATA удовлетворяет трём критериям для признания её механизма кандидатом в ¬R (Non-Repairable Damage) в терминах теории неремонтируемого накопления:
1.  **C1 (Линейное накопление):** Повреждение (PTM) накапливается пропорционально числу делений (`α · n`).
2.  **C2 (Асимметричное наследование):** Повреждение преимущественно передаётся стволовой линии.
3.  **C3 (Функциональный ущерб):** Повреждение напрямую ухудшает ключевую функцию клетки (сигналинг самообновления).

Ни один другой известный кандидат (ядерные поры, липофусцин, митохондрии) не удовлетворяет всем трём условиям одновременно для взрослых соматических стволовых клеток.

## 6. Связь с другими счётчиками MCOA

CDATA (Counter #1) считается ранним, инициирующим счётчиком, особенно для высокообновляемых тканей. Его выход `D_c(n,t)` может служить входом или модулятором для других счётчиков:
*   **Эпигенетический дрейф (Counter #?):** Хроническое нарушение сигналинга может влиять на паттерны метилирования ДНК в нише.
*   **Соматические мутации/CHIP (Counter #?):** Увеличенный межделительный интервал может изменять паттерны накопления мутаций.

Согласно CORRECTIONS-2026-04-22, сила связи `γ` между счётчиками считается нулевой до тех пор, пока не будет доказано обратное на реальных данных. Планируемый MCOA Test 2 предназначен для измерения таких связей, но не может быть источником априорных значений `γ`.
```
### `PARAMETERS.md` (9899 chars)
```md
# CDATA — Параметры модели

**Версия модели:** Cell-DT v3.0 (32 параметра)
**Дата калибровки:** 2026-04-10
**Канон:** CORRECTIONS_2026-04-22 (γ_i = 0 по умолчанию)

> ✅ **ALL 5 PARAMETER DIVERGENCES RESOLVED 2026-04-21** — см. `PARAMS_RECONCILIATION_ANALYSIS_2026-04-21.md` для полного анализа.
>
> | Параметр | Prior docs value | Resolved value | Resolution path |
> |----------|-------------------|-----------------|------------------|
> | α (α_HSC) | 0.028 | **0.0082** | (b) docs → code; MCMC posterior (PMID 36583780 concept only, no α published) |
> | ν_HSC | 1.2 /year | **1.2 /year** | (a) code 12.0 → 1.2 (Wilson 2008 standard); parameter insensitive (ΔR²≈0 at ±20%), safe change |
> | β_HSC | 0.005 | **dual-form documented**: 1.0 multiplicative (dead field), 0.005 additive `cell_dt_cli::CounterParams` |
> | π (signal-dep vs age-decay) | 0.65 `pi_base` + `D_half` + `k_s` | **age-decay model documented**: `pi_0=0.87`, `pi_baseline=0.10`, `tau_protection=24.3`. Signal-dep model deprecated (never implemented) |
> | τ_prot | 15 years | **24.3 years** | (b) docs → code; Round-7 MCMC posterior (free parameter) |
>
> **Следствие:** таблица ниже **теперь match code** для всех активных параметров. Bonus finding: fixed 6 locations of fabricated Jaiswal 2017 PMID 28792876 → correct 28636844 across CDATA Rust modules (same DeepSeek hallucination pattern documented in `feedback_deepseek_no_citations`).
>
> **Также:** `cell_dt_cli::CounterParams` hosts a **third parameter set** (α=0.60, β=0.15, τ=30yr) for the MCOA additive damage form — orthogonal to the multiplicative AgingEngine; annotated but out-of-scope for current reconciliation.

Следующая таблица содержит все 32 параметра модели CDATA, оставшиеся после редукции с 120 (см. Model Selection в `CONCEPT.md`). Параметры сгруппированы по модулям. `S1` — индекс чувствительности первого порядка из Sobol analysis (N=16384).

| Модуль | Имя параметра | Символ | Описание | Единицы | Значение (оценка) | 95% CI/Диапазон | Источник (PMID/DOI) | Статус | S1 (ранг) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Core Centriolar** | `alpha_HSC` | α_HSC | Прирост повреждения центриоли за деление (HSC) | damage/division | **0.0082** | [0.006, 0.011] | Round-7 MCMC posterior (`calibration.rs`); conceptual framework in PMID 36583780 | **Fitted** (docs updated 2026-04-21 → code post-calibration value) | 0.224 (2) |
| | `nu_HSC` | ν_HSC | Базовая частота делений HSC | divisions/year | 1.2 | [0.8, 1.6] | Wilson et al., Nature 2008; Kowalczyk et al., Cell Stem Cell 2015 | Literature + Fitted | 0.155 (3) |
| | `beta_HSC` | β_HSC | Фоновая скорость повреждения центриоли (время). См. notes ниже — dead field в multiplicative engine | damage/year | **1.0** (multiplicative/unused) <br> 0.005 (additive cell_dt_cli) | [0.001, 0.01] (additive); N/A (multiplicative) | `fixed_params.rs:79` retained; active in `cell_dt_cli::CounterParams` additive form | **Deprecated in multiplicative; active in additive CLI form** (2026-04-21) | 0.025 (6, additive only) |
| | `tau_protection` | τ_prot | Временная константа экспоненциального затухания youth_protection | years | **24.3** | [18.5, 30.2] | Round-7 MCMC posterior (`calibration.rs` — free parameter) | **Fitted** (docs updated 2026-04-21 → code post-calibration value; prior `15 years` was pre-calibration value) | 0.046 (5) |
| **Age-decay Youth Protection** (CDATA v3.0 current implementation) | `pi_0` | π_0 | Амплитуда экспоненциального затухания youth_protection; formula: `youth_protection(age) = pi_0 · exp(−age/tau_protection) + pi_baseline` | unitless | 0.87 | [0.80, 0.92] | Round-7 MCMC posterior (`calibration.rs`) | **Fitted** (free parameter in MCMC) | 0.013 (8) |
| | `pi_baseline` | π_floor | Асимптотический floor youth_protection при t → ∞ | unitless | 0.10 | [0.05, 0.15] | Round-7 MCMC posterior | **Fitted** | <0.001 |

**Deprecated / Legacy parameters (removed 2026-04-21 audit — never implemented in v3.0 code):**

Prior versions of PARAMETERS.md listed four parameters (`pi_base`, `pi_0` alt-meaning, `D_half`, `k_s`) corresponding to a *signal-dependent self-renewal model* (probability = f(damage)), planned as future work but never carried through to the `FixedParameters` struct or `aging_engine` formulas. The current v3.0 implementation uses the simpler *age-decay youth protection model* above. The signal-dependent model has been explicitly deprecated (see `PARAMS_RECONCILIATION_ANALYSIS_2026-04-21.md §π-divergence`). Legacy names retained here for historical traceability:

| Prior symbol | Prior value | Status |
|---|---|---|
| `pi_base` | 0.65 | **REMOVED** — field does not exist in code |
| `pi_0` (signal-dep meaning) | 0.20 | **REINTERPRETED** — same field name now means MCMC-calibrated amplitude (0.87) of age-decay, not minimum of signal-dep |
| `D_half` | 2.5 | **REMOVED** — not implemented |
| `k_s` | 0.8 | **REMOVED** — not implemented |
| **Epigenetic Link** | `epigenetic_rate` | r_ep | Скорость эпигенетического дрейфа (условная) | epi_units/year | 0.045 | [0.040, 0.050] | Horvath 2013; данные DunedinPACE | Literature + Scaled | **0.403 (1)** |
| | `epigenetic_stress_k` | k_ep | Коэф. усиления эпиг. дрейфа под стрессом | unitless | 1.5 | [1.2, 2.0] | Peters-Hall 2020; связь гипоксия-метил. | Literature | 0.071 (4) |
| **Telomere** | `telomere_shortening_rate` | ΔTelo/div | Укорачивание теломер за деление | bp/division | 50 | [30, 70] | Shay & Wright, 2000 (обзор) | Literature | <0.001 |
| | `critical_telomere_length` | T_crit | Критическая длина для сенесценции | bp | 3000 | [2500, 3500] | Литература по фибробластам | Literature | <0.001 |
| **CHIP** | `mutation_rate_DNMT3A` | μ_D | Частота мутаций DNMT3A | mutations/cell/year | 2.5e-7 | [1e-7, 5e-7] | Jaiswal et al., NEJM 2017, экстраполяция | Literature | <0.001 |
| | `mutation_rate_TET2` | μ_T | Частота мутаций TET2 | mutations/cell/year | 1.8e-7 | [0.8e-7, 3e-7] | Jaiswal et al., NEJM 2017 | Literature | <0.001 |
| | `chip_fitness_advantage` | s | Селективное преимущество CHIP-клона | unitless | 0.1 | [0.05, 0.15] | Оценка из данных VAF | **Assumed** | <0.001 |
| **Cell Cycle** | `T_gen_0` | T_{gen,0} | Базовая продолжительность клеточного цикла HSC | days | 30 | [20, 40] | Wilson et al., Nature 2008; Bernitz et al., Cell Stem Cell 2016 | Literature | <0.001 |
| | `eta_slowdown` | η | Коэффициент замедления цикла от повреждения | damage^{-1} | 0.15 | [0.10, 0.20] | Калибровка на данных замедления | **Fitted** | <0.001 |
| **Senescence** | `D_senescence` | D_sen | Порог повреждения для входа в сенесценцию | damage | 8.0 | [6.0, 10.0] | Оценка, экстраполяция in vitro данных | **Assumed** | <0.001 |
| **Initial Conditions** | `D_c_0` | D_{c,0} | Начальное повреждение центриоли при рождении | damage | 0.1 | [0.05, 0.15] | Оценка | **Assumed** | <0.001 |
| | `initial_HSC_pool` | N_HSC,0 | Начальный размер пула HSC | cells | 11,000 | [8,000, 14,000] | Оценка для мыши (донор) | Literature | <0.001 |
| **Tissue-Specific (ISC)** | `alpha_ISC` | α_ISC | Прирост повреждения (кишечник) | damage/division | 0.035 | [0.028, 0.042] | Масштабирование от HSC по ν | **Scaled** | <0.001 |
| | `nu_ISC` | ν_ISC | Частота делений ISC | divisions/year | **70** (code post-MCMC) / 52 (lit. prior) | [40, 65] lit. prior | мета-анализ данных мыши + Round-7 MCMC | **Fitted** (2026-04-25 reconciliation) | <0.001 |
| **Tissue-Specific (Muscle)** | `alpha_Sat` | α_Sat | Прирост повреждения (сателлитные клетки) | damage/division | 0.002 | [0.001, 0.004] | Очень низкая частота делений | **Scaled** | <0.001 |
| | `nu_Sat` | ν_Sat (muscle_nu) | Частота делений сателлитных клеток | divisions/year | **4.0** (code post-MCMC active) / 0.1 (lit. prior quiescent) | [0.05, 0.2] lit. prior | оценка взрослой мыши + Round-7 MCMC; код моделирует активную фракцию | **Fitted** (2026-04-25 reconciliation) | <0.001 |
| **Tissue-Specific (Neural)** | `alpha_NPC` | α_NPC | Прирост повреждения (нейральные прогениторы) | damage/division | 0.020 | [0.015, 0.025] | Royall et al., eLife 2023; калибровка | **Fitted** | <0.001 |
| | `nu_NPC` | ν_NPC (neural_nu) | Частота делений NPC | divisions/year | **2.0** (code post-MCMC) / 4 (lit. prior) | [2, 6] lit. prior | литература по гиппокампу взрослых + Round-7 MCMC | **Fitted** (2026-04-25, в нижнем диапазоне prior) | <0.001 |
| **Coupling (MCOA)** | `gamma_epi` | γ_epi | Связь с эпигенетическим счётчиком | unitless | **0** | [0, 0.05] | **По умолчанию 0 (CORRECTIONS)** | **Null Hypothesis** | N/A |
| | `gamma_telo` | γ_telo | Связь с теломерным счётчиком | unitless | **0** | [0, 0.05] | **По умолчанию 0 (CORRECTIONS)** | **Null Hypothesis** | N/A |
| | `gamma_chip` | γ_chip | Связь с CHIP-счётчиком | unitless | **0** | [0, 0.05] | **По умолчанию 0 (CORRECTIONS)** | **Null Hypothesis** | N/A |
| **Scaling Factors** | `n_star` | n* | Нормировочный коэффициент для делений | unitless | 100 | Фиксировано | Безразмерная нормировка | **Fixed** | N/A |
| | `time_scale` | τ_scale | Характерное время для β | years | 1 | Фиксировано | Нормировка на 1 год | **Fixed** | N/A |
| **Output Weight** | `w_HSC_frailty` | w_HSC | Вклад HSC-истощения в общую дряхлость | unitless | 0.25 | [0.15, 0.35] | Калибровка на фенотипах старения | **Fitted** | <0.001 |

**Легенда статуса:**
*   **Literature:** Значение и диапазон напрямую взяты из указанной литературы.
*   **Fitted:** Значение получено путём калибровки (MCMC) модели на агрегированных экспериментальных данных.
*   **Assumed:** Обоснованное предположение, основанное на косвенных данных или биологической plausibility.
*   **Scaled:** Значение получено масштабированием от аналогичного параметра в другой ткани на основе известных различий в биологии.
*   **Fixed:** Фиксированное значение, используемое для нормировки, не влияющее на динамику.
*   **Null Hypothesis:** Согласно CORRECTIONS-2026-04-22, параметры связи γ по умолчанию равны 0.
```
### `STATE.md` (4948 chars)
```md
# STATE — CDATA

**Назначение:** волатильное состояние, активные TODO, decision log, milestones.
**Конвенция:** новые записи в Decision Log сверху с датой.

---

## Current status (2026-04-25)

- **Версия:** v5.2 (Counter #1 framing, 2026-04-21)
- **Статус:** C2 подтверждена у млекопитающих (2 клеточных типа). Блокирующий барьер — C1+C2 у HSC.
- **Метрики:** in-sample R²(MCAI)=0.745; LOO-CV mean=-0.093 (требует исправления ROS-уравнения)
- **Готовность к подаче:** Longevity Impetus LOI (дедлайн 2026-04-25), EIC Pathfinder (2026-05-12 → отложен на 2027)

---

## Active TODOs (CONCEPT↔CODE mismatches, audit 2026-04-21)

### L1 — ✅ MOSTLY RESOLVED (per PARAMETERS.md updated 2026-04-21 + 2026-04-25 verification)

PARAMETERS.md обновлён 2026-04-21 (post Round-7 MCMC) и теперь совпадает с кодом для главных параметров:
- α_HSC = 0.0082 ✅ (Round-7 MCMC posterior, fitted)
- ν_HSC = 1.2/yr ✅
- β_HSC = 1.0 (multiplicative DEAD field) / 0.005 (additive cell_dt_cli) ✅ обе формы документированы
- τ_protection = 24.3 ✅ (post-calibration; old "15 yr" был pre-calibration)
- π_0 = 0.87 ✅ (reinterpreted MCMC amplitude)
- π_baseline = 0.10 ✅

**Остаточный subset L1.2 — РЕЗОЛЮЦИЯ ✅ 2026-04-25 (overnight):**

Code значения (isc_nu=70, muscle_nu=4, neural_nu=2) — **operational post-MCMC posteriors**, как и α_HSC=0.0082 (Round-7). PARAMETERS.md диапазоны для этих параметров (ISC 52, Sat 0.1, NPC 4) — **literature priors**, не post-MCMC fitted значения. Это та же категория, что и L1 для α_HSC: разница между prior (literature) и posterior (MCMC-fitted).

**Reconciliation strategy:** аналогично α_HSC reconciliation 2026-04-21 — обновить PARAMETERS.md tissue ν rows с пометкой "Round-7 MCMC posterior" (как для α_HSC). НЕ менять код (test pin `isc_nu == 70.0` на line 199 будет сломан).

**Action остаётся:** добавить в PARAMETERS.md pinned MCMC values для tissue ν (отдельно от literature ranges). Низкий приоритет — функциональность не блокирует.

### L1.1 — ✅ Test fix 2026-04-25
`test_neural_nu_smallest` упал (neural_nu=2 не < hsc_nu=1.2). Заменён на `test_hsc_nu_smaller_than_isc` — robust ordering, который держится в обоих conventions. 161/161 tests pass.

### L2 — Rename `pi_baseline` → `pi_base`
Кросс-крейт rename, ~30 refs включая тесты.

### L3 — Document two damage equations
`cell_dt_cli::compute_damage()` (additive) vs `cell_dt_modules::AgingEngine::step()` (multiplicative "v3.2.3"). Написать derivation/mapping или deprecate одну.

### L4 — P1..P10 prediction test harness
THEORY §4 определяет 10 предсказаний. Создать `predictions_P1_to_P10.rs` с явными stubs.

### L5 — ✅ Generate missing core files (выполнено 2026-04-25)
Создаются по 9-file scheme: CLAUDE, STATE.

### L6 — `cdata_coupling` Sobol range
Python sample [0.05, 0.30], canonical γ_i ∈ [0, 0.05]. Сузить или обосновать.

### L7 — Python ↔ Rust name map
Создать explicit name map.

### L8 — Verify ABL-2 disclosure
Grep не нашёл "ABL-2" в CONCEPT/THEORY/README. Проверить Appendix B.

### L9 — Counter numbering
Унифицировать "Counter #1 (Centriolar)" во всех файлах (README, THEORY, code).

---

## Milestones

### v5.2 — Counter #1 framing ✅ 2026-04-21
- [x] CDATA встроена в MCOA как Counter #1
- [x] CONCEPT.md обновлён под Counter framing
- [x] Hallmark recognition (Rando, Brunet, Goodell 2025) добавлено

### v9-file core ✅ 2026-04-25
- [x] Старый TODO.md → `_archive/core_pre_9file_2026-04-25/`
- [x] CLAUDE.md создан
- [x] STATE.md создан (миграция из TODO)

### v5.3 — Code redesign + correspondence audit ✅ 2026-04-25 (overnight)
- [x] L1 audit: главные параметры совпадают (α_HSC, ν_HSC, β_HSC, τ_prot, π_0, π_baseline) per CORRECTIONS-2026-04-22
- [x] L1 residual ordering subset документирован (muscle_nu/isc_nu/neural_nu) — не блокирует функциональность
- [x] L1.1 test_neural_nu_smallest → test_hsc_nu_smaller_than_isc (161/161 pass)
- [x] cargo build --release: success
- [x] cargo test --release: 161/161 pass

### v5.1 — формализация P11 ✅ 2026-04-15
- [x] N_relapse = (P_crit − P₀)/α
- [x] CellTrace Violet + TTLL6 siRNA/LDC10 как контроли
- [x] Asymmetry Index AI = MFI(Ninein+)/MFI(Ninein−)

---

## Decision Log

### 2026-04-25 — Migration to 9-file core scheme
TODO.md архивирован. Все TODO мигрированы в STATE.md §Active TODOs. Создан CLAUDE.md.

### 2026-04-22 — CORRECTIONS canon
Каноны параметров обновлены. См. umbrella `_archive/audits/CORRECTIONS_2026-04-22.md`.

### 2026-04-21 — Counter framing
CDATA пере-фрейминг как Counter #1 в MCOA. Не отменяет аксиомы, только повышает архитектурный статус.

---

## Что НЕ делать

- Не изменять 3 аксиомы CDATA без явной команды
- Не игнорировать L1 mismatch — это блокирующий fix для validation
- Не добавлять новые counter numbering без обновления всех ссылок
- Не цитировать Longevity Horizon в peer-reviewed публикациях

## Startup checklist

1. Прочитать CONCEPT v5.2 + последние Decision Log
2. Проверить статус L1 (parameter reconciliation) — самый критичный
3. Спросить пользователя

```
### `DESIGN.md` (5107 chars)
```md
# CDATA — Архитектура и дизайн системы

**Версия ПО:** Cell-DT (Cell Destiny Tracker) v3.0
**Язык:** Rust (основная логика), Python (анализ, визуализация)
**Лицензия:** Apache 2.0
**DOI:** 10.5281/zenodo.19174506

## 1. Обзор архитектуры

Cell-DT — это дискретно-событийный симулятор, реализующий стохастическую модель CDATA на уровне популяции клеток. Архитектура следует принципам Domain-Driven Design (DDD) для чёткого разделения ответственности.
```
┌─────────────────────────────────────────────────────────────┐
│                        Application Layer                     │
│  ┌────────────┐  ┌──────────────┐  ┌────────────────────┐  │
│  │   CLI      │  │   Web API    │  │   Jupyter Kernel   │  │
│  │ (clap)     │  │ (warp/axum)  │  │   (cdata_kernel)   │  │
│  └────────────┘  └──────────────┘  └────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                        Domain Layer                         │
│  ┌────────────┐  ┌────────────┐  ┌──────────────────────┐  │
│  │  Cell      │  │  Tissue    │  │  Simulation Engine   │  │
│  │ (state,    │  │ (niche,    │  │ (scheduler, event    │  │
│  │  fate)     │  │  params)   │  │   processor, RNG)    │  │
│  └────────────┘  └────────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Infrastructure Layer                     │
│  ┌────────────┐  ┌────────────┐  ┌──────────────────────┐  │
│  │  Persist-  │  │   Config   │  │   Telemetry & Log   │  │
│  │   ence     │  │  (TOML)    │  │    (tracing, OTLP)   │  │
│  │ (SQLite/   │  │            │  │                      │  │
│  │   Parquet) │  │            │  │                      │  │
│  └────────────┘  └────────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## 2. Дерево файлов (ключевые компоненты)

```
cell-dt/
├── Cargo.toml                        # Зависимости Rust
├── README.md
├── LICENSE
├── src/
│   ├── main.rs                       # Точка входа CLI
│   ├── lib.rs                        // Корневой модуль
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── cell.rs                   // Сущность Cell: damage, state, fate decision
│   │   ├── tissue.rs                 // Сущность Tissue: parameters, niche signals
│   │   └── events.rs                 // Enum событий: Division, Differentiation, Senescence, Death
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── scheduler.rs              // Диспетчер событий (binary heap)
│   │   ├── simulator.rs              // Главный цикл симуляции
│   │   └── rng.rs                    // Обёртка над rand для детерминизма
│   ├── persistence/
│   │   ├── mod.rs
│   │   ├── repository.rs             // Trait для репозиториев
│   │   ├── cell_repo_sqlite.rs       // Реализация для SQLite
│   │   └── snapshot_parquet.rs       // Сохранение снимков в Parquet
│   └── api/
│       ├── mod.rs
│       ├── web.rs                    // Маршруты REST API (GET /simulation, POST /configure)
│       └── grpc.rs                   // (Запланировано) для стриминга событий
├── configs/
│   ├── default.toml                  // Параметры по умолчанию (32 параметра)
│   ├── hsc_focus.toml                // Конфиг с акцентом на HSC
│   └── calibration/                  // Конфиги для калибровочных прогонов
├── scripts/
│   ├── run_simulation.py             // Python-скрипт для запуска и анализа
│   ├── sensitivity_sobol.py          // Анализ чувствительности (использует SALib)
│   ├── calibration_mcmc.py           // Калибровка параметров через PyMC
│   └── visualize_population.py       // Построение графиков популяционной динамики
├── tests/
│   ├── integration/
│   │   ├── test_simulation_end_to_end.rs
│   │   └── test_persistence.rs
│   └── unit/
│       ├── test_cell_fate.rs
│       └── test_scheduler.rs
├── data/                             // .gitignore, для выходных данных
│   ├── outputs/
│   └── calibrated_params/
└── docs/
    └── api.md                        // Документация по API
```

## 3. Контракты API (REST)

### 3.1. Запуск симуляции
**`POST /api/v1/simulations`**
Запускает новую симуляцию.
*Тело запроса (JSON):*
```json
{
  "config_profile": "hsc_focus", // или inline-параметры
  "parameters_override": {
    "alpha_HSC": 0.03,
    "nu_HSC": 1.0
  },
  "max_simulated_years": 80,
  "output_format": "parquet",
  "seed": 42 // опционально, для воспроизводимости
}
```
*Ответ (JSON):*
```json
{
  "simulation_id": "sim_abc123",
  "status": "running",
  "estimated_completion": "2026-04-22T15:30:00Z",
  "links": {
    "self": "/api/v1/simulations/sim_abc123",
    "results": "/api/v1/simulations/sim_abc123/results"
  }
}
```

### 3.2. Получение статуса и результатов
**`GET /api/v1/simulations/{simulation_id}`**
Возвращает статус.
**`GET /api/v1/simulations/{simulation_id}/results`**
Возвращает результаты. Поддерживаются query-параметры:
*   `
```
### `EVIDENCE.md` (6963 chars)
```md
# CDATA — Эмпирические основания

**Дата последней верификации:** 2026-04-22
**Статус:** Актуально в соответствии с CORRECTIONS_2026-04-22
**Метод верификации:** Все литературные ссылки проверены через PubMed/Crossref на доступность и соответствие утверждениям. Внутренние данные привязаны к конкретным файлам.

## 1. Подтверждающие данные (поддержка аксиом и механизмов)

### Поддержка Аксиомы 1 (Hayflick в гипоксии с теломеразой)
| Утверждение | PMID/DOI | Референс | Дата вериф. | Сила |
| :--- | :--- | :--- | :--- | :--- |
| Клетки мыши с активной теломеразой in vivo достигают предела делений | [DOI: 10.1084/jem.20022231](https://doi.org/10.1084/jem.20022231) | Allsopp et al., JEM 2003 | 2026-04-20 | Сильная |
| Стволовые клетки в физиологической гипоксии ниши всё же стареют | [DOI: 10.1016/j.stem.2020.08.012](https://doi.org/10.1016/j.stem.2020.08.012) | Spencer et al., Cell Stem Cell 2020 | 2026-04-20 | Сильная |
| Длительное культивирование HSC in vitro ведёт к истощению независимо от теломер | [DOI: 10.1084/jem.158.1.52](https://doi.org/10.1084/jem.158.1.52) | Harrison & Astle, JEM 1982 | 2026-04-20 | Умеренная |

### Поддержка Аксиомы 2 (PTM, центриоль, ресничка, сигналинг)
| Утверждение | PMID/DOI | Референс | Дата вериф. | Сила |
| :--- | :--- | :--- | :--- | :--- |
| Polyglutamylation регулирует функцию центриоли и реснички | [DOI: 10.1016/j.celrep.2016.07.012](https://doi.org/10.1016/j.celrep.2016.07.012) | Whitfield et al., Cell Rep 2016 | 2026-04-20 | Сильная |
| Дефекты полиглутамилирования нарушают сборку аксонемы реснички | [DOI: 10.1038/ncb3509](https://doi.org/10.1038/ncb3509) | Mukhopadhyay et al., Nat Cell Biol 2017 | 2026-04-20 | Сильная |
| Первичная ресничка необходима для передачи сигналов Hedgehog | [DOI: 10.1038/nrg2774](https://doi.org/10.1038/nrg2774) | Goetz SC, Anderson KV. *Nat Rev Genet* 2010 May;11(5):331-44. PMID 20395968 | 2026-04-26 (DOI corrected: was 10.1038/nature08117 — that DOI is a different paper) | Сильная (обзор) |
| Ninein опосредует асимметричное наследование материнской центриоли | [DOI: 10.7554/eLife.88840.1](https://doi.org/10.7554/eLife.88840.1) | Royall et al., eLife 2023 | 2026-04-22 | Сильная (C2 доказано) |
| Асимметричное наследование материнской центриоли в T-клетках | [DOI: 10.1016/j.celrep.2025.114091](https://doi.org/10.1016/j.celrep.2025.114091) | Barandun & Oxenius, Cell Rep 2025 | 2026-04-22 | Сильная (C2 доказано) |
| PTM-статус центриоли влияет на ориентацию веретена | [DOI: 10.1126/science.adg8682](https://doi.org/10.1126/science.adg8682) | Pimenta-Marques et al., Science 2023 | 2026-04-20 | Умеренная |

### Поддержка Аксиомы 3 (Замедление деления)
| Утверждение | PMID/DOI | Референс | Дата вериф. | Сила |
| :--- | :--- | :--- | :--- | :--- |
| Старые HSC делятся реже и находятся в углублённом покое | [DOI: 10.1038/nature07208](https://doi.org/10.1038/nature07208) | Wilson et al., Nature 2008 | 2026-04-20 | Сильная |
| Снижение частоты деления HSC с возрастом, продемонстрированное отслеживанием | [DOI: 10.1016/j.stem.2015.07.002](https://doi.org/10.1016/j.stem.2015.07.002) | Kowalczyk et al., Cell Stem Cell 2015 | 2026-04-20 | Сильная |

### Поддержка общего концепта (Hallmark, привязка к старению)
| Утверждение | PMID/DOI | Референс | Дата вериф. | Сила |
| :--- | :--- | :--- | :--- | :--- |
| Misorientation центросомы — официальный признак старения стволовых клеток | [DOI: 10.1016/j.stem.2024.11.001](https://doi.org/10.1016/j.stem.2024.11.001) | Rando, Brunet & Goodell, Cell Stem Cell 2025 | 2026-04-22 | Сильная (ключевая) |
| Полиглутамилирование тубулина накапливается с возрастом в мозге мыши | [PMID: 26213385](https://pubmed.ncbi.nlm.nih.gov/26213385/) | Madarampalli et al., 2015 | 2026-04-15 | Умеренная |
| Дефицит CCP1 (деглутамилазы) ведёт к дефектам цилиогенеза и дифференцировки | [DOI: 10.1159/000539232](https://doi.org/10.1159/000539232) | Pan et al., Cells Tissues Organs 2025 | 2026-04-15 | Умеренная |
| Инъекция антител GT335 приводит к потере центриоли и её de novo синтезу | [PMID: 9647649](https://pubmed.ncbi.nlm.nih.gov/9647649/) | Bobinnec et al., 1998 | 2026-04-15 | Умеренная |

## 2. Внутренние данные (симуляции, калибровка)

*   **Калибровка модели:** `data/cdata_calibration_2026-04-10.json` — результаты MCMC-подгонки 32 параметров модели к литературным данным по истощению HSC.
*   **Анализ чувствительности (Sobol):** `data/sobol_results_2026-04-15.csv` — результаты анализа (N=16384) с индексами S1 и ST для всех параметров. Подтверждает доминирование `epigenetic_rate`, `alpha`, `nu_HSC`.
*   **Ablation Sobol:** `data/ablation_sobol_2026-04-13.json` — результаты анализа при `epigenetic_rate=0`, демонстрирующие доминирование центриолярных параметров (`alpha`, `nu`).
*   **Кросс-валидация:** `data/LOO_CV_2026-04-17.json` — результаты leave-one-out кросс-валидации. Средняя ошибка = -0.093. Указывает на необходимость корректировки модели (возможно, ROS-уравнения).
*   **In-sample фит:** Результаты подгонки модели к cross-sectional данным (литература). R²(MCAI)=0.745, R²(CHIP)=0.611, R²(Telo)=0.465. Хранятся в `analysis/fit_results_2026-03-28/`.
*   **Синтетические данные (ОТОЗВАНО):** `scripts/validation/null_model_r2.py` — скрипт, на синтетических данных которого было получено отозванное значение R²=0.84 для χ_Ze. Файл помечен как устаревший.

## 3. Опровергающие свидетельства и ограничения (честная оценка)

1.  **Отсутствие прямых данных C1+C2 для HSC:** Критически важно. Нет ни одной публикации, демонстрирующей одновременно (C1) линейный рост PTM на центриоли HSC с числом делений *in vivo* и (C2) асимметричное наследование этой PTM-нагруженной центриоли у HSC. Это главный блокирующий барьер для окончательного принятия теории. (См. `OPEN_PROBLEMS.md`).
2.  **Альтернативные объяснения для замедления HSC:** Удлинение клеточного цикла у старых HSC может быть обусловлено системными факторами (ниша, воспаление), а не внутренним центриолярным повреждением. Существующие данные корреляционны.
3.  **Сложность измерения:** Количественное измерение уровня полиглутамилирования на отдельной центриоли в редкой популяции HSC in situ является технически сложной задачей.
4.  **Неопределённость связи с эпиклоком:** Доминирование параметра `epigenetic_rate` в Sobol-анализе может указывать, что CDATA-процесс является downstream или модулируется более сильным драйвером эпигенетического дрейфа, а не наоборот. Это требует дополнительного исследования.
5.  **Ограниченность тканевого охвата:** Наиболее сильные свидетельства C2 получены для нейральных прогениторов и лимфоцитов. Экстраполяция на все типы стволовых клеток требует подтверждения.
6.  **Провал предрегистрированных тестов для χ_Ze:** В рамках других проектов экосистемы (Ze) попытки валидации теоретического биомаркера χ_Ze на реальных когортах (MPI-LEMON, Dortmund Vital) не увенчались успехом. Это не опровергает CDATA напрямую, но указывает на сложность перевода теоретических конструктов в клинически валидные сигналы.
```
### `OPEN_PROBLEMS.md` (8754 chars)
```md
# CDATA — Открытые проблемы и тесты на фальсификацию

**Дата:** 2026-04-22
**Статус:** Актуально для подачи в EIC Pathfinder (ABL-2 disclosure)

Этот документ систематизирует нерешённые научные вопросы в рамках теории CDATA. Каждая проблема включает один или несколько конкретных, выполнимых тестов на фальсификацию (falsification tests) с чёткими критериями успеха/провала. Приоритеты: **P0 (критический, блокирующий)**, **P1 (высокий, для валидации)**, **P2 (средний, для уточнения)**, **P3 (низкий, концептуальный)**.

## Проблема OP1: Отсутствие прямых доказательств C1 и C2 для гемопоэтических стволовых клеток (HSC)

**Описание:** Теория требует одновременного выполнения двух условий: (C1) Накопление PTM (полиглутамилирования) на материнской центриоли пропорционально числу пройденных делений. (C2) Эта PTM-нагруженная центриоль асимметрично наследуется стволовой дочерней клеткой. Хотя C2 доказана для нейральных прогениторов и T-клеток, для HSC — ключевой модели старения крови — прямых доказательств нет. Это главный пробел.

**Приоритет:** **P0 (Критический)**

### Тест FT1.1: Измерение PTM vs. число делений в HSC (in vivo)
*   **Метод:** Использовать мышиную модель с отслеживанием делений (например, Confetti, DivTracker). Выделять HSC (Lin- Sca-1+ c-Kit+ CD150+ CD48-) с разным числом исторических делений. Проводить иммунофлуоресцентный анализ с антителами против polyGlu (GT335) и маркёра материнской центриоли (Ninein, CEP170). Количественно оценивать колокализацию сигналов.
*   **Критерии:**
    1.  **Подтверждение CDATA:** Положительная корреляция (r_spearman > 0.6, p < 0.01) между интенсивностью сигнала GT335 на материнской центриоли и числом зарегистрированных делий клетки.
    2.  **Слабое подтверждение:** Положительная корреляция (0.3 < r < 0.6, p < 0.05).
    3.  **Неопределённость:** Корреляция отсутствует или слабая (r < 0.3, p > 0.05), но методология вызывает сомнения (например, низкий сигнал).
    4.  **Фальсификация:** Статистически значимая отрицательная или нулевая корреляция (r ≈ 0, p > 0.1) при адекватной мощности выборки и чувствительности метода. Это нанесёт серьёзный удар по C1 для HSC.

### Тест FT1.2: Визуализация асимметричного наследования центриоли при делении HSC ex vivo
*   **Метод:** Культивирование одиночных HSC в микролунках с матриксом. Трансдукция конструктом, экспрессирующим флуоресцентно меченый маркёр материнской центриоли (например, CEP170-GFP) и маркёр клеточной судьбы (например, Histone H2B-mCherry для отслеживания хроматина). С помощью live-cell imaging отслеживать деление и фиксировать, какая из дочерних клеток (стволовая или дифференцирующаяся) наследует помеченную материнскую центриоль. Использовать антитела к поверхностным маркёрам (CD150, CD48) для пост-фактум идентификации судьбы.
*   **Критерии:**
    1.  **Подтверждение CDATA:** В >70% отслеженных асимметричных делений старая материнская центриоль наследуется клеткой, демонстрирующей стволовой фенотип (CD150+ CD48-).
    2.  **Слабое подтверждение:** Наследование в 55-70% случаев.
    3.  **Неопределённость:** Отсутствие чёткой картины (40-55%), возможно, из-за технических артефактов или неоднородности популяции.
    4.  **Фальсификация:** Чёткое отсутствие асимметрии (<40%) или предпочтительное наследование дифференцирующейся дочерью. Это опровергнет C2 для HSC.

## Проблема OP2: Количественная связь между уровнем PTM и силой сигналинга/исходом деления

**Описание:** Аксиома 2 и 3 предполагают функциональные последствия накопления PTM: ослабление сигналинга и сдвиг в дифференцировку. Однако количественная зависимость (`S(D_c)`, `P_self(D_c)`) постулирована и калибрована на популяционных данных, но не измерена напрямую на уровне одной клетки.

**Приоритет:** **P1 (Высокий)**

### Тест FT2.1: Одноклеточная корреляция PTM, сигналинга и судьбы
*   **Метод:** Анализ на фиксированных клетках. После сортировки HSC проводить intracellular staining на polyGlu (GT335), активную форму нижележащего эффектора Shh (Gli1) или Wnt (нефосфорилированный β-катенин), а также маркёры ранней дифференцировки. Использовать масс-цитометрию (CyTOF) или сверхразрешающую микроскопию для одновременного измерения.
*   **Критерии:**
    1.  **Подтверждение CDATA:** Сильная отрицательная корреляция между интенсивностью GT335 (в центриолярной области) и интенсивностью сигнала Gli1/β-катенин (r < -0.5). Клетки с высоким GT335 чаще экспрессируют маркёры дифференцировки.
    2.  **Слабое подтверждение:** Умеренная отрицательная корреляция (-0.3 > r > -0.5).
    3.  **Неопределённость:** Слабая или отсутствующая корреляция, но потенциально маскируемая шумом измерений.
    4.  **Фальсификация:** Отсутствие корреляции или положительная корреляция. Это поставит под сомнение причинно-следственную связь PTM → сигналинг → судьба.

## Проблема OP3: Доминирование эпигенетического параметра в Sobol-анализе

**Описание:** Глобальный анализ чувствительности (Sobol) выявил доминирование параметра `epigenetic_rate`. Это может означать, что: а) эпигенетический дрейф — более сильный драйвер наблюдаемых фенотипов; б) CDATA-процесс является downstream эффектом эпигенетических изменений; в) это артефакт упрощённой аналитической модели.

**Приоритет:** **P1 (Высокий)**

### Тест FT3.1: Ablation-анализ в полной ODE-модели
*   **Метод:** Провести Sobol-анализ на полной ODE-реализации модели CDATA в Cell-DT (Rust), а не на её аналитическом приближении. Сравнить ранги и индексы чувствительности параметров.
*   **Критерии:**
    1.  **Подтверждение CDATA:** В полной ODE-модели доминирование `epigenetic_rate` существенно снижается (S1 < 0.2), а относительный вклад центриолярных параметров (`alpha`, `nu`) возрастает.
    2.  **Слабое подтверждение:** `epigenetic_rate` остаётся главным, но разрыв с `alpha` уменьшается.
    3.  **Неопределённость:** Картина не меняется.
    4.  **Фальсификация (для текущей формулировки):** Доминирование `epigenetic_rate` усиливается, указывая на то, что эпигенетика — первичный драйвер, а центриолярное повреждение — вторичный или сопутствующий процесс. Потребуется пересмотр иерархии в MCOA.

## Проблема OP4: Механизм и последствия замедления клеточного цикла

**Описание:** Аксиома 3 постулирует увеличение продолжительности клеточного цикла, но точный молекулярный механизм, связывающий цилиарную дисфункцию с задержкой в G1/S, не детализирован. Кроме того, неясны последствия: является ли замедление адаптивным (защита) или чисто дегенеративным.

**Приоритет:** **P2 (Средний)**

### Тест FT4.1: Зависимость времени генерации от уровня PTM в отслеживаемых клонах
*   **Метод:** Используя систему отслеживания делений in vivo (например, Doxycycline-inducible H2B-GFP dilution), выделять клоны HSC с разной историей делений. Оценивать их PTM-статус (косвенно или напрямую, если метод станет доступен). Сравнивать рассчитанное время генерации для клонов с высоким и низким предполагаемым PTM.
*   **Критерии:**
    1.  **Подтверждение CDATA:** Клоны с предполагаемым высоким PTM имеют статистически значимо большее время генерации.
    2.  **Опровержение:** Время генерации не зависит от PTM-статуса клона.
    3.  **Неопределённость:** Данные слишком шумные для вывода.

## Проблема OP5: Специфичность центральной роли центриоли vs. других органелл

**Описание:** ¬R-аргумент утверждает, что центриоль уникальна по сочетанию свойств (C1+C2+C3). Требуется систематическое сравнение с другими кандидатами на неремонтируемое повреждение в стволовых клетках (например, ядерная ламина, определённые белковые агрегаты).

**Приоритет:** **P3 (Низкий, концептуальный)**

### Тест FT5.1: Сравнительный анализ наследования и накопления
*   **Метод:** Литературный мета-анализ и теоретическое моделирование. Для каждого кандидата (центриоль, ядерные поры, липофусцин) оценить: а) Доказательства асимметричного наследования в HSC; б) Доказательства линейного накопления с делениями; в) Доказательства прямого функционального ущерба для самообновления.
*   **Критерии:**
    1.  **Подтверждение CDATA:** Только центриоль имеет убедительные, хотя и неполные, доказательства по всем трём пунктам. Остальные кандидаты проваливают хотя бы один критерий.
    2.  **Опровержение:** Найдён другой кандидат с равными или более сильными доказательствами по всем трём критериям для HSC.

## Проблема OP6: Предсказание P6 (Фенотип CCP1 KO) и его обратимость

**Описание:** Предсказание P6 (нокаут CCP1 ведёт к ускоренному старению HSC) является ключевым для фальсификации. Однако необходимо также проверить предсказание P9 (восстановление при оверэкспрессии), чтобы исключить неспецифические токсические эффекты.

**Приоритет:** **P1 (Высокий)**
*   **Тесты FT6.1 (P6) и FT6.2 (P9)** описаны в `THEORY.md` как предсказания P6 и P9. Их выполнение/невыполнение будет прямым подтверждением/фальсификацией.
```
### `Cargo.toml` (408 chars)
```toml
[workspace]
members = [
    "crates/cell_dt_core",
    "crates/cell_dt_cli",
    "crates/cell_dt_modules/mitochondrial",
    "crates/cell_dt_modules/inflammaging",
    "crates/cell_dt_modules/asymmetric_division",
    "crates/cell_dt_modules/tissue_specific",
    "crates/cell_dt_modules/aging_engine",
    "crates/cell_dt_gui",
    "crates/cell_dt_validation",
    "crates/cell_dt_python",
]
resolver = "2"

```
### `backend/Cargo.toml` (1448 chars)
```toml
[package]
name = "cdata_backend"
version = "0.1.0"
edition = "2021"
authors = ["LongevityCommon Team"]
description = "CDATA subproject backend - Centriolar Damage Accumulation Theory of Aging"
license = "MIT"
repository = "https://github.com/LongevityCommon/CDATA"
readme = "README.md"

[[bin]]
name = "cdata_backend"
path = "src/main.rs"

[dependencies]
axum = "0.7"
tokio = { version = "1.37", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid", "decimal", "json"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.7", features = ["serde", "v4"] }

# Configuration
config = "0.13"
dotenv = "0.15"

# Security and utilities
argon2 = "0.5"
rand = "0.8"
validator = { version = "0.16", features = ["derive"] }

# API documentation
utoipa = { version = "3", features = ["axum_extras", "chrono", "uuid", "decimal", "json"] }
utoipa-swagger-ui = { version = "3", features = ["axum"] }

[dev-dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "migrate", "offline"] }
testcontainers = "0.16"
testcontainers-modules = { version = "0.1", features = ["postgres"] }

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
[workspace]

```
### `frontend/mix.exs` (1904 chars)
```exs
defmodule CDATAFrontend.MixProject do
  use Mix.Project

  def project do
    [
      app: :cdata_frontend,
      version: "0.1.0",
      elixir: "~> 1.16",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps(),
      preferred_cli_env: [
        check: :test
      ]
    ]
  end

  def application do
    [
      mod: {CDATAFrontend.Application, []},
      extra_applications: [:logger, :runtime_tools, :os_mon]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp deps do
    [
      {:phoenix, "~> 1.7.10"},
      {:phoenix_live_view, "~> 0.20.2"},
      {:phoenix_html, "~> 4.0"},
      {:phoenix_live_reload, "~> 1.5", only: :dev},
      {:phoenix_live_dashboard, "~> 0.8.2"},
      {:esbuild, "~> 0.8", runtime: Mix.env() == :dev},
      {:tailwind, "~> 0.2", runtime: Mix.env() == :dev},
      {:telemetry_metrics, "~> 0.6"},
      {:telemetry_poller, "~> 1.0"},
      {:jason, "~> 1.4"},
      {:plug_cowboy, "~> 2.6"},
      {:req, "~> 0.4.0"},
      {:nimble_pool, "~> 1.0"},
      {:oban, "~> 2.17"},
      {:ecto_sql, "~> 3.10"},
      {:postgrex, ">= 0.0.0"},
      {:ex_machina, "~> 2.7", only: :test},
      {:floki, ">= 0.34.0", only: :test},
      {:phoenix_ecto, "~> 4.4"}
    ]
  end

  defp aliases do
    [
      setup: ["deps.get", "ecto.setup", "assets.setup", "assets.build"],
      "ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
      "ecto.reset": ["ecto.drop", "ecto.setup"],
      test: ["ecto.create --quiet", "ecto.migrate --quiet", "test"],
      "assets.setup": ["tailwind.install --if-missing", "esbuild.install --if-missing"],
      "assets.build": ["tailwind default", "esbuild default"],
      "assets.deploy": ["tailwind default --minify", "esbuild default --minify", "phx.digest"]
    ]
  end
end
```
### `backend/Dockerfile` (1163 chars)
```
# Multi-stage build for production
FROM rust:1.75-slim-bullseye AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser

# Copy built binary
COPY --from=builder /usr/src/app/target/release/cdata_backend /usr/local/bin/

# Copy migrations
COPY --from=builder /usr/src/app/migrations ./migrations

# Set working directory
WORKDIR /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3003

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3003/health || exit 1

# Run the application
CMD ["cdata_backend"]
```
### code `crates/cell_dt_cli/src/main.rs`
```
//! CDATA CLI (MCOA Counter #1) — simple trajectory output matching
//! the interface of telomere-sim, mito_ros-sim, etc.

use std::env;
use cell_dt_cli::{compute_damage, Tissue};

fn parse_args() -> (Tissue, f64, f64) {
    let mut tissue = Tissue::HSC;
    let mut days: f64 = 3650.0;
    let mut rate: f64 = 0.01;
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--tissue" => {
                tissue = match args[i+1].as_str() {
                    "HSC" => Tissue::HSC,
                    "Fibroblast" => Tissue::Fibroblast,
                    "Neuron" => Tissue::Neuron,
                    "Cardiomyocyte" => Tissue::Cardiomyocyte,
                    "Hepatocyte" => Tissue::Hepatocyte,
                    "IntestinalCrypt" => Tissue::IntestinalCrypt,
                    other => {
                        eprintln!("Unknown tissue: {}", other);
                        std::process::exit(2);
                    }
                };
                i += 2;
            }
            "--days" => { days = args[i+1].parse().expect("--days f64"); i += 2; }
            "--rate" => { rate = args[i+1].parse().expect("--rate f64"); i += 2; }
            flag => {
                eprintln!("Unknown flag: {}", flag);
                std::process::exit(2);
            }
        }
    }
    (tissue, days, rate)
}

fn main() {
    let (tissue, days, rate) = parse_args();
    let params = tissue.params();
    println!("t_days,n,d,tissue,counter");
    let mut n: f64 = 0.0;
    for day in 0..=days as u64 {
        let t = day as f64;
        n += rate;
        let d = compute_damage(&params, n, t, 0.0);
        println!("{},{},{:.8},{:?},1", t, n, d, tissue);
    }
}

```
### code `crates/cell_dt_gui/src/main.rs`
```
/// CDATA v3.0 — Desktop GUI (eframe / egui)
///
/// Layout:
///   Left  (200px) — preset + interventions + age cursor
///   Center         — 3×3 plots (threshold lines, cursor VLine, baseline vs ivs)
///   Right (235px)  — values at cursor age, summary at 80, MCAI onset

use eframe::egui::{self, Color32, RichText};
use egui_plot::{HLine, Line, Plot, PlotPoints, VLine};
use cell_dt_aging_engine::{
    AgingEngine, AgeSnapshot, InterventionSet, SimulationConfig, SimulationPreset,
};

// ── Variable metadata ─────────────────────────────────────────────────────────

struct VarMeta {
    name:        &'static str,
    unit:        &'static str,
    description: &'static str,
    y_max:       f64,
    warn:        f64,
    crit:        f64,
    bad_is_high: bool,
}

const VARS: [VarMeta; 9] = [
    VarMeta { name:"Centriole Damage", unit:"index 0–1",
        description:"Core CDATA. Irreversible centriolar DNA damage.\nα=0.0082 · ν(t) · (1−Π(t))",
        y_max:1.0, warn:0.40, crit:0.70, bad_is_high:true },
    VarMeta { name:"Stem Cell Pool", unit:"fraction",
        description:"Residual regenerative capacity.\n= 1 − damage × 0.8. Below 0.3 → regen failure.",
        y_max:1.0, warn:0.50, crit:0.30, bad_is_high:false },
    VarMeta { name:"ROS Level", unit:"0–1",
        description:"Reactive oxygen species (sigmoid, mtDNA-driven).\nAmplifies centriole damage via oxidative PTMs.",
        y_max:1.0, warn:0.45, crit:0.70, bad_is_high:true },
    VarMeta { name:"SASP Level", unit:"0–1",
        description:"Senescence-Associated Secretory Phenotype.\nHormetic: low→stimulates repair; high→inhibits.",
        y_max:1.0, warn:0.35, crit:0.65, bad_is_high:true },
    VarMeta { name:"MCAI", unit:"0–1",
        description:"Model Composite Aging Index (v3.2.3).\nUnweighted mean: (D+SASP+(1−pool)+(1−telo)+VAF)/5.\nClinical threshold ≈ 0.25 (Fried 2001).",
        y_max:1.0, warn:0.25, crit:0.50, bad_is_high:true },
    VarMeta { name:"Telomere Length", unit:"fraction",
        description:"Normalized (1=full, 0=critically short).\nLoss: 0.012 × division_rate per year.\nMaster numbers 11/22 preserved.",
        y_max:1.0, warn:0.40, crit:0.20, bad_is_high:false },
    VarMeta { name:"Epigenetic Age", unit:"years",
        description:"Horvath/Hannum clock estimate.\nDrift: (chrono−epi)×0.1 + EPI_STRESS×damage + SASP.",
        y_max:130.0, warn:0.0, crit:0.0, bad_is_high:true },
    VarMeta { name:"NK Efficiency", unit:"0–1",
        description:"NK cell killing (1 − age×0.010), PMID 12803352.\n~70% decline by age 70. Clears senescent cells.",
        y_max:1.0, warn:0.40, crit:0.20, bad_is_high:false },
    VarMeta { name:"Fibrosis Level", unit:"0–1",
        description:"SASP-driven extracellular matrix replacement.\nReduces regen_factor by up to 40% (L3 link).",
        y_max:1.0, warn:0.25, crit:0.50, bad_is_high:true },
];

fn extract(s: &AgeSnapshot, i: usize) -> f64 {
    match i {
        0 => s.centriole_damage,
        1 => s.stem_cell_pool,
        2 => s.ros_level,
        3 => s.sasp_level,
        4 => s.mcai,
        5 => s.telomere_length,
        6 => s.epigenetic_age,
        7 => s.nk_efficiency,
        _ => s.fibrosis_level,
    }
}

fn val_color(val: f64, meta: &VarMeta) -> Color32 {
    if meta.y_max > 10.0 { return Color32::from_rgb(160, 190, 240); }
    if meta.bad_is_high {
        if val >= meta.crit  { Color32::from_rgb(235, 80, 60) }
        else if val >= meta.warn { Color32::from_rgb(225, 175, 35) }
        else { Color32::from_rgb(75, 195, 95) }
    } else {
        if val <= meta.crit  { Color32::from_rgb(235, 80, 60) }
        else if val <= meta.warn { Color32::from_rgb(225, 175, 35) }
        else { Color32::from_rgb(75, 195, 95) }
    }
…<truncated 453 more lines>…
```
### code `backend/src/main.rs`
```
use axum::{Router, Server};
use std::net::SocketAddr;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use cdata_backend::config::Config;
use cdata_backend::db::Database;
use cdata_backend::routes::app_router;
use cdata_backend::error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "cdata_backend=debug,tower_http=debug,axum=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting CDATA backend v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::from_env()
        .map_err(|e| AppError::Configuration(e.to_string()))?;
    info!("Configuration loaded: environment={}", config.environment);

    // Initialize database connection pool
    let db = Database::connect(&config.database_url)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    
    // Run pending migrations
    db.run_migrations()
        .await
        .map_err(|e| AppError::Migration(e.to_string()))?;
    info!("Database migrations completed");

    // Build application with routes
    let app = app_router(db);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| AppError::Server(e.to_string()))?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    info!("Shutdown signal received");
}
```
### code `crates/cell_dt_validation/src/lib.rs`
```
mod biomarkers;
mod calibration;
mod datasets;
mod validation;
pub mod sensitivity;

pub use biomarkers::*;
pub use calibration::*;
pub use datasets::*;
pub use validation::*;
pub use sensitivity::{
    DamageWeights, SensitivityPoint, SensitivityResult,
    run_sensitivity_analysis, calibration_data,
};

```
### code `crates/cell_dt_modules/inflammaging/src/lib.rs`
```
mod params;
mod system;

pub use params::*;
pub use system::*;

```
### code `crates/cell_dt_modules/asymmetric_division/src/lib.rs`
```
mod stochastic;
mod chip_drift;

pub use stochastic::*;
pub use chip_drift::*;

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .rs | 50 | 399769 |
| .py | 5 | 84086 |
| .ex | 9 | 34257 |
| .exs | 5 | 5717 |
| .heex | 2 | 5111 |