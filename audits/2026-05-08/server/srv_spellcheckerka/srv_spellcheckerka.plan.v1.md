## План улучшений

### P0 (блокеры)

| # | Пункт | Файлы | Трудоёмкость | Риск |
|---|-------|-------|-------------|------|
| 1 | **Синхронизировать PARAMETERS.md с кодом по confusion table** — заменить таблицу «8–9 пар» на 25 клауз из `georgian_confusions/1`, удалить несуществующую строку `ზ↔ჟ`. | `PARAMETERS.md` | S | low |
| 2 | **Убрать из THEORY.md упоминание weighting 0.5 для confusion substitutions** — описать текущую реализацию (candidate-pool expansion, uniform cost 1.0). | `THEORY.md` | S | low |
| 3 | **Добавить верифицируемые тесты для accuracy predictions P1–P6** — создать регрессионный корпус и property-based тесты, измеряющие заявленные метрики. | `test/spellcheckerka/accuracy_test.exs`, `test/test_helper.exs` | M–L | medium (сложность сбора корпуса) |
| 4 | **Устранить misrepresentation компонента mailer** — либо реализовать и подключить, либо удалить заглушку и убрать упоминания из документации. | `lib/spellcheckerka/mailer.ex`, `DESIGN.md`, `CONCEPT.md`, `STATE.md` | S | low |
| 5 | **Исправить неверное утверждение "stateless" в DESIGN.md** — заменить на "stateful with ETS + user_words.txt persistence". | `DESIGN.md` (раздел 1 или 4) | S | low |

### P1 (важно)

| # | Пункт | Файлы |
|---|-------|-------|
| 6 | **Добавить базовый мониторинг** — метрики времени ответа, ошибок 402/429, размера ETS, hit/miss ratio. Настроить sink (например, Prometheus или `Logger.metadata`). | `lib/spellcheckerka_web/telemetry.ex`, `config/runtime.exs` |
| 7 | **Разделить таблицу API endpoints в CONCEPT.md / DESIGN.md на "Implemented" + "Planned"** — убрать путаницу между живыми и будущими эндпоинтами. | `CONCEPT.md`, `DESIGN.md` |
| 8 | **Создать CREDITS.md** — указать происхождение Hunspell ka_GE (LGPL), авторов, ссылки. | `CREDITS.md` (новый файл) |
| 9 | **Дополнить README.md Georgian tree модулями `lang_dictionary.ex` и `usage_tracker.ex`**. | `README.md` |
| 10 | **Настроить pre-commit hook с `mix test`** — предотвратить коммиты с падающими тестами. | `.git/hooks/pre-commit` (или скрипт в репозитории) |
| 11 | **Внедрить базовый CI/CD (GitHub Actions)** — сборка, тесты, проверка форматирования, деплой-скрипт. | `.github/workflows/ci.yml` |

### P2 (nice-to-have)

| # | Пункт | Файлы |
|---|-------|-------|
| 12 | **Перенести `_archive/` под `docs/archive/`** — не загрязнять корень. | `docs/archive/core_pre_9file_2026-04-25/` |
| 13 | **Документировать CORS-политику в DESIGN.md** — добавить примечание о preflight и разрешённых origin’ах. | `DESIGN.md` (секция 3.1) |
| 14 | **Добавить в DESIGN.md описание `extension/icons/generate_icons.py`** — упомянуть как инструмент генерации иконок. | `DESIGN.md` |
| 15 | **Проверить и поправить опечатку `georgian` → `georgian` в PARAMETERS.md**. | `PARAMETERS.md` |
| 16 | **Добавить проверку зависимости Docker в `run.sh docker-build`** — выводить понятную ошибку если Docker не установлен. | `run.sh` |