# Plan of Improvements

## P0 — Blockers

1. **Add `.gitignore` and remove dev database files from git history**  
   - Затронутые файлы: `.gitignore`, `numerology_dev.db`, `numerology_dev.db-shm`, `numerology_dev.db-wal`  
   - Effort: S, Risk: Low

2. **Create initial Ecto migrations for production PostgreSQL schema**  
   - Затронутые файлы: `priv/repo/migrations/` (создать), `config/config.exs` (проверить настройки репо)  
   - Effort: M, Risk: Medium (требуется знание текущей модели данных)

3. **Synchronize `AGENTS.md` with actual dependencies**  
   - Вариант А: добавить `{:req, "~> 0.5"}` в `mix.exs`  
   - Вариант Б: удалить или изменить инструкцию про `Req` в `AGENTS.md`  
   - Effort: S, Risk: Low

4. **Remove unused dependencies**  
   - Проверить наличие кода, использующего `stripity_stripe`, `swoosh`, `bcrypt_elixir`, `dns_cluster`; если нет — удалить из `mix.exs` и запустить `mix deps.clean`  
   - Effort: S, Risk: Medium (если какая-то зависимость всё же используется скрыто)

5. **Remove unnecessary Node.js installation from Dockerfile**  
   - Удалить строки `RUN apt-get install -y nodejs npm` и, возможно, `git` (если не нужен для компиляции)  
   - Затронутый файл: `Dockerfile`  
   - Effort: S, Risk: Low

6. **(Уже покрыто п.1) Добавить стандартный `.gitignore` для Elixir/Phoenix**  
   - Может быть объединено с п.1  
   - Effort: S, Risk: Low

---

## P1 — Important

7. **Update outdated build tool versions**  
   - В `mix.exs`: `esbuild ~> 0.10` → `~> 0.17`, `tailwind ~> 0.3` → `~> 0.14`  
   - Затем `mix deps.update esbuild tailwind` и проверить совместимость  
   - Effort: S, Risk: Low

8. **Streamline Mix aliases**  
   - Исправить дублирование: `assets.setup` вызывает `assets.build`? Лучше разделить: `setup` → `deps.get`, `ecto.setup`, `assets.setup` (без `assets.build`), а `assets.build` вызывать отдельно  
   - Затронутый файл: `mix.exs` (раздел `aliases`)  
   - Effort: S, Risk: Low

9. **Optimize Dockerfile build steps**  
   - Объединить `mix deps.get && mix deps.compile` в одну инструкцию `RUN mix do deps.get, deps.compile`  
   - Добавить кэширование слоёв (сначала копировать только `mix.exs` и `mix.lock`)  
   - Effort: S, Risk: Low

10. **Trim AGENTS.md to project-specific rules**  
    - Удалить общие инструкции (Phoenix, Elixir, Ecto guidelines), оставить только то, что относится к данному проекту  
    - Затронутый файл: `AGENTS.md`  
    - Effort: M, Risk: Low

11. **Provide minimal nginx / docker‑compose config for production**  
    - Добавить пример `nginx.conf` и `docker-compose.yml` с пояснениями, как подключать PostgreSQL  
    - Это позволит оценить корректность production‑развёртывания  
    - Effort: M, Risk: Medium (если инфраструктура не определена)

---

## P2 — Nice-to-have

12. **Parameterize Elixir version in Dockerfile**  
    - Использовать `ARG ELIXIR_VERSION=1.16` и подставлять в `FROM elixir:${ELIXIR_VERSION}-slim`  
    - Затронутый файл: `Dockerfile`  
    - Effort: S, Risk: Low

13. **Add CI pipeline (GitHub Actions)**  
    - Базовый workflow: `mix setup`, `mix format --check-formatted`, `mix compile --warnings-as-errors`, `mix test`  
    - Создать `.github/workflows/ci.yml`  
    - Effort: M, Risk: Low

14. **Improve test coverage**  
    - Написать хотя бы один тест для контроллера/LiveView (сейчас только `test_helper.exs` и пустой `test/numerology_web/`)  
    - Затронутые файлы: `test/`  
    - Effort: M, Risk: Low

15. **Add static analysis tools**  
    - Включить `{:credo, "~> 1.7", only: [:dev, :test]}` и `{:dialyxir, "~> 1.4", only: [:dev], runtime: false}`  
    - Добавить алиас `precommit`: `["compile --warnings-as-errors", "format --check-formatted", "credo", "deps.unlock --unused", "test"]`  
    - Effort: M, Risk: Low