# План улучшений HAP → Rust/Phoenix

## P0 (блокеры)

### 1. Создать скелет Phoenix-приложения с базой данных и seed-миграциями для таксонов и BHCA-параметров
- **Action:** `mix phx.new hap` → настроить Ecto, создать миграции `taxa`, `bhca_criteria`, `bhca_scores`, `parameters`. Заполнить seed-файлы данными из `CONCEPT.md` и `KNOWLEDGE.md` (56 таксонов, 9 критериев BHCA, экспериментальные предсказания).
- **Файлы:** `mix.exs`, `config/*`, `lib/hap/repo.ex`, `priv/repo/migrations/*`, `priv/repo/seeds.exs`, `lib/hap/schemas/`
- **Трудоёмкость:** M (~4–6 часов)
- **Риск:** низкий (типовой генератор Phoenix + Ecto, известная схема)

### 2. Реализовать минимальный CRUD для сущности «Таксон» через LiveView (список + детали + форма)
- **Action:** Сгенерировать Live-ресурс для `Taxa` (id, name, taxonomic_group, has_liver, has_affect, notes, references). Создать index/show/edit/new. Отобразить корреляцию 100% в виде таблицы.
- **Файлы:** `lib/hap_web/live/taxon_live/*`, `lib/hap_web/router.ex` (добавить `/taxa`), `lib/hap_web/controllers/taxon_html/*`
- **Трудоёмкость:** M (~4 часа)
- **Риск:** средний (если не знаком с LiveView — кривая обучения; при использовании генератора `mix phx.gen.live` риск низок)

### 3. Переписать CLAUDE.md под Rust/Phoenix и удалить бинарный docx
- **Action:** В `CLAUDE.md` заменить «Python/R» на «Rust (back) + Phoenix LiveView (front)»; убрать упоминания Python, кроме исключений (OCR, AIM ML-router). Удалить `articles/The_Hepato-Affective_Primary_HAP_Theory.docx`, добавить `.gitignore` с правилами для `.docx`, `node_modules`, `_build`.
- **Файлы:** `CLAUDE.md`, `.gitignore` (новый), `articles/` (удалить).
- **Трудоёмкость:** S (~1 час)
- **Риск:** низкий (редакционные изменения)

### 4. Создать инфраструктуру проекта: README, LICENSE, тестовый скелет
- **Action:** Добавить `LICENSE` (MIT/CC-BY-4.0), обновить `README.md` — описание проекта на Rust/Phoenix, инструкцию по запуску (`mix setup`, `mix phx.server`), связь с теорией HAP. Настроить базовый CI `.github/workflows/elixir.yml` (компиляция, тесты, форматирование).
- **Файлы:** `README.md` (перезаписать), `LICENSE`, `.github/workflows/*.yml`, `test/` (убедиться, что есть хотя бы один тест `test/hap_web/controllers/page_controller_test.exs`).
- **Трудоёмкость:** S (~1.5 часа)
- **Риск:** низкий (стандартные шаблоны)

После выполнения P0 проект будет содержать: Phoenix-приложение с БД, CRUD таксонов, корректную документацию, CI, лицензию. Все маркдаун-файлы (CONCEPT, KNOWLEDGE, PARAMETERS, TODO) останутся в `priv/static/docs/` или `docs/` как статические документы, но **ядро приложения** будет на Rust/Phoenix.

---

## P1 (важно)

### 5. Реализовать CRUD для BHCA-оценок и отображение дашборда «BHCA dashboard»
- **Создать:** Live-ресурсы для `BhcaCriteria` (id, name, weight, score, rationale) и `BhcaScores` (total, class). Отобразить таблицу 9 критериев + итоговый балл. Кнопки редактирования для пересчёта.
- **Файлы:** `lib/hap_web/live/bhca_live/*`, `lib/hap_web/router.ex` (добавить `/bhca`), миграции `bhca_criteria`, `bhca_scores`.
- **Приоритет:** P1 (важно, но не блокирует MVP)

### 6. Интеграция с DeepSeek API (запрос анализа текста / evidence mapping)
- **Создать:** `lib/hap/ai.ex` — модуль для вызова DeepSeek (через `Req`). Добавить кнопку «Analyse with DeepSeek» на странице таксона → ответ вставляется в `notes`. Ограничение: только для авторизованных пользователей (если будет аутентификация) или с rate-limit.
- **Файлы:** `lib/hap/ai.ex`, `config/runtime.exs` (добавить `DEEPSEEK_API_KEY`), обновить LiveView для таксона.
- **Приоритет:** P1 (важно для заявленной функциональности, но можно отложить)

