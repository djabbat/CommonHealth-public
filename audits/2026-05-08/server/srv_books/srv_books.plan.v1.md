## План улучшений `srv_books`

Проект полностью пуст и не соответствует стеку. Требуется создать серверное приложение с нуля, строго соблюдая архитектуру: **Rust** для высоконагруженного бэкенда, **Phoenix (Elixir)** для веб-слоя и LiveView. **Python** допустим только для legacy OCR/PDF и AIM ML-роутера (изолированные микросервисы). Ниже — конкретные шаги по восстановлению.

---

### P0 — Блокеры (обязательно к выполнению)

| # | Действие | Затронутые файлы | Трудоёмкость | Риск |
|---|---------|-----------------|--------------|------|
| 1 | **Инициализировать Rust-бэкенд** – создать `Cargo.toml` с зависимостями (actix-web/tokio, serde, sqlx) и минимальный `src/main.rs` с `GET /health`. | `Cargo.toml`, `src/main.rs`, `src/lib.rs` | M | Средний – неправильный выбор веб-фреймворка может затянуть |
| 2 | **Инициализировать Phoenix-фронтенд** – выполнить `mix phx.new srv_books --live`, создать базовую структуру с LiveView и конфигурацией HTTP (порт, endpoint). | `mix.exs`, `lib/srv_books_web/endpoint.ex`, `config/config.exs`, `lib/srv_books_web/router.ex` | M | Средний – нужна совместимость версий Elixir/OTP |
| 3 | **Настроить сборку и запуск** – добавить `Makefile` / `docker-compose.yml` с описанием сервисов (rust-backend, phoenix-web, postgres). | `Makefile`, `docker-compose.yml` | S | Низкий – стандартная конфигурация |
| 4 | **Удалить мусор** – очистить рабочую директорию от `index.html.bak.2026-05-07`, `favicon.*`. | (удаляемые файлы) | S | Низкий |
| 5 | **Проверить runtime** – установить Rust toolchain и Elixir/Erlang на сервер, добавить systemd unit для каждого сервиса. | `/etc/systemd/system/books-backend.service`, `/etc/systemd/system/books-web.service` | M | Высокий – проблемы с окружением (отсутствие нужных версий) |

---

### P1 — Важно (следующий приоритет)

| # | Действие | Затронутые файлы |
|---|---------|-----------------|
| 6 | **Реализовать API-маршрутизацию** – в Rust-бэкенде добавить эндпоинты: `GET /books`, `POST /books`, `GET /books/:id`. Ответы в JSON. | `src/routes/mod.rs`, `src/models/book.rs`, `src/handlers/books.rs`, `Cargo.toml` (добавить сериализацию) |
| 7 | **Добавить интеграцию с БД** – создать миграции (sqlx) для таблицы `books` (id, title, author, content_text, created_at). | `migrations/20260508_initial.sql`, `src/db.rs`, `config/database.toml` (или переменные окружения) |
| 8 | **Создать LiveView компонент** – список книг с поиском и пагинацией через Phoenix LiveView. | `lib/srv_books_web/live/book_live/index.ex`, `lib/srv_books_web/templates/book/index.html.heex`, `lib/srv_books_web/router.ex` |
| 9 | **Подключить Python-сервис OCR** – оформить как отдельный микросервис (FastAPI) с эндпоинтом `/ocr/upload`, использовать для обработки PDF/изображений книг. | `ocr_service/main.py`, `ocr_service/requirements.txt`, `ocr_service/Dockerfile`, `docker-compose.yml` (добавить сервис) |
| 10 | **Настроить межсервисное взаимодействие** – Rust-бэкенд вызывает Python OCR по gRPC или HTTP, Phoenix-фронтенд получает статус через WebSocket. | `src/ocr_client.rs`, `lib/srv_books_web/channels/ocr_channel.ex`, `config/ocr_config.exs` |

---

### P2 — Nice-to-have (улучшения)

| # | Действие | Затронутые файлы |
|---|---------|-----------------|
| 11 | **Добавить метрики и логирование** – Prometheus+grafana для Rust, Sentry для Elixir. | `src/metrics.rs`, `lib/srv_books_web/telemetry.ex`, `docker-compose.yml` (добавить prometheus) |
| 12 | **Реализовать кэширование** – Redis для частых запросов (список книг). | `src/cache.rs`, `Cargo.toml` (добавить redis), `config/config.exs` (добавить redis_url) |
| 13 | **Добавить CI/CD** – GitHub Actions для сборки, тестов и деплоя. | `.github/workflows/ci.yml`, `.github/workflows/deploy.yml` |
| 14 | **Документация** – `README.md` с описанием архитектуры, запуска и API. | `README.md` |
| 15 | **Тесты** – unit-тесты для Rust (cargo test), тесты Phoenix (mix test). | `src/***.rs` (тесты внутри модулей), `test/srv_books_web/**` |

---

**Ключевые файлы, которые должны появиться к концу P0:**  
`Cargo.toml`, `src/main.rs`, `mix.exs`, `lib/srv_books_web/endpoint.ex`, `config/config.exs`, `docker-compose.yml`, `Makefile`.

**Риски P0:**  
- Отсутствие runtime на сервере (Rust, Elixir). **Решение:** заранее проверить установку.  
- Конфликт портов (если legacy index.html занимал порт 80). **Решение:** указать нестандартный порт в Phoenix (например, 4000), а Rust-бэкенд на 8080.  
- Несовместимость версий библиотек. **Решение:** фиксировать версии в `Cargo.toml` и `mix.exs`.

План выполняется последовательно: P0 → P1 → P2. Оценка сроков выполнения P0 — 1–2 рабочих дня (с учетом установки toolchain).