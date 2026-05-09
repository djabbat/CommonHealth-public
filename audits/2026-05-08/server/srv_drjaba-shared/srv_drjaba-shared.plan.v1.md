## План улучшений — `srv_drjaba-shared`

### P0 — Блокеры (критичные, без них проект неработоспособен)

1. **Инициализация Rust-бэкенда**  
   - Создать Cargo-проект с минимальным HTTP-сервером (actix-web/axum).  
   - Реализовать endpoint `GET /health` → `200 OK`.  
   - **Файлы:** `backend/Cargo.toml`, `backend/src/main.rs`, `backend/.gitignore`  
   - **Effort:** M — **Risk:** Low (стандартная настройка Rust-проекта)

2. **Инициализация Phoenix LiveView-фронтенда**  
   - Создать Elixir-проект `mix phx.new frontend --live` в отдельной директории.  
   - Настроить проксирование API-запросов к Rust-бэкенду в `dev.exs`.  
   - **Файлы:** `frontend/mix.exs`, `frontend/config/`, `frontend/lib/`  
   - **Effort:** M — **Risk:** Medium (необходимость синхронизации портов, WebSocket-прокси)

3. **Приведение структуры репозитория к модульной**  
   - Переместить `assets/drjaba-logo.png`, `assets/drjaba-mark.png` → `frontend/priv/static/images/`.  
   - Удалить `dr-inject.js` (не относится к серверу) или перенести в отдельную документацию.  
   - Создать корневые каталоги: `backend/`, `frontend/`, `docs/`, `scripts/`.  
   - **Файлы:** перемещение существующих, создание пустых папок  
   - **Effort:** S — **Risk:** Low (чисто организационное)

4. **Добавление минимальных инструментов сборки/запуска**  
   - Для Rust: скомпилировать и проверить `cargo build --release`.  
   - Для Phoenix: создать `mix release` и `Dockerfile` для production.  
   - Создать `docker-compose.yml` для одноручного запуска обеих частей.  
   - **Файлы:** `backend/Dockerfile`, `frontend/Dockerfile`, `docker-compose.yml`, `.env`  
   - **Effort:** M — **Risk:** Medium (деплой постфактум может выявить проблемы с сетью)

5. **Создание systemd-юнитов для сервисов**  
   - Написать `drjaba-backend.service` (Rust-бинарник) и `drjaba-frontend.service` (Phoenix-релиз).  
   - Обеспечить автозапуск и зависимость `network-online.target`.  
   - **Файлы:** `scripts/drjaba-backend.service`, `scripts/drjaba-frontend.service`  
   - **Effort:** S — **Risk:** Low (шаблонные юниты)

---

### P1 — Важные улучшения (не блокируют запуск, но критичны для качества)

1. **Реализация маршрута `GET /api/v1/status` в Rust** и отображение данных на LiveView-странице.  
   - **Файлы:** `backend/src/routes.rs`, `frontend/lib/.../status_live.ex`, `frontend/lib/.../status_live.html.heex`  
   - **Effort:** S

2. **Базовая LiveView-компонента** (счётчик / чат) для демонстрации real-time взаимодействия.  
   - **Файлы:** `frontend/lib/.../demo_live.ex`  
   - **Effort:** S

3. **Оптимизация изображений** — сжатие PNG (pngquant) до 80% качества, либо замена на SVG-иконки.  
   - **Файлы:** `frontend/priv/static/images/drjaba-logo.png`, `drjaba-mark.png`  
   - **Effort:** S

4. **Документация (README)** — назначение проекта, архитектура (Rust ↔ Phoenix), инструкции по локальному запуску, тестированию.  
   - **Файлы:** `README.md`  
   - **Effort:** S

---

### P2 — Nice-to-have (улучшения без срочности)

1. **CI-пайплайн** (GitHub Actions) — сборка Rust и Phoenix, прогон тестов, публикация Docker-образов.  
   - **Файлы:** `.github/workflows/ci.yml`  
   - **Effort:** M

2. **Юнит-тесты** — для Rust хотя бы 2 теста (health-check), для Phoenix — тест LiveView (например, проверка отображения статуса).  
   - **Файлы:** `backend/tests/`, `frontend/test/`  
   - **Effort:** M

3. **Docker-образы в реестре** (GitHub Container Registry или Docker Hub) с тегом `latest` по коммиту.  
   - **Файлы:** (изменение CI)  
   - **Effort:** M

4. **Линтеры и форматтеры** — `rustfmt`, `mix format`, `credo` для Elixir, `shellcheck` для скриптов.  
   - **Файлы:** `backend/.cargo/config.toml`, `frontend/.credo.exs`  
   - **Effort:** S