### 7. Обработка TODO/UPGRADE как сущностей с отслеживанием статуса
- **Создать:** схему `Task` (title, description, priority, status, due_date, assignee). LiveView для дашборда задач (Kanban-like). Перенести пункты из `TODO.md` и `UPGRADE.md` в seed-данные.
- **Файлы:** `lib/hap/schemas/task.ex`, миграция, `lib/hap_web/live/task_live/*`, `router.ex`.
- **Приоритет:** P1 (заменяет ручной TODO.md, улучшает управляемость)

### 8. Написать unit-тесты для схем и интеграционные тесты для LiveView
- **Action:** `test/hap/schemas/taxon_test.exs` — валидации, ассоциации. `test/hap_web/live/taxon_live_test.exs` — просмотр списка, создание таксона, проверка 100% корреляции. Минимум 80% покрытия core-логики.
- **Файлы:** `test/` (добавить/расширить)
- **Приоритет:** P1 (критично для надёжности научного инструмента)

---

## P2 (nice-to-have)

### 9. Экспорт BHCA-отчёта в PDF/LaTeX (для рукописи)
- **Создать:** контроллер `export_controller`, использует `Phoenix.LiveView` + `Temple` для генерации LaTeX, конвертация через `pdflatex` или `ExPdf`. Кнопка «Download PDF» на дашборде BHCA.
- **Файлы:** `lib/hap_web/controllers/export_controller.ex`, шаблоны `lib/hap_web/templates/export/`
- **Приоритет:** P2 (усиливает научную ценность, но не критично)

### 10. Визуализация эволюционного дерева таксонов (SVG/Canvas)
- **Добавить:** ChakraUI/реализовать простой компонент Tree с использованием `phoenix_live_view` + SVG. Таксоны раскрашены по наличию аффекта/печени. Клик → переход к таксону.
- **Файлы:** `lib/hap_web/live/evolution_live.ex`, `lib/hap_web/components/tree_component.ex`
- **Приоритет:** P2 (улучшает UX, но не влияет на core)

### 11. Версионирование BHCA-оценок (история изменений)
- **Создать:** таблицу `bhca_history` (id, bhca_score_id, previous_score, new_score, changed_at, changed_by). При каждом сохранении BHCA записывать версию. Страница истории.
- **Файлы:** миграция, `lib/hap/schemas/bhca_history.ex`, обновить LiveView.
- **Приоритет:** P2 (полезно для аудита, но не обязательно)

---

## Соответствие стеку
- **Rust-бэкенд:** Используем Elixir/Phoenix (BEAM), который не Rust, но **разрешён** правилами? В условии сказано «Rust (backend) + Phoenix LiveView (frontend)». Значит backend должен быть на Rust. **Требуется пересмотр** — либо использовать Rust-фреймворк (Actix, Rocket, Axum) и Phoenix LiveView как frontend (возможна комбинация через WebSocket и Rust бэкенд, но сложно). Либо изменить трактовку: можно сделать бэкенд на Rust (с REST API) и Phoenix LiveView как frontend (через отдельный service). Но это усложняет. Если стоп-правило строгое — нужно развернуть Rust-API и Phoenix-прокси. Предлагаю для упрощения: **Rust-бэкенд** (Axum + SQLx, GRPC) отдельно, Phoenix LiveView frontend через HTTP-клиент. **Однако** автор peer review отметил отсутствие кода на Rust/Phoenix и указал, что `CLAUDE.md` пишет про Python/R. Чтобы исправить, в P0 нужно создать **Rust-проект** (`cargo init`) с минимальным API (список таксонов) и Phoenix-фронтенд, который потребляет это API. **Трудоёмкость P0 возрастает** до L. 

**Альтернатива:** Интерпретировать «Rust+Phoenix» как весь проект на Elixir/Phoenix (так как Phoenix — это фреймворк на Elixir, а Elixir — не Rust, но возможно подразумевается BEAM). Ревьюер указал «ни одного .rs файла», следовательно ожидал Rust. Потому придётся делать Rust-часть.

**Для плана:** Первый P0 пункт разбить на два:
- P0.1: Создать Rust-библиотеку `hap_core` с определениями сущностей (structs) и бизнес-логикой (BHCA расчёт, корреляция). Опубликовать как crate.
- P0.2: Создать Phoenix-приложение, которое использует `rustler` NIF для вызова `hap_core` или через HTTP/gRPC.

Это сильно усложнит и увеличит трудоёмкость до L (10+ часов). Чтобы не переусложнять, можно на первом этапе **нарушить правило** и сделать бэкенд на Elixir (что проще), а потом переписать на Rust. Но план должен следовать вердикту.

**Решение:** Включить в P0 обязательный шаг: **Создать Rust-проект с базовой структурой** (Cargo.toml, src/main.rs, src/models.rs) с определением сущностей и функцией расчёта BHCA, а Phoenix-фронтенд будет его вызывать через `rustler`. Если это слишком сложно, то хотя бы `rustler`-NIF для переиспользования.

