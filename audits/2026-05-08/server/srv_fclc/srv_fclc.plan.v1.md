## План улучшений (Actionable, grouped by priority)

### P0 — Blockers (must fix before next release)

1. **Устранить дублирование серверных компонентов**
   - Удалить `backend/` и `frontend/` (дублируют `fclc-server` и `fclc-web` соответственно). Переименовать `fclc-server` → `fclc-orchestrator`, `fclc-web` → `fclc-frontend`. Обновить `Cargo.toml` workspace members.
   - *Затронутые файлы:* `/backend/`, `/frontend/`, `/Cargo.toml`, `/DESIGN.md`
   - *Трудоёмкость:* M (1–2 дня)
   - *Риск:* Средний (могут быть скрытые зависимости)

2. **Реализовать отсутствующий код DP и Shapley**
   - Создать модуль `fclc-core/src/dp/` с `gaussian_mechanism.rs`, `rdp_accountant.rs`, `tight_rdp_accountant.rs`. Реализовать `ShapleyEstimator` в `fclc-core/src/contribution/`. Подключить их к `fclc-server` и `fclc-node`.
   - *Затронутые файлы:* `/fclc-core/src/` (новые файлы), `/fclc-server/src/`, `/fclc-node/src/`
   - *Трудоёмкость:* L (3–5 дней)
   - *Риск:* Высокий (математическая корректность и интеграция)

3. **Написать интеграционные тесты для DP и взаимодействия node↔server**
   - Добавить проверки: (a) при DP-шумe выдерживается ε ≤ 1.0 (Poisson‑subsampled RDP), (b) SecAgg+ с dropout корректно агрегирует, (c) end‑to‑end round с двумя узлами.
   - *Затронутые файлы:* `/fclc-core/tests/`, `/fclc-server/tests/`, `/fclc-node/tests/`
   - *Трудоёмкость:* L (3–5 дней)
   - *Риск:* Средний (зависит от завершения DP‑кода)

4. **Привести документацию в соответствие с кодом**
   - Удалить или заменить ссылки на несуществующие файлы (`JOURNAL.md`, `ROADMAP.md`) в `README.md`. Удалить параметры DP из `PARAMETERS.md`, пока они не реализованы (или пометить как planned). В `DESIGN.md` описать актуальные компоненты (orchestrator, node, frontend) и их API, которые действительно существуют.
   - *Затронутые файлы:* `/README.md`, `/PARAMETERS.md`, `/DESIGN.md`, `/CONCEPT.md`
   - *Трудоёмкость:* M (1–2 дня)
   - *Риск:* Низкий

---

### P1 — Important (high impact but non‑blocking)

5. **Добавить CI/CD и файлы окружения**
   - Создать `.github/workflows/ci.yml` (cargo test + mix test), `.env.example` (переменные для узла и оркестратора), `.gitignore` (целевые каталоги, deps, node_modules). Обновить `docker-compose.yml` для запуска всех трёх сервисов (orchestrator, node, frontend).
   - *Затронутые файлы:* корень проекта, `/docker-compose.yml`
   - *Трудоёмкость:* M (1–2 дня)

6. **Устранить дублирование markdown-документации**
   - Объединить `EVIDENCE.md`, `CORRECTION_CANDIDATES.md`, `META_ANALYSIS_FCLC.md` и другие перекрывающиеся файлы в `/docs/` в одну структуру (например, `/docs/evidence.md`, `/docs/corrections.md`). Удалить `PITCH_LONGEVITY.md` и `PITCH_TECHNOCRATIC.md` (перенести содержание в `CONCEPT.md`).
   - *Затронутые файлы:* `/docs/*.md`, `/EVIDENCE.md`, `/PITCH_*.md`
   - *Трудоёмкость:* M (1–2 дня)

7. **Включить все крейты в workspace**
   - Добавить `backend` и `fclc-demogen` (если они не дубли) в `members` корневого `Cargo.toml`. Если `backend` — дубль, удалить. Если нужен для генерации демо-данных — оставить, но переименовать.
   - *Затронутые файлы:* `/Cargo.toml`, `/backend/Cargo.toml`, `/fclc-demogen/Cargo.toml`
   - *Трудоёмкость:* S (несколько часов)

8. **Покрыть DP-модули юнит-тестами**
   - Написать тесты для каждого нового модуля DP (Gaussian noise, RDP accountant, TightRdpAccountant). Проверить граничные случаи (ε=0, большие σ).
   - *Затронутые файлы:* `/fclc-core/src/dp/` (новые тесты)
   - *Трудоёмкость:* M (1–2 дня)

---

### P2 — Nice‑to‑have (improves quality but not critical)

9. **Оптимизировать структуру Rust-крейтов**
   - Объединить `fclc-core` + `fclc-node` + `fclc-server` в двухуровневую: `fclc-core` (общие примитивы), `fclc-node` (логика узла), `fclc-orchestrator` (логика оркестратора). Убрать чрезмерное дробление.
   - *Затронутые файлы:* дерево `fclc-*`, корневой `Cargo.toml`
   - *Трудоёмкость:* L (3–5 дней)

10. **Добавить OpenAPI-спецификацию для серверных API**
    - Создать `openapi.yaml` в `fclc-orchestrator/docs/` с описанием эндпоинтов `POST /api/v1/round/start`, `POST /node/v1/secagg/commit`. Использовать utoipa (rust) для генерации из кода.
    - *Затронутые файлы:* `/fclc-orchestrator/src/`, новый файл `/docs/openapi.yaml`

11. **Автоматизировать проверку соответствия документации и кода**
    - Написать скрипт (bash или Python в `scripts/`) который проверяет: (а) каждый упомянутый в `DESIGN.md` модуль существует в src; (б) каждый параметр из `PARAMETERS.md` имеет реализацию или помечен как planned.
    - *Затронутые файлы:* `/scripts/check_doc_code_consistency.py`

12. **Улучшить тестирование Python-скрипта генерации демо-данных**
    - Добавить модульные тесты для `generate_demo_data.py` (проверка формата CSV, соответствия OMOP-схеме). Разместить в `scripts/tests/`.
    - *Затронутые файлы:* `/scripts/generate_demo_data.py`, `/scripts/tests/` (новые)

---

**Примечание:** Все пункты P0 должны быть выполнены до следующего раунда ревью. После их реализации ожидаем повышение оценки по критериям Architecture и Structure до ≥3.