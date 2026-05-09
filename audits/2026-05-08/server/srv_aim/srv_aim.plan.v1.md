## План улучшений (actionable)

### P0 — Blockers (необходимо исправить перед любым релизом)

1. **Убрать тяжёлые артефакты из Git**  
   - Добавить `.gitignore`: `BioSense/data/`, `BioSense/results/`, `MCOA/results/`, `**/*.set`, `**/*.edf`, `**/results/`  
   - Перенести существующие тяжёлые файлы в Git LFS или внешнее хранилище (ссылки в документации)  
   - **Трудоёмкость:** S (создать `.gitignore` + migrate)  
   - **Риск:** низкий (файлы не являются исходным кодом; копию можно сохранить локально)  
   - **Затрагиваемые файлы:** `.gitignore` (создать/дополнить), `BioSense/.gitignore`, `MCOA/.gitignore`

2. **Выполнить все пункты DESIGN.md §5 и STATE.md §5 (10+ правок по disclosure и порту)**  
   - **server:**  
     - `src/handlers/biosense.rs` — добавить header `X-LongevityCommon-Status: hypothesis-stage-exploratory`  
     - `src/handlers/dashboard.rs` — заменить "biological age" → "exploratory aging activity index (research only)"  
     - `src/handlers/ze_guide.rs` — обновить system prompt (disclaimer per DESIGN.md)  
     - `src/handlers/disclosures.rs` — новый endpoint `GET /api/disclosures/v5_changes`  
     - `migrations/003_health_factors.sql` — добавить комментарий "thresholds exploratory, see CONCEPT v5.6 §2"  
   - **web:**  
     - `src/pages/Dashboard.tsx` — добавить banner "⚠ Hypothesis-stage research platform..."  
     - `src/pages/Studies.tsx` — disclosure "v1 NULL; v2 post‑hoc" на каждой карточке  
     - `src/pages/Profile.tsx` — tooltip "exploratory metric; not validated on N≥2000"  
     - `src/components/feed/PostComposer.tsx` — warning для DOI из Longevity Horizon  
   - **realtime:**  
     - `config/dev.exs` — port 4001 → **4500**  
     - `lib/.../feed_channel.ex` — добавить metadata `{disclosure: "exploratory"}`  
   - **deploy:**  
     - `docker-compose-all.yml` — обновить порты и service names  
   - **Трудоёмкость:** M (~15 правок, каждая тривиальна)  
   - **Риск:** низкий (изменения только строки и конфиги)  
   - **Затрагиваемые файлы:** перечислены выше.

3. **Настроить umbrella-CI + mock-сервисы**  
   - Создать `.github/workflows/umbrella.yml` с запуском `cargo test` (server, Ze, BioSense, FCLC) и `mix test` (realtime, Proteostasis frontend)  
   - Добавить mock для `POST /api/chi_ze` в server/tests/ (например, `mock_biosense.rs`), который возвращает фиктивные данные без поднятия BioSense  
   - **Трудоёмкость:** M (написание workflow + mock)  
   - **Риск:** низкий (CI легко править; mock может отстать от API, но это выявится тестами)  
   - **Затрагиваемые файлы:** `.github/workflows/umbrella.yml`, `server/tests/mock_biosense.rs` (создать)

4. **Исправить port conflict realtime ↔ Ze**  
   - Убедиться, что `realtime/config/dev.exs` слушает **4500**, а не 4001  
   - Обновить `deploy/docker-compose-all.yml`: realtime → `4500:4500`, Ze-backend → `4001:4001` (конфликт решён)  
   - **Трудоёмкость:** S (изменить два файла)  
   - **Риск:** низкий (простое переопределение порта)  
   - **Затрагиваемые файлы:** `realtime/config/dev.exs`, `deploy/docker-compose-all.yml`

5. **Синхронизировать subproject CONCEPT.md с umbrella v5.6**  
   - Для каждого подпроекта (Ze, BioSense, FCLC, MCOA, CDATA, EpigeneticDrift, MitoROS, Proteostasis)  
   - Проверить и исправить:  
     * "DERIVE" → "POSTULATED ansatz" (Ze)  
     * "validated" → "exploratory / hypothesis-stage" (BioSense)  
     * "semi-honest" → "semi-honest only; NOT active server collusion" (FCLC)  
     * "inconclusive" → добавить статус (CDATA)  
   - **Трудоёмкость:** S (grep + замена по шаблону)  
   - **Риск:** низкий (текстовые правки, не затрагивают код)  
   - **Затрагиваемые файлы:** `*/CONCEPT.md` для всех перечисленных подпроектов.