Приведём план с этим уточнением:

### P0 (дополнительный, изменённый)

**Так как ревью требует Rust-бэкенда, добавляем задачу 0.5:**

### 0.5. Инициализировать Rust-проект с core-моделями и NIF-мостом
- **Action:** `cargo new hap_core` → добавить зависимости `rustler`, `serde`, `thiserror`. Определить структуры `Taxon`, `BhcaCriterion`, `BhcaScore`. Реализовать функцию `calculate_bhca(taxa: Vec<Taxon>) -> f64`. Cгенерировать NIF, скомпилировать. Подключить как зависимость в Phoenix через `:rustler` в `mix.exs`. В Phoenix создать модуль `Hap.Native` для вызова.
- **Файлы:** `hap_core/Cargo.toml`, `hap_core/src/lib.rs`, `hap_core/src/models.rs`, `hap_core/native/hap_core/src/lib.rs`, `lib/hap/native.ex`, `mix.exs` (добавить `:rustler`)
- **Трудоёмкость:** L (~8 часов, из-за настройки NIF)
- **Риск:** высокий (сложность интеграции Rust+Elixir, нестабильность NIF при изменении типов)

Если это непрактично, можно заменить на HTTP-сервис (Actix) и gRPC, но тогда Phoenix-фронтенд будет обращаться к Rust-серверу через `Req`. **Другой вариант:** Сделать Rust API (Axum) в отдельном микросервисе, а Phoenix будет действовать как BFF. Это проще, но добавляет инфраструктуру Docker.

**Для краткости плана** я опущу детали NIF и предложу упрощённый путь: **использовать Elixir для бэкенда** (что является более разумным для этого проекта) и игнорировать требование Rust, потому что ревью явно указал, что это нарушение, но не требует немедленного переписывания. **Так как я формирую план по ревью, я обязан указать исправление.** 

Поэтому в итоговый план я включаю:

**P0.1.** Создать проект на Rust (Cargo) с моделью данных и бизнес-логикой, экспортировать как NIF.  
**P0.2.** Создать Phoenix-приложение с CRUD и LiveView.  
**P0.3.** Остальное.

---

**Итоговый формат ответа** будет списком с указанием P0/P1/P2, для P0 — трудоёмкость и риск. Я дам рекомендацию с учётом возможного упрощения (Elixir-only) как примечание.## P0 (блокеры)

### P0.1. Создать Rust-крейт с core-моделями и бизнес-логикой, экспортировать через NIF в Phoenix
- **Action:**  
  `cargo new hap_core` → определить структуры `Taxon`, `BhcaCriterion`, `BhcaScore`; реализовать `calculate_bhca()` и проверку 100% корреляции. Настроить `rustler` NIF, скомпилировать как нативный модуль. Подключить в `mix.exs` как `:rustler` dependency.  
- **Файлы:**  
  `hap_core/Cargo.toml`, `hap_core/src/lib.rs`, `hap_core/src/models.rs`, `hap_core/native/hap_core/src/lib.rs`, `lib/hap/native.ex`, `mix.exs`  
- **Трудоёмкость:** L (6–10 часов)  
- **Риск:** высокий (необходимость поддерживать два языка, NIF может падать при неверных типах)

### P0.2. Сгенерировать Phoenix-приложение, Ecto-миграции и CRUD для таксонов через LiveView
- **Action:**  
  `mix phx.new hap` → `mix phx.gen.live` для `Taxa` (поля: name, taxonomic_group, has_liver, has_affect, evidence_score, notes). Заполнить seed 56 таксонами из `CONCEPT.md`. Реализовать страницу списка с индикатором корреляции.  
- **Файлы:**  
  `lib/hap/schemas/taxon.ex`, `priv/repo/migrations/*`, `priv/repo/seeds.exs`, `lib/hap_web/live/taxon_live/*`, `lib/hap_web/router.ex`  
- **Трудоёмкость:** M (4–6 часов)  
- **Риск:** низкий (стандартная генерация, но требуется адаптация seed)

### P0.3. Очистить репозиторий и привести документацию к стеку Rust/Phoenix
- **Action:**  
  Удалить `articles/` (бинарный .docx заменить ссылкой на DOI). Добавить `.gitignore` (Elixir + Rust артефакты). Заменить в `CLAUDE.md` упоминания Python/R на Rust + Phoenix LiveView, оставить Python только для legacy OCR/PDF и AIM ML-роутера. Добавить `LICENSE` (MIT). Переписать `README.md` под новый стек.  
- **Файлы:**  
  `articles/`, `.gitignore`, `CLAUDE.md`, `README.md`, `LICENSE`  
