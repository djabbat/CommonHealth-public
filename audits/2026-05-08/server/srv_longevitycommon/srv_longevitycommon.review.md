## VERDICT
**MAJOR_REVISION**

---

## SCORES (1-5)

| Criteria | Score | Rationale |
|---|---|---|
| **Architecture** | 4 | Двухслойная модель (научный слой + социальный слой) продумана, иерархия authority и границы ответственности описаны корректно. |
| **Optimality** | 3 | Присутствует дублирование (Python в BioSense/Ze, множество подпроектов с разными технологиями), большой объём репозитория (961M), портовый конфликт не исправлен. |
| **Structure / Modularity** | 3 | Subproject-границы определены, но TOXIC-проекты (HAP, Ontogenesis) не удалены, нарушая модульность. Внутри одного репозитория смешаны Rust, Python, Elixir, Node — это затрудняет независимую разработку. |
| **Systematicity (cross-file consistency)** | 2 | Core-файлы (CONCEPT, DESIGN, STATE) согласованы между собой, но код социального слоя не отражает обещанные изменения (см. STATE.md §5). Port conflict упомянут в DESIGN.md, но не исправлен. Subproject CONCEPTs не обновлены. |
| **Core-files vs code alignment** | 2 | Множество action-пунктов из STATE.md не выполнены (disclosure headers, banner, tooltips, endpoint). Стек реально использует Python, хотя заявлен «Rust+Phoenix only». |
| **Stack-rule compliance (Rust+Phoenix only)** | 1 | Серьёзное нарушение: **359 Python-файлов** (BioSense/src, Proteostasis/scripts, AIM, Ze/scripts, EpigeneticDrift/scripts, MCOA/scripts). Кроме того, присутствует Node (web/), который не относится к Phoenix. |
| **Modernity of stack** | 4 | Rust/axum, Elixir/Phoenix, React+TypeScript — современные технологии. Python используется для ML-скриптов, что допустимо, но нарушает правило моностека. |
| **Quality of processes / connections** | 2 | Отсутствует CI для umbrella-стека, нет интеграционных тестов, нет mock для внешних API. Port conflict не разрешён. Subproject CONCEPTs не синхронизированы. Процесс регенерации core .md не автоматизирован. |

---

## CRITICAL ISSUES

1. **Нарушение стека (Python)**  
   - `BioSense/src/ze_alpha_peak.py`, `eeg_ze_processor.py` и др. — 7 Python-файлов, критических для биомаркерного пайплайна.  
   - `AIM/` — полноценный Python-сервис (telegram_bot.py, llm.py, medical_system.py).  
   - `Proteostasis/scripts/calibrate.py`, `MCOA/scripts/compare_mcoa_cdata.py`, `EpigeneticDrift/scripts/calibrate.py` — калибровочные скрипты на Python.  
   - В code histogram `py 359` — нарушение «Rust+Phoenix only» даже для научного слоя.

2. **Port conflict realtime ↔ Ze (4001) не исправлен**  
   - `DESIGN.md §5.3` указывает перенести realtime на 4500, но `realtime/config/dev.exs` всё ещё использует 4001 (конфликт с `Ze/ze-backend`). Фактическая конфигурация не обновлена.

3. **Не выполнены обязательные изменения после CONCEPT v5.6**  
   - `STATE.md §5.1-5.4` содержит 18 пунктов (disclosure header, banner, tooltip, endpoint `/api/disclosures/v5_changes`, обновление system prompt). Ни один из них не отмечен как выполненный. Код социального слоя не соответствует актуальной научной политике (hypothesis-stage, NULL retraction, exploratory only).

4. **Subproject CONCEPT.md не синхронизированы**  
   - `OPEN_PROBLEMS.md §2.4` явно указывает на расхождение. Например, `Ze/CONCEPT.md` всё ещё может использовать старую формулировку «derived» для `dτ/dt`, тогда как umbrella CONCEPT v5.6 требует «ansatz». Проверка не проведена.

