# План улучшений — srv_longevitycommon

## P0 — Блокеры (fail без этого)

### 1. Удалить Python-стек из основного репозитория (кроме AIM)
**Что:** Выделить все Python-пайплайны (BioSense EEG, Proteostasis calibrate, MCOA scripts, EpigeneticDrift) в отдельные микросервисы с HTTP API. Rust-код вызывает их только через REST. Сами Python-файлы удалить из корневого дерева.
**Файлы:**  
- `BioSense/src/ze_alpha_peak.py`, `eeg_ze_processor.py`, `ze_batch_pipeline.py`, … (все 7) → перенести в `BioSense/eeg-service/main.py` + новый Dockerfile  
- `Proteostasis/scripts/calibrate.py` → перенести в `Proteostasis/calibrate-service/`  
- `MCOA/scripts/compare_mcoa_cdata.py` → в `MCOA/analysis-service/`  
- `EpigeneticDrift/scripts/calibrate.py` → в `EpigeneticDrift/calibrate-service/`  
- `server/Cargo.toml` — удалить зависимость от Python subprocess (если есть)  
- `server/src/handlers/biosense.rs` — заменить прямой вызов на HTTP-запрос к eeg-service  
- `AIM/` оставить как допустимый ML-роутер, но перенести в `AIM/ml-router/` и добавить границу в DESIGN.md  
**Трудоёмкость:** M (3-4 дня)  
**Риск:** Medium — ломает интеграцию, требует новой настройки развёртывания

### 2. Исправить port конфликт realtime ↔ Ze :4001
**Что:** Изменить порт realtime Phoenix на 4500 во всех конфигах и в docker-compose.
**Файлы:**  
- `realtime/config/dev.exs` — заменить `port: 4001` → `port: 4500`  
- `realtime/config/prod.exs` (аналогично)  
- `deploy/docker-compose-all.yml` — обновить `ports:` для realtime  
**Трудоёмкость:** S (30 мин)  
**Риск:** Low

### 3. Выполнить все disclosure-изменения после CONCEPT v5.6 (18 пунктов из STATE.md §5)
**Что:** Реализовать каждый пункт из списка STATE.md §5.1–5.4 в коде социального слоя.
**Файлы:**  
- `server/src/handlers/biosense.rs` — добавить header `X-LongevityCommon-Status: hypothesis-stage-exploratory`  
- `server/src/handlers/dashboard.rs` — заменить "biological age" → "exploratory aging activity index (research only)"  
- `server/src/handlers/ze_guide.rs` — обновить system prompt (буквально из DESIGN.md §5.1)  
- `server/src/migrations/003_health_factors.sql` — добавить комментарий "thresholds exploratory, see CONCEPT v5.6 §2"  
- `server/src/main.rs` или `routes.rs` — добавить `GET /api/disclosures/v5_changes`  
- `web/src/pages/Dashboard.tsx` — banner "⚠ Hypothesis-stage research platform…"  
- `web/src/pages/Studies.tsx` — disclosure на каждой карточке  
- `web/src/pages/Profile.tsx` — tooltip "exploratory metric; not validated…"  
- `web/src/components/feed/PostComposer.tsx` — DOI warning для Longevity Horizon  
- `realtime/lib/longevitycommon_realtime_web/channels/feed_channel.ex` — metadata `{disclosure: "exploratory"}`  
- `deploy/docker-compose-all.yml` — env `LONGEVITYCOMMON_VERSION=v5.6`  
**Трудоёмкость:** L (2-3 дня на все правки и тестирование)  
**Риск:** Medium — пропуск одного пункта оставит несоответствие

### 4. Синхронизировать subproject CONCEPT.md с umbrella v5.6
**Что:** Проверить и отредактировать каждый subproject CONCEPT.md:  
- `Ze/CONCEPT.md` — заменить "derived" → "postulated ansatz" для `dτ/dt`  
- `BioSense/CONCEPT.md` — добавить "hypothesis-stage", "post-hoc multimodal", убрать "validated"  
- `FCLC/CONCEPT.md` — уточнить threat model "semi-honest only; not active server collusion"  
- `MCOA/CONCEPT.md` — добавить M4 порог (N≥2000, α=0.001, partial r²<0.05)  
- `CDATA/CONCEPT.md` — статус "inconclusive", Sobol p=0.12, deferred to Cell-DT v4.0  
**Файлы:** Указанные 5 файлов.  
**Трудоёмкость:** M (1 день на аудит + правки)  
**Риск:** Low

### 5. Создать umbrella CI + интеграционные тесты
**Что:** Добавить GitHub Actions workflow, который:  
- собирает каждый подпроект (Rust, Elixir, Node)  
- запускает cargo test, mix test, npm test  
- запускает интеграционный тест server → mock BioSense (wiremock) — проверяет header и disclosure  
- проверяет отсутствие Python файлов вне разрешённых (AIM/ml-router)  
**Файлы:**  
- `.github/workflows/ci.yml` (новый)  
- `server/tests/integration/` — добавить тест `test_biosense_passthrough.rs`  
- `server/Cargo.toml` — добавить `wiremock` в dev-dependencies (если нужно)  
**Трудоёмкость:** L (2-3 дня на настройку CI + написание тестов)  
**Риск:** Medium — возможно, потребуется исправить существующие тесты

