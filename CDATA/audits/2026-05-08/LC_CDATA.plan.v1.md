## План улучшений CDATA — P0, P1, P2

### P0 — Блокирующие (S/M/L + риск)

| # | Пункт | Файлы | Оценка | Риск |
|---|-------|-------|--------|------|
| 1 | **Выбрать каноническое damage-уравнение** — устранить дуализм `compute_damage()` (additive) vs `AgingEngine::step()` (multiplicative). Одно сделать основным, второе пометить `#[deprecated]` с явным указанием на замену. Обновить `STATE.md` (L3) и добавить mapping в `DESIGN.md`. | `crates/cell_dt_cli/src/main.rs`, `crates/cell_dt_modules/aging_engine/src/simulator.rs`, `STATE.md`, `DESIGN.md` | **M** | **High** (неправильный выбор ломает научную согласованность) |
| 2 | **Синхронизировать PARAMETERS.md с actual code для tissue ν** (L1.2) — заменить литературные priors на post-MCMC значения в основной таблице, литературные диапазоны вынести в примечание. Добавить отметку «Round-7 MCMC posterior». | `PARAMETERS.md` (строки `nu_ISC`, `nu_Sat`, `nu_NPC`), `crates/cell_dt_core/src/params.rs` | **S** | **Low** |
| 3 | **Создать `predictions.rs` с юнит-тестами для P1–P10** — для каждого предсказания минимальный тест, проверяющий модель на синтетических данных. Интегрировать в `cell_dt_validation`. | `crates/cell_dt_validation/src/predictions.rs` (новый), `THEORY.md` §4, `STATE.md` (L4) | **L** | **Medium** (тесты могут выявить несоответствия, требующие доработки модели) |
| 4 | **Удалить дублирующий Python GUI** (`gui/cdata_gui.py`) — нарушение stack-rule (Rust+Phoenix). Вся функциональность перенесена в `crates/cell_dt_gui` (Rust/egui). | `gui/cdata_gui.py` (удалить) | **S** | **Low** |
| 5 | **Закрыть все активные TODO из STATE.md (L2–L9)** — после выполнения п.1 (L3) и п.3 (L4) выполнить: L2 (rename `pi_baseline` → `pi_base`), L6 (сузить Sobol range в Python‑скриптах до канонического `[0, 0.05]`), L7 (создать Python ↔ Rust name map), L9 (унифицировать «Counter #1»). | `STATE.md`, `crates/cell_dt_core/src/params.rs`, `scripts/cdata_sobol_ci.py`, `CONCEPT.md`, `README.md`, `THEORY.md` | **M** | **Low** |

### P1 — Важно

| # | Пункт | Файлы | Оценка |
|---|-------|-------|--------|
| 6 | **Внедрить CI (GitHub Actions)** — `cargo test`, `cargo fmt --check`, `cargo clippy`, `pytest` для Python‑скриптов, проверка соответствия PARAMETERS.md коду (скрипт валидации). | `.github/workflows/ci.yml` (новый) | **M** |
| 7 | **Заменить ручной парсинг CLI на `clap`** — использовать derive-макросы. | `crates/cell_dt_cli/src/main.rs`, `Cargo.toml` 🐟 (добавить `clap`) | **S** |
| 8 | **Удалить пустые/нереализованные модули** в `cell_dt_validation` — `biomarkers.rs`, `calibration.rs`, `datasets.rs`, если они не содержат кода. | `crates/cell_dt_validation/src/lib.rs`, соответствующие `.rs` файлы | **S** |
| 9 | **Обновить версию Rust в Dockerfile** с `1.75` → `1.85` (актуальная stable). | `backend/Dockerfile` | **S** |
| 10 | **Унифицировать `beta_HSC`** — удалить dead field из `FixedParameters` (multiplicative engine), оставить только аддитивную форму в `CounterParams`. Обновить PARAMETERS.md. | `crates/cell_dt_core/src/params.rs`, `PARAMETERS.md` | **S** |
| 11 | **Создать единый runtime‑конфиг** — перенести все настраиваемые параметры (из `params.rs`, `Cargo.toml`, `.env`) в центральный `config.toml` с резолвером через `config` crate. | `configs/default.toml`, `crates/cell_dt_core/src/config.rs` (новый), `backend/Cargo.toml` | **M** |
| 12 | **Исправить устаревшие ссылки в EVIDENCE.md** — запустить скрипт автоматической верификации PMID/DOI (например, через PubMed API). | `EVIDENCE.md`, `scripts/verify_references.py` (новый) | **S** |

### P2 — Nice‑to‑have

| # | Пункт | Файлы | Оценка |
|---|-------|-------|--------|
| 13 | **Оптимизировать CONCEPT.md** — вынести историю рецензий, таблицы Sobol, ABL‑2 обсуждение в отдельные документы (`_archive/`). Уменьшить размер до <50 KB. | `CONCEPT.md` → `_archive/CONCEPT_HISTORY.md`, `_archive/SOBOL_ANALYSIS.md` | **M** |
| 14 | **Добавить pre‑commit hook** для автоматической валидации PARAMETERS.md против коммитаемого кода при каждом коммите. | `.pre-commit-config.yaml` (новый), скрипт валидации | **S** |
| 15 | **Реализовать автоматическую верификацию PMID/DOI в EVIDENCE.md** — GitHub Action, запускаемый при PR, проверяет каждую ссылку. | `.github/workflows/verify-references.yml`, `scripts/verify_references.py` | **M** |