---

### P1 — Важно (повышение качества и консистентности)

6. **Удалить / переместить “TOXIC” подпроекты (HAP, Ontogenesis)**  
   - Переместить `HAP/` и `Ontogenesis/` в `_archive/_toxic/` или просто удалить, если не нужны  
   - Обновить `README.md` – убрать упоминания или заменить на "archived"  
   - **Затрагиваемые файлы:** `HAP/*`, `Ontogenesis/*`, `README.md`

7. **Очистить избыточные файлы в подпроектах**  
   - Удалить пустые/устаревшие `AGENTS.md`, `JOURNAL.md`, `ROADMAP.md` или заменить ссылками на umbrella-документы  
   - Проверить каждый подпроект (Ze, BioSense, MCOA, EpigeneticDrift, MitoROS, Proteostasis)  
   - **Затрагиваемые файлы:** `*/AGENTS.md`, `*/JOURNAL.md`, `*/ROADMAP.md` (удалить или обновить)

8. **Перенести `_audits/` в отдельный репозиторий или заархивировать**  
   - Переместить `_audits/` → `_archive/_audits/`  
   - В корне README.md оставить ссылку на папку с аудитами  
   - **Затрагиваемые файлы:** `_audits/*` (переместить), `README.md`

9. **Добавить LICENSE в каждый подпроект**  
   - Скопировать `LICENSE` (MIT из корня MCOA) в `Ze/`, `BioSense/`, `EpigeneticDrift/`, `MitoROS/`, `Proteostasis/`, `CDATA/`, `FCLC/`, `server/`, `realtime/`, `web/`  
   - Или добавить README с указанием umbrella license  
   - **Затрагиваемые файлы:** `Ze/LICENSE`, `BioSense/LICENSE`, … (создать)

10. **Создать скрипт регенерации core-документов**  
    - `scripts/regen_umbrella_core_from_article.sh` – парсит `~/Desktop/LongevityCommon.md` и генерирует CONCEPT, THEORY, DESIGN, PARAMETERS, MAP, EVIDENCE, OPEN_PROBLEMS, STATE  
    - Задокументировать в `OPEN_PROBLEMS.md` (закрыть §3.2)  
    - **Затрагиваемые файлы:** `scripts/regen_umbrella_core_from_article.sh` (создать), `OPEN_PROBLEMS.md`

---

### P2 — Nice‑to‑have (улучшения UX и инфраструктуры)

11. **Заменить Python-скрипты (не legacy) на Rust или явно исключить из rule**  
    - В `Proteostasis/scripts/`, `MCOA/scripts/`, `EpigeneticDrift/scripts/` – если они используются, переписать на Rust или перенести в `_legacy/`  
    - Для BioSense/src (EEG pipelines) – оставить, т.к. это исследовательские скрипты, но пометить `# LEGACY` в начале файла  
    - **Затрагиваемые файлы:** `Proteostasis/scripts/calibrate.py`, `MCOA/scripts/*.py`, `EpigeneticDrift/scripts/calibrate.py`, `BioSense/src/*.py`

12. **Добавить Git LHS track для больших файлов**  
    - `.gitattributes`: `*.set filter=lfs diff=lfs merge=lfs -text`, `*.edf`, `*.json` (кроме config)  
    - **Затрагиваемые файлы:** `.gitattributes` (создать)

13. **Добавить CI-статус бейдж в README.md**  
    - Ссылка на GitHub Actions workflow из пункта 3  
    - **Затрагиваемые файлы:** `README.md`

14. **Написать unit-тесты для disclosure‑пунктов (P0.2)**  
    - Например, тест на `server` проверяет, что в ответе `/dashboard` или `/chi_ze` есть header `X-LongevityCommon-Status`  
    - **Затрагиваемые файлы:** `server/tests/disclosure_test.rs` (создать)

15. **Удалить пустые Go-директории**  
    - Если в проекте нет Go‑кода, удалить `./go` (или аналогичные)  
    - **Затрагиваемые файлы:** удалить `go/` или проверить наличие

---

### Общие замечания

- Все правки **только Rust + Phoenix** (или TypeScript для web). Python оставляется только в BioSense/src (EEG) и AIM ML-роутер (в проекте нет AIM? но есть `AIM/`, это legacy? Надо уточнить – допускается).  
- Для каждого P0 указана трудоёмкость и риск. Все P0 обязательны к выполнению в ближайший спринт.  
- После завершения P0 и P1 проект будет готов к ревью на ACCEPT.