5. **Отсутствие CI и интеграционных тестов**  
   - `OPEN_PROBLEMS.md §3.1` — нет umbrella-пайплайна. Subproject тесты автономны, но социальный слой не проверяется вместе с Backend-симуляторами. Нет mock для BioSense API в тестах server/ (проблема §3.3).

6. **Большой размер репозитория (961M) со множеством мёртвых артефактов**  
   - `AIM/aim.db.backup-20260502-154658` (около нескольких МБ), `BioSense/data/`, `_archive/`, `_audits/`, много `node_modules` в `realtime/deps/`. Это затрудняет клонирование и CI.

---

## MINOR ISSUES

1. **TOXIC-проекты не удалены из дерева**  
   - `HAP/`, `Ontogenesis/` помечены как halted/failed PMID audits, но физически присутствуют, увеличивая шум.

2. **Нет автоматизации регенерации core .md**  
   - `scripts/regen_umbrella_core_from_article.sh` — пустой плейсхолдер. Дрифт между article и core-файлами может накапливаться.

3. **Дублирование конвенций для v (Ze velocity)**  
   - `THEORY.md §1` отмечает две конвенции (Python ∈ [0,1], Article ∈ [-1,+1]). Это потенциальный источник багов при интеграции.

4. **Отсутствует лицензия в подпроектах**  
   - Например, `AIM/`, `MCOA/` не имеют собственного LICENSE, хотя umbrella имеет MIT. Для публичных repo это требуется.

5. **Нестандартное расположение `AIM/`**  
   - AIM — отдельный Python-сервис с собственным Dockerfile и docker-compose.yml. Его логика пересекается с социальным слоем, но нет чёткой границы в DESIGN.md.

6. **Файлы `.db.backup` в репозитории**  
   - `AIM/aim.db.backup-20260502-154658` — SQLite dump; не должен быть под версионным контролем.

---

## STRENGTHS

-  **Проработанная двухслойная архитектура** с чётким разделением научного и социального слоя, API-контрактами (DESIGN.md §4).  
-  **Качественная документация core-файлов** – CONCEPT, THEORY, DESIGN, EVIDENCE, OPEN_PROBLEMS, STATE – с иерархией authority, falsifiability criteria и версионированием.  
-  **Честная рефлексия статуса** – открыто указаны NULL results, post-hoc nature, hypothesis-stage, threats, open blockers. Это редкость в современных проектах.

---

## ROOT CAUSES

1. **Отсутствие дисциплины моностека.** Архитектурное решение «Rust+Phoenix only» принято, но на практике научные пайплайны и ML-утилиты пишутся на Python без формального выделения в отдельный сервис с чётким интерфейсом.  
2. **Несинхронизированный цикл «документация → код».** Изменения в CONCEPT не проходят принудительную проверку на соответствие кода. Action list в STATE.md не имеет тикета/CI-шага, поэтому остаётся невыполненным.  
3. **Инкрементальное накопление артефактов.** Проект развивается долго, но нет процедур очистки (удаление TOXIC-подпроектов, бэкапов, депс-директорий). Отсутствует CI, который бы блокировал слияние с нарушением стека.  
4. **Отсутствие цикломатической интеграции.** Subproject автономны, но отсутствует тест на совместную работоспособность (например, server → BioSense passthrough). Это маскирует конфликты портов и API-несовместимости.

---

**Recommendation:** Немедленно удалить Python-зависимости из социального слоя, перенеся их в отдельные микросервисы с HTTP-интерфейсом. Исправить port conflict. Привязать к STATE.md CI-проверки (type-check, lint, diff against CONCEPT). Запустить umbrella-интеграционные тесты с mocks. Очистить репозиторий от мёртвых файлов. Без этих исправлений кодовая база не может считаться «reproducible research infrastructure».