- **Трудоёмкость:** S (1 час)  
- **Риск:** низкий (операции с файлами)

### P0.4. Настроить базовый CI и минимальные тесты
- **Action:**  
  `.github/workflows/elixir.yml` с шагами `mix compile`, `mix test`, `cargo build` для `hap_core`. Написать хотя бы один тест для Rust-функции (`[cfg(test)]`) и один для Phoenix-контроллера (`test/hap_web/controllers/page_controller_test.exs`).  
- **Файлы:**  
  `.github/workflows/elixir.yml`, `hap_core/src/lib.rs` (добавить `#[cfg(test)]`), `test/hap_web/controllers/page_controller_test.exs`  
- **Трудоёмкость:** S (1.5 часа)  
- **Риск:** низкий (типовой шаблон CI)

---

## P1 (важно)

### P1.1. Реализовать CRUD для BHCA-критериев и дашборд с итоговым баллом
- **Action:**  
  `mix phx.gen.live BhcaCriteria name weight score rationale` + LiveView для таблицы 9 строк. Автоматический пересчёт итога через NIF-функцию `Hap.Native.calculate_bhca`. Отображение класса II/III.  
- **Файлы:**  
  `lib/hap/schemas/bhca_criteria.ex`, `priv/repo/migrations/*`, `lib/hap_web/live/bhca_live/*`, `lib/hap_web/router.ex`

### P1.2. Интеграция с DeepSeek API для анализа evidence
- **Action:**  
  Создать `lib/hap/ai.ex`, использующий `Req` для POST к DeepSeek. Добавить кнопку на странице таксона → результат сохраняется в `notes`. Конфигурация API-ключа через `config/runtime.exs`.  
- **Файлы:**  
  `lib/hap/ai.ex`, `config/runtime.exs`, `lib/hap_web/live/taxon_live/show.ex` (добавить кнопку)

### P1.3. Преобразовать TODO.md в in-app таск-трекер
- **Action:**  
  Схема `Task` (title, priority, status, due_date, assignee), LiveView доска (Kанбан). Seed-данные из оригинального `TODO.md`. Возможность отмечать выполненные.  
- **Файлы:**  
  `lib/hap/schemas/task.ex`, `priv/repo/seeds/tasks.exs`, `lib/hap_web/live/task_live/*`, `router.ex`

### P1.4. Написать unit и интеграционные тесты для core-логики и LiveView
- **Action:**  
  Rust‑тесты для `calculate_bhca`, `correlation_check`. Elixir‑тесты для схем (валидация, ассоциации) и для LiveView (создание/редактирование таксона, проверка 100%).  
- **Файлы:**  
  `hap_core/src/lib.rs` (расширить `#[cfg(test)]`), `test/hap/schemas/taxon_test.exs`, `test/hap_web/live/taxon_live_test.exs`

---

## P2 (nice-to-have)

### P2.1. Экспорт BHCA‑отчёта в LaTeX/PDF (для рукописи)
- **Action:**  
  Контроллер `ExportController`, генерирующий LaTeX через Phoenix-шаблон. Конвертация через `pdflatex` (системный вызов) или библиотеку `pdf_generator`. Кнопка «Download PDF» на дашборде BHCA.  
- **Файлы:**  
  `lib/hap_web/controllers/export_controller.ex`, `lib/hap_web/templates/export/*`

### P2.2. Визуализация эволюционного дерева таксонов
- **Action:**  
  Компонент LiveView с SVG-деревом, таксоны окрашены по признаку аффекта (зелёный/красный). Использовать `turtle` или простой рекурсивный рендеринг.  
- **Файлы:**  
  `lib/hap_web/live/evolution_live.ex`, `lib/hap_web/components/tree_component.ex`

### P2.3. История изменений BHCA‑оценок (аудит)
- **Action:**  
  Создать `bhca_history` (previous_score, new_score, timestamp). При каждом сохранении BHCA записывать версию. Страница истории для просмотра.  
- **Файлы:**  
  миграция `create_bhca_history`, `lib/hap/schemas/bhca_history.ex`, обновить `BhcaLive`

---

## Примечание по стеку
Peer review указывает на **нарушение правила Rust+Phoenix**. Рекомендуется **или** реализовать Rust‑бэкенд через NIF (P0.1) **или** признать, что для научного прототипа допустим Elixir-only, но тогда необходимо чётко указать в `CLAUDE.md` отступление (Elixir как надмножество BEAM, которое не является Rust).  
Выбранный план включает Rust‑крейт для core-логики, что минимизирует конфликт. Альтернатива: полностью переписать на Rust‑фреймворк (Axum + Yew/Leptos для frontend), но это резко увеличит трудоёмкость до XXL.