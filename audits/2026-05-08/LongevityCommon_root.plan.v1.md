# План улучшений для LongevityCommon

## P0 — Блокеры

### P0.1: Устранить дублирование CLI-бинарей (Proteostasis / MitoROS)

**Действие:** Выделить общий crate `counter-cli` с конфигурацией (tissue, counter_number). Заменить `proteostasis_counter/src/main.rs` и `mito_ros_counter/src/main.rs` на вызовы библиотеки.  
**Файлы:** 
- `Proteostasis/crates/proteostasis_counter/src/main.rs`
- `MitoROS/crates/mito_ros_counter/src/main.rs`
- Создать `common/crates/counter-cli/` с `src/lib.rs` и `config.yaml`  
**Трудоёмкость:** S (1–2 дня)  
**Риск:** Низкий (логика идентична, изменится только точка входа)

### P0.2: Аудит и консолидация документации (594 .md → ~200)

**Действие:** 
- Удалить дублирующие stub-файлы (`Proteostasis/DESIGN.md` с заглушкой, `AutomatedMicroscopy/OPEN_PROBLEMS.md`, `AGENTS.md` и т.п.)
- Консолидировать `AIM/CLAUDE.md` (21k chars) — разнести immutable правила в `AIM/THEORY.md`, roadmap в `docs/roadmaps/`
- Удалить `STATE.md`, `TODO.md`, `ROADMAP.md`, `UPGRADE.md`, `CHANGELOG.md`, `REMINDER.md`, `STRATEGY.md` из каждого подпроекта — оставить один `RELEASE_NOTES.md` и один `ACTIVE_TASKS.md` на umbrella  
**Файлы:** Все `.md` в корне и подпроектах  
**Трудоёмкость:** M (1 неделя)  
**Риск:** Средний (можно потерять полезную информацию; необходимо backup + diff)

### P0.3: Удалить мёртвые подпроекты HAP и Ontogenesis

**Действие:** Переместить `HAP/` и `Ontogenesis/` в `_archive/` (или удалить из master-ветки).  
**Файлы:** 
- `./HAP/` (весь каталог)
- `./Ontogenesis/` (весь каталог)
- Удалить упоминания в `CONCEPT.md`, `MAP.md`, `CLAUDE.md`  
**Трудоёмкость:** S (1 день)  
**Риск:** Низкий (код помечен как toxic, не используется)

### P0.4: Исправить социальный слой (server/web/realtime) — конфликт портов, тесты

**Действие:** 
- Изменить `realtime/config/dev.exs` → port 4500 (вместо 4001)
- Запустить `cargo run --release` в `server/` и проверить миграции (`migrations/003_health_factors.sql`)
- Написать минимальный integration test: `POST /api/chi_ze` через мок BioSense backend  
**Файлы:** 
- `realtime/config/dev.exs`
- `server/src/` (возможно правка handlers)
- `server/tests/` (создать `integration_test.rs`)  
**Трудоёмкость:** M (3–5 дней)  
**Риск:** Средний (если код server/ не собран ранее, могут всплыть ошибки зависимостей)

### P0.5: Формализовать легаси Python в AIM и создать план миграции

**Действие:** 
- Создать файл `AIM/LEGACY_PYTHON.md` со списком всех 361 `.py`, разбив их на категории:
  - **legitimate** (OCR/PDF/Whisper — оставить)
  - **to-migrate** (все остальные с назначенным сроком миграции)
- Добавить в `STACK.md` жёсткое правило: новые Python-файлы запрещены без explicit approve
- Обновить `AIM/CLAUDE.md` → убрать утверждение "Rust+Phoenix only", заменить на "Rust+Phoenix preferred; исключения — в LEGACY_PYTHON.md"  
**Файлы:** 
- `AIM/LEGACY_PYTHON.md` (новый)
- `AIM/STACK.md`
- `AIM/CLAUDE.md`  
**Трудоёмкость:** L (2 недели на инвентаризацию + документирование)  
**Риск:** Высокий (без этого правила будут продолжать писать Python)

