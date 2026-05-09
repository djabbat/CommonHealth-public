## План улучшений для MitoROS

### P0 — Блокеры (проект неработоспособен или консистентность разрушена)

| # | Пункт | Трудоёмкость | Риск | Затронутые файлы |
|---|-------|-------------|------|------------------|
| 1 | **Привести архитектурную документацию в соответствие с кодом** <br>• `DESIGN.md`: заменить Python-дерево на Rust/Elixir-архитектуру, указать, что `mito_ros_counter` — библиотека, backend — Axum-сервис, frontend — LiveView-заглушка. <br>• `CLAUDE.md`: удалить строку «Web/server presence: нет», указать фактический стек (Rust+Phoenix+LiveView). | S | Низкий | `DESIGN.md` (полностью), `CLAUDE.md` (строка 11) |
| 2 | **Интегрировать crate `mito_ros_counter` в backend и реализовать эндпоинт `/api/compute_d3`** <br>• Добавить `mito_ros_counter` как зависимость в `backend/Cargo.toml` <br>• Создать модуль `backend/src/routes/compute.rs`, вызывающий `compute_damage` <br>• Зарегистрировать маршрут в `routes::api_routes()` | M | Средний | `backend/Cargo.toml`, `backend/src/routes.rs` (новый файл `routes/compute.rs`), `backend/src/main.rs` |
| 3 | **Синхронизировать параметры модели и вынести в единый конфиг** <br>• Создать `config/model_params.toml` с набором из `PARAMETERS.md` <br>• В `mito_ros_counter` загружать параметры из этого файла (удалить `Default::default()` с жёсткими значениями) <br>• Убрать противоречивые значения из `backend/README.md` (заменить на ссылку на конфиг) | M | Средний | `config/model_params.toml`, `crates/mito_ros_counter/src/lib.rs` (`Default` impl), `backend/README.md` |
| 4 | **Привести frontend в рабочее состояние или явно маркировать как заглушку** <br>• Создать минимальные модули `DashboardLive` и `DetailLive` (пустые LiveView, рендерящие статический текст) <br>• Или, если нет API — удалить LiveView роуты и оставить только заглушку `/`. | S | Низкий | `frontend/lib/mitoros_frontend_web/router.ex`, новые файлы: `dashboard_live.ex`, `detail_live.ex` |
| 5 | **Удалить все заглушки (`…<truncated …>`, `TODO`, `...`)** <br>• Завершить `crates/mito_ros_counter/src/lib.rs` (дописать оставшиеся 66 строк) <br>• Проверить все файлы на наличие недопустимых placeholder-ов | S | Низкий | `crates/mito_ros_counter/src/lib.rs`, `backend/README.md`, другие файлы с `...` |
| 6 | **Легализовать Python-скрипт или перенести на Rust** <br>• Если `calibrate.py` — ML/OCR-роутер (допустимый Python), добавить исключение в `CLAUDE.md` с явным указанием области <br>• Иначе переписать на Rust и поместить в `crates/mito_ros_counter/src/bin/calibrate.rs` | S | Низкий | `scripts/calibrate.py`, `CLAUDE.md` (правило стека) |
| 7 | **Добавить unit-тесты для ключевой функции `compute_damage`** <br>• Создать модуль `tests/test_lib.rs` в crate, покрыть пограничные случаи (α=0, β=0, n_star=0 → panic, d0<0 и т.д.) <br>• В `backend` добавить интеграционный тест для нового эндпоинта | M | Низкий | `crates/mito_ros_counter/tests/test_lib.rs`, `backend/tests/api_test.rs` |

---

### P1 — Важно (повышают качество, необходимы для production-readiness)

| # | Пункт | Затронутые файлы |
|---|-------|------------------|
| 1 | **Создать единый `ARCHITECTURE.md`** с диаграммой компонентов (crate → backend → frontend), описание контрактов и потоков данных. | `ARCHITECTURE.md` (новый) |
| 2 | **Реализовать хотя бы один CRUD-эндпоинт из `backend/README.md`** (например, `POST /api/tissues`) с валидацией и ответом по JSON:API. | `backend/src/routes/tissues.rs`, `backend/src/main.rs` |
| 3 | **Добавить CI (GitHub Actions)** для запуска тестов и линтинга (`cargo test`, `cargo clippy`). | `.github/workflows/ci.yml` |
| 4 | **Создать `JOURNAL.md`** и добавить первую запись: «2026-05-08: Приведение кода к канону после peer review». | `JOURNAL.md` |
| 5 | **Добавить CI-шаг для автоматической верификации PMID** (скрипт на Python или Rust, запускается еженедельно). | `.github/workflows/verify_refs.yml`, `scripts/verify_pmid.py` (Rust-версия) |
| 6 | **Исправить ошибку в `CLAUDE.md` про Counter #2** (Centriolar/Telomere конфликт) — уточнить, что Telomere — #2, Centriolar — #1. | `CLAUDE.md` |

---

### P2 — Nice-to-have (улучшения для удобства и расширяемости)

| # | Пункт | Затронутые файлы |
|---|-------|------------------|
| 1 | **Добавить обработку ошибок во всех обработчиках backend** (кастомные error response с `AppError`). | `backend/src/error.rs` (уже есть, доработать) |
| 2 | **Добавить DEBUG-логирование для входящих запросов** (через `tracing::debug`). | `backend/src/main.rs`, `backend/src/routes/*.rs` |
| 3 | **Реализовать полноценную LiveView-панель Dashboard** после готовности API (график D3(t), форма ввода параметров). | `frontend/lib/mitoros_frontend_web/dashboard_live.ex`, `frontend/lib/mitoros_frontend_web/dashboard_live.html.heex` |
| 4 | **Настроить Docker Compose** для поднятия PostgreSQL + backend + frontend одной командой. | `docker-compose.yml`, `frontend/Dockerfile` |
| 5 | **Покрыть интеграционными тестами CRUD-эндпоинты** (минимум 2-3 сценария). | `backend/tests/integration_test.exs` (Elixir) или `backend/tests/*.rs` (Rust) |

---

**Примечание по стеку:** Весь код, кроме указанных исключений (legacy OCR/PDF, ML-роутер), должен быть только на Rust (backend) и Elixir/Phoenix (frontend). Python-скрипт `calibrate.py` после легализации остаётся, но не расширяется.