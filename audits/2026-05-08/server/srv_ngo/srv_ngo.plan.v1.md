## План улучшений srv_ngo

### P0 (блокеры) — необходимые для запуска проекта

1. **Создать новый проект Phoenix LiveView (frontend) + Rust backend (Axum/Actix)**
   - Инициализировать `mix phx.new ngo_frontend` и `cargo init ngo_backend`, настроить базовую архитектуру
   - Файлы: полностью новые (mix.exs, config/, lib/, Cargo.toml, src/)
   - Трудоёмкость: L (крупная переработка) | Риск: высокий (нужна интеграция двух сервисов, настройка CORS/прокси)

2. **Удалить все бэкап-файлы и дубликаты ассетов**
   - Выполнить: `find . -name '*.bak*' -delete`, удалить `assets/logo.jpg`, `assets/logo.jpg.bak`, `eco-inject.js.bak.*`, `index.html.bak.hive-*`
   - Файлы: все перечисленные выше
   - Трудоёмкость: S (5 минут) | Риск: низкий (контент в нормальных файлах сохранён)

3. **Перенести содержимое статических HTML в шаблоны HEEx+Layout**
   - Создать `lib/ngo_web/controllers/page_controller.ex`, прописать маршруты (`/about`, `/team`, …) и перенести HTML-контент в соответствующие `*.heex` с единым layout
   - Файлы: все `index.html` в подпапках → `lib/ngo_web/templates/page/*.heex`, `lib/ngo_web/router.ex`
   - Трудоёмкость: M (перекопирование, адаптация ссылок) | Риск: средний (возможны битые ссылки, потеря контента)

4. **Разработать базовый REST API на Rust для бизнес-логики (например, обработка обращений, получение данных из базы)**
   - Создать эндпоинты: `POST /api/contact`, `GET /api/news`, `POST /api/research`. Подключить SQLite или PostgreSQL.
   - Файлы: `ngo_backend/src/main.rs`, `*/routes.rs`, `*/handlers.rs`, `*/models.rs`, `Cargo.toml` (добавить зависимости axum, serde, sqlx)
   - Трудоёмкость: L | Риск: высокий (спроектировать API с учётом потребностей фронта, настройка базы)

5. **Настроить systemd unit-ы для запуска обоих сервисов**
   - Создать `ngo-frontend.service` (Phoenix через elixir release) и `ngo-backend.service` (Rust бинарник)
   - Файлы: `/etc/systemd/system/ngo-*.service`
   - Трудоёмкость: M | Риск: средний (нужна корректная конфигурация путей, переменных окружения, портов)

### P1 (важно) — улучшение качества и функциональности

6. **Реализовать LiveView-компоненты для динамических разделов (лента новостей, форма обратной связи)**
   - Создать `lib/ngo_web/live/contact_live.ex` с валидацией и асинхронной отправкой на Rust API
   - Файлы: `lib/ngo_web/live/*.ex`, `lib/ngo_web/templates/live/*.heex`

7. **Оптимизировать ассеты: заменить logo.png/logo.jpg на WebP, удалить дубли**
   - Сконвертировать в WebP, оставить один вариант, обновить пути в `app.css`/layout
   - Файлы: `assets/static/images/logo.webp`, `lib/ngo_web/templates/layout/app.html.heex`

8. **Настроить сборку статики (esbuild + tailwind) через `assets/` в Phoenix**
   - Включить esbuild и tailwind в `mix.exs`, добавить конфигурацию
   - Файлы: `assets/package.json`, `assets/tailwind.config.js`, `config/config.exs`

9. **Создать миграции Ecto для хранения контента (новости, публикации, заявки)**
   - `mix ecto.gen.migration`, определить схемы, выполнить миграции
   - Файлы: `priv/repo/migrations/*.exs`, `lib/ngo_web/schemas/*.ex`

10. **Интегрировать существующий eco-inject.js (если легитимен) или заменить на LiveView-аналог**
    - Проверить код скрипта; если безопасен — перенести как `static/eco-inject.js`, если нет — переписать на LiveView
    - Файлы: `assets/static/eco-inject.js` (или удалён)

### P2 (nice-to-have) — долгосрочные улучшения

11. **Добавить тесты (ExUnit для Phoenix, Rust интеграционные тесты)**
    - `test/` для Phoenix, `#[cfg(test)]` модули в Rust
    - Файлы: `test/ngo_web/*_test.exs`, `ngo_backend/tests/`

12. **Настроить CI/CD (например, GitHub Actions)**
    - Сборка, линтеры, прогон тестов, деплой на сервер
    - Файлы: `.github/workflows/ci.yml`

13. **Уменьшить размер ассетов (сжатие, CDN, lazy load)**
    - Оптимизация изображений через `imagemagick`, настройка кэширования в Phoenix
    - Файлы: `config/prod.exs`, `priv/static/cache.manifest`

14. **Удалить eco-inject.js, если он не имеет назначения, или переписать на Rust/Phoenix**
    - Полностью исключить из сборки
    - Файлы: удалить `assets/static/eco-inject.js`

15. **Написать README с инструкцией по запуску, архитектурой и контактами**
    - Файлы: `README.md`