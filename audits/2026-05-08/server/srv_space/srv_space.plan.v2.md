## P0 — Blockers

### 1. Add `.gitignore` and remove dev database files from git history
- **Файлы:** `.gitignore` (создать), удалить из трека `numerology_dev.db`, `numerology_dev.db-shm`, `numerology_dev.db-wal`  
- **Effort:** S, **Risk:** Low  
- **Действие:** Создать стандартный `.gitignore` для Elixir/Phoenix, добавить в него `*.db *.db-shm *.db-wal`, выполнить `git rm --cached`.

### 2. Create initial Ecto migrations for production PostgreSQL schema
- **Файлы:** `priv/repo/migrations/` (создать), `config/config.exs` (проверить настройки репо)  
- **Effort:** M, **Risk:** Medium (требуется знание модели данных)  
- **Действие:** Сгенерировать минимум одну миграцию (например, `create users`), запустить `mix ecto.gen.migration`.

### 3. Synchronize `AGENTS.md` with actual dependencies
- **Файлы:** `mix.exs` или `AGENTS.md`  
- **Effort:** S, **Risk:** Low  
- **Действие:** Добавить `{:req, "~> 0.5"}` в `mix.exs` **или** удалить/исправить упоминание `Req` в `AGENTS.md`.

### 4. Remove unused dependencies from `mix.exs`
- **Файлы:** `mix.exs`  
- **Effort:** S, **Risk:** Medium (проверить, что `stripity_stripe`, `swoosh`, `bcrypt_elixir`, `dns_cluster` действительно не используются)  
- **Действие:** Закомментировать/удалить из `deps`, выполнить `mix deps.clean --unlock`.

### 5. Install `libpq-dev` in Dockerfile to compile `postgrex`
- **Файлы:** `Dockerfile` (stage `builder`)  
- **Effort:** S, **Risk:** Low  
- **Действие:** Добавить `libpq-dev` в список пакетов `apt-get install` (после `build-essential git`). **Важно:** не удалять `git`, так как он нужен для `heroicons` (зависимость из GitHub).

### 6. Configure production database connection via `DATABASE_URL`
- **Файлы:** `config/runtime.exs` (или `config/prod.exs`)  
- **Effort:** S, **Risk:** Medium (необходимо задать ожидаемые переменные окружения)  
- **Действие:** В `config/runtime.exs` добавить чтение `DATABASE_URL` или явно указать `hostname`, `port`, `database`, `username`, `password` из окружения. Убедиться, что` Ecto.Repo` использует эти настройки.

### 7. Remove unnecessary Node.js installation from Dockerfile (ещё не сделано)
- **Файлы:** `Dockerfile` (stage `builder`)  
- **Effort:** S, **Risk:** Low  
- **Действие:** Убрать `nodejs npm` из команды `apt-get install`. (Esbuild/Tailwind работают как OTP‑бинарники.)

---

## P1 — Important

### 8. Update outdated build tool versions
- **Файлы:** `mix.exs`  
- **Действие:** `esbuild ~> 0.10` → `~> 0.17`, `tailwind ~> 0.3` → `~> 0.14`, затем `mix deps.update esbuild tailwind` и проверить совместимость.

### 9. Streamline Mix aliases (убрать дублирование)
- **Файлы:** `mix.exs` (раздел `aliases`)  
- **Действие:** `assets.setup` не должен вызывать `assets.build`. Переопределить: `setup` → `deps.get`, `ecto.setup`, `assets.setup`; `assets.build` вызывать отдельно.

### 10. Optimize Dockerfile build steps
- **Файлы:** `Dockerfile`  
- **Действие:** Объединить `mix deps.get && mix deps.compile` в `RUN mix do deps.get, deps.compile`. Убедиться, что слой кэширования работает (сначала копировать только `mix.exs` и `mix.lock`).

### 11. Trim AGENTS.md to project‑specific rules
- **Файлы:** `AGENTS.md`  
- **Действие:** Удалить общие инструкции (Phoenix, Elixir, Ecto guidelines), оставить только правила, уникальные для numerology.

### 12. Provide minimal nginx / docker‑compose config for production
- **Файлы:** `nginx.conf` и `docker-compose.yml` (уже есть? проверить содержимое)  
- **Действие:** Дополнить примером, показывающим связь с PostgreSQL и настройку `DATABASE_URL`.

### 13. Add CI pipeline (GitHub Actions)
- **Файлы:** `.github/workflows/ci.yml` (создать)  
- **Действие:** Workflow: `mix setup`, `mix compile --warnings-as-errors`, `mix test`, `mix format --check-formatted`.

---

## P2 — Nice-to-have

### 14. Parameterize Elixir version in Dockerfile
- **Файлы:** `Dockerfile`  
- **Действие:** Добавить `ARG ELIXIR_VERSION=1.16` и использовать в `FROM elixir:${ELIXIR_VERSION}-slim`.

### 15. Add static analysis tools (Credo, Dialyzer)
- **Файлы:** `mix.exs`  
- **Действие:** Добавить `{:credo, "~> 1.7", only: [:dev, :test]}`, `{:dialyxir, "~> 1.4", only: [:dev], runtime: false}`. Включить в `precommit` aliases.

### 16. Improve test coverage
- **Файлы:** `test/numerology_web/` (создать тесты для LiveView/контроллеров)  
- **Действие:** Написать хотя бы один тест для существующего LiveView или контроллера, используя `NumerologyWeb.ConnCase` и `async: true`.

---

**Примечание:** Пункты 1–7 являются блокерами, без которых production‑сборка невозможна или небезопасна. После их выполнения можно приступать к P1 и P2.