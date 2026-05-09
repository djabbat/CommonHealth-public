## План улучшений на основе peer review

### P0 — Блокеры (без них проект неработоспособен или критически некорректен)

1. **Реализовать недостающие модули backend-сервера**  
   Создать `routes.rs`, `models.rs`, `db.rs`, `error.rs`, `config.rs` с полным CRUD (Counters, Measurements, Parameters) и подключением к PostgreSQL.  
   **Файлы:** `backend/src/routes.rs`, `models.rs`, `db.rs`, `error.rs`, `config.rs` (новые); `backend/src/main.rs` (адаптация).  
   **Трудоёмкость:** L | **Риск:** high (без них бэкенд не запускается).

2. **Создать обязательные модули в crate epigenetic_counter**  
   Реализовать `tissue.rs` (enum Tissue) и `trajectory.rs` (структура `TrajectoryRequest` + функция `run_trajectory`).  
   **Файлы:** `crates/epigenetic_counter/src/tissue.rs`, `trajectory.rs`.  
   **Трудоёмкость:** M | **Риск:** medium (без них crate не компилируется).

3. **Синхронизировать значения параметров между PARAMETERS.md и кодом**  
   Установить `tau_days = 3650` (10 лет), `alpha = 0.05` (в соответствии с канонической таблицей).  
   **Файлы:** `crates/epigenetic_counter/src/lib.rs` (строки 44-50), `PARAMETERS.md` (если требуется уточнение).  
   **Трудоёмкость:** S | **Риск:** low (несоответствие ведёт к неверным симуляциям).

4. **Переписать DESIGN.md в соответствии с реальным стеком (Rust + Phoenix)**  
   Удалить устаревшее описание Python-архитектуры, заменить на актуальные модули backend, frontend, crate.  
   **Файл:** `DESIGN.md`.  
   **Трудоёмкость:** S | **Риск:** low (иначе документация дезориентирует).

5. **Реализовать отсутствующие LiveView модули frontend**  
   Создать `DashboardLive`, `DetailLive`, `CounterRegistryLive`, `SobolSensitivityLive`, `HSCTrackingLive` (минимум скелетные реализации + шаблоны .heex).  
   **Файлы:** `frontend/lib/epigeneticdrift_frontend_web/live/` (5 новых файлов); update `router.ex`.  
   **Трудоёмкость:** L | **Риск:** high (без них фронтенд не работает, маршруты ведут в никуда).

---

### P1 — Важно (существенно повышают качество, без проекта можно запустить, но с рисками)

1. **Исправить Dockerfile и неиспользуемые зависимости**  
   Установить `wget` или заменить `HEALTHCHECK` на `curl`; удалить/добавить конфиги для sentry/dialyxir.  
   **Файлы:** `backend/Dockerfile`, `frontend/mix.exs`.  
   **Трудоёмкость:** S | **Риск:** low.

2. **Добавить скелетные тесты для backend и frontend**  
   Написать хотя бы 1 интеграционный тест для health endpoint (Rust) и 1 LiveView тест (Elixir).  
   **Файлы:** `backend/tests/health_test.rs` (новый), `frontend/test/epigeneticdrift_frontend_web/controllers/health_controller_test.exs` (новый).  
   **Трудоёмкость:** M | **Риск:** low.

3. **Исправить опечатки и несоответствия в README**  
   Привести имя БД к единому виду (`epigeneticdrift_db` → `epigenetic_drift_db`); явно задать порт frontend (4007) в конфигах.  
   **Файлы:** `backend/README.md`, `frontend/README.md`, `frontend/config/*.exs`.  
   **Трудоёмкость:** S | **Риск:** low.

4. **Создать ROADMAP.md**  
   На основе OPEN_PROBLEMS.md сформулировать план зависимых этапов (приоритеты, временные оценки).  
   **Файл:** `ROADMAP.md`.  
   **Трудоёмкость:** S | **Риск:** low.

---

### P2 — Nice-to-have (улучшения без срочности)

1. **Обновить phoenix_live_view до 0.20+**  
   Изменить версию и проверить совместимость.  
   **Файл:** `frontend/mix.exs`.  
   **Трудоёмкость:** S | **Риск:** low.

2. **Добавить CI (GitHub Actions)**  
   Собрать, протестировать, пролинтовать оба компонента.  
   **Файл:** `.github/workflows/ci.yml` (новый).  
   **Трудоёмкость:** S | **Риск:** low.

3. **Дополнить документацию примерами API и развёртывания**  
   Расширить `README.md` бэкенда и фронтенда.  
   **Файлы:** `backend/README.md`, `frontend/README.md`.  
   **Трудоёмкость:** S | **Риск:** low.