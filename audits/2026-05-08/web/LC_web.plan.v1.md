## План улучшений (stack‑aware)

### P0 — Блокеры (полная переработка стека)

1. **Заменить React‑фронтенд на Phoenix LiveView**  
   Удалить `src/`, `package.json`, `tsconfig*.json`, `vite.config.ts`, `scripts/`, `Dockerfile` (старый).  
   Создать новый Phoenix‑проект (`mix phx.new lc_web --live`) внутри `web/`.  
   *Файлы:* весь `web/` (кроме `deploy/`).  
   **Трудозатраты:** L · **Риск:** высокий — требуется Elixir/Phoenix + LiveView навыки.

2. **Перенести существующую функциональность страниц в `.heex`‑шаблоны и Live‑компоненты**  
   Заменить React‑маршруты на эквивалентные LiveView‑роуты, переиспользовать API‑вызовы через `HTTPoison`/`Req` к Rust‑backend.  
   *Файлы:* `lib/lc_web_web/live/*`, `lib/lc_web_web/router.ex`.  
   **Трудозатраты:** M · **Риск:** средний — корректная миграция логики.

3. **Обновить `Dockerfile` для Elixir‑сборки**  
   База: `hexpm/elixir:1.16-alpine` + `erlang:26`.  
   Вынести nginx‑конфиг в отдельный файл, удалить heredoc.  
   *Файлы:* `Dockerfile`, `deploy/nginx/default.conf`.  
   **Трудозатраты:** S · **Риск:** низкий.

### P1 — Важно (структура и безопасность)

1. **Добавить `engine` restrictions**  
   В `mix.exs` указать минимальные версии Elixir и Erlang (например, `~> 1.16`, `~> 26.0`).  
   *Файлы:* `mix.exs`.  
   **Трудозатраты:** S

2. **Создать API‑клиент для Rust‑сервера**  
   Один модуль (`lib/lc_web/api.ex`) со всеми HTTP‑вызовами, обработкой ошибок и сериализацией.  
   *Файлы:* `lib/lc_web/api.ex`.  
   **Трудозатраты:** M

3. **Вынести nginx‑конфигурацию из heredoc**  
   Переместить блок `server` в `deploy/nginx/default.conf` и скопировать его в Dockerfile.  
   *Файлы:* `deploy/nginx/default.conf`, `Dockerfile`.  
   **Трудозатраты:** S

### P2 — Nice‑to‑have

1. **Добавить CI‑конфигурацию (GitHub Actions)**  
   Проверка форматирования (`mix format`), Credo, Dialyzer, тесты.  
   *Файлы:* `.github/workflows/ci.yml`.  
   **Трудозатраты:** M

2. **Удалить устаревшие артефакты**  
   `gen-icons.mjs`, `sharp` из зависимостей.  
   *Файлы:* `scripts/gen-icons.mjs`, `package.json` (если остался).  
   **Трудозатраты:** S

3. **Написать тесты для LiveView и API‑клиента**  
   Использовать `Phoenix.LiveViewTest` и `ExUnit` + `bypass` для мока HTTP.  
   *Файлы:* `test/lc_web_web/live/*`, `test/lc_web/api_test.exs`.  
   **Трудозатраты:** M