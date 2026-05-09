# План улучшений LC_MCOA

## P0 — Блокеры

| # | Шаг | Файлы | Трудоёмкость | Риск |
|---|-----|-------|--------------|------|
| 1 | Объединить `backend/` и `crates/mcoa_api` в единый Axum-сервер `mcoa_backend`, где БД-слой и REST-маршруты живут в одном crate, а логика `mcoa_core` подключается как workspace-зависимость. Перенести все маршруты из `mcoa_api` в `backend/src/routes/`, убрать `backend/Cargo.toml` изолированный workspace. | `crates/mcoa_api/`, `backend/`, корневой `Cargo.toml` | M (2–3 дня) | Средний: слом существующих эндпоинтов при некорректном слиянии; требуется ревью каждого роута |
| 2 | Синхронизировать документацию: в `CLAUDE.md` заменить «React/TypeScript» на «Phoenix LiveView»; в `DESIGN.md` удалить секции про `mcoa_interfaces`/`mcoa_tools` (их нет), добавить реальную структуру crates. `STATE.md` очистить от записей «Python scripts → Rust port ✅» (скрипты остались). | `CLAUDE.md`, `DESIGN.md`, `STATE.md` | S (2–3 часа) | Низкий |
| 3 | В `mcoa_core` создать конфигурируемый `WeightMap` (структуру `HashMap<(Counter, Tissue), f64>`) с загрузкой из `weights.toml` (assets). Реализовать публичную функцию `tissue_load(tissue, counter_damages: [f64; N_COUNTERS], weights: &WeightMap) -> f64`. Убрать жёстко закодированные веса из `mcoa_simulation`. | `crates/mcoa_core/src/lib.rs` (новый модуль `weights`), `crates/mcoa_core/assets/weights.toml`, `crates/mcoa_simulation/src/lib.rs` | M (1–2 дня) | Низкий: расширение API без слома существующих тестов |
| 4 | Удалить Python-скрипты `scripts/compare_mcoa_cdata.py` и `scripts/compare_all.py`. Перенести недостающую функциональность (plot, markdown reports) в `mcoa_compare` (уже есть). Если plot generation не нужен — просто удалить скрипты. | `scripts/compare_mcoa_cdata.py`, `scripts/compare_all.py` | S (1 час) | Низкий |
| 5 | Включить `backend/` в workspace корневого `Cargo.toml` (добавить `"backend"` в `members`). Удалить пустой `[workspace]` из `backend/Cargo.toml`. Привести версии зависимостей к workspace (`tokio -> "1"`, `axum -> "0.7"`, `serde` и т.д.). | Корневой `Cargo.toml`, `backend/Cargo.toml` | S (1–2 часа) | Средний: возможны конфликты версий; нужно прогнать `cargo build --release` |
| 6 | Реализовать валидацию тканей и парсинга в `mcoa_core` (один `FromStr` для `Tissue`), убрать дубликаты `parse_tissue` из `mcoa_cli` и `mcoa_api`. Заменить `unwrap()`/`unwrap_or(NaN)` на `Result` с `anyhow`/`thiserror` во всех crate. В API-хендлерах возвращать 400/500 с сообщением об ошибке. | `crates/mcoa_core/src/tissue.rs`, `crates/mcoa_cli/src/main.rs`, `crates/mcoa_api/src/main.rs`, `crates/mcoa_compare/src/lib.rs` | M (1 день) | Низкий |

## P1 — Важно

| # | Шаг | Файлы |
|---|-----|-------|
| 1 | Добавить `tracing` middleware в единый backend (создать `layer` для логирования запросов/ответов). | `backend/src/main.rs` (после объединения) |
| 2 | Наполнить пустой харнесс `mcoa_tests` интеграционными тестами: запуск симуляции + проверка, что `tissue_load` не падает и выдаёт ожидаемый диапазон. | `crates/mcoa_tests/src/lib.rs`, `crates/mcoa_tests/tests/` |
| 3 | Создать `mcoa_core/assets/weights.toml` со значениями из `PARAMETERS.md` (пометить `# placeholders`). Загружать его в `WeightMap` при инициализации. | `crates/mcoa_core/assets/`, `crates/mcoa_core/src/weights.rs` |
| 4 | Пройтись по всем `unwrap()` в коде (кроме тестов) и заменить на `?` / `.expect("...")` с контекстом. | `crates/mcoa_api/`, `crates/mcoa_cli/`, `crates/mcoa_compare/` |
| 5 | Убедиться, что `backend/migrations/` содержит хотя бы одну миграцию (создание таблиц counters, tissues). Если нет — создать. | `backend/migrations/` |
| 6 | Проверить и почистить `backend/Cargo.toml`: убрать `once_cell`, `config`, `dotenvy`, если не используются. | `backend/Cargo.toml` |
| 7 | Обновить `PARAMETERS.md` — отметить, что веса теперь хранятся в `weights.toml` и загружаются через `WeightMap`. | `PARAMETERS.md` |

## P2 — Nice-to-have

| # | Шаг | Файлы |
|---|-----|-------|
| 1 | Добавить GitHub Actions: `cargo build --release`, `cargo test`, `cargo clippy`, `cargo fmt --check` на каждый push/PR. | `.github/workflows/ci.yml` |
| 2 | Развернуть объединённый backend на сервере (через Dockerfile + docker-compose с PostgreSQL). | `Dockerfile`, `docker-compose.yml`, `README.md` |
| 3 | Реализовать бинар `mcoa-plot` (отдельный crate `mcoa_plot` или в `mcoa_compare`) с генерацией PNG-графиков через `plotters`. | `crates/mcoa_plot/` (новый) |
| 4 | Создать `ADTR.md` (Architecture Decision Records) для ключевых решений: объединение backend, весовая конфигурация, удаление Python. | `docs/ADTR/` |
| 5 | Настроить `pre-commit` hook для запуска `cargo clippy` и `cargo test` перед коммитом. | `.githooks/pre-commit` |

---

**Примечание:** Для P0 указаны трудоёмкость и риск; P1/P2 не требуют оценки по заданию. Все шаги учитывают жёсткое правило стека (Rust + Phoenix; Python удалён из main репозитория).