### P0.6: Унифицировать нотацию v* (Article vs Python)

**Действие:** 
- Выбрать единую константу (Article-convention `-0.08738`)
- Ввести функцию `v_article_to_python(v) = (v + 1)/2` и `v_python_to_article`
- Исправить все вхождения в файлах:
  - `BioSense/KNOWLEDGE.md` (Python-convention)
  - `BioSense/TODO.md`
  - `Ze/CONCEPT.md`
  - `BioSense/src/eeg_ze_processor.py`
- Убедиться, что `PARAMETERS.md` §1 является единственным SOT  
**Файлы:** 
- `PARAMETERS.md`
- `BioSense/KNOWLEDGE.md`
- `BioSense/TODO.md`
- `BioSense/src/eeg_ze_processor.py`
- `Ze/CONCEPT.md`  
**Трудоёмкость:** S (1–2 дня)  
**Риск:** Низкий (математическое преобразование детерминировано)

### P0.7: Внедрить CI и линтеры

**Действие:** 
- Создать `.github/workflows/ci.yml`:
  - `cargo test --release` во всех подпроектах (рабочий стол workspace)
  - `mix test` в realtime
  - `npm run lint` в web
  - **по желанию:** `cargo clippy` + `rustfmt --check`
- Добавить `pyproject.toml` с `[tool.pylint]` для Python-легаси  
**Файлы:** 
- `.github/workflows/ci.yml` (новый)
- `Cargo.toml` (корень) — workspace для umbrella test  
**Трудоёмкость:** L (2 недели на настройку + исправление найденных ошибок)  
**Риск:** Средний (может потребоваться исправление многих поломанных тестов)

### P0.8: Очистить vapor-модули и противоречия Docker

**Действие:** 
- Удалить упоминания KIMI, Qwen, aim-media из `AIM/CONCEPT.md`, `AIM/PARAMETERS.md`, `AIM/MAP.md`
- Удалить `AIM/Dockerfile` (если Docker запрещён) или добавить exception в `STACK.md`
- Удалить `deploy/docker-compose-all.yml` (если запрещён Docker) или объяснить в `STACK.md`  
**Файлы:** 
- `AIM/CONCEPT.md`
- `AIM/PARAMETERS.md`
- `AIM/MAP.md`
- `AIM/Dockerfile`
- `deploy/docker-compose-all.yml`  
**Трудоёмкость:** M (3–4 дня)  
**Риск:** Низкий (чистка doc + удаление мёртвых файлов)

### P0.9: Обеспечить защиту PII в AIM

**Действие:** 
- В `agents/intake.py` добавить вызов `anonymize()` перед отправкой любого текста в LLM-роутер
- Убедиться, что `_anonymize()` удаляет phone, email, passport, address, Georgian ID (`XXX-XXXX-XXXXX`)
- Добавить unit-test: `tests/test_anonymize.py` с примерами
- Логироовать факт анонимизации (без данных)  
**Файлы:** 
- `agents/intake.py`
- `agents/doctor.py`
- `agents/generalist.py`
- `tests/test_anonymize.py` (новый)  
**Трудоёмкость:** M (3 дня)  
**Риск:** Средний (необходимо тщательное тестирование)

---

## P1 — Важные улучшения

### P1.1: Консолидировать файлы состояния в AIM

**Действие:**  
- Объединить `STATE.md`, `TODO.md`, `ROADMAP.md`, `UPGRADE.md`, `CHANGELOG.md`, `REMINDER.md`, `STRATEGY.md` в один `ACTIVE_TASKS.md` + `RELEASE_NOTES.md`  
- Перенести долгосрочные планы в `docs/roadmaps/`  
**Файлы:** Все `.md` корня AIM + `docs/roadmaps/`

### P1.2: Унифицировать порты через env config