### 6. Очистить репозиторий от мёртвых артефактов
**Что:**  
- Удалить `AIM/aim.db.backup-*`  
- Удалить `HAP/`, `Ontogenesis/` (TOXIC) — или перенести в `_archive/trash/`  
- Добавить в `.gitignore`: `node_modules/`, `*.db.backup`, `*.pyc`, `__pycache__/`, `target/` (если нет)  
- Удалить `BioSense/data/`, `_archive/`, `_audits/`? (оставить, если нужны для истории, но вынести в git LFS или отдельный репозиторий)  
- Очистить `realtime/deps/` — они не должны быть под git (уже в .gitignore?)  
**Файлы:** множество.  
**Трудоёмкость:** S (полдня)  
**Риск:** Low

---

## P1 — Важно (необходимо в ближайшие 2 недели)

### 1. Автоматизировать регенерацию core .md
**Что:** Реализовать скрипт `scripts/regen_umbrella_core_from_article.sh`, который:  
- вычисляет md5 от `~/Desktop/LongevityCommon.md`  
- сравнивает с текущим md5 в `CONCEPT.md` (поле article_md5)  
- при несовпадении архивирует старые core-файлы и перегенерирует новые из article  
**Файлы:** `scripts/regen_umbrella_core_from_article.sh` (новый), `CONCEPT.md` (добавить/обновить поле `article_md5`).  
**Трудоёмкость:** M (1 день)  
**Риск:** Low

### 2. Устранить дуализм конвенции v (Ze velocity)
**Что:** Выбрать единую конвенцию — `v ∈ [0,1]` (Python-convention) — и переписать Article, THEORY.md, BioSense Rust-код и Python-код в одном стиле.  
**Файлы:**  
- `THEORY.md` §1 — удалить "two conventions"  
- `BioSense/src/` (Rust) — проверить, что везде `[0,1]`  
- `BioSense/eeg-service/` (Python) — привести к `[0,1]`  
- `Article` (на десктопе) — исправить при следующей ревизии  
**Трудоёмкость:** M (полдня на поиск + правки)  
**Риск:** Low (аккуратно проверить математику)

### 3. Добавить лицензии во все подпроекты
**Что:** Скопировать `LICENSE` (MIT) в подпроекты, где его нет: `AIM/`, `MCOA/`, `BioSense/`, `Ze/`, `CDATA/`, `FCLC/`  
**Файлы:** по одному LICENSE на каждый подпроект.  
**Трудоёмкость:** S (15 мин)  
**Риск:** Low

### 4. Удалить .db.backup из git и добавить в .gitignore
**Что:** `git rm --cached` файлы `*.db.backup`; добавить `*.db.backup` в `.gitignore`.  
**Файлы:** `.gitignore` + удаление `AIM/aim.db.backup-*` и любых других.  
**Трудоёмкость:** S (10 мин)  
**Риск:** Low

### 5. Определить границы AIM в DESIGN.md
**Что:** Добавить раздел в `DESIGN.md` (например, §9) с описанием, что AIM — отдельный Python-сервис (ML-роутер), вызывается через HTTP, не влияет на социальный слой.  
**Файлы:** `DESIGN.md`  
**Трудоёмкость:** S (30 мин)  
**Риск:** Low

---

## P2 — Nice-to-have (после P0/P1)

### 1. Pre-commit hook для проверки стека
**Что:** Скрипт pre-commit, который запрещает коммит, если в staged файлах есть `.py` вне разрешённых директорий (`AIM/`, `services/`).  
**Файлы:** `.pre-commit-config.yaml` (если pre-commit) или `.githooks/pre-commit`.  
**Трудоёмкость:** S (1 час)

### 2. Внедрить Git LFS для больших файлов
**Что:** Перенести `BioSense/data/` (если >10MB) и `_archive/` в LFS.  
**Файлы:** `.gitattributes` + команды LFS.  
**Трудоёмкость:** S (30 мин)  
**Риск:** Low (если данные нужны в репозитории)

### 3. CI-проверка синхронизации core-файлов
**Что:** В GitHub action добавить шаг, который сравнивает md5 `~/Desktop/LongevityCommon.md` (или его копию в репо) с полем `article_md5` в `CONCEPT.md`. При несовпадении — warning.  
**Файлы:** `.github/workflows/ci.yml` (добавить шаг).  
**Трудоёмкость:** S (1 час)

### 4. Улучшить docstring и комментарии в server/handlers
**Что:** В `handlers/biosense.rs`, `dashboard.rs`, `ze_guide.rs` добавить краткие docstring с указанием experimental status и ссылку на CONCEPT.md.  
**Трудоёмкость:** S (30 мин)