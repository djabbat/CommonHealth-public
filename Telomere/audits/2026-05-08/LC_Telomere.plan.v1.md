## План улучшений LC_Telomere (на основе peer review)

Приоритеты:
- **P0** – блокеры: без исправления проект не может быть интегрирован/работоспособен. Для каждого P0 указана трудоёмкость (S/M/L) и риск.
- **P1** – важно: заметно повышает качество, устраняет мелкие ошибки, упрощает поддержку.
- **P2** – nice-to-have: полезные улучшения без критической срочности.

---

### P0 (блокеры)

**P0.1 – Согласовать параметры модели между PARAMETERS.md, `crates/telomere_counter` и backend**  
Трудоёмкость: **M**  
Риск: Без этого симуляции бессмысленны, весь код модели невалиден.  
- Перенести значения из `PARAMETERS.md` в единый конфигурационный файл (например, `parameters.toml`) и загружать его в `CounterParams::default()`.  
- Привести `alpha`, `beta`, `tau_days` к единицам документации (bp/PD, bp, yr).  
- Убедиться, что backend использует те же параметры через импорт crate.  
Затронутые файлы: `PARAMETERS.md`, `crates/telomere_counter/src/lib.rs`, `backend/src/models.rs`, возможно новый `parameters.toml`.

**P0.2 – Переработать DESIGN.md: удалить Python-код, заменить на архитектурную схему**  
Трудоёмкость: **S**  
Риск: Низкий, документ дезориентирует разработчиков.  
- Убрать класс `TelomereCounter` на Python.  
- Описать архитектуру компонентов (crate ↔ backend ↔ frontend) на уровне диаграмм или псевдокода без привязки к языку.  
Затронутые файлы: `DESIGN.md`.

**P0.3 – Подключить backend к библиотеке `crates/telomere_counter`**  
Трудоёмкость: **M**  
Риск: Устраняет дублирование доменной логики и рассинхронизацию.  
- Добавить зависимость в `backend/Cargo.toml`: `telomere_counter = { path = "../crates/telomere_counter" }`.  
- Перенести структуры `CounterState`, `CounterParams`, функции `compute_damage` и `is_above_critical` из `backend/src/models.rs` в crate и использовать их в роутах backend.  
Затронутые файлы: `backend/Cargo.toml`, `backend/src/models.rs`, `backend/src/routes.rs`, `crates/telomere_counter/src/lib.rs`.

**P0.4 – Добавить базовые тесты (unit + health-check smoke)**  
Трудоёмкость: **S**  
Риск: Невозможно верифицировать работоспособность API.  
- Написать unit-тесты для `CounterParams::validate()` и `compute_damage()` в crate.  
- Добавить интеграционный тест в backend: `GET /health` → 200.  
Затронутые файлы: `crates/telomere_counter/src/lib.rs` (добавить `#[cfg(test)]`), `backend/tests/health_test.rs` (новый файл).

**P0.5 – Соединить frontend с backend (HTTP-клиент в Phoenix)**  
Трудоёмкость: **M**  
Риск: Frontend бесполезен без данных.  
- В Phoenix-проекте настроить HTTP-клиент (использовать `Req`) в контексте `TelomereCounter` для вызова API backend.  
- В LiveView `DashboardLive` загрузить данные через этот клиент и отобразить.  
Затронутые файлы: `frontend/lib/telomere_frontend/telomere_client.ex` (новый), `frontend/lib/telomere_frontend_web/live/dashboard_live.ex`.

**P0.6 – Разрешить конфликт нумерации Counter #2 с CDATA**  
Трудоёмкость: **S**  
Риск: Блокирует интеграцию в MCOA.  
- Согласовать с user/командой: либо переименовать Telomere в `#2a` или другой номер, либо изменить CDATA.  
- Обновить `CLAUDE.md` и `CONCEPT.md` после решения.  
Затронутые файлы: `CLAUDE.md`, `CONCEPT.md`, `backend/src/routes.rs` (список counters).

---

### P1 (важно)

**P1.1 – Дописать Dockerfile**  
- Добавить `CMD ["telomere_server"]` или `ENTRYPOINT`.  
Затронутые файлы: `backend/Dockerfile`.

**P1.2 – Документировать или удалить `scripts/calibrate.py`**  
- Добавить `README` в `scripts/` или интеграцию с Rust через PyO3; если не нужно – удалить.  
Затронутые файлы: `scripts/README.md`, `scripts/calibrate.py`.

**P1.3 – Добавить LICENSE файл (MIT)**  
- Создать `LICENSE` с текстом MIT-лицензии.  
Затронутые файлы: `LICENSE` (новый).

**P1.4 – Убрать неиспользуемые зависимости из backend/Cargo.toml**  
- Удалить `bb8-postgres`, `argon2` (если не используются).  
Затронутые файлы: `backend/Cargo.toml`.

**P1.5 – Добавить `--help` в CLI crate**  
- Использовать `clap` или вручную выводить usage при флагах `--help`/`-h`.  
Затронутые файлы: `crates/telomere_counter/src/main.rs`.

**P1.6 – Улучшить обработку ошибок CLI**  
- Заменить `eprintln` + `exit(2)` на `clap::Error` или более структурированный вывод.  
Затронутые файлы: `crates/telomere_counter/src/main.rs`.

**P1.7 – Провести ревью единиц измерения и дублирования параметров**  
- Убедиться, что в коде `alpha`, `beta` и `tau_days` имеют явные комментарии с единицами (например, `// bp/PD`).  
- Проверить, что backend не держит второй копии параметров.  
Затронутые файлы: `crates/telomere_counter/src/lib.rs`, `backend/src/models.rs`.

---

### P2 (nice-to-have)

**P2.1 – Внедрить CI (GitHub Actions)**  
- Добавить файл `.github/workflows/ci.yml`: build+test для Rust и Elixir.  
Затронутые файлы: `.github/workflows/ci.yml` (новый).

**P2.2 – Автоматическая кодогенерация параметров из `parameters.toml`**  
- Написать build-скрипт (build.rs) или хук, генерирующий константы `CounterParams` из единого источника.  
Затронутые файлы: `crates/telomere_counter/build.rs` (новый), `parameters.toml`.

**P2.3 – Интеграционные тесты для API endpoint’ов**  
- Добавить тесты в `backend/tests/` с использованием `reqwest` для проверки CRUD measurement/parameters.  
Затронутые файлы: `backend/tests/api_test.rs` (новый).

**P2.4 – Логирование запросов с метриками**  
- Подключить tracing + metrics (например, `metrics-exporter-prometheus`) к Axum-роутам.  
Затронутые файлы: `backend/src/main.rs`, `backend/Cargo.toml`.

**P2.5 – Health check с проверкой БД**  
- Заменить простой `200 OK` на проверку соединения с PostgreSQL.  
Затронутые файлы: `backend/src/routes.rs`.