# План улучшений LC_Ontogenesis

## P0 — Блокеры (необходимо для прохождения ревью)

### P0.1 Инициализировать Rust-проект с базовой структурой
- Создать `Cargo.toml` с метаданными, зависимостями (например, `actix-web` или `axum` для HTTP-сервера, `serde`, `tokio`).
- Создать `src/main.rs` (или `lib.rs`) с минимальной точкой входа и примитивным health-check endpoint.
- **Затронутые файлы:** `Cargo.toml`, `src/main.rs`
- **Трудоёмкость:** S (1–2 часа)
- **Риск:** Низкий (стандартная процедура)

### P0.2 Создать Phoenix LiveView-приложение для frontend
- Выполнить `mix phx.new ontogenesis_front --live`, настроить сборку в корне пакета или в отдельной папке `frontend/`.
- Убедиться, что `mix.exs` и все конфигурации присутствуют.
- Добавить простой LiveView (например, заглушку с `render`).
- **Затронутые файлы:** `frontend/mix.exs`, `frontend/lib/ontogenesis_front_web.ex`, `frontend/config/`, `frontend/assets/`
- **Трудоёмкость:** M (1–2 дня с учётом настройки)
- **Риск:** Средний (необходимо проверить совместимость версий Elixir/Erlang)

### P0.3 Определить Python-пакеты для legacy OCR/PDF и ML-роутера
- Создать `scripts/ocr_pdf/` с `requirements.txt`, `main.py` (заглушка функции).
- Создать `scripts/ml_router/` с `requirements.txt`, `main.py` (заглушка изолированного сервиса).
- Добавить `README.md` с указанием, что Python используется только для этих узких задач.
- **Затронутые файлы:** `scripts/ocr_pdf/requirements.txt`, `scripts/ocr_pdf/main.py`, `scripts/ml_router/requirements.txt`, `scripts/ml_router/main.py`
- **Трудоёмкость:** S (2–3 часа)
- **Риск:** Низкий

### P0.4 Наполнить `data/` хотя бы одним тестовым файлом-образцом
- Создать `data/ontogenesis_example.json` с минимальной схемой (например, `{"version":1}`).
- **Затронутый файл:** `data/ontogenesis_example.json`
- **Трудоёмкость:** S (15 минут)
- **Риск:** Отсутствует

### P0.5 Добавить лицензию и README с описанием
- Создать `LICENSE` (MIT).
- Создать `README.md` с целью пакета, как собрать, как запустить Rust и Phoenix части, где вызываются Python-скрипты.
- **Затронутые файлы:** `LICENSE`, `README.md`
- **Трудоёмкость:** S (1 час)
- **Риск:** Отсутствует

---

## P1 — Важно (повышают качество, но не блокируют прохождение)

### P1.1 Реализовать минимальный OCR-пайплайн на Python (заглушка с эмуляцией)
- В `scripts/ocr_pdf/main.py` добавить функцию, принимающую путь к PDF и возвращающую распознанный текст (пока захардкожен).
- Написать тест (`test_ocr.py`).
- **Затронутые файлы:** `scripts/ocr_pdf/main.py`, `scripts/ocr_pdf/test_ocr.py`
- **Трудоёмкость:** S (3–4 часа)
- **Риск:** Низкий

### P1.2 Реализовать ML-роутер с базовой маршрутизацией
- В `scripts/ml_router/main.py` реализовать REST-эндпоинт (Flask или FastAPI), который принимает тип задачи и возвращает результат заглушки.
- Добавить `Dockerfile` для каждого Python-сервиса (опционально, но повышает воспроизводимость).
- **Затронутые файлы:** `scripts/ml_router/main.py`, `scripts/ml_router/Dockerfile`, `scripts/ocr_pdf/Dockerfile`
- **Трудоёмкость:** M (1 день)
- **Риск:** Средний (зависимости Python)

### P1.3 Настроить CI (GitHub Actions) для автоматической проверки сборки Rust и Elixir
- `.github/workflows/ci.yml` с шагами: установка Rust, проверка `cargo build`, установка Elixir, `mix deps.get && mix compile`.
- **Затронутый файл:** `.github/workflows/ci.yml`
- **Трудоёмкость:** S (2–3 часа)
- **Риск:** Низкий

### P1.4 Интегрировать Phoenix LiveView с Rust-бекендом через HTTP/gRPC
- В `frontend/lib/ontogenesis_front_web/live/` создать LiveView, который вызывает Rust-сервис (пока заглушка).
- В Rust-бекенде добавить `/api/status` endpoint.
- **Затронутые файлы:** `frontend/lib/ontogenesis_front_web/live/status_live.ex`, `src/main.rs`
- **Трудоёмкость:** M (1–2 дня)
- **Риск:** Средний (синхронизация API)

---

## P2 — Nice-to-have (улучшают архитектуру, но не критичны)

### P2.1 Добавить тесты для Rust-модулей
- Написать unit-тесты в `src/` (например, `mod tests`).
- **Затронутый файл:** `src/main.rs` (или отдельный `tests/`).
- **Трудоёмкость:** S (2–3 часа)
- **Риск:** Низкий

### P2.2 Создать Makefile для сквозных команд
- `Makefile` с целями `build-rust`, `build-phoenix`, `run-all`, `test`.
- **Затронутый файл:** `Makefile`
- **Трудоёмкость:** S (1 час)
- **Риск:** Низкий

### P2.3 Документировать API в OpenAPI/Swagger
- Для Rust-бекенда подключить `utoipa` или `paperclip` и сгенерировать спецификацию.
- **Затронутые файлы:** `Cargo.toml`, `src/main.rs`
- **Трудоёмкость:** M (1 день)
- **Риск:** Средний (дополнительная зависимость)

### P2.4 Добавить pre-commit хуки (clippy, formatter)
- `.pre-commit-config.yaml` с запуском `cargo clippy`, `mix format`.
- **Затронутый файл:** `.pre-commit-config.yaml`
- **Трудоёмкость:** S (30 минут)
- **Риск:** Низкий

---

## Резюме по P0 (оценки и риски)

| # | Пункт | Трудоёмкость | Основной риск |
|---|-------|--------------|---------------|
| P0.1 | Инициализация Rust | S (2ч) | Нет |
| P0.2 | Phoenix LiveView | M (2д) | Версии Elixir/Erlang |
| P0.3 | Python скрипты | S (3ч) | Нет |
| P0.4 | Data-образец | S (15мин) | Нет |
| P0.5 | README + лицензия | S (1ч) | Нет |

**Итого на P0:** ~3–4 дня (при полном рабочем дне). Для прохождения ревью достаточно выполнить все P0. P1–P2 могут быть отложены, но без P0 ревью будет повторно REJECT.