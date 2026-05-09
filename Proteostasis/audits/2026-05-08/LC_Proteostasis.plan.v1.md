# План улучшений Proteostasis (peer review MAJOR_REVISION)

## P0 – Блокеры (оценка трудоёмкости + риск)

1. **Единый источник параметров модели**  
   Создать `config/parameters.toml` с единственными значениями (из PARAMETERS.md). Загружать в `crates/proteostasis_counter` через serde, в бэкенд – через `sqlx` seed или конфиг. Удалить жёстко закодированные дефолты в `lib.rs` и `backend/README.md`.  
   **Файлы:** `crates/proteostasis_counter/src/lib.rs` (убрать default), `backend/Cargo.toml` (добавить serde+toml), `config/parameters.toml` (новый), `PARAMETERS.md` (дополнить ссылкой на toml).  
   **Трудоёмкость:** S (2–4 часа). **Риск:** низкий – механическая замена.

2. **Интегрировать crate как библиотеку в backend**  
   Сделать `crates/proteostasis_counter` зависимостью бэкенда через workspace: `backend/Cargo.toml` → `proteostasis-counter = { path = "../crates/proteostasis_counter" }`. Переписать endpoint `POST /proteostasis/compute` на вызов `proteostasis_counter::compute_damage`. Удалить дублирующую логику из `backend/src/routes/mod.rs`.  
   **Файлы:** `backend/Cargo.toml`, `backend/src/routes/proteostasis.rs`, `crates/proteostasis_counter/Cargo.toml` (публичная lib).  
   **Трудоёмкость:** M (4–8 часов). **Риск:** средний – требуется рефакторинг обработчиков, возможны конфликты типов.

3. **Unit-тесты для вычислительного ядра**  
   Написать тесты в `crates/proteostasis_counter/tests/test_compute.rs`:  
   - `compute_damage` с известными n,t,coupling → проверка по формуле из THEORY.md.  
   - `is_above_critical` с граничными значениями.  
   - Параметры берутся из единого TOML.  
   **Файлы:** `crates/proteostasis_counter/tests/test_compute.rs`, `Cargo.toml` крейта (добавить `[dev-dependencies]`).  
   **Трудоёмкость:** S (2–3 часа). **Риск:** низкий – тесты пишутся по спецификации.

4. **Реализовать базовый LiveView компонент (Dashboard)**  
   Создать `DashboardLive` с отображением параметров из API (запрос к backend) и формой для вычисления D₅. Использовать `Phoenix.LiveView` с assign и handle_event.  
   **Файлы:** `frontend/lib/proteostasis_frontend_web/live/dashboard_live.ex`, `dashboard_live.html.heex`, `frontend/lib/proteostasis_frontend_web/router.ex` (уже есть).  
   **Трудоёмкость:** M (6–10 часов). **Риск:** средний – требует понимания LiveView и интеграции с REST.

5. **Привести DESIGN.md в соответствие с реализацией**  
   Заменить описание Python-пакета на описание текущей Rust/Elixir архитектуры: workspace, crates, backend (Axum+SQLx), frontend (Phoenix LiveView). Указать путь к `config/parameters.toml`. Удалить ссылки на `pyproject.toml`, `calibrate.py` (оставить только как legacy).  
   **Файлы:** `DESIGN.md` (полный rewrite).  
   **Трудоёмкость:** S (2–3 часа). **Риск:** низкий – документация.

---

## P1 – Важно

6. **Валидация конфигурации при запуске backend**  
   В `backend/src/config.rs` добавить проверки: `DATABASE_URL` не пустой, `PORT` в [1024..65535], `LOG_LEVEL` – известное значение (info, debug, error). При ошибке выводить сообщение и завершаться с exit code 1.  
   **Файлы:** `backend/src/config.rs`.  
   **Трудоёмкость:** S (1 час).

7. **Исправить HEALTHCHECK в Dockerfile**  
   Заменить `CMD ["curl", "-f", "http://localhost:3008/health"]` на `CMD ["sh", "-c", "curl -f http://localhost:${PORT:-3008}/health"]` или передать порт через ARG.  
   **Файлы:** `backend/Dockerfile`.  
   **Трудоёмкость:** S (0.5 часа).

8. **Удалить неиспользуемые зависимости**  
   Из `backend/Cargo.toml` убрать `argon2`, `bb8-postgres`, если они не задействованы. Проверить наличие вызовов в коде.  
   **Файлы:** `backend/Cargo.toml`.  
   **Трудоёмкость:** S (0.5 часа).

9. **Добавить CI (GitHub Actions)**  
   Создать `.github/workflows/ci.yml`:  
   - Сборка workspace (`cargo build --all`),  
   - Запуск тестов (`cargo test --all`),  
   - Линтинг (`cargo clippy --all-targets`),  
   - Форматирование (`cargo fmt --check`).  
   Для frontend – `mix format --check-formatted` (опционально).  
   **Файлы:** `.github/workflows/ci.yml`.  
   **Трудоёмкость:** S (2 часа).

10. **Обновить backend/README.md**  
    - Указать актуальные параметры из `config/parameters.toml`.  
    - Описать процедуру калибровки (скрипт `scripts/calibrate.py`).  
    - Добавить раздел "Development setup" с командами для запуска миграций и тестов.  
    **Файлы:** `backend/README.md`.  
    **Трудоёмкость:** S (1–2 часа).

---

## P2 – Nice-to-have

11. **Объединить CONCEPT.md и THEORY.md**  
    Перенести аксиоматику и кинетику из THEORY.md в соответствующий раздел CONCEPT.md, удалить THEORY.md (или сделать его коротким summary). Обновить ссылки.  
    **Файлы:** `CONCEPT.md`, `THEORY.md`.  
    **Трудоёмкость:** S (1–2 часа).

12. **Улучшить обработку ошибок в CLI crate**  
    Заменить `expect()` на возврат `Result` с `eprintln!` и `process::exit(1)`. Добавить хелп по флагам.  
    **Файлы:** `crates/proteostasis_counter/src/main.rs`.  
    **Трудоёмкость:** S (1 час).

13. **Настроить логирование для frontend**  
    В `config/config.exs` добавить уровень логов (например, `config :logger, level: :info`). Подключить `telemetry` интеграцию с LiveView.  
    **Файлы:** `frontend/config/config.exs`, `frontend/lib/proteostasis_frontend_web/telemetry.ex`.  
    **Трудоёмкость:** S (1–2 часа).

14. **Добавить скрипт/CI-шаг проверки CORRECTIONS**  
    Написать простой Python/Rust скрипт, который проверяет:  
    - Все `.md` не содержат упоминаний `Health Score`, `χ_Ze`, `MCOA Test 2` (по AGENTS.md).  
    - В коде `gamma` по умолчанию равен 0.0.  
    **Файлы:** `scripts/check_corrections.py`, `.github/workflows/ci.yml` (добавить шаг).  
    **Трудоёмкость:** S (2–3 часа).

---

## Приоритет выполнения

1. **Неделя 1** – P0: 1, 3, 5 (параметры, тесты, дизайн).  
2. **Неделя 2** – P0: 2, 4 (интеграция крейта, фронтенд).  
3. **Неделя 3** – P1: 6–10.  
4. **Неделя 4** – P2: 11–14.

После завершения P0 провести повторный аудит (mini-review) для подтверждения стабильности.