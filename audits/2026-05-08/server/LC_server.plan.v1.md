# План улучшений (actionable)

## P0 (блокеры) — требуют немедленного устранения

| # | Действие | Файлы | Трудоёмкость | Риск | Примечание |
|---|----------|-------|---------------|------|------------|
| 1 | **Добавить Phoenix LiveView приложение для фронтенда** <br>Создать каталог `frontend/` с новым Phoenix-проектом, который будет использовать существующий Rust API (через REST). В `Dockerfile` добавить multi-stage сборку для Phoenix (Elixir + Erlang). | Весь проект: новый каталог `frontend/`, доработка `Dockerfile`, `docker-compose.yml` | **L** (2–3 дня) | **Средний** – необходимо настроить интеграцию (CORS, прокси) | Удовлетворяет требованию стека `Rust (backend) + Phoenix LiveView (frontend)` |
| 2 | **Добавить HEALTHCHECK в Dockerfile** <br>```dockerfile HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 CMD curl -f http://localhost:8080/health || exit 1``` | `Dockerfile` | **S** (5 мин) | **Низкий** | Без HEALTHCHECK контейнер не управляется оркестратором |
| 3 | **Удалить неиспользуемую зависимость `reqwest`** (или реализовать интеграцию с DeepSeek) <br>Если DeepSeek не нужна сейчас – удалить строку `reqwest = ...` из `Cargo.toml` и из `src/` если есть реэкспорты. | `Cargo.tomл`, `src/lib.rs` (если есть `use reqwest`) | **S** (10 мин) | **Низкий** | Нарушение YAGNI; dead code увеличивает время сборки |

---

## P1 (важно) — улучшают качество и надёжность

| # | Действие | Файлы | Трудоёмкость |
|---|----------|-------|--------------|
| 4 | **Удалить `regex-lite` из зависимостей** <br>Проверить `git grep regex` — если не используется → удалить. | `Cargo.toml` | S |
| 5 | **Убрать закомментированную зависимость `tower_governor`** из `Cargo.toml` (оставить только в roadmap) | `Cargo.toml` | S |
| 6 | **Добавить middleware для rate limiting (заглушка)** <br>Создать `src/middleware/rate_limit.rs` с `tower_governor::GovernorLayer` (или аналогичной) и подключить через `ServiceBuilder`. | `Cargo.toml` (добавить `tower-governor`), `src/middleware/mod.rs`, `src/main.rs` | M |
| 7 | **Применить `tower::ServiceBuilder` для композиции middleware** <br>```rust let app = ServiceBuilder::new().layer(cors).layer(TraceLayer::new_for_http()).service(routes::all_routes(state));``` | `src/main.rs` | S |
| 8 | **Добавить `.dockerignore`** <br>Исключить `target/`, `.git`, `*.md`, `tests/`, `deploy/` | `.dockerignore` (новый файл) | S |
| 9 | **Вынести версию из `Cargo.toml` в переменную окружения** <br>Через `build.rs` сгенерировать константу `VERSION` на основе `env!("CARGO_PKG_VERSION")` и использовать в `Dockerfile` вместо hardcoded `v5.6`. | `Dockerfile`, `build.rs`, `src/main.rs` (логирование) | M |
| 10 | **Добавить строгую проверку обязательных переменных окружения при старте** <br>В `config.rs` валидировать `database_url`, `jwt_secret` и т.д., а не полагаться на `dotenvy().ok()`. | `src/config.rs`, `src/main.rs` | S |
| 11 | **Добавить fallback для CORS при пустом `allowed_origins`** <br>Если список пуст – установить `AllowOrigin::any()` и вывести `warn!`. | `src/main.rs` (функция `build_cors`) | S |

---

## P2 (nice-to-have) — улучшают сопровождаемость

| # | Действие | Файлы |
|---|----------|-------|
| 12 | **Добавить эндпоинт `/health` для проверки БД и внешних зависимостей** | `src/routes.rs`, новый `src/handlers/health.rs` |
| 13 | **Убрать `allow_credentials(true)` если не используется** (иначе CORS с `AllowOrigin::any()` несовместим) | `src/main.rs` |
| 14 | **Переместить roadmap-комментарии из `Cargo.toml` в отдельный `ROADMAP.md`** | `Cargo.toml`, новый `ROADMAP.md` |
| 15 | **Добавить интеграционные тесты для middlewar'ов (CORS, rate limit)** | `tests/` |
| 16 | **Добавить `tower-http` middleware `CompressionLayer` (gzip) – уже в зависимостях** | `src/main.rs` |

---

## Приоритеты выполнения

1. **P0** – заблокировать до внедрения (без Phoenix LiveView и healthcheck проект не принимается).
2. **P1** – выполнить сразу после P0, т.к. закрывают явные дыры в надёжности.
3. **P2** – по желанию, в порядке уменьшения трудоёмкости.