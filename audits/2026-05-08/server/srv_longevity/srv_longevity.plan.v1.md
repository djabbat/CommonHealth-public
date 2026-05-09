## План улучшений srv_longevity (по результатам peer review)

Приоритеты: **P0** (блокеры) → **P1** (важно) → **P2** (nice-to-have).  
Все пункты учитывают обязательный стек **Rust (backend) + Phoenix LiveView (frontend)**; Python — только для legacy OCR/PDF и AIM ML-роутера.

---

### P0 — Блокеры (без них проект неприемлем)

1. **Миграция core-логики на Rust (Actix‑Web или Axum) + Elixir/Phoenix для LiveView**  
   * Создать новый репозиторий `longevity_core` на Rust, реализовать CRUD для статей, выпусков, пользователей (DAO → репозитории + PostgreSQL).  
   * Переписать frontend на Phoenix LiveView (HEEx), используя существующую БД как источник истины.  
   * Файлы: `/rust-backend/src/{articles,submissions,issues,auth}/`, `/phoenix-app/lib/longevity_live/`.  

2. **Удалить из продакшна конфиденциальные и тестовые артефакты**  
   * Настроить `.htaccess` / nginx — запретить доступ к `.bak`, `.env.json`, `.dist.xml`, `SECURITY.md`, `Cypress` и т.д.  
   * Удалить:  
     `config.inc.php.bak.*`  
     `cypress.travis.env.json`  
     `phpdoc.dist.xml`  
     `SECURITY.md`  
     `adsense_instructions.md`  
   * Риск: L (утечка паролей БД); трудоёмкость: S.

3. **Контейнеризация и оркестрация (Docker + Compose / K8s)**  
   * Написать `Dockerfile` для Rust-бэкенда и Phoenix-фронтенда, `docker-compose.yml` для локального запуска.  
   * Включить `composer.json`-эквивалент (Cargo.toml + mix.exs).  
   * Файлы: `./Dockerfile.backend`, `./Dockerfile.phoenix`, `./docker-compose.yml`.  
   * Риск: M (изменение процесса развёртывания); трудоёмкость: M.

4. **Переписать схему данных и миграции на SQL миграции (Rust‑based barrel/diesel)**  
   * Создать Rust-проект для миграций (например, `migration-tool/`), перенести все `dbscripts/xml` в читаемые SQL-скрипты с версионированием.  
   * Удалить `dbscripts/xml/` после завершения.  
   * Файлы: `migration-tool/src/`, `sql/V1_initial.sql`.  
   * Риск: H (сложность переноса триггеров и индексов); трудоёмкость: L.

---

### P1 — Важные улучшения (критическое влияние на поддерживаемость)

1. **Удалить неиспользуемые и дублирующиеся файлы**  
   * Очистить `locale/` — оставить только `en`, `ka`, `ru`, удалить 40+ неиспользуемых языков.  
   * Удалить дублирующиеся директории плагинов: `plugins/theme`, `plugins/themes`, `plugins/them` (оставить одну `themes`).  
   * Удалить `tools/cleanReviewerInterests.php`, `tools/resolveAgencyDuplicates.php` — они не нужны после перехода на Rust.  
   * Файлы: `locale/{sid,rue,dsb,...}`; `plugins/theme/`, `plugins/themes/`, `plugins/them/`.  

2. **Переписать авторизацию и аутентификацию на JWT (Rust)**  
   * Реализовать модуль `auth` на Rust с refresh/access токенами, исключить старую сессионную аутентификацию OJS.  
   * Файлы: `/rust-backend/src/auth/{login,register,tokens}.rs`.  

3. **Разделить монолит на микросервисы (событийно-ориентированная архитектура)**  
   * Выделить сервисы: `articles`, `users`, `reviews`, `statistics`, `payments`. Каждый — отдельный Rust-крейт, общаются через `nats`/`kafka`.  
   * Файлы: `services/articles/Cargo.toml`, `services/users/Cargo.toml`, ...  

4. **Добавить полноценное CI/CD (GitHub Actions)**  
   * Проверка типов (Rust + Elixir), линтеры (clippy, credo), тесты, сборка Docker-образов.  
   * Файлы: `.github/workflows/ci.yml`, `.github/workflows/deploy.yml`.  

5. **Убрать старую PHP-точку входа и перенаправить трафик на Phoenix/Rust**  
   * Настроить nginx так, чтобы `index.php` обрабатывался только до полной миграции; после — 301 редирект на новый API.  
   * Файлы: `/etc/nginx/sites-available/longevity` (nginx config).  

---

### P2 — Nice-to-have (улучшение качества без критической необходимости)

1. **Перевести все шаблоны с Smarty на Phoenix LiveView (HEEx)**  
   * Заменить `.tpl`-файлы на `.heex` компоненты, используя ту же структуру данных.  
   * Файлы: `templates/*.tpl` → `phoenix-app/lib/longevity_live_web/templates/*.heex`.  

2. **Оптимизировать размер репозитория**  
   * Добавить `.gitignore`: `cache/*`, `files/temp/*`, `node_modules/`, `vendor/`, `*.bak`.  
   * Очистить историю git от бинарных файлов (использовать `git filter-branch`).  

3. **Отрефакторить legacy Python-скрипты под микросервис (Python только для OCR/PDF)**  
   * Вынести `validate.py` и планируемый OCR-сервис в отдельный Python-контейнер, общение через gRPC/HTTP.  
   * Файлы: `services/ocr_pdf/Dockerfile`, `services/ocr_pdf/main.py`.  

4. **Интегрировать мониторинг и алертинг**  
   * Добавить health-check endpoint (Rust `GET /health`), метрики Prometheus, дашборд Grafana.  
   * Файлы: `/rust-backend/src/monitoring.rs`, `ci/grafana/dashboard.json`.  

5. **Удалить неиспользуемые PHP-хендлеры и контроллеры**  
   * После полной миграции удалить `controllers/`, `pages/`, `api/v1/` (все на PHP).  
   * Файлы: `controllers/*`, `pages/*`, `api/v1/*`.  

---

### Оценка трудоёмкости для P0

| Пункт | Оценка | Риск |
|-------|--------|------|
| Миграция core‑логики на Rust + Phoenix | M (2-3 месяца) | H — высокая сложность переноса бизнес-логики |
| Удаление конфид. файлов | S (1 час) | L |
| Контейнеризация | M (3-5 дней) | M |
| SQL миграции | S (2-3 дня) | L |

---

**Итоговое замечание:** План предполагает **полную переработку** существующей PHP-кодовой базы на стек Rust+Phoenix. Первые четыре P0 должны быть закрыты **до любого дальнейшего development** — без них проект не проходит архитектурный аудит в принципе.