## План улучшений — PR-1 (по ревью проекта CytogeneticTree)

**Общая цель:** Перевести проект из фазы «только документация» в минимально работоспособный программный продукт на стеке Rust + Phoenix, сохранив научную целостность.

---

### P0 – Блокеры (обязательны для релиза)

| № | Действие | Затронутые файлы | Трудоёмкость | Риск |
|---|----------|------------------|--------------|------|
| 0.1 | **Добавить Cargo.toml и пустую crate-структуру для GenealogyReconstruction** — минимальный Rust-крейт с petgraph, serde, clap. Удалить/заменить Python-зависимости. | `GenealogyReconstruction/Cargo.toml`, `GenealogyReconstruction/src/lib.rs`, `GenealogyReconstruction/src/main.rs` | S (1 ч) | Низкий |
| 0.2 | **Реализовать базовый DAG (LineageGraph) на petgraph** — структуры `DivisionEvent`, `CellNode`, функции добавления деления, топологическая сортировка. | `GenealogyReconstruction/src/lineage.rs`, `GenealogyReconstruction/src/lib.rs` | M (4 ч) | Средний (синтаксис petgraph) |
| 0.3 | **Добавить юнит-тесты для LineageGraph** — корректность добавления, циклическая проверка, сериализация в JSON. | `GenealogyReconstruction/src/lineage.rs`, `GenealogyReconstruction/tests/test_lineage.rs` | S (2 ч) | Низкий |
| 0.4 | **Создать GitHub Actions CI** — сборка, тесты, clippy, rustfmt. Заблокировать мерж при failure. | `.github/workflows/ci.yml` | S (1 ч) | Низкий |
| 0.5 | **Добавить `.gitignore` (Rust-специфичный), `rust-toolchain.toml`, `Cargo.lock`** — для воспроизводимости сборки. | `.gitignore`, `rust-toolchain.toml` | S (0.3 ч) | Низкий |
| 0.6 | **Заменить Python-зависимости в других модулях (кроме разрешённых) на Rust-эквиваленты** — например, ImageAnalysis (CellProfiler → Rust-крейт imageproc, ndarray). Создать заглушки. | `ImageAnalysis/Cargo.toml`, `ImageAnalysis/src/lib.rs`, `ImageAnalysis/src/pipeline.rs` | L (16 ч) | Высокий (перенос алгоритмов) |
| 0.7 | **Определить protobuf-схемы для межмодульного взаимодействия** — хотя бы для `SegmentationOutput` и `AblationCommand`. | `protos/segmentation.proto`, `protos/ablation.proto` | M (6 ч) | Средний |

---

### P1 – Важные улучшения (повышают качество и расширяемость)

| № | Действие | Затронутые файлы | Трудоёмкость |
|---|----------|------------------|--------------|
| 1.1 | **Реализовать заглушку MicroscopeController на Rust (TCP/JSON-RPC)** — имитирует приём команд и отправку статуса. | `MicroscopeController/Cargo.toml`, `MicroscopeController/src/main.rs`, `MicroscopeController/src/controller.rs` | M (6 ч) |
| 1.2 | **Добавить модуль AICoordinator как Elixir/Phoenix GenServer** — минимальный цикл: читает segmentation output → классифицирует (mock) → шлёт команду MicroscopeController. | `AICoordinator/mix.exs`, `AICoordinator/lib/aicoordinator.ex`, `AICoordinator/lib/orchestrator.ex` | L (12 ч) |
| 1.3 | **Создать интеграционный тест между GenealogyReconstruction и AICoordinator** — на mock-данных. | `GenealogyReconstruction/tests/integration_test.rs`, `AICoordinator/test/orchestrator_test.exs` | M (4 ч) |
| 1.4 | **Ввести Rust workspace** — объединить все Rust-крейты в один монорепозиторий. | Корневой `Cargo.toml` (workspace), перемещение поддиректорий | S (1 ч) |
| 1.5 | **Перенести операционные параметры (токеновый бюджет, стоимость API) из PARAMETERS.md в отдельный `/ops/` каталог** — очистить архитектурные файлы от операционных данных. | Создать `ops/budget.md`, `ops/api_costs.md`; удалить лишние строки из `AICoordinator/PARAMETERS.md` | S (0.5 ч) |
| 1.6 | **Добавить примеры входных/выходных данных (json/csv) для ImageAnalysis и CellPose_Segmentation** — в `examples/` каталог. | `ImageAnalysis/examples/`, `CellPose_Segmentation/examples/` | S (1 ч) |

---

### P2 – Nice-to-have (улучшения документации и удобства)

| № | Действие | Затронутые файлы |
|---|----------|------------------|
| 2.1 | **Удалить дублирующиеся описания из README субпроектов** — оставить только уникальные детали, добавить ссылки на корневой CONCEPT.md. | Все `*/README.md` |
| 2.2 | **Разделить TODO.md на engineering roadmap и scientific plan** — engineering часть в ISSUES, научная в `SCIENTIFIC_PLAN.md`. | `TODO.md`, новый `SCIENTIFIC_PLAN.md` |
| 2.3 | **Добавить `.editorconfig` и `flake.nix` / `docker-compose.yml` для разработки** — упростить окружение. | `.editorconfig`, `flake.nix`, `docker-compose.yml` |
| 2.4 | **Переименовать Python-модули, которые не являются legacy OCR/PDF или AIM, в соответствии со стеком** — пометить как deprecated, если временно оставлены. | `ImageAnalysis/pipelines/*.py` → перенести в `deprecated/` с комментарием о будущем рефакторинге. |

---

### Важные замечания

1. **Python разрешён только для legacy OCR/PDF и AIM ML-роутера.** Все модули CytogeneticTree (GenealogyReconstruction, AICoordinator, MicroscopeController, ImageAnalysis, StatisticalAnalysis и др.) должны быть реализованы на Rust (core logic) или Elixir/Phoenix (LiveView frontend и оркестрация). В плане выше мы уже убрали Python из ключевых модулей.
2. **Для AICoordinator допускается интеграция с внешними LLM (Claude, DeepSeek) через HTTP API, но сам оркестратор — на Elixir GenServer.**
3. **После выполнения P0 проект получит статус «Minimum Viable Codebase»** — можно будет запустить CI, увидеть тесты, скомпилировать. Это минимум для повторного рассмотрения.

**Рекомендуемый порядок:** P0.1 → P0.2 → P0.3 → P0.4 → P0.5 → P0.7 → P0.6 (самый трудоёмкий) → P1 → P2.

**Оценка общего времени на P0:** ~26-30 человеко-часов (при условии, что разработчик знаком с Rust и Elixir).  
**Риски:** Высокий для P0.6 (перенос image analysis с Python на Rust), средний для P0.7 (согласование схем).