**Действие:**  
- Создать общий `deploy/port-config.env`  
- В каждом subproject-backend читать порт из env с fallback (текущее значение)  
- Удалить отдельные константы из `PARAMETERS.md` §8  
**Файлы:** `deploy/port-config.env`, `PARAMETERS.md`, конфиги backend-ов

### P1.3: Удалить заглушки (stub) и наполнить реальным содержимым

**Действие:**  
- `Proteostasis/DESIGN.md` — заменить реальным описанием структуры крейтов  
- `AutomatedMicroscopy/OPEN_PROBLEMS.md` — написать осмысленное содержание  
**Файлы:** `Proteostasis/DESIGN.md`, `AutomatedMicroscopy/OPEN_PROBLEMS.md`

### P1.4: Создать интеграционный тест umbrella

**Действие:**  
- Написать скрипт/тест, который:
  1. Запускает `Ze/ze-backend` и `BioSense/biosense-simulator` (через Cargo)
  2. Делает POST на `/api/chi_ze` через `server/`
  3. Проверяет корректность ответа (код 200, поле `composite`)
- Можно оформить как `tests/umbrella_integration_test.py` (Python) или `rust tests/integration/`  
**Файлы:** `tests/umbrella_integration_test.rs` или `.py`

### P1.5: Явно пометить нерецензированные ссылки

**Действие:**  
- В `THEORY.md` и `CONCEPT.md` добавить сноску: "not peer-reviewed" ко всем DOI вида 10.65649/...  
- В `EVIDENCE.md` §4 указать, что Longevity Horizon manuscripts — internal sources  
**Файлы:** `THEORY.md`, `CONCEPT.md`, `EVIDENCE.md`

### P1.6: Создать реестр легаси Python (дополнение к P0.5)

**Действие:**  
- В `AIM/LEGACY_PYTHON.md` явно перечислить каждый `.py` файл с причиной оставления  
- Для каждого файла из разряда "to-migrate" указать соответствующую `aim-*` Rust crate или Phoenix LiveView  
**Файлы:** `AIM/LEGACY_PYTHON.md`

### P1.7: Написать план миграции Python→Rust+Phoenix

**Действие:**  
- Создать `docs/migration/MIGRATION_PLAN_PYTHON.md` с расписанием (по фазам) и метриками прогресса  
**Файлы:** `docs/migration/MIGRATION_PLAN_PYTHON.md`

---

## P2 — Nice-to-have

### P2.1: Убрать закомментированные зависимости

**Действие:** Удалить `# tower_governor = ...` из `server/Cargo.toml`  
**Файлы:** `server/Cargo.toml`

### P2.2: Улучшить комментарии в Rust коде

**Действие:** Добавить doc-comments к основным публичным функциям, особенно в `cell_dt_cli`, `MCOA`  
**Файлы:** `CDATA/crates/cell_dt_cli/src/main.rs`, `MCOA/crates/mcoa_core/src/lib.rs`

### P2.3: Использовать clap derive для всех CLI бинарей

**Действие:** Заменить ручной парсинг аргументов на `clap::Parser` (например, в `proteostasis_counter/src/main.rs`)  
**Файлы:** `Proteostasis/crates/proteostasis_counter/src/main.rs`, `MitoROS/crates/mito_ros_counter/src/main.rs`, `AIM/SSA/backend/src/main.rs`

### P2.4: Включить coverage в CI

**Действие:** Добавить шаг `cargo tarpaulin` или `cargo llvm-cov` в workflow  
**Файлы:** `.github/workflows/ci.yml`

### P2.5: Устранить дублирование описаний между umbrella CONCEPT.md и subproject CONCEPT.md

**Действие:** В subproject CONCEPT.md заменить секции, повторяющие umbrella, на ссылки вида `см. umbrella CONCEPT.md §3`  
**Файлы:** `Ze/CONCEPT.md`, `BioSense/CONCEPT.md`, `FCLC/CONCEPT.md`, `CDATA/CONCEPT.md`, `MCOA/CONCEPT.md`