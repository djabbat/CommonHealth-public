# План улучшений BioSense

## P0 — Блокеры (без них проект неприемлем для production)

### P0.1 Единый источник истины для v* и констант
- Создать `config/constants.toml` с v*_active, f_opt, параметрами датасетов.
- Rust: генерировать `src/constants.rs` через `build.rs` (или читать `.toml` через `serde`).
- Python: читать тот же файл или импортировать из `constants.py`, который парсит `.toml`.
- Удалить дубликаты из `PARAMETERS.md`, `KNOWLEDGE.md`, `MEMORY.md`, `main.rs`, скриптов.
- **Затронутые файлы:** `backend/src/main.rs`, `src/eeg_ze_processor.py`, `src/*.py`, `config/constants.toml` (новый), `backend/build.rs` (новый), `PARAMETERS.md`, `KNOWLEDGE.md`, `MEMORY.md`.
- **Оценка:** M (2 дня); **Риск:** M (необходимо синхронизировать все ссылки).

### P0.2 Полное покрытие тестами (Rust + Python)
- Rust backend: unit-тесты для core-логики (например, `compute_chi_ze`), integration-тесты через `axum::test`, coverage >80%.
- Python: unit-тесты для `eeg_ze_processor.py` (`ze_cheating_index`, `narrowband_ze`, `group_statistics`), mock для загрузки данных.
- CI: `cargo test`, `pytest` обязательны перед мержем.
- **Затронутые файлы:** `backend/src/main.rs` (добавить `#[cfg(test)]` модули), `backend/tests/` (новый), `src/tests/` (новый), `.github/workflows/ci.yml` (новый).
- **Оценка:** M (3 дня); **Риск:** M (выявит существующие баги).

### P0.3 Валидация входных данных в Rust-эндпоинтах
- Добавить десериализацию с проверкой: `NaN`, `Inf`, диапазоны (например, `v` ∈ [0,1], `age` ∈ [0,150]).
- Возвращать `422 Unprocessable Entity` с описанием ошибки.
- **Затронутые файлы:** `backend/src/main.rs` (добавить `#[derive(Deserialize)]` с `#[serde(deny_unknown_fields)]` и кастомные `deserialize_with`).
- **Оценка:** S (0.5 дня); **Риск:** L (изолированное изменение).

### P0.4 Структурирование Python-кода в пакет
- Создать `src/biosense/` с подпапками `core/` (eeg_ze_processor), `analysis/` (ze_cuban_analysis и т.д.), `utils/` (загрузка данных).
- Добавить `__init__.py` в каждую папку; импорты — через `from biosense.core import ...`.
- Обновить все скрипты и `biosense.sh`.
- **Затронутые файлы:** `src/` (переименовать и переместить), `README.md` (обновить структуру), `MAP.md`.
- **Оценка:** M (1 день); **Риск:** L (изоляция, регрессия маловероятна).

### P0.5 Чёткое правило исключения для Python в TODO.md
- В `TODO.md` изменить «если нет явного указания — Rust» на «Python допустим ТОЛЬКО для анализа EEG/HRV (научные скрипты) и для AIM ML-роутера. Весь production-код (backend, деплой) — Rust.»
- **Затронутые файлы:** `TODO.md`, `CLAUDE.md`.
- **Оценка:** S (0.1 дня); **Риск:** L.

### P0.6 Исправление ChiZeRequest и `/api/v_star` конвертации
- Добавить в `backend/src/main.rs` структуру `ChiZeRequest` с `#[serde(alias)]` для legacy полей.
- Реализовать `/chi_ze`: вычислять χ_Ze по формуле (Python form), но возвращать в Article form (умножать? по PARAMETERS.md: Article = 2·Python − 1). Уточнить с автором.
- Гарантировать, что `/api/v_star` возвращает Article form `-0.08738` и документация `CLAUDE.md` явно указывает формат.
- **Затронутые файлы:** `backend/src/main.rs` (добавить `ChiZeRequest`, реализацию), `CLAUDE.md`.
- **Оценка:** S (1 день); **Риск:** M (высока вероятность несоответствия ожиданиям клиентов).

---

## P1 — Важно (существенно влияет на разработку и поддержку)

### P1.1 .gitignore
- Добавить `data/`, `__pycache__/`, `*.pyc`, `*.egg-info`, `target/`, `*.mat`, `.env`.
- **Затронутые файлы:** `.gitignore` (новый).
- **Трудоёмкость:** S (0.1 дня).

### P1.2 Фиксация версий зависимостей
- `src/requirements.txt`: заменить `>=` на `==` с конкретными версиями (например, `mne==1.6.1`). Использовать `pip freeze`.
- `backend/Cargo.toml`: зафиксировать версии через `major.minor.patch` (но уже достаточно).
- **Затронутые файлы:** `src/requirements.txt`, возможно `backend/Cargo.lock` (уже lock).
- **Трудоёмкость:** S (0.5 дня).

### P1.3 Перенос организационных правил из TODO.md
- Из `TODO.md` переместить раздел «📌 Правило: язык…» и «📌 Правило: DeepSeek…» в `CLAUDE.md`.
- В `TODO.md` оставить только задачи, а правила — вынести в `RULES.md` или `CLAUDE.md`.
- **Затронутые файлы:** `TODO.md`, `CLAUDE.md`.
- **Трудоёмкость:** S (0.2 дня).

### P1.4 CI-пайплайн
- GitHub Actions: `cargo build`, `cargo test`, `cargo clippy`, `pytest`, `flake8` (или `ruff`).
- Добавить badge в `README.md`.
- **Затронутые файлы:** `.github/workflows/ci.yml` (новый), `README.md`.
- **Трудоёмкость:** M (1 день).

### P1.5 Управление JSON-результатами
- Переместить все `.json` (кроме служебных) в `results/` (уже частично). Проверить, что не закоммичены лишние `.json` в корень.
- Добавить `results/*.json` в `.gitignore`, если они генерируются; иначе оставить, но добавить `**/results/*.json` в `git lfs` или ограничить размер.
- **Затронутые файлы:** `.gitignore`, возможно `results/`.
- **Трудоёмкость:** S (0.5 дня).

---

## P2 — Nice-to-have (улучшения, которые можно отложить)

### P2.1 OpenAPI-спецификация
- Создать `openapi.yaml` для Rust backend с описанием всех эндпоинтов.
- **Затронутые файлы:** `docs/openapi.yaml` (новый).
- **Трудоёмкость:** M (1-2 дня).

### P2.2 Logging и метрики в Python
- Заменить `print` в Python-скриптах на `logging` с уровнями `INFO`, `DEBUG`, `ERROR`.
- **Затронутые файлы:** `src/biosense/core/*.py`, `src/biosense/analysis/*.py`.
- **Трудоёмкость:** S (0.5 дня).

### P2.3 Мониторинг для Rust backend
- Добавить метрики (`/metrics` с prometheus), трейсинг (OpenTelemetry).
- **Затронутые файлы:** `backend/src/main.rs` (добавить `axum-prometheus`).
- **Трудоёмкость:** M (1-2 дня).

### P2.4 Удаление dead code и выравнивание import-ов
- Проверить Python-скрипты на неиспользуемые функции, дублирование логики (например, загрузка данных).
- **Затронутые файлы:** все `.py`.
- **Трудоёмкость:** S (0.5 дня).

### P2.5 Автоматическая генерация документации
- `cargo doc`, `pydoc` или `sphinx` для Python.
- **Затронутые файлы:** `Makefile` или `justfile`.
- **Трудоёмкость:** M (1 день).