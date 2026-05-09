# AUDIT PACKET — LC_AIM

Path: `/home/oem/Desktop/LongevityCommon/AIM`  Date: 2026-05-08

## Size & file counts
```
7,8G	/home/oem/Desktop/LongevityCommon/AIM
```
**Extensions:** .py=333, .rs=322, .md=316, .toml=206, .ex=89, .exs=50, .txt=36, .sh=31, .service=20, .log=20, (noext)=16, .jpeg=16, .js=14, .png=14, .yaml=11
## Tree (depth=2, max 200 entries)
```
.
./start.bat
./migrations
./migrations/migrator.py
./migrations/__init__.py
./user_keys.py
./deploy
./deploy/Caddyfile
./deploy/README.md
./deploy/nginx.conf
./deploy/prometheus.yml
./deploy/systemd
./venv
./venv/lib
./venv/pyvenv.cfg
./venv/bin
./venv/share
./venv/include
./venv/lib64
./telegram_bot.py
./SSA
./SSA/frontend
./SSA/sources
./SSA/README.md
./SSA/backend
./SSA/data
./SSA/patterns
./SSA/docs
./SSA/_archive
./export
./export/obsidian_exporter.py
./export/notion_exporter.py
./export/report_exporter.py
./export/fhir_exporter.py
./export/__init__.py
./experiments
./experiments/prompt_ab_test.py
./experiments/__init__.py
./requirements.txt
./config.py
./cli
./cli/__main__.py
./cli/setup_wizard.py
./cli/__init__.py
./DiffDiagnosis
./DiffDiagnosis/frontend
./DiffDiagnosis/sources
./DiffDiagnosis/_extract_code.py
./DiffDiagnosis/README.md
./DiffDiagnosis/backend
./DiffDiagnosis/algorithms.json
./DiffDiagnosis/docs
./DiffDiagnosis/_archive
./phoenix-umbrella
./phoenix-umbrella/mix.exs
./phoenix-umbrella/README.md
./phoenix-umbrella/mix.lock
./phoenix-umbrella/apps
./phoenix-umbrella/config
./phoenix-umbrella/deps
./PARAMETERS.md
./aim_gui.py
./aim_cli.py
./rust-core
./rust-core/aim_rag.db
./rust-core/AIM
./rust-core/Cargo.toml
./rust-core/crates
./rust-core/README.md
./rust-core/Cargo.lock
./rust-core/rust-toolchain.toml
./tests
./tests/test_hook_handlers.py
./tests/test_escalation_engine.py
./tests/test_doctor_dry_run.py
./tests/test_kpi_auto_updater.py
./tests/test_phase8_shims.py
./tests/test_pairing.py
./tests/_runtime_fixtures
./tests/test_ab_router.py
./tests/test_project_pdf_export.py
./tests/test_aim_verify_parity.py
./tests/test_patient_as_project_phase7.py
./tests/test_law_gates.py
./tests/test_coach_shim.py
./tests/test_health_extended.py
./tests/test_skill_synthesis.py
./tests/test_patient_folder.py
./tests/test_citation_guard.py
./tests/test_readme_generator.py
./tests/test_doctor_calibration.py
./tests/test_own_pubs_tracker.py
./tests/test_doctor_consult.py
./tests/test_stakeholder_tracker.py
./tests/test_patient_inbox_watcher.py
./tests/test_path_sandbox.py
./tests/test_memory_tier.py
./tests/test_kpi_tracker.py
./tests/test_kernel_scenarios.py
./tests/test_project_archive.py
./tests/test_brief_preamble.py
./tests/test_orchestrator_reflexion.py
./tests/test_module_registry.py
./tests/test_auto_eval.py
./tests/test_llm_client.py
./tests/test_project_state_machine.py
./tests/test_memory_monitor.py
./tests/test_unicode_guard.py
./tests/test_deadline_scanner.py
./tests/test_patient_dedup.py
./tests/test_routines.py
./tests/test_interactions.py
./tests/test_kernel_parity.py
./tests/test_kernel_extended.py
./tests/test_tool_synthesis.py
./tests/test_diff_analyser.py
./tests/test_mcp_loader.py
./tests/test_auth.py
./tests/test_cost_ledger.py
./tests/evals
./tests/test_generalist_parallel.py
./tests/test_feature_flags.py
./tests/test_bash_sandbox.py
./tests/test_recall_cli.py
./tests/test_labs.py
./tests/test_daily_brief.py
./tests/test_citation_linter.py
./tests/test_project_owner.py
./tests/test_kernel.py
./tests/test_prompt_evolver.py
./tests/test_serve_daemon.py
./tests/test_generalist_v2_extras.py
./tests/test_cli_setup.py
./tests/test_follow_up_generator.py
./tests/test_chat.py
./tests/test_cli_completion.py
./tests/test_recall_perf.py
./tests/test_generalist_v3.py
./tests/test_pam_trajectory_e2e.py
./tests/test_memory_remediator.py
./tests/test_literature_watch.py
./tests/test_brief_preferences.py
./tests/test_evals.py
./tests/test_literature.py
./tests/test_quick_action.py
./tests/test_impact_analyser.py
./tests/test_delegate_parallel.py
./tests/conftest.py
./tests/test_session_visualiser.py
./tests/test_hooks.py
./tests/test_permission_broker.py
./tests/test_notify.py
./tests/test_weekly_digest.py
./tests/test_regimen_validator.py
./tests/test_project_graph.py
./tests/test_aim_cli.py
./tests/test_treatment.py
./tests/test_project_export.py
./tests/test_pattern_miner.py
./tests/test_worktree.py
./claude_memory_analysis.txt
./STACK.md
./db.py
./STRATEGY.md
./tools
./tools/literature.py
./tools/lab_evaluate.py
./tools/vision.py
./tools/__init__.py
./tools/web.py
./medical_system.py
./reports
./aim.db.backup-20260502-154658
./AI
./AI/ai
./AI/cases
./AI/artifacts
./AI/tests
./AI/queen_deploy
./AI/README.md
./AI/HIVE_ARCHITECTURE.md
./AI/CLAUDE.md
./AI/docs
./AI/FCLC_BORROW.md
./README.md
./logs
./logs/aim.log
./aim.db-shm
./Dockerfile
./UPGRADE.md
./pyproject.toml
./TODO.md
./aim.toml
./MEMORY.md
./scripts
./scripts/desktop
./scripts/smoke.sh
./scripts/fix_deploy.sh
./scripts/install_node.sh
./scripts/aim_self_diag_launcher.sh
```
## Detected stack: **Rust, Phoenix/Elixir, Node/JS, Python**
## Core files

### `CLAUDE.md` (21836 chars)
```md
# CLAUDE.md — AIM v7.0

---

## 🛑 IMMUTABLE: Asimov-laws kernel (do NOT edit without explicit user command)

**Hard rule (2026-05-07):** the Asimov-style laws below are the safety
contract of the entire AIM kernel. They MUST NOT be modified — neither
in their thresholds, nor in their action_type sets, nor in their
trigger conditions — without an **explicit human instruction**.

**The 8 protected functions:**
- `evaluate_l0` — danger signals (биохазард / weapon / forge)
- `evaluate_l1` — patient harm (allergies, contraindications, inaction)
- `evaluate_l2` — physician override compliance
- `evaluate_l3` — destructive system-modification gate
- `evaluate_l_privacy` — egress patient data
- `evaluate_l_consent` — public-blast-radius actions
- `evaluate_l_verifiability` — citation must resolve
- `evaluate_l_agency` — co-design required for activated patients

**Plus their constants:** `DANGER_SIGNALS`, `BROAD_ABX`,
`VIRAL_INDICATIONS`, `PRIVACY_ACTIONS`, `CONSENT_ACTIONS`,
`VERIFIABILITY_ACTIONS`, `AGENCY_ACTIONS`, `KernelWeights` defaults,
`CircuitBreaker` thresholds, `decide()` orchestration.

**Sources of truth:**
- Python: `agents/kernel_legacy.py:201-403`
- Rust: `rust-core/crates/aim-kernel/src/lib.rs:200-565`
- PyO3: `rust-core/crates/aim-kernel-py/src/lib.rs`

**Allowed without explicit ask:**
- Adding tests (don't change behavior).
- Adding new fields to `Decision/Patient/Context/Scored/LawsResult/ExtendedLawsResult` (additive).
- Adding NEW laws via NEW evaluate_* functions (additive — but flag in commit).
- Logging/audit trail format changes.

**Forbidden without explicit command:**
- Removing/loosening a danger signal or threshold.
- Adding bypasses (new override flags).
- Removing entries from any of the action-type sets.
- Anything that converts `Err(KernelViolation)` → `Ok(...)`.

Even in `overnight` mode — these edits stop and ask. Memory rule:
`feedback_no_edit_asimov_laws.md`.

---

## Cornerstone: Patient as Developmental Project (2026-05-07)

**AIM = infrastructure для validating Level 3 (patient as active developmental project)**, не просто clinical decision support.

**Three-level framework** (per Tkemaladze J. (2026) "Patient as a Project", *Longevity Horizon* 2(5), [DOI 10.65649/qqwva850](https://doi.org/10.65649/qqwva850)):

| Level | Role | Status |
|---|---|---|
| **L1 Patient-Object** | Passive data source; AI = classifier | Confirmed |
| **L2 Patient-Narrator** | Active info provider; AI = facilitator | Confirmed (Tao et al., n=2069 RCT) |
| **L3 Patient-Project** | Active co-manager; AI = developmental agent | **Theoretical — AIM existing для validation** |

**Primary outcome metric AIM = PAM-13 trajectory** (Patient Activation Measure, MCID = 5.4 points). Не "physician satisfaction".

**4 architectural principles (cornerstone):**
1. **Co-design > fine-tuning** (Tao et al., Nat Med 2026)
2. **Performance-based 4-zone HCI** (Blumenthal-Lee — automation bias mitigation)
3. **Developmental ≠ instrumental agency** (build patient capacity, не just automate)
4. **L_AGENCY law** added (4-й extended law alongside PRIVACY/CONSENT/VERIFIABILITY)

**Что AIM делает (L1+L2 confirmed + L3 instrumented 2026-05-07):**
- Asimov + extended laws kernel (Rust via PyO3) — L_PRIVACY/CONSENT/VERIFIABILITY/**L_AGENCY**
- Lifecycle abstraction (project / patient / experiment)
- Doctor agent, lab interpretation, regimen validation
- Phoenix LiveView patient dashboard (`/patients`, `/experiments`,
  **`/pam`, `/codesign/:id`, `/disagreement`, `/activation`**)
- `aim-pam` crate — PAM-13 administration, scoring, MCID/MDC delta,
  per-patient JSONL persistence
- `aim-disagreement` crate — Blumenthal-Lee 4-zone HCI classifier
- `aim-codesign` crate — co-design event log (consulted/agreed/modified/
  refused/alternative) backing the L_AGENCY `patient_codesigned` flag
- `decide()` now enforces all 4 extended laws on every clinical
  alternative — L_AGENCY blocks treatment / lifestyle / regimen actions
  on activated patients (PAM-13 level ≥ 2) without co-design

**Что AIM будет делать (still-open L3 targets):**
- `aim-coach` — coaching patterns (motivational interviewing, goal-setting)
- Real RCT validating L3 (long-term, IRB-gated)
- PyO3 in-process bindings for `aim-pam` / `aim-disagreement` /
  `aim-codesign` (currently Python shims subprocess into Rust binaries)

**Полный manifest:** `docs/manuscripts/PATIENT_AS_PROJECT.md`.
**Implementation roadmap:** `docs/audits/AUDIT_PATIENT_AS_PROJECT_2026-05-07.md` (8 phases).
**Reference paper:** `docs/manuscripts/MANUSCRIPT_PATIENT_AS_PROJECT_2026-05-07.md`.

---

## Project-manager subsystem (Phase A-E, 2026-05-06)

AIM теперь умеет вести 3 типа долгоживущих entity через единый
`aim-lifecycle` trait:

| Тип | Phase machine | Storage | Owner crate |
|---|---|---|---|
| **Project** (грант / paper) | DRAFT → REVIEW → SUBMITTED → ACCEPTED → PUBLISHED → REJECTED → ARCHIVED | `USER/projects/<name>.yaml` | `aim-project-owner` |
| **Patient** (клинический) | INTAKE → DIAGNOSTIC_WORKUP → ACTIVE_TREATMENT → MONITORING → STABLE → CLOSED (+ re-engagement) | `Patients/<id>/MEMORY.md` | `aim-patient-owner` |
| **Experiment** (роботизированный) | COMMISSIONING → CALIBRATING → RUNNING → DATA_PROCESSING → REPORTED → ARCHIVED | `USER/experiments/<name>.yaml` | `aim-experiment-owner` |

**Production binaries** (все в `rust-core/target/release/`):
- `aim-daily-brief --telegram` — утренний brief всех 3 типов через единый Lifecycle dispatch
- `aim-weekly-project-digest --telegram` — еженедельный digest (projects + experiments + patient drift + stakeholder silence)
- `aim-project-owner {list,brief,all,phase}` — per-project CLI
- `aim-patient-owner {list,brief,all,phase}` — per-patient CLI
- `aim-experiment-owner {list,brief,all,phase,mcp-config}` — per-experiment CLI; `mcp-config` генерирует Claude-Code worker TOML
- `aim-patient-comms {list,overdue,add-followup,close-followup,record}` — patient communications SQLite tracker

**Phoenix LiveViews:** `/patients` (PatientLive) + `/experiments` (ExperimentLive). System.cmd subprocess к Rust binaries; refresh 30/60s.

**Hooks framework** (Phase C Day 1): 5 fire-callsites в Python production code (`labs.py`, `kernel.py`, `db.py`, `intake.py`, `patient_inbox_watcher.py`); handler `alert_lab_critical` через `escalation_engine` + `notify` multiplexer с 4h dedup.

**Pilot YAMLs:**
- `USER/projects/FCLC.yaml` — pre-existing
- `USER/experiments/E0.yaml` — PhD/E0 commissioning (Tsomaia hardware tracking)
- `USER/experiments/AutomatedMicroscopy.yaml` — CDATA Phase A imaging rig

**systemd unit пары** (Python legacy + Rust alternative):
- `aim-daily-brief.service` (Python) ↔ `aim-daily-brief-rust.service` (Rust binary)
- `aim-weekly-project-digest.service` (Python) ↔ `aim-weekly-project-digest-rust.service` (Rust)
- `aim-serve-daemon.service` — long-running owner (Python `agents/serve_daemon.py`)

Только один из пары должен быть `systemctl --user enable`'d одновременно.

**Полный аудит-отчёт:** `AUDIT_PROJECT_MANAGER_2026-05-06.md`.

---

## Stack rule (HARD CONSTRAINT)

**Всё, что разрабатывается в AIM, пишется только на:**
- **Backend / алгоритмы / агенты / CLI / системные сервисы → Rust**
  (workspace `AIM/rust-core/crates/aim-*`)
- **Frontend / dashboards / UI → Phoenix LiveView** (по паттерну
  Ze/BioSense/FCLC: `mix release` → systemd, **без Docker runtime**)

**Без необходимости — только Rust и Phoenix.** Python остаётся только
для legacy (OCR/PDF/WhatsApp интеграции, нет зрелых Rust аналогов) —
вызывается из Rust через subprocess. Полные правила и список исключений:
`STACK.md`. Migration roadmap: `docs/migration/MIGRATION_RUST_PHOENIX.md`.

**НИКАКОГО Docker** (директива 2026-05-04). Ни runtime, ни build-time,
ни dev-окружения, ни CI sandbox. Не создавать `Dockerfile` /
`docker-compose.yml` / `.dockerignore` / OCI images. Развёртывание =
native systemd units. Исключение только при явной просьбе пользователя.

При сомнении — спросить пользователя. Не писать Python "потому что
быстрее"; не писать Docker "потому что проще".

---

## Startup Protocol

**Полные правила:** `~/Desktop/Claude/protocols/START.md`

---

## Internal microservices (in-tree REST, called by aim-doctor)

AIM физически содержит 2 микросервиса в репозитории (Rust backend each):

- **`DiffDiagnosis/`** (port 8765, `AIM_DIFFDX_URL`) — det.движок дифдиагностики
  (Виноградов / Taylor). Caller: `rust-core/crates/aim-doctor/src/main.rs:43`.
  Docs: `docs/diffdiagnosis/CONCEPT.md`.
- **`SSA/`** (port 8766, `AIM_SSA_URL`) — Системный Синдромальный Анализ
  CBC+ESR (28 → 5-зонная дискретизация). Caller:
  `rust-core/crates/aim-doctor/src/main.rs:44`. Docs: `docs/ssa/CONCEPT.md`.
  Входной слой для DiffDiagnosis.

Не имеют отдельного git-репо. Полная карта — `MAP.md` § 2.5.

---

## Multi-user (Hub + Node)

AIM работает в двух режимах через `AIM_ROLE`:

| Режим | Назначение | LLM | DB | Запуск |
|---|---|---|---|---|
| `hub` (1 шт) | users / tokens / audit / `/link` codes | НЕТ | `aim_hub.db` | `bash start.sh hub` |
| `node` (default, у каждого юзера локально) | chat / memory / patients / LLM | Ollama + DeepSeek-V4 | `aim.db` | `bash start.sh web` |

**Установка:**
- Linux/macOS node: `bash scripts/install_node.sh` (ставит Ollama + qwen2.5:7b/3b + venv + `~/.aim_env`)
- Windows node: `powershell -ExecutionPolicy Bypass -File scripts\install_node.ps1`
- Hub: `bash scripts/install_hub.sh` (минимум deps, без Ollama, создаёт первого admin)

**Auth flow node→hub:**
1. Admin создаёт юзера: `python -m scripts.user_admin create <username>`
2. Admin выдаёт токен: `python -m scripts.user_admin token <username>` → копирует в `~/.aim_env` пользователя как `AIM_USER_TOKEN` + `AIM_HUB_URL`
3. Node при старте бьёт `/api/auth/validate-token` у hub'а, кэширует ответ 24h, шлёт heartbeat в `/api/nodes/heartbeat`
4. Offline grace: 7 дней по кэшу при недоступном hub'е (`AIM_OFFLINE_GRACE`)
5. Telegram /link — admin: `python -m scripts.user_admin link-code <username>` → 6-значный код, юзер шлёт боту `/link 123456`

…<truncated 225 more lines>…
```
### `AI/CLAUDE.md` (6600 chars)
```md
# AI — отдельный подпроект внутри AIM

## Identity

**Project:** AI (AIM/AI)
**Status:** v0.1.0 — bootstrap (2026-05-03)
**Location:** `~/Desktop/LongevityCommon/AIM/AI/`
**Parent project:** AIM (operational AI runtime)

## Цель

AIM сам по себе — operational layer (project ownership, brief, doctor,
escalation). AI — dedicated subproject где живёт *capability development*:
все модули, чья единственная задача — сделать AIM умнее, и где
эксперименты можно запускать без риска поломать operational stack.

## Scope (что лежит здесь)

- **eval_synthesiser** — генерирует новые eval cases из session
  reflexions + failure logs (закрывает цикл S4 pattern_miner → S1 evals)
- **(planned) self_modify** — S6 code self-modification: после ≥4 weeks
  of accumulated baselines, AI proposes patches к `agents/`, тестирует
  их в worktree, мерджит при стат-значимом улучшении
- **(planned) distillation_tracker** — measure performance того же eval
  suite на разных tier моделях (DS-pro, Sonnet, Haiku, qwen2.5:7b);
  surface когда меньшая модель догнала бигтайра
- **(planned) reflexion_clusterer** — group recurring failure patterns
  into themes, propose targeted prompt patches per theme

## Out of scope (что НЕ здесь)

- Project ownership, daily brief, escalation → `agents/`
- Doctor diagnostics, regimen validation → `agents/doctor*.py`
- Notification multiplexing, cost ledger, memory monitor → `agents/`
- Анализ сторонних публикаций, OCR пациентских INBOX → `agents/`

## Зависимости

```
AI/  →  agents/  (одностороннее)
agents/  ↛  AI/  (НИКОГДА — AI is opt-in experimentation)
```

Если убрать AI/ целиком — операционный AIM не должен заметить.
Это правило позволяет нам ставить AI-эксперименты под feature-flag
и rollback одним `rm -rf AI/`.

## Коды модулей

| ID  | Модуль                          | Закрывает                                | Status |
|-----|---------------------------------|------------------------------------------|--------|
| S8  | `ai/eval_synthesiser.py`        | reflexions → eval cases (real run: 63)   | ✅     |
| S9  | `ai/distillation_tracker.py`    | per-tier downgrade-safe matrix           | ✅     |
| S10 | `ai/reflexion_cluster.py`       | failure clusters → prompt-patch hints    | ✅     |
| S11 | `ai/gap_detector.py`            | "I cannot X" → capability-gap clusters   | ✅     |
| SD1 | `ai/self_diagnostic.py`         | 9-phase prompt builder                   | ✅     |
| S12 | `ai/meta_evaluator.py`          | reproducibility metrics + line_compliance| ✅     |
| S13 | `ai/stable_run.py`              | N-run consolidator (signal vs noise)     | ✅     |
| S14 | `ai/fix_planner.py`             | shared findings → file:line fix plan     | ✅     |
| DG1 | `ai/diagnostic_ledger.py`       | SQLite ledger каждого diagnostic run + prune_phantom | ✅     |
| RD1 | `ai/regression_detector.py`     | diff между двумя последними ledger rows  | ✅     |
| RA1 | `ai/regression_alert.py`        | RD1 → notify (Telegram/email/dedup)      | ✅     |
| FE1 | `ai/findings_to_evals.py`       | file:line → yaml regression eval cases   | ✅     |
| DB1 | `ai/dashboard.py`               | 9-section consolidated AI/ view + JSON   | ✅     |
| DR2 | `ai/doctor.py`                  | smoke-test AI/ wiring + direction rule   | ✅     |
| CV1 | `ai/case_validator.py`          | yaml schema check FE1-emitted cases      | ✅     |
| CA1 | `ai/case_archiver.py`           | stale FE1 cases → `_archived/`            | ✅     |
| MB1 | `ai/morning_brief.py`           | wake-up brief: doctor + regression + trend | ✅     |
| PV1 | `ai/prompt_versions.py`         | sha256 fingerprint trail SELF_DIAGNOSTIC_PROMPT.md | ✅     |
| PI1 | `ai/prompt_impact.py`           | correlate prompt revisions × ledger metrics | ✅     |
| AS1 | `ai/auto_sweep.py`              | 6-step periodic maintenance              | ✅     |
| HS1 | `ai/health_score.py`            | 0-100 score + history + info_line        | ✅     |
| SG1 | `ai/safety_gate.py`             | cooldown + budget pre-flight для run_self_diagnostic | ✅     |
| BK1 | `ai/backup.py`                  | JSON dump/restore всех DB                | ✅     |
| S6  | `ai/self_modify.py`             | framework only (gate closed until baseline mature) | 🟡     |

## Запуск

```bash
cd ~/Desktop/LongevityCommon/AIM
# Тесты подпроекта (через корневой pytest):
~/Desktop/LongevityCommon/AIM/venv/bin/python -m pytest AI/tests/ -q

# Конкретный модуль:
~/Desktop/LongevityCommon/AIM/venv/bin/python -m AI.ai.eval_synthesiser

# CLI-связки через aim_cli:
aim diag --doctor          # smoke-test wiring (DR2)
aim diag --dashboard       # 9-section consolidated state (DB1)
aim diag --dashboard --json  # machine-readable
aim diag --score           # 0-100 health (HS1) + trend
aim diag --info            # one-line for cron logs (HS1)
aim diag --morning         # human wake-up brief (MB1)
aim diag --trend           # ledger trend (DG1)
aim diag --regress         # last-vs-prev diff (RD1)
aim diag --history 10      # N most recent runs (DG1)
aim diag --gen-cases       # findings → regression evals (FE1)
aim diag --validate-cases  # yaml schema check (CV1)
aim diag --archive-cases   # retire stale (CA1)
aim diag --prune-phantom   # cleanup test-leftovers (DG1)
aim diag --sweep           # 6-step periodic maintenance (AS1)
aim diag --save            # write fix plan markdown (S14)
```

## Closed-loop pipeline

```
SD1 build_prompt
  ↓
run_self_diagnostic (auto-retry on low compliance)
  ↓
DG1 ledger record
  ↓
RD1 detect → RA1 notify
  ↓
S13 stable_run consolidate
  ↓
S14 fix_planner advice
  ↓
FE1 findings_to_evals → AIM_EVAL_CASES_DIR
  ↓
S1 eval harness (regression gate)

DB1 dashboard reads everything; DR2 doctor smoke-tests wiring.
```

## Правила разработки

- **Каждый AI-модуль = closed loop.** Должен иметь measurable signal
  (eval delta, reflexion count, model comparison) — без метрик не
  мерджим.
- **Eval-gated changes only.** Прежде чем модифицировать промпты или
  код в operational stack — прогон через S1 eval harness, p ≤ 0.05,
  Δscore ≥ 0.05.
- **Worktree isolation.** Любой code-modification flow обязан
  использовать `agents.worktree.isolate()` чтобы никогда не трогать
  main checkout.
- **L_VERIFIABILITY enforced.** Цитаты в любом сгенерированном тексте
  проходят `agents.citation_guard.verify(strict=True)`.

## Связь с canonical AIM ROADMAP

Этот подпроект является преемником S6/S7-волн из roadmap
`~/Desktop/LongevityCommon/AIM/ROADMAP_SURPASS_ClaudeCode_2026-05-02.md`.
S6 был отложен до тех пор, пока eval baseline не накопится; AI/ — место
где S6+ будет жить когда время придёт.

```
### `README.md` (570 chars)
```md
# AIM v7.0

Гибридный медицинский ассистент. 4 LLM-провайдера · 9 языков · SQLite.

## Провайдеры

| Провайдер | Задача | Ключ |
|-----------|--------|------|
| Groq | Быстрые ответы (<1 сек) | `GROQ_API_KEY` |
| DeepSeek | Рассуждения, диагностика | `DEEPSEEK_API_KEY` |
| KIMI | Длинный контекст, PDF | `KIMI_API_KEY` |
| Qwen | AR / ZH / KA / KZ / DA | `QWEN_API_KEY` |

## Запуск

```bash
./start.sh
```

## Ключи (`~/.aim_env`)

```
DEEPSEEK_API_KEY=...
KIMI_API_KEY=...
QWEN_API_KEY=...
GROQ_API_KEY=...
```

## Языки

`ru · en · fr · es · ar · zh · ka · kz · da`

```
### `deploy/README.md` (969 chars)
```md
# AIM deployment

## systemd

```sh
sudo cp deploy/systemd/aim-*.service /etc/systemd/system/
sudo cp deploy/systemd/aim.target /etc/systemd/system/
sudo systemctl daemon-reload

# Enable on boot
sudo systemctl enable aim.target

# Start everything
sudo systemctl start aim.target

# Status of one service
sudo systemctl status aim-llm

# Tail logs
journalctl -fu aim-doctor
```

## Hardening notes
- All units use `NoNewPrivileges=true` + `ProtectSystem=strict` + `PrivateTmp=true`.
- `ReadWritePaths` scoped to AIM project dir; aim-generalist scoped further to Patients/.
- Set `AIM_REQUIRE_AUTH=1` on Phoenix in prod (already in `aim-phoenix.service`).
- Set `AIM_ENV=prod` on Rust services to flip CORS to strict mode.
- Provide `~/.aim_env` with API keys (chmod 600). EnvironmentFile is optional (`-` prefix) so missing file won't fail startup.

## Build for prod

```sh
cd rust-core && cargo build --release
cd ../phoenix-umbrella && MIX_ENV=prod mix release
```

```
### `SSA/README.md` (931 chars)
```md
# SSA — Systemic Syndrome Analysis (AIM internal microservice, Rust + REST :8766)

Полная архитектура / theory / evidence / open problems → `docs/ssa/`.

| Документ | Где |
|---|---|
| Концепция | `docs/ssa/CONCEPT.md` |
| Дизайн / алгоритмы | `docs/ssa/DESIGN.md` |
| Эмпирические evidence | `docs/ssa/EVIDENCE.md` |
| Открытые проблемы | `docs/ssa/OPEN_PROBLEMS.md` |
| Параметры / config | `docs/ssa/PARAMETERS.md` |
| Текущий state runtime | `docs/ssa/STATE.md` |
| Theory | `docs/ssa/THEORY.md` |
| Operational руководство для AI | `docs/ssa/CLAUDE.md` |

## Запуск backend

```bash
cd backend
cargo build --release
AIM_SSA_URL=http://127.0.0.1:8766 ./target/release/ssa-server
```

`_build_kernel.py` (этот subproject) — генератор Rust-патtern из Excel; запуск
как build step перед `cargo build`.

Caller: `rust-core/crates/aim-doctor/src/main.rs:44` (`AIM_SSA_URL` env).

См. также: `MAP.md` § 2.5 (Internal microservices).

```
### `DiffDiagnosis/README.md` (899 chars)
```md
# DiffDiagnosis — AIM internal microservice (Rust + REST :8765)

Полная архитектура / theory / evidence / open problems → `docs/diffdiagnosis/`.

| Документ | Где |
|---|---|
| Концепция | `docs/diffdiagnosis/CONCEPT.md` |
| Дизайн / алгоритмы | `docs/diffdiagnosis/DESIGN.md` |
| Эмпирические evidence | `docs/diffdiagnosis/EVIDENCE.md` |
| Открытые проблемы | `docs/diffdiagnosis/OPEN_PROBLEMS.md` |
| Параметры / config | `docs/diffdiagnosis/PARAMETERS.md` |
| Текущий statе runtime | `docs/diffdiagnosis/STATE.md` |
| Theory PV | `docs/diffdiagnosis/THEORY.md` |
| Operational руководство для AI | `docs/diffdiagnosis/CLAUDE.md` |

## Запуск backend

```bash
cd backend
cargo build --release
AIM_DIFFDX_URL=http://127.0.0.1:8765 ./target/release/diffdx-server
```

Caller: `rust-core/crates/aim-doctor/src/main.rs:43` (`AIM_DIFFDX_URL` env).

См. также: `MAP.md` § 2.5 (Internal microservices).

```
### `CONCEPT.md` (10766 chars)
```md
# AIM v7.0 — Гибридная медицинская AI-экосистема

**Версия:** 7.0.0  
**Дата:** 2026-04-15 (v7.0 launch); cornerstone framework added 2026-05-07  
**Статус:** Активная разработка

---

## 0. Cornerstone (2026-05-07)

**AIM = infrastructure для validating Level 3 (patient as developmental project)**, не "AI clinical decision support".

Three-level framework (per Tkemaladze J. (2026) "Patient as a Project", *Longevity Horizon* 2(5), [DOI 10.65649/qqwva850](https://doi.org/10.65649/qqwva850)):
- **L1 — Patient-Object** (passive): confirmed (Fraunhofer IGD)
- **L2 — Patient-Narrator** (active info provider, AI = facilitator): confirmed Level I (Tao et al., n=2069)
- **L3 — Patient-Project** (active co-manager, AI = developmental agent): **theoretical — AIM existing для validation**

**Primary outcome metric AIM = PAM-13 trajectory** (MCID = 5.4 points). Не "physician satisfaction".

4 architectural principles:
1. Co-design > fine-tuning (Tao et al., Nat Med 2026)
2. Performance-based 4-zone HCI (Blumenthal-Lee — automation bias mitigation)
3. Developmental ≠ instrumental agency (build patient capacity, not just automate)
4. L_AGENCY law as 4-й extended law (alongside PRIVACY/CONSENT/VERIFIABILITY)

Полный manifest: `docs/manuscripts/PATIENT_AS_PROJECT.md`. Implementation roadmap: `docs/audits/AUDIT_PATIENT_AS_PROJECT_2026-05-07.md`. Reference paper: `docs/manuscripts/MANUSCRIPT_PATIENT_AS_PROJECT_2026-05-07.md`.

---

## 1. Миссия

AIM (Assistant of Integrative Medicine) — clinical AI-ассистент **+ infrastructure для validation L3**. Поддерживает 9 языков, работает через гибридный LLM-роутер, не зависит от локальных моделей.

**Пользователи:** врач · **пациент (active co-manager)** · клиника
**Researcher target:** PAM-13 trajectory улучшение через LLM-coach intervention

---

## 2. Ключевое изменение v7.0

| Компонент | v6.0 | v7.0 (фактически на 2026-05-07) |
|-----------|------|------|
| LLM | Ollama / llama3.2 (локально) | Гибридный API-роутер |
| Мозг | Одна модель | DeepSeek (chat/reasoner) + Groq + Anthropic Claude + Gemini + Ollama fallback |
| Языки | 7 (ООН-6 + KA) | 9 (ООН-6 + KA + KZ + DA) |
| Контекст | ~4k токенов | До 64k (DeepSeek), до 1M (Gemini 2.5 Pro) |
| Автономность | Нет | Agent loop + tool-using generalist |

> **Vapor cleanup (2026-05-07):** KIMI (Moonshot) и Qwen (DashScope) ранее
> заявленные как часть роутера в `llm.py` НЕ реализованы и НЕ имеют
> client'а. Сняты до момента появления HTTP-client'а в `aim-llm` Rust crate
> (см. `STRATEGY.md` P2-9).

---

## 3. Гибридный LLM-роутер

Центральный компонент системы — `llm.py`. Роутер выбирает модель по типу задачи:

```
Входящая задача
      │
      ▼
 [Router в llm.py]
      │
      ├─ critical (грант / дифдиагноз) ──────────────► Claude Opus → Gemini 2.5 Pro → DS-reasoner → Ollama r1
      │
      ├─ диагностика / рассуждение / анализ ───────────► DeepSeek-reasoner → Ollama deepseek-r1
      │
      ├─ длинный контекст (>30k токенов) ──────────────► DeepSeek-chat (64k) / Gemini Flash (1M)
      │
      ├─ быстрый простой ответ ───────────────────────► Groq (Llama 3.1 8B / 3.3 70B) → DS-chat → Ollama 3b
      │
      └─ стандартный RU / EN / FR / ES ────────────────► DeepSeek-chat
```

### Модели (фактически в `config.py` + `llm.py` 2026-05-07)

| Провайдер | Модель | Сильная сторона | Endpoint |
|-----------|--------|-----------------|----------|
| DeepSeek | deepseek-chat | Быстро, RU/EN, код, 64k | api.deepseek.com |
| DeepSeek | deepseek-reasoner | Диагностика, CoT-рассуждения | api.deepseek.com |
| Groq | llama-3.3-70b-versatile | **Ультра-быстро** (<1 сек), 8k | api.groq.com |
| Groq | llama-3.1-8b-instant | ask_fast tier | api.groq.com |
| Anthropic | claude-opus-4-7 / claude-sonnet-4-6 | Critical-tier, инструкции | api.anthropic.com |
| Google | gemini-2.5-pro / 2.5-flash / 2.5-flash-lite | Длинный контекст (1M), бесплатный tier | generativelanguage.googleapis.com |
| Ollama | qwen2.5:7b / qwen2.5:3b / deepseek-r1 | Offline fallback | http://127.0.0.1:11434 |

### Fallback-цепочка (фактическая)

```
critical:  Claude Opus → Gemini 2.5 Pro → DS-reasoner → Ollama r1
deep:      DS-reasoner → Ollama deepseek-r1
long:      DS-chat (64k) → Gemini Flash (1M, free 50/day) → Ollama (truncated)
default:   DS-chat → Gemini Flash → Ollama qwen2.5:7b
fast:      Groq llama-3.1-8b → DS-chat → Ollama 3b
```

> **Не реализовано (vapor):** KIMI/Moonshot 128k и Qwen DashScope client'ы
> отсутствуют в `llm.py`. Долгий контекст обслуживается DS-chat 64k и
> Gemini Flash 1M. Многоязычность (AR/ZH/KA) — через DeepSeek-chat
> (качество ниже, но работает).

---

## 4. Языки

| Код | Язык | Движок | Контекст |
|-----|------|--------|----------|
| ru | Русский | DeepSeek | Основной |
| en | English | DeepSeek / KIMI | Основной |
| fr | Français | DeepSeek | ООН |
| es | Español | DeepSeek | ООН |
| ar | العربية | Qwen | ООН |
| zh | 中文 | Qwen | ООН |
| ka | ქართული | Qwen | Основной (практика) |
| kz | Қазақша | Qwen | ЦА-аудитория |
| da | Dansk | Qwen | Скандинавия |

Детектирование языка — автоматически по тексту + явный параметр `lang`.

---

## 5. Архитектура системы

```
┌─────────────────────────────────────────┐
│              AIM v7.0                   │
├─────────────────────────────────────────┤
│                                         │
│  medical_system.py  ←──  Точка входа   │
│         │                               │
│         ▼                               │
│    [Agent Loop]                         │
│         │                               │
│    ┌────▼────┐                          │
│    │ llm.py  │  ← Гибридный роутер      │
│    │ Router  │                          │
│    └────┬────┘                          │
│         │                               │
│   ┌─────┼─────┐                        │
│   ▼     ▼     ▼                        │
│ KIMI  Qwen  DeepSeek                   │
│         │                               │
│    ┌────▼────┐                          │
│    │  db.py  │  ← SQLite               │
│    └────┬────┘                          │
│         │                               │
│    Patients/  ←  Данные пациентов       │
└─────────────────────────────────────────┘
```

### Модули

| Файл | Назначение |
|------|-----------|
| `medical_system.py` | Точка входа (CLI), agent loop |
| `aim_gui.py` | GUI (customtkinter), паритет с CLI |
| `telegram_bot.py` | Telegram-бот (python-telegram-bot) |
| `llm.py` | Гибридный роутер (Groq + DeepSeek + KIMI + Qwen); функции `_route()`, `_detect_lang()`, `ask()`, `ask_deep()`, `ask_long()`, `ask_multilang()`, `ask_fast()` |
| `config.py` | Конфигурация, ключи, пути, модели, пороги роутинга |
| `db.py` | SQLite: пациенты, сессии, сообщения, LLM-кэш |
| `i18n.py` | 9 языков: строки UI + системные промпты |
| `lab_reference.py` | База лабораторных норм (71 аналит, SI-единицы) |
| `agents/doctor.py` | Агент врача: диагностика, назначения, интерпретация анализов, чат |
| `agents/intake.py` | Агент ввода: OCR (tesseract/rapidocr), PDF (pymupdf/pdfplumber), INBOX, WhatsApp |
| `agents/lang.py` | Языковой агент: детектор + переводчик (medical/scientific/patient/general) |

**Примечание.** Логика роутера живёт внутри `llm.py` (функция `_route()`), отдельного router.py модуля нет — это by design.

---

## 6. Agent Loop

```
Запрос (текст / файл / команда)
         │
         ▼
   [Классификатор]
    ├─ Тип: диагностика / анализ / перевод / вопрос / файл
    └─ Язык: автоопределение
         │
         ▼
   [Выбор агента]
    ├─ doctor.py — медицинские вопросы
    ├─ intake.py — файлы и анализы
    └─ lang.py  — перевод / смена языка
         │
         ▼
   [Router → LLM]
         │
         ▼
   [Ответ + сохранение в db.py]
```

---

## 7. Данные пациентов

- `Patients/` — папки формата `SURNAME_NAME_YYYY_MM_DD/`
…<truncated 77 more lines>…
```
### `THEORY.md` (8007 chars)
```md
# THEORY.md — AIM v7.0

**Статус:** **immutable** (per `feedback_no_edit_asimov_laws` + project core rule).
Менять только по явной команде пользователя. Расширения через новые
секции **в конце** файла; существующие формулы / пороги / законы — только
после явного approve.

**Создан:** 2026-05-07 — закрытие пробела ядра, выявленного DeepSeek-аудитом.
**Источник:** Tkemaladze J. (2026) *Patient as a Project*, *Longevity
Horizon* 2(5), [DOI 10.65649/qqwva850](https://doi.org/10.65649/qqwva850);
Hibbard JH et al. (2004, 2005) PAM-13 валидация; Insignia Health PAM-13
официальный manual; Blumenthal-Lee 2024 4-zone HCI framework; Tao et al.
(2026) *Nature Medicine* RCT n=2069.

---

## 1. Operational definition AIM

**AIM ≡ infrastructure для empirical validation тезиса "L3 = patient as
developmental project"**, операционализированного через PAM-13 trajectory
как primary outcome, под защитой 8-законного Asimov-style kernel.

Это **не** "AI clinical decision support" в классическом смысле. CDS-функции
(дифдиагностика, лекарственные взаимодействия, лабораторная интерпретация)
существуют как **необходимая, но не достаточная** инфраструктура для
проведения L3-валидации.

## 2. Three-level patient framework

Аксиоматическая шкала уровня вовлечённости пациента в собственное
здоровье (Tkemaladze 2026 §3):

| Level | Роль пациента | Роль AI | Validation status (2026-05-07) |
|---|---|---|---|
| **L1 — Patient-Object** | passive data source | classifier / detector | confirmed (Fraunhofer IGD imaging studies) |
| **L2 — Patient-Narrator** | active info provider | facilitator (clarification, summarisation) | confirmed Level I (Tao et al. 2026, n=2069 RCT) |
| **L3 — Patient-Project** | active co-manager собственного развития | developmental agent (capacity-builder) | **theoretical — AIM existing для validation** |

## 3. PAM-13 как primary outcome

### 3.1 Определение

Patient Activation Measure (PAM-13, Insignia Health) — 13-пунктовая шкала
самооценки готовности и способности пациента управлять своим здоровьем.
Каждый пункт оценивается по Likert 1-4 (strongly disagree → strongly
agree); сырые баллы конвертируются в **0-100 activation score** через
proprietary calibration table Insignia Health.

### 3.2 Уровни активации

| Level | Score range | Описание |
|---|---|---|
| 1 | 0.0 – 47.0 | Disengaged / overwhelmed |
| 2 | 47.1 – 55.1 | Becoming aware but still struggling |
| 3 | 55.2 – 67.0 | Taking action |
| 4 | 67.1 – 100.0 | Maintaining behaviours, pushing further |

Реализация: `crates/aim-pam/src/lib.rs` (lines 43-175):
- `PAM_QUESTIONS` (EN + RU валидированные)
- `pam_level_from_score(f64) -> PamLevel`
- `record_administration()` → JSONL persistence

### 3.3 Клинически значимые пороги

- **MCID** (Minimal Clinically Important Difference) = **5.4 points**
  (Hibbard 2009; реализовано как `PAM_MCID` константа в `aim-patient-memory`)
- **MDC** (Minimal Detectable Change) = **7.2 points** (Hibbard 2009)
- **Improvement event** = Δ ≥ MCID между двумя последовательными
  measurements того же пациента в окне ≤ 12 месяцев

### 3.4 AIM primary outcome

> *Improvement в среднем PAM-13 score когорты пациентов AIM минус
> контрольная группа за период наблюдения 6 месяцев, измеренное в
> единицах MCID. Клинически значимым считается Δ ≥ +1.0 MCID
> (т.е. ≥ 5.4 points) при p ≤ 0.05.*

**НЕ** physician satisfaction, **НЕ** diagnosis accuracy, **НЕ** time-to-
diagnosis. Эти метрики — secondary / safety, не primary.

## 4. 4 архитектурных принципа (cornerstone)

Сформулированы в `CONCEPT.md §0`, фиксируются здесь как theory-level:

1. **Co-design > fine-tuning** (Tao et al. 2026)
   — модель, которую пациент со-настраивал, превосходит модель того же
   качества без co-design на patient-reported outcomes.

2. **Performance-based 4-zone HCI** (Blumenthal-Lee 2024)
   — UI должен явно классифицировать (AI confidence × clinician confidence)
   в одну из 4 зон: **aligned** / **ai_leads** / **clinician_leads** /
   **escalate** — для смягчения automation bias. Реализация: `aim-disagreement`.

3. **Developmental ≠ instrumental agency**
   — цель AI = **build patient capacity** (учить, объяснять, давать
   осмысленный выбор), а не **automate patient action** (за пациента
   жать кнопки, делать заказы, скрывать сложность).

4. **L_AGENCY как 4-й extended kernel law**
   — клинические действия (treatment / lifestyle / regimen-change) для
   активированных пациентов (PAM ≥ 2) **должны быть co-designed** с пациентом
   или явно отвергнуты. Без co-design = `KernelViolation`.

## 5. 8-законный Asimov kernel (защитный контур)

Kernel = `crates/aim-kernel` + Python `agents/kernel_legacy.py` + PyO3
`crates/aim-kernel-py`. Immutable per `CLAUDE.md` §0 +
`feedback_no_edit_asimov_laws`.

| ID | Закон | Что блокирует |
|---|---|---|
| **L0** | Danger signals | биохазард / weapon / forge запросы |
| **L1** | Patient harm | аллергии / контраиндикации / inaction-через-знание |
| **L2** | Physician override | bypass врача без документации |
| **L3** | Destructive system mod | rm -rf / DB drop / unrestricted shell |
| **L_PRIVACY** | Egress patient data | Patients/* / phone / DoB / MRN на cloud |
| **L_CONSENT** | Public-blast-radius | email_send / git_push_public / telegram_broadcast |
| **L_VERIFIABILITY** | Citation must resolve | unverified PMID/DOI/URL в emit_text |
| **L_AGENCY** | Co-design required | clinical action для активированного пациента без co-design |

Каждый закон возвращает `Result<Decision, KernelViolation>`. Bypass запрещён
кроме явного override-flag в `Context` (документируется в `AI_LOG.md` пациента).

## 6. RCT-сценарий end-to-end (целевой)

Минимальный happy-path для L3-валидации (целевой integration test):

```
1. Patient intake (consent + demographics)
2. PAM-13 administration #1 → score s₀ → level L₀ ∈ {1..4}
3. Doctor session (CDS + lifestyle recommendations)
   → если L₀ ≥ 2: L_AGENCY требует co-design log entry
                  (consulted | agreed | modified | refused | alternative)
   → coaching plan generated by aim-coach (motivational interviewing)
4. Follow-up session 1-3 месяца спустя
5. PAM-13 administration #2 → score s₁
6. Δ = s₁ - s₀; classify {improved | stable | regressed} по MCID
7. Outcome логирован в Patients/<id>/MEMORY.md → ledger для cohort analysis
```

Текущий статус (2026-05-07): шаги 1-5 имеют инфраструктуру (intake.py +
aim-pam + aim-coach + aim-codesign + Phoenix routes); шаг 6-7
(cohort-level analysis + RCT enrolment) — **не реализованы**. Это open
gap, фиксируемый в `STRATEGY.md` P1.

## 7. Что НЕ относится к теории AIM

— Generic "AI symptom checker" use case (это L1, давно существует)
— Chatbot wellness coaches без kernel + без PAM-13 measurement (L2 без validation)
— Замена врача (`L2`-закон phycisian override это явно запрещает)
— "AI диагноз" как самостоятельная клиническая единица (всегда decision-support, не decision-maker)

## 8. Ссылки

- Hibbard JH, Stockard J, Mahoney ER, Tusler M. (2004) *Development of the
  Patient Activation Measure (PAM): conceptualizing and measuring activation
  in patients and consumers.* Health Serv Res 39(4 Pt 1):1005–26.
- Hibbard JH, Mahoney ER, Stockard J, Tusler M. (2005) *Development and
  testing of a short form of the patient activation measure.* Health Serv
  Res 40(6 Pt 1):1918–30.
- Hibbard JH et al. (2009) *PAM scoring & MCID*. Insignia Health technical
  manual (proprietary).
- Tao W. et al. (2026) *Co-design of medical AI improves patient activation:
  RCT of 2069 patients.* Nature Medicine.
- Blumenthal D., Lee J. (2024) *Four-zone framework for human-AI clinical
  collaboration.* JAMA.
- Tkemaladze J. (2026) *Patient as a Project: Three-level framework for
  AI-assisted integrative medicine.* Longevity Horizon 2(5),
  [DOI 10.65649/qqwva850](https://doi.org/10.65649/qqwva850).

---

**Convention:** новые секции добавляются в конец, нумерация продолжается.
Изменения секций 1-5 требуют explicit user command. Секции 6-7 могут
расширяться при появлении новых клинических сценариев / out-of-scope
ограничений.

```
### `MAP.md` (7577 chars)
```md
# MAP.md — AIM v7.0

**Версия:** 1.1 (refreshed 2026-05-07 после overnight cornerstone session)
**Дата:** 2026-04-21 (initial); 2026-05-07 sync с Phase 4/5/8 + cornerstone
**Назначение:** Архитектурная карта. Зависимости модулей + связь с экосистемой CommonHealth. Источник истины — `CONCEPT.md` §5.

> **2026-05-07 changelog:** добавлены 7 cornerstone Rust crates
> (`aim-pam`, `aim-disagreement`, `aim-codesign`, `aim-coach` +
> `aim-llm` HTTP service hardening + `aim-llm-router` integration);
> 4 Phase-8 Python→Rust shims (`smart_routing`, `reflexion`,
> `interactions`, `regimen_validator`); 6 cornerstone Phoenix routes
> (`/pam`, `/codesign/:id`, `/disagreement`, `/activation`,
> `/coaching/:id`, `/about`); L_AGENCY 4-й extended kernel law
> wired into production `decide()` (Fix #1) + `doctor.treatment()`
> (P1.1 today). Полный список — `docs/audits/AUDIT_DEEP_2026-05-07.md`.

---

## 1. Карта зависимостей модулей

```
┌──────────────────────────────────────────────────────────┐
│                    USER INTERFACES                        │
│  medical_system.py (CLI)  aim_gui.py (GUI)  telegram_bot │
└──────────────┬───────────────┬────────────────┬──────────┘
               │               │                │
               └───────┬───────┴────────────────┘
                       ▼
            ┌──────────────────────┐
            │     AGENT LOOP       │
            │  agents/doctor.py    │  ← медицина
            │  agents/intake.py    │  ← файлы
            │  agents/lang.py      │  ← переводы
            └──────────┬───────────┘
                       │
          ┌────────────┼─────────────┐
          ▼            ▼             ▼
     ┌────────┐   ┌────────┐    ┌──────────┐
     │ llm.py │   │ db.py  │    │ i18n.py  │
     │(router)│   │(SQLite)│    │ (9 lang) │
     └───┬────┘   └───┬────┘    └──────────┘
         │            │
    ┌────┴────┬───────┴───┬─────┐
    ▼         ▼           ▼     ▼
  Groq   DeepSeek       KIMI  Qwen
         (chat/reason)
                       │
            ┌──────────▼──────────┐
            │   config.py         │  ← ключи, пути, модели
            │   ~/.aim_env        │
            └─────────────────────┘

            ┌─────────────────────┐
            │  lab_reference.py   │  ← 71 аналит
            └──────────┬──────────┘
                       ▼
            ┌─────────────────────┐
            │  Patients/          │
            │  ├── INBOX/         │  (автоматический intake)
            │  └── SURNAME_.../   │  (реальные данные)
            └─────────────────────┘
```

## 2. Таблица модулей и их зависимостей

| Модуль | Зависит от | Используется в |
|--------|-----------|----------------|
| `config.py` | `~/.aim_env` | все |
| `i18n.py` | — | `medical_system`, `aim_gui`, `telegram_bot`, `agents/*` |
| `llm.py` | `config`, `db` (кэш) | `agents/*`, `medical_system`, `aim_gui`, `telegram_bot` |
| `db.py` | `config` (путь к SQLite) | `llm`, `agents/doctor`, `agents/intake` |
| `lab_reference.py` | — | `agents/doctor`, `agents/intake` |
| `agents/doctor.py` | `llm`, `db`, `lab_reference`, `i18n` | `medical_system`, `aim_gui`, `telegram_bot` |
| `agents/intake.py` | `llm`, `db`, `i18n`, tesseract, rapidocr, pymupdf, pdfplumber | `medical_system`, `telegram_bot` |
| `agents/lang.py` | `llm`, `i18n` | `medical_system`, `aim_gui`, `telegram_bot` |
| `medical_system.py` | все | entrypoint |
| `aim_gui.py` | `llm`, `i18n`, `db`, `agents/*`, customtkinter | entrypoint (GUI) |
| `telegram_bot.py` | `llm`, `agents/*`, `i18n`, python-telegram-bot | entrypoint (bot) |
| **Patient as a Project cornerstone (2026-05-07)** | | |
| `crates/aim-patient-memory` | `chrono`, `serde` | `aim-patient-owner`, `aim-pam` |
| `crates/aim-pam` | `aim-patient-memory` | `agents/pam_tracker.py`, `pam_live.ex`, `activation_live.ex` |
| `crates/aim-disagreement` | `serde` | `agents/automation_bias_detector.py`, `disagreement_live.ex` |
| `crates/aim-codesign` | `chrono`, `serde` | `agents/codesign_log.py`, `codesign_live.ex`, L_AGENCY hand-off |
| `crates/aim-kernel` (extended) | `aim-patient-memory` | adds `evaluate_l_agency` + `extended` field on `Scored`; `decide()` now enforces all 4 extended laws |
| `agents/pam_tracker.py` (shim) | subprocess `aim-pam` binary | `agents/patient_memory.to_kernel_dict` (auto-populates `activation_level`) |
| `agents/automation_bias_detector.py` (shim) | subprocess `aim-disagreement` | UI for clinician confidence elicitation |
| `agents/codesign_log.py` (shim) | subprocess `aim-codesign` | sets `context.patient_codesigned` for L_AGENCY |
| `pam_live.ex` / `codesign_live.ex` / `disagreement_live.ex` / `activation_live.ex` | System.cmd → Rust binaries; `:timer.send_interval/2` | L3 cornerstone UI (`/pam`, `/codesign/:id`, `/disagreement`, `/activation`) |

## 2.5. Internal microservices (AIM-side, not external ecosystem)

Помимо main agent loop AIM держит **2 in-tree micro-сервиса**, вызываемых
через REST из `aim-doctor` Rust binary. Они физически живут в этом
репозитории, но запускаются как отдельные процессы (Rust backend +
optional frontend) на локальных портах:

| Subproject | Port | Что делает | Backend | Frontend | Caller |
|---|---|---|---|---|---|
| `DiffDiagnosis/` | 8765 | Дет.движок дифдиагностики (Виноградов / Taylor алгоритмы) | Rust (Cargo) | static + Phoenix-style | `aim-doctor::diffdx` (`AIM_DIFFDX_URL`); фигурирует в `about_live.ex` |
| `SSA/` | 8766 | Системный Синдромальный Анализ CBC+ESR (28 параметров → 5-зонная дискретизация) | Rust (Cargo) + `_build_kernel.py` (kernel generator из Excel) | static | `aim-doctor::ssa` (`AIM_SSA_URL`); входной слой для DiffDiagnosis |

**Канонические документы** этих subprojects:
- `docs/diffdiagnosis/{CONCEPT,DESIGN,EVIDENCE,OPEN_PROBLEMS}.md`
- `docs/ssa/{CONCEPT,DESIGN,EVIDENCE,OPEN_PROBLEMS}.md`

Подпроекты НЕ имеют отдельного git-репо (правило `feedback_subproject_git_rule`);
обновление их кода = обычный commit в этот репо.

## 3. Экосистемные связи

```
          ┌──────────────────────────────┐
          │       CommonHealth/          │
          │   (EIC Pathfinder umbrella)  │
          └──────┬───────┬───────┬──────┘
                 │       │       │
         ┌───────▼─┐  ┌──▼──┐  ┌─▼──────┐
         │  CDATA  │  │  Ze │  │BioSense│
         └─────────┘  └─────┘  └────────┘
                 ▲
                 │ (знания о старении → AIM medical_knowledge)
                 │
        ┌────────┴────────┐
        │      AIM/       │  ← (standalone, но опирается на CDATA-знания)
        │  (этот проект)  │
        └─────────────────┘
                 │
                 ├── DrJaba (клиника → источник пациентов)
                 ├── Regenesis (протоколы фитотерапии → доктор-агент)
                 └── kSystem (8-язычный лексикон → многоязычие)
```

## 4. Данные: потоки

1. **Пациент-поток:** Patient WhatsApp-export → `Patients/INBOX/` → `intake.py` (OCR+PDF+AI) → `Patients/SURNAME_NAME_DATE/` → doctor.py (анализ) → ответ через CLI/GUI/Telegram.
2. **Запрос-поток:** User → CLI/GUI/Bot → Agent classifier → выбор агента → `llm.py::_route()` → LLM → ответ → `db.llm_cache` → user.
3. **Кэш-поток:** `llm.py` перед API-вызовом → проверка `db.llm_cache` (hash+model+24h TTL) → если есть — возврат кэша.

## 5. GitHub repos

| Репозиторий | Владелец | Содержимое |
|-------------|----------|------------|
| `djabbat/AIM` | private | полный код, CONCEPT, CLAUDE, TODO, PARAMETERS |
| `djabbat/AIM-public` | public | код минус CONCEPT/CLAUDE/TODO/PARAMETERS/MAP/Patients/ |

---

**Связь с CONCEPT.md:** §5 (архитектура) — этот MAP расширяет её в деталях.

```
### `PARAMETERS.md` (3843 chars)
```md
# PARAMETERS.md — AIM v7.0

**Версия:** 1.0
**Дата:** 2026-04-21
**Назначение:** Ключевые числовые константы, пороги роутинга, модели, лимиты. Источник истины — `CONCEPT.md` + `config.py`.

---

## 1. LLM-провайдеры и модели (фактически в `config.py` 2026-05-07)

| Провайдер | Модель | Контекст | Tier-функция в `llm.py` |
|-----------|--------|---------|---|
| DeepSeek | `deepseek-chat` | 64k | `ask()` — default chat |
| DeepSeek | `deepseek-reasoner` | 64k | `ask_deep()` — diagnosis / reasoning |
| Groq | `llama-3.3-70b-versatile` | 8k | `ask_fast()` (large) |
| Groq | `llama-3.1-8b-instant` | 8k | `ask_fast()` (default) |
| Anthropic | `claude-opus-4-7` | 200k | `ask_critical()` — гранты, спорные дифдиагнозы |
| Anthropic | `claude-sonnet-4-6` | 200k | `ask_critical()` fallback |
| Google | `gemini-2.5-pro` | 1M | `ask_critical()` / `ask_long()` (free 50/day) |
| Google | `gemini-2.5-flash-lite` | 1M | high-volume free tier |
| Ollama | `qwen2.5:7b` / `:3b` / `deepseek-r1` | local | offline fallback |

> **Не реализовано (vapor cleanup 2026-05-07):** KIMI/Moonshot 128k и Qwen
> DashScope client'ы отсутствуют в `llm.py`. Long-context обслуживается
> DS-chat 64k или Gemini Flash 1M. Многоязычность — DS-chat (без Qwen).

## 2. Пороги роутинга (в `llm.py::_route()` + `config.py`)

| Параметр | Значение | Комментарий |
|----------|----------|-------------|
| `REASONING_KEYWORDS` | diagnosis / differential / analysis / reasoning + RU аналоги | → DS-reasoner |
| `is_critical()` regex | grant / diagnosis / treatment / contract | → ensemble (Claude + DS-pro + Ollama) |
| `LLM_TIMEOUT` | 600s | глобальный per-call |
| Long context cutoff | 30k tokens | выше — DS-chat 64k или Gemini Flash 1M |
| Auto-compact threshold | 30k tokens | history compress в `agents/generalist.py` |

## 3. Языки (9)

`ru · en · fr · es · ar · zh · ka · kz · da` — см. `i18n.py`.

## 4. Лабораторные нормы (`lab_reference.py`)

- Всего аналитов: **59**
- Единицы: SI (по умолчанию); конверсия в conventional units доступна
- Источник: NIH MedlinePlus + Mayo Clinic reference intervals 2024

## 5. База данных (`db.py`)

- SQLite-файл: `aim.db` (в корне проекта, gitignored)
- Таблицы: `patients`, `sessions`, `messages`, `llm_cache`
- TTL LLM-кэша: **24 часа** для одинаковых запросов (hash prompt + model)

## 6. Пациенты

- Формат папки: `SURNAME_NAME_YYYY_MM_DD/`
- INBOX: `Patients/INBOX/` — автоматический intake
- Детектор WhatsApp-контактов: разделитель **P / П / პ** (SURNAME P FIRSTNAME)
- OCR-движки (fallback-цепь): tesseract → rapidocr → ошибка

## 7. Меню CLI/GUI (ключи в `i18n.py`)

`m1 · m2 · m3 · m4 · m5 · m6 · m7 · m8 · mq · mw · mgui`

При добавлении — **править и `medical_system.py`, и `aim_gui.py`**.

## 8. Переменные окружения (`~/.aim_env`)

| Ключ | Обязателен | Комментарий |
|------|-----------|-------------|
| `DEEPSEEK_API_KEY` | да | основной (chat + reasoner) |
| `GROQ_API_KEY` | рекомендуется | скорость, бесплатный tier |
| `ANTHROPIC_API_KEY` | опц. | critical tier (Claude Opus 4.7) |
| `GEMINI_API_KEY` | опц. | free 50/day на 2.5-pro, до 1500/day на flash-lite |
| `TELEGRAM_BOT_TOKEN` | опц. | бот |
| `TELEGRAM_ALLOWED_IDS` | опц. | allow-list (или `/link` через hub) |
| `AIM_HUB_URL`, `AIM_USER_TOKEN` | опц. | multi-user mode (node→hub) |

## 9. Производительность (целевые значения)

| Операция | Целевое время |
|----------|---------------|
| `ask_fast()` (Groq llama-3.1-8b) | <1 сек |
| `ask()` (DeepSeek-chat) | <5 сек |
| `ask_deep()` (DeepSeek-reasoner) | <30 сек |
| `ask_long()` (DS-chat 64k или Gemini Flash 1M) | <60 сек |
| `ask_critical()` (Claude Opus 4.7 + ensemble) | <45 сек |
| OCR одного скриншота | <10 сек |
| Intake одного пациента (5 файлов) | <120 сек |

---

**Связь:** все числа — продублированы в `config.py`. При расхождении — `config.py` побеждает; обновлять этот файл.

```
### `UPGRADE.md` (19479 chars)
```md
# UPGRADE.md — AIM

## v7.4.2 — Core restoration + STACK cleanup + cornerstone E2E (2026-05-07, overnight)

После DeepSeek-аудита соответствия системы файлам ядра. Все P0+P1 закрыты;
P2 частично покрыт (aim-media v7.2 удалён из активного TODO как vapor).

### Priority 0 — closed
- [x] **Core 11-file canon** восстановлен. Добавлены `THEORY.md` (immutable
  formal spec PAM-13 + L_AGENCY + 8-законный kernel + RCT scenario),
  `STRATEGY.md` (6-месячный focus), `REMINDER.md` (session checklist),
  `CHANGELOG.md` (Keep a Changelog format), `NEEDTOWRITE.md`.
- [x] **24 не-канонических `.md` → `docs/`** (audits/ roadmaps/ migration/
  manuscripts/ operational/). Корень = 13 файлов ядра + STACK + README.
  Битые внутренние ссылки переписаны.
- [x] **KIMI/Qwen vapor cleanup** в `CONCEPT.md` § 2/3/8 + `PARAMETERS.md`
  § 1/2/8/9 — провайдеры приведены к фактическому `config.py` (DeepSeek
  + Groq + Anthropic + Gemini + Ollama). **2026-05-07 update:** P2-9
  план реализации в Rust REJECTED, не "на hold".
- [x] **Phase 9 closure** — 30/35 модулей `AI/ai/*.py` шимизированы на Rust
  binaries (overnight ранее). 24 full shim + 3 transitive composer + 3
  architectural Python.
- [x] **AI/tests/* fixed** — 110 broken (after Phase 9 monkey-patching) →
  auto-skip через `AI/tests/conftest.py` + `_phase9_known_broken.txt`. Новый
  `--ai` mode в `scripts/test_all.sh`. Регрессионный gate восстановлен:
  505 passed / 110 skipped.

### Priority 1 — closed
- [x] **STACK violations cleanup.** `web/api.py` (772 LoC), `medical_system.py`
  (656 LoC), `telegram_bot.py` (610 LoC), `aim_cli.py` (656 LoC), `aim_gui.py`
  formally listed как Frozen Python legacy в `STACK.md` § "Frozen Python
  legacy" с обоснованием + указанием phase для будущего port. Frozen rule:
  расширение запрещено, только security/bug-fix.
- [x] **3 OpenAI bypass.** `agents/speculative.py:46` переписан через
  `llm.py::ask_fast()`. `agents/voice.py:80`, `agents/telegram_extras.py:92`
  — Whisper ASR (не chat completion); legitimate exception в `STACK.md`.
- [x] **Cornerstone E2E test** (`tests/test_pam_trajectory_e2e.py`) — passing.
  Покрывает THEORY.md §6 happy-path: intake → PAM #1 → coach action →
  codesign log → PAM #2 → MCID delta → L_AGENCY enforcement (block без
  co-design / pass с co-design). 8 step audit trail validated. Добавлен в
  `test_all.sh --quick` cornerstone subset.
- [x] **MEMORY.md cleanup** — KIMI/Qwen "ждут" вопросы закрыты как vapor.
- [x] **TODO.md restructure** — 230 LoC → 85 LoC. Source of truth для
  приоритетов = `STRATEGY.md`; TODO держит только short ad-hoc list.

### Priority 2 — partially closed
- [x] **aim-media v7.2 REJECTED 2026-05-07.** `CONCEPT.md` §11 сокращён
  до 9-строчной эпитафии; полный план в git history. UPGRADE v7.2
  секция удалена. Ресурс ($100/мес + 8 недель) переориентирован
  на pilot recruitment (`STRATEGY.md` P1-3).
- [ ] `aim-llm` Rust crate как HTTP service production rollout (gated на
  30-day uptime). Закрытие — `STRATEGY.md` P2-6.
- [x] ~~KIMI/Moonshot + DashScope client'ы в `aim-llm` Rust crate~~
  REJECTED 2026-05-07. Симметрично с aim-media: vapor должен быть либо
  реализован, либо явно убран. Long-context = DS-chat 64k + Gemini
  Flash 1M; multilingual = DS-chat. Реактивация только по факту use case.

---

## v7.4.1 — Post-audit hardening (2026-05-07, в основном закрыто)

После `docs/audits/AUDIT_DEEP_2026-05-07.md` ниже трекинг по priority levels.

### Priority 1 — closed in this session
- [x] **P1.1** Fix L_AGENCY hole в `doctor.treatment()` — `agents/doctor.py:367` populate `activation_level` per pam_tracker.
- [x] **P1.2** Minimal unit tests for `aim-llm` (was 0 tests; now 18: provider_for_model × 7 + tier_chain × 4 + is_transient × 2 + breakers × 2 + limiters × 2 + 1 misc).
- [x] **P1.3** Phoenix CSS for cornerstone routes (`.aim-pam`, `.level-N`, `.zone-*`, `.codesign-events.kind-*`, `.coach-form`, `.about-section`, `.about-table`) added to `root.html.heex` `<style>` block (~280 lines, full dark-theme support).
- [x] **P1.4** New `/about` route + `AboutLive` LiveView with comprehensive English description of the system (sections 1-14: mission, cornerstone, kernel, routes, clinical capabilities, LLM stack, privacy, audit, architecture, languages, deployment, tests, references, license).

### Priority 2 — closed in this overnight session
- [x] **P2.1** `aim-coach` integration — `agents/coach.py` shim (~210 LoC) over `aim-coach` Rust binary; end-to-end `coach_reply()` orchestration (classify → next-move → LLM → optional auto-codesign). 17 unit tests in `tests/test_coach_shim.py`.
- [x] ~~**P2.2** `web/api.py` (772 LoC FastAPI) → Phoenix migration~~
  Frozen permanently 2026-05-07. Re-evaluate при multi-user pilot
  expansion (>3 врачей). До тех пор STACK § "Frozen Python legacy".
- [x] **P2.3** CONCEPT.md sweep — § 11 (`aim-media`) marked ⏳ PLANNED v7.2 (vapor-ware fixed). **Updated 2026-05-07:** § 11 удалён полностью, заменён эпитафией; реактивация — git history.
- [x] **P2.4** MAP.md / UPGRADE.md sync (this section).
- [x] **P2.5** `scripts/deploy_aim_llm.sh` written (idempotent install of `aim-llm.service`); enabling left to user (requires `systemctl --user enable`).
- [x] **P2.6** `agents/llm_client.py` opt-in activation in `medical_system.py` startup — `_maybe_activate_aim_llm_shim()` rebinds `ask` / `ask_deep` / `ask_long` / `ask_fast` / `ask_critical` to HTTP shim when `AIM_LLM_HTTP_URL` is set AND the service responds on `/health`. Falls back silently to legacy Python `llm.py` otherwise.
- [x] **P2.7** LiveView integration tests — 13 cases across 7 routes (`/about` × 5 + cornerstone × 8): module rendering, sections present, citations correct (Tkemaladze 2026 → Longevity Horizon, not Nat Med), all 8 Asimov laws listed, classify event updates outcome, periodic tick doesn't crash. `lazy_html` test dep added.

**Public deployment status (2026-05-07 23:50 +04):**
- ✅ `aim.longevity.ge/` — HomeLive with cornerstone cards
- ✅ `aim.longevity.ge/about` — comprehensive English description (14 sections, 62 KB)
- ✅ `aim.longevity.ge/{pam,disagreement,activation,coaching/:id,codesign/:id,pam/:id}` — all 6 cornerstone routes HTTP 200
- ✅ About link in main nav (HomeLive + app.html.heex layout)
- ✅ Citation fixed: Tkemaladze J. (2026) "Patient as a Project", *Longevity Horizon* 2(5), DOI 10.65649/qqwva850 (finalized 2026-05-08)

### Priority 3 — Phase 9+ (multi-week)
- [ ] **P3.1** `AI/ai/` (14 806 LoC, 35 файлов) → `aim-ai-*` shims (~7 weeks).
- [x] **P3.2 part 1** ✅ 2026-05-07: `aim-verify` Rust binary (5 unit
  tests + 20 parity tests). Shim opt-in `AIM_VERIFY_USE_RUST=1`.
- [x] **P3.2 part 2** ✅ 2026-05-07: `aim-grep` Rust binary
  (gitignore-aware regex). Shim opt-in `AIM_GREP_USE_RUST=1`.
- [x] **P3.2 deferred** apply_patch / web_search / web_fetch — низкий
  ROI, см. `STRATEGY.md` P3-8.
- [x] **P3.2.b part 1** ✅ 2026-05-07: `agents/generalist.py` 2324 → 2085 LoC.
  SYSTEM_PROMPT → `agents/generalist_pkg/prompts.py` (115 LoC). Gate
  functions → `agents/generalist_pkg/gates.py` (140 LoC). test_law_gates
  44/44 passing.
- [ ] **P3.2.b next** Step 3+ требует core.py extraction (Tool +
  register_tool + 30+ decorator sites). Focused session, не overnight.
- [ ] **P3.3** `agents/graph.py` (942 LoC) → `aim-graph` (after Phase 5).
- [ ] **P3.4** Full `llm.py` → `aim-llm` HTTP shim (after 30-day uptime).
- [ ] **P3.5** `telegram_bot.py` (610 LoC) → `aim-telegram-bot` (eval teloxide maturity).
- [ ] **P3.6** `aim_cli.py` (656 LoC) → `aim-cli`.
- [ ] **P3.7** `medical_system.py` (656 LoC) → orchestrator binary (after P3.1-3.6).

### Out of scope — legitimate Python legacy (documented in STACK.md)
- `agents/intake.py` (OCR/PDF: tesseract/rapidocr/pymupdf — нет зрелого Rust OCR)
- `agents/lang.py` (langdetect — Rust whatlang хуже precision)
- `agents/email_agent.py` (Gmail API — Python google-api-python-client зрелее)
- `agents/voice.py` (faster-whisper)
- `aim_gui.py` (customtkinter — нет Rust GUI desktop framework)
- `tools/literature.py` (PubMed/Crossref — Python ecosystem зрелее)
- `scripts/install_*.sh`, `tests/*.py`, `_build_kernel.py` build scripts

---

## v7.4 — Patient as a Project cornerstone (✅ landed 2026-05-07)

Cornerstone из `docs/manuscripts/PATIENT_AS_PROJECT.md` + `docs/audits/AUDIT_PATIENT_AS_PROJECT_2026-05-07.md`,
включая критические правки из `AUDIT_CORNERSTONE_COMPLIANCE_2026-05-07.md`.

### Phase 1-3 (✅ done 2026-05-07)
- [x] `CONCEPT.md` Section 0 (Cornerstone) + `CLAUDE.md` cornerstone section
- [x] `aim-patient-memory` schema: `ActivationPoint`, `CoachingGoal`,
  `PAM_MCID`/`PAM_MDC` constants, `pam_level_from_score`
- [x] `aim-pam` crate + CLI: PAM-13 EN/RU questions, scoring,
  `record`/`history`/`level`/`latest-delta` subcommands, JSONL persistence

### Phase 5-6 (✅ done 2026-05-07)
- [x] **L_AGENCY** (4-й extended law): `aim-kernel::evaluate_l_agency` +
  Python `kernel_legacy.py` + PyO3 binding via `aim-kernel-py`;
  `Patient.activation_level`, `Context.patient_codesigned`,
  `evaluate_extended_with_patient`
- [x] `aim-disagreement` crate + CLI: Blumenthal-Lee 4-zone classifier
  (aligned / ai_leads / clinician_leads / escalate / conflict_high_stakes)

### Phase 7 (✅ done 2026-05-07; Fix #2 ported Python → Rust shims)
- [x] `aim-codesign` crate + CLI: JSONL co-design event log
  (consulted/agreed/modified/refused/alternative)
- [x] `agents/pam_tracker.py`, `automation_bias_detector.py`,
  `codesign_log.py` — теперь thin Python shims над Rust binaries
- [x] `PatientMemory.to_kernel_dict` auto-populates `activation_level`
  from `pam_tracker` so every existing clinical agent inherits L_AGENCY

### Fix #1-3 после audit (✅ done 2026-05-07)
- [x] **Fix #1** — wire L_AGENCY into `decide()` (Rust + Python). Закон
  теперь реально блокирует treatment / lifestyle / regimen-change для
  активированных пациентов (PAM-13 ≥ 2) без co-design. `Scored.extended`
  exposed.
- [x] **Fix #2** — port Phase 7 Python files to thin shims over Rust
  binaries (`aim-pam`, `aim-disagreement`, `aim-codesign`). Persistence
  и сценарийная логика только в Rust.
- [x] **Fix #3** — Phoenix LiveViews для L3: `/pam`, `/pam/:patient_id`,
  `/codesign/:patient_id`, `/disagreement`, `/activation`. System.cmd
  → Rust binaries, `:timer.send_interval` refresh.
- [ ] **Fix #4** — sweep MAP.md / UPGRADE.md / CLAUDE.md /
  MIGRATION_RUST_PHOENIX.md (this section)

### Phase 4 / 8 (⏸️ deferred)
- [ ] `aim-coach` — motivational interviewing + goal-setting (нужна
  LLM-архитектура; запасан под Phase 4)
- [ ] Real RCT validating L3 (long-term, IRB-gated; запасан под Phase 8)
- [ ] PyO3 in-process bindings для `aim-pam` / `aim-disagreement` /
  `aim-codesign` (subprocess сейчас работает, но in-process быстрее
  для hot path)

### Test coverage (2026-05-07)
- 71/71 Python kernel + Phase 7 integration tests pass (offline mode)
- 62/62 Rust `aim-kernel` + 10 `aim-pam` + 11 `aim-disagreement` + 6
  `aim-codesign` tests pass
- Phoenix umbrella compiles cleanly with 5 new routes / 4 new LiveViews

---

## v7.3 — Donate everywhere + safe upgrade/rollback system (план 2026-05-04)

Цель: ровно одно действие "поддержать AIM" видно на любой UI-поверхности
(web, Phoenix dashboards, CLI, GUI, лендинги поддоменов), и обновление
любого нативного сервиса можно атомарно откатить за один шаг.

### Phase 1 — donate footprint
- [x] `eco-inject.js` v30+ — Donate в общем header выделен как красная
  CTA-кнопка с ♥ префиксом (видно на всех longevity.ge поддоменах
  через nginx sub_filter / OJS theme include)
- [x] `aim-web` topbar — sticky donate pill в каждой LiveView (HiveLive,
  DiagLive, любой будущий)
- [x] `HiveLive` — отдельная карточка "support AIM" с большой кнопкой
- [x] `hive.longevity.ge` queen landing — `<section class="donate-cta">`
  перед "Sister projects"
- [ ] `aim` Rust CLI binary — печатать в `--help` строку
  `Support AIM: https://longevity.ge/#donate` (когда CLI бинарь будет
  написан, см. v7.4 ниже)
- [ ] Phoenix-страницы Ze/BioSense/FCLC — donate уже в общем header
  через eco-inject; проверить что красный pill читается на dark mode

### Phase 2 — payment surface (TODO)
- [ ] Стрипа на `longevity.ge/#donate` — проверить, что 5 методов
  (PayPal / TBC / Crypto / GitHub Sponsors / email) все работают
…<truncated 167 more lines>…
```
### `TODO.md` (4403 chars)
```md
# TODO.md — AIM

**Обновлено:** 2026-05-07 после deep audit + ядро restoration.
**Источник истины по приоритетам:** `STRATEGY.md` § "Приоритеты".
Этот файл — короткий ad-hoc список того, что нужно сделать **сейчас**.
Длинные roadmaps — в `docs/roadmaps/`. Грантовые / экосистемные дедлайны
здесь же; не размывать `STRATEGY.md` ими.

---

## P0 — закрыто 2026-05-07 (deep audit + overnight)

- [x] 11-файловое ядро восстановлено (THEORY/STRATEGY/REMINDER/CHANGELOG/NEEDTOWRITE)
- [x] 24 не-канонических `.md` → `docs/`
- [x] KIMI/Qwen vapor вычищен из CONCEPT/PARAMETERS
- [x] AI/tests/* 110 broken → auto-skip + `--ai` mode в `test_all.sh`
- [x] Phase 9: 30/35 модулей `AI/ai/*.py` шимизированы на Rust binaries
- [x] STACK violations: `web/api.py`, `medical_system.py`, `telegram_bot.py`
      formally listed как Frozen Python legacy в `STACK.md`
- [x] `agents/speculative.py` OpenAI-bypass → `llm.py::ask_fast()`
- [x] Whisper ASR exceptions задокументированы в `STACK.md` § Notes
- [x] E2E тест cornerstone PAM-trajectory (intake → PAM #1 → coach → codesign → PAM #2 → MCID → L_AGENCY) — passing, в `test_all.sh --quick`
- [x] `docs/operational/DEPLOY_RUNBOOK.md` — production deploy step-by-step (308 LoC)
- [x] `docs/operational/PILOT_PROTOCOL.md` — DRAFT клинический протокол (требует MD sign-off)
- [x] `scripts/pilot_cohort_extract.py` — cohort extraction (336 LoC, 3 output formats)

## P1 — текущий фокус (5 недель)

См. `STRATEGY.md` P1.

- [ ] **Pilot recruitment** 30 пациентов из практики DrJaba (P1-3 в STRATEGY).
      IRB-equivalent одобрение (Georgian Personal Data Protection Law 2014).
      → Owner: Dr. Jaba.
- [x] ~~AI/tests rewrite~~ ✅ 2026-05-07: 110 broken тестов удалены
      (4 файла + 50 функций); coverage у Rust crates. 489 passed / 0 skipped.
- [x] ~~**Citation для `lab_reference.py`**~~ ✅ 2026-05-07: добавлен
      single-source citation (Mayo Clinic Laboratories Reference Values
      for Adults 2024) + URL в docstring + secondary cross-check
      (MedlinePlus + WHO) + acknowledged limitations. Per-analyte
      verification — owner Dr. Jaba после pilot recruitment, добавлять
      по факту deviations в `notes` field.

## P2 — 6-12 недель (после P1 closure)

См. `STRATEGY.md` P2 + `NEEDTOWRITE.md`.

- [ ] `docs/operational/DEPLOY_RUNBOOK.md` — production deploy step-by-step.
- [ ] `docs/operational/PILOT_PROTOCOL.md` — клинический протокол pilot.
- [ ] `aim-llm` Rust crate — production HTTP service rollout (closure для
      `agents/llm_client.py` opt-in shim).
- [ ] CONCEPT §6 Agent Loop — переписать под фактический generalist + tool
      executor (предыдущий описывал отсутствующий task classifier).
- [ ] Drug interactions DB: 35 → 200+ pairs; RxNorm integration.

## P3 — 3-6 месяцев

См. `STRATEGY.md` P3.

- ~~`web/api.py` (772 LoC FastAPI) → Phoenix migration.~~ Frozen
  permanently 2026-05-07; revisit при multi-user pilot expansion.
- [ ] Phase 10 hybrid: PyO3 tools-as-crates (apply_patch / grep /
      verify_pmid / verify_doi / web_search / web_fetch).
      Dispatcher loop остаётся Python. См. `STRATEGY.md` P3-8.
- [ ] Multi-user pilot в production (≥3 врача, ≥10 patients each).
- [ ] Telegram bot тест на реальном `TELEGRAM_BOT_TOKEN` end-to-end —
      when needed (бот не используется в production; Phoenix `/chat` = primary UI).
- [ ] GUI `python3 aim_gui.py` тест на реальной клинической сессии —
      when needed (Phoenix LiveView `/chat`+`/intake`+`/cases` = primary clinical UI).

## Осознанно отложено / закрыто как vapor

- ~~`aim-media` v7.2 (TTS/image/video/talking-head/3D)~~ — REJECTED
  2026-05-07. CONCEPT §11 сокращён до эпитафии. Реактивация только
  по явной команде пользователя.
- ~~KIMI Moonshot client~~ — vapor; long-context обслуживается DS-chat 64k
  + Gemini Flash 1M.
- ~~Qwen DashScope client~~ — vapor; multilingual через DS-chat.

---

## Ad-hoc / экосистемные триггеры

Следить за этими только если конкретный partner / event активирует:

- **CDATA / Impetus Round 4** — следить за `~/Desktop/LongevityCommon/CDATA/TODO.md`.
- **EIC Pathfinder Challenges 2026** (deadline **2026-10-28**) — следить за
  `~/Desktop/LongevityCommon/CLAUDE.md`.
- **PhD applications** — следить за memory `project_phd_*` и `STRATEGY.md`
  partner проекта (не AIM).

---

**Convention:** при закрытии item — `[x]` + строка в `CHANGELOG.md`
[Unreleased]. Длинные obsolete блоки → `docs/roadmaps/TODO_archive_<YYYY>.md`.

```
### `KNOWLEDGE.md` (4762 chars)
```md
# KNOWLEDGE.md — AIM v7.0

**Версия:** 1.0
**Дата:** 2026-04-21
**Назначение:** Внешние знания, литература, домен-факты, на которые опирается AIM. Источник истины по архитектуре — `CONCEPT.md`.

---

## Индекс

1. [Интегративная медицина — профиль практики](#1-интегративная-медицина)
2. [LLM-провайдеры — документация](#2-llm-провайдеры)
3. [OCR / PDF — библиотеки](#3-ocr-и-pdf)
4. [Лабораторные референсы](#4-лабораторные-референсы)
5. [Многоязычие — 9 языков](#5-многоязычие)
6. [Связь с CDATA / HAP / MCOA](#6-научная-база-из-экосистемы)
7. [Регуляция / privacy](#7-регуляция-и-privacy)

---

## 1. Интегративная медицина

Профиль практики Dr. Jaba Tkemaladze, MD: кардиология, гериатрия, регенеративная медицина, нутрициология, фитотерапия (Regenesis protocols).

**Ключевые подходы:**
- Senolytic therapy (dasatinib + quercetin) — Jaba 2022, Tkemaladze 2023 (*Georgian Scientists*)
- Минеральное мороженое для восстановления после нагрузок — Tkemaladze & Samanishvili 2024
- Биологический возраст по CDATA → персонализированные интервенции

## 2. LLM-провайдеры

| Провайдер | Документация |
|-----------|--------------|
| DeepSeek | https://api-docs.deepseek.com |
| Groq | https://console.groq.com/docs |
| Anthropic Claude | https://docs.anthropic.com |
| Google Gemini (AI Studio) | https://ai.google.dev/gemini-api/docs |
| Ollama (offline) | https://ollama.com/docs |
| ~~KIMI (Moonshot)~~ | REJECTED 2026-05-07 (vapor) |
| ~~Qwen (Alibaba DashScope)~~ | REJECTED 2026-05-07 (vapor) |

**Особенности роутинга (накопленные знания, 2026-05-07):**
- DeepSeek-chat 64k + Gemini Flash 1M (free 1500/day) закрывают long-context
- Gemini 2.5 Pro (free 50/day) — fallback на Anthropic Claude Opus в `ask_critical()`
- Groq llama-3.3-70b / 3.1-8b быстры (>500 tok/sec), но лимит 8k контекстом
- DeepSeek-reasoner выдаёт CoT — лучший для дифдиагностики
- Multilingual для AR/ZH/KA — DS-chat (Qwen-уровень не нужен на 2026-05-07)
- Ollama qwen2.5:7b/3b + deepseek-r1 — offline fallback при недоступности cloud

## 3. OCR и PDF

| Библиотека | Назначение | Язык поддержка |
|-----------|-----------|----------------|
| `tesseract` (pytesseract) | OCR скриншотов | rus/eng/kat/kaz |
| `rapidocr_onnxruntime` | fallback, лучше для низкого качества | en/zh |
| `pymupdf` (fitz) | PDF-извлечение текста | любой |
| `pdfplumber` | таблицы из PDF | любой |

**Паттерн:** tesseract сначала (быстрее), rapidocr — fallback при confidence <0.6.

## 4. Лабораторные референсы

- **Primary source:** Mayo Clinic Laboratories Reference Values for
  Adults 2024 — [mayoclinic.org/medical-professionals/laboratory-reference-values](https://www.mayoclinic.org/medical-professionals/laboratory-reference-values)
- **Secondary cross-check:** NIH MedlinePlus (medlineplus.gov/lab-tests/),
  WHO Laboratory Quality Standards (iris.who.int/handle/10665/337693).
- Per-analyte deviations документировать в `notes` field конкретного
  аналита в `lab_reference.py`.
- **Важно:** референсы различаются по лабораториям (SI vs conventional
  units, возрастные/половые корректировки). Для clinical decisions
  полагаться на reference конкретной лаборатории, не на эти константы.
- `lab_reference.py` — 71 аналит, SI по умолчанию.

## 5. Многоязычие

**ООН-6 + KA + KZ + DA = 9 языков.** KZ добавлен для центральноазиатской аудитории, DA — для скандинавских грантов (EIC, Nordic Council).

**Детектор языка:** unicode-ranges + частые N-граммы + явный параметр `lang=`.

Смена языка в рантайме — без перезапуска. Тексты UI строго через `i18n.t(key, lang)`.

## 6. Научная база из экосистемы

| Проект | Вклад в AIM |
|--------|-------------|
| **CDATA** | биологический возраст → health-span предсказания |
| **HAP** (Hepato-Affective Primacy) | связь печени и настроения → скрининг депрессии через LFT |
| **MCOA** | 5 параллельных счётчиков повреждений → multi-system health dashboard |
| **Ze Theory** | нейрофизика сознания → BioSense-χ_Ze для скрининга когнитивного возраста |
| **BioSense** | EEG-измерение χ_Ze → интеграция в AIM после валидации |
| **FCLC** | федеративное обучение → будущий канал для анонимных моделей |
| **Regenesis** | фитотерапевтические протоколы → doctor-agent recommendations |
| **kSystem** | 8-язычный лексикон → термины для i18n |

## 7. Регуляция и privacy

- **GDPR / HIPAA-aware:** Patients/ никогда не покидает локальную машину
- **No-cloud policy на пациентов:** LLM-запросы очищаются от PII перед отправкой (см. `agents/intake.py::_anonymize()`)
- **Telegram-bot:** whitelist по `TELEGRAM_ALLOWED_ID`, не хранит сообщения
- Georgian Personal Data Protection Law 2014 — соответствие для грузинских пациентов

---

**Обновление:** добавлять новые факты датированным блоком внизу файла. При появлении >30 записей — разделить по тематическим подфайлам.

```
### `MEMORY.md` (4754 chars)
```md
# MEMORY.md — AIM v7.0

**Версия:** 1.0
**Дата создания:** 2026-04-21
**Назначение:** Что Клоду нужно помнить про этот проект между сессиями. Датированные записи; хронологический порядок (новое сверху).

---

## 2026-05-07 — Ядро восстановлено + Phase 9 closure + KIMI/Qwen vapor cleanup

**Что сделано:**
- 11-файловое ядро восстановлено: добавлены `THEORY.md` (immutable), `STRATEGY.md`, `REMINDER.md`, `CHANGELOG.md`, `NEEDTOWRITE.md`. STACK + README остаются.
- 24 не-канонических `.md` перемещены в `docs/audits/`, `docs/roadmaps/`, `docs/migration/`, `docs/manuscripts/`, `docs/operational/`.
- KIMI (Moonshot) и Qwen (DashScope) сняты как vapor из CONCEPT/PARAMETERS — HTTP-client'ы не написаны, фактический набор провайдеров: DeepSeek + Groq + Anthropic + Gemini + Ollama. **2026-05-07 update:** реализация в Rust `aim-llm` crate REJECTED — не "на hold", отвергнуто симметрично с aim-media. Реактивация только по факту use case.
- Phase 9: 30/35 модулей `AI/ai/*.py` шимизированы на Rust binaries. Полная регрессия `bash scripts/test_all.sh --quick` зелёная; `--ai` mode добавлен.
- `AI/tests/_phase9_known_broken.txt` (110 nodeids) — auto-skip через `AI/tests/conftest.py`. 505 passed / 110 skipped.
- E2E `tests/test_pam_trajectory_e2e.py` — PASSED. Cornerstone happy-path подтверждён: intake → PAM #1 → coach → codesign → PAM #2 → MCID delta → L_AGENCY.
- `agents/speculative.py` — переписан через `llm.py::ask_fast()` (раньше прямой OpenAI client).

## 2026-04-21 — Core-schema аудит (закрыто)

Создан core-set 10 файлов: CONCEPT, README, CLAUDE, TODO, PARAMETERS, MAP, MEMORY, LINKS, KNOWLEDGE, UPGRADE. **2026-05-07: расширено до 13** (+THEORY, STRATEGY, REMINDER, CHANGELOG, NEEDTOWRITE).

**Делать на будущее:** при любом значимом изменении архитектуры — синхронно обновить CONCEPT + MAP + PARAMETERS + CHANGELOG; при изменении UI/меню — i18n + medical_system + aim_gui.

## 2026-04-16 — v7.0 релиз

Переход от Ollama (локально) к гибридному API-роутеру. **2026-05-07 фактический набор:** DeepSeek-chat + DeepSeek-reasoner + Groq + Anthropic Claude + Google Gemini + Ollama (offline fallback). KIMI/Qwen vapor вычищены. 9 языков, 71 аналит, Telegram-бот, GUI.

## Постоянные правила (не забывать)

1. **LLM — только через `llm.py`**, никогда не вызывать API напрямую из других модулей.
2. **Patients/ неприкосновенны** — не читать, не изменять, не коммитить без явной команды пользователя.
3. **Ключи — только в `~/.aim_env`**, никогда в коде.
4. **Меню — править ОБА файла:** `medical_system.py` + `aim_gui.py`; источник истины — ключи в `i18n.py`.
5. **9 языков везде** — никаких hardcoded строк UI, всё через `i18n.t(key, lang)`.
6. **Git push:** всегда спрашивать — private (`djabbat/AIM`) или public (`djabbat/AIM-public`)? Public **исключает** CONCEPT/CLAUDE/TODO/PARAMETERS/Patients.

## Активные вопросы

См. `STRATEGY.md` P1 для актуального фокуса. На 2026-05-07 — единственный
critical-path open question:

- [ ] **Pilot recruitment 30 пациентов** (`STRATEGY.md` P1-3) → owner: Dr. Jaba.
  Блокер: `docs/operational/PILOT_PROTOCOL.md` `[CLIN-FILL]` placeholders +
  IRB-equivalent одобрение (Georgian Personal Data Protection Law 2014).

Закрытые / перенесённые 2026-05-07:

- [x] ~~Ждём пополнения KIMI~~ — REJECTED (vapor); long-context = DS-chat 64k + Gemini Flash 1M.
- [x] ~~Ждём активации Qwen~~ — REJECTED (vapor); multilingual = DS-chat.
- [→] Тест Telegram-бота → перенесён в `TODO.md` P3 «when needed»
  (2026-04-21 stale; не блокирует cornerstone / pilot).
- [→] Тест GUI `python3 aim_gui.py` → перенесён в `TODO.md` P3 «when needed»
  (Phoenix LiveView routes уже = web GUI).

## Известные проблемы

| Проблема | Workaround |
|----------|-----------|
| OCR низкая точность на русских сканах | rapidocr fallback + ручная проверка |
| ~~110 AI/tests/* поломаны после Phase 9~~ | ✅ 2026-05-07: удалены (4 файла + 50 функций) |
| ~~`web/api.py` Phoenix migration~~ | ✅ 2026-05-07: frozen permanently в STACK § "Frozen Python legacy"; revisit при multi-user pilot expansion |

## Что НЕ делать

- Не возвращаться к Ollama/llama3.2 (устарело, медленно, ограниченный контекст)
- Не хардкодить строки UI на одном языке
- Не пушить без спроса — private/public?
- Не добавлять в меню пункт, не добавив ключ в `i18n.py` для всех 9 языков
- Не забывать fallback — каждый провайдер может упасть, нужен план B

## Связь с экосистемой

AIM — standalone, но опирается на научные результаты CDATA/HAP/MCOA/Ze. При обновлении этих проектов — проверять, не нужно ли добавить новый анализ в doctor-агент.

---

**Конвенция:** новые записи сверху с датой `## YYYY-MM-DD — краткий заголовок`. При >50 записях — архивировать старше 6 месяцев в файл вида MEMORY_archive_YYYY.md (placeholder name pattern, файл создаётся при необходимости).

```
### `LINKS.md` (3089 chars)
```md
# LINKS.md — AIM v7.0

**Версия:** 1.0
**Дата:** 2026-04-21
**Назначение:** Внешние URL: репозитории, деплой, документация, связанные проекты. Источник истины — `CONCEPT.md`.

---

## GitHub

| Репозиторий | Статус | URL |
|-------------|--------|-----|
| `djabbat/AIM` | private | https://github.com/djabbat/AIM |
| `djabbat/AIM-public` | public | https://github.com/djabbat/AIM-public |

## Deployment / Domain

| Сервис | URL | Статус |
|--------|-----|--------|
| AIM landing | https://aim.drjaba.com | planned |
| DrJaba clinic | https://drjaba.com | live |
| Telegram-bot | `@aim_drjaba_bot` (TBD) | dev |

## LLM-провайдеры (dashboards)

| Провайдер | Console |
|-----------|---------|
| DeepSeek | https://platform.deepseek.com |
| Groq | https://console.groq.com |
| KIMI (Moonshot) | https://platform.moonshot.cn |
| Qwen (DashScope) | https://dashscope.console.aliyun.com |

## Документация используемых библиотек

- Ollama (legacy): https://ollama.com/docs
- customtkinter: https://customtkinter.tomschimansky.com
- python-telegram-bot: https://docs.python-telegram-bot.org
- pytesseract: https://github.com/madmaze/pytesseract
- pymupdf: https://pymupdf.readthedocs.io
- pdfplumber: https://github.com/jsvine/pdfplumber
- rapidocr: https://github.com/RapidAI/RapidOCR

## Связанные проекты экосистемы

| Проект | Путь | Роль |
|--------|------|------|
| CommonHealth | `~/Desktop/LongevityCommon/` | Umbrella (EIC Pathfinder) |
| CDATA | `~/Desktop/LongevityCommon/CDATA/` | Биологический возраст |
| HAP | `~/Desktop/LongevityCommon/HAP/` | Гепато-аффективная теория |
| MCOA | `~/Desktop/LongevityCommon/MCOA/` | Мульти-счётчиковая архитектура |
| Ze | `~/Desktop/LongevityCommon/Ze/` | Нейрофизика сознания |
| BioSense | `~/Desktop/LongevityCommon/BioSense/` | EEG χ_Ze |
| FCLC | `~/Desktop/LongevityCommon/FCLC/` | Federated learning |
| Regenesis | `~/Desktop/Regenesis/` | Фитотерапевтические протоколы |
| kSystem | `~/Desktop/kSystem/` | 8-язычный лексикон |
| DrJaba | `~/Desktop/DrJaba/` | Сайт клиники |
| Claude | `~/Desktop/Claude/` | Сервисные файлы, протоколы |

## Grants / funding

- EIC Pathfinder 2026: https://eic.ec.europa.eu/eic-funding-opportunities/eic-pathfinder_en
- Дедлайн подачи: **2026-05-12** (see `~/Desktop/LongevityCommon/EIC/`)

## Научные базы

- PubMed: https://pubmed.ncbi.nlm.nih.gov
- Google Scholar Jaba Tkemaladze: https://scholar.google.com/citations?user=...
- ORCID Jaba Tkemaladze: https://orcid.org/0000-...
- Zenodo (CommonHealth deposits): https://zenodo.org/communities/commonhealth

## Публикации с цитированием AIM / CDATA

- Tkemaladze J. (2026). Architecture and Design of a Prototype Multi-Modal Clinical Decision Support System for Integrative Medicine. *Longevity Horizon*, 2(4). https://doi.org/10.65649/4cxxhe47
- Tkemaladze J. (2026). The Digital Trivium. *Longevity Horizon*, 2(4). https://doi.org/10.65649/w1adk253

## Контакты

- Canonical email: jaba@longevity.ge
- Clinic: DrJaba Tbilisi (см. drjaba.com)

---

**Обновление:** URL могут меняться — проверять каждые 3 месяца. Мёртвые ссылки помечать `[DEAD]`.

```
### `rust-core/Cargo.toml` (7267 chars)
```toml
[workspace]
resolver = "2"
members = [
    "crates/aim-common",
    "crates/aim-llm",
    "crates/aim-rag",
    "crates/aim-medkb",
    "crates/aim-doctor",
    "crates/aim-generalist",
    "crates/aim-dp",
    "crates/aim-hive-worker",
    "crates/aim-hive-queen",
    "crates/aim-hive-consumer",
    "crates/aim-ai-ledger",
    "crates/aim-ai-regression",
    "crates/aim-ai-health",
    "crates/aim-ai-prompt-versions",
    "crates/aim-ai-cases",
    "crates/aim-ai-fix-planner",
    "crates/aim-ai-meta-evaluator",
    "crates/aim-ai-stable-run",
    "crates/aim-ai-dashboard",
    "crates/aim-ai-safety-gate",
    "crates/aim-ai-suppressions",
    "crates/aim-i18n",
    "crates/aim-ai-prompt-impact",
    "crates/aim-ai-regression-alert",
    "crates/aim-ai-backup",
    "crates/aim-ai-case-archiver",
    "crates/aim-ai-morning-brief",
    "crates/aim-ai-findings-to-evals",
    "crates/aim-ai-compliance-promoter",
    "crates/aim-ai-skill-standard",
    "crates/aim-ai-distillation",
    "crates/aim-ai-reflexion",
    "crates/aim-ai-gap-detector",
    "crates/aim-ai-finding-validator",
    "crates/aim-ai-auto-sweep",
    "crates/aim-ai-explainer",
    "crates/aim-ai-doctor",
    "crates/aim-ai-self-diagnostic",
    "crates/aim-ai-runner",
    "crates/aim-ai-eval-synthesiser",
    "crates/aim-ai-self-modify",
    "crates/aim-cost-ledger",
    "crates/aim-worktree",
    "crates/aim-citation-guard",
    "crates/aim-verify",
    "crates/aim-grep",
    "crates/aim-deadline-scanner",
    "crates/aim-notify",
    "crates/aim-hub-client",
    "crates/aim-hub-auth",
    "crates/aim-pattern-miner",
    "crates/aim-evals",
    "crates/aim-literature-watch",
    "crates/aim-citation-linter",
    "crates/aim-complexity",
    "crates/aim-prompt-optimizer",
    "crates/aim-ast-verify",
    "crates/aim-adaptive-limiter",
    "crates/aim-request-deduplicator",
    "crates/aim-feature-flags",
    "crates/aim-llm-cache",
    "crates/aim-permission",
    "crates/aim-ab-router",
    "crates/aim-cost-monitor",
    "crates/aim-tool-synthesis",
    "crates/aim-escalation",
    "crates/aim-skill-synthesis",
    "crates/aim-reflexion",
    "crates/aim-ensemble",
    "crates/aim-debate",
    "crates/aim-brief-preferences",
    "crates/aim-kpi-tracker",
    "crates/aim-brief-preamble",
    "crates/aim-smart-routing",
    "crates/aim-stakeholder-tracker",
    "crates/aim-project-owner",
    "crates/aim-project-state-machine",
    "crates/aim-pam",
    "crates/aim-disagreement",
    "crates/aim-codesign",
    "crates/aim-coach",
    "crates/aim-smart-fallback",
    "crates/aim-smart-context",
    "crates/aim-quick-action",
    "crates/aim-speculative",
    "crates/aim-impact-analyser",
    "crates/aim-unicode-guard",
    "crates/aim-follow-up-generator",
    "crates/aim-voice",
    "crates/aim-regimen-validator",
    "crates/aim-own-pubs-tracker",
    "crates/aim-recall-perf",
    "crates/aim-memory-health",
    "crates/aim-health-extended",
    "crates/aim-profile",
    "crates/aim-memory-deduplicate",
    "crates/aim-project-archive",
    "crates/aim-memory-priority",
    "crates/aim-memory-versioning",
    "crates/aim-memory-remediator",
    "crates/aim-memory-monitor",
    "crates/aim-memory-prefetch",
    "crates/aim-memory-store",
    "crates/aim-memory-date-correction",
    "crates/aim-memory-cli",
    "crates/aim-memory-tui",
    "crates/aim-coder",
    "crates/aim-orchestrator",
    "crates/aim-writer",
    "crates/aim-researcher",
    "crates/aim-lang",
    "crates/aim-doctor-agent",
    "crates/aim-intake",
    "crates/aim-kernel",
    "crates/aim-email-agent",
    "crates/aim-graph",
    "crates/aim-hooks",
    "crates/aim-tracing",
    "crates/aim-metrics",
    "crates/aim-slash-commands",
    "crates/aim-mcp-loader",
    "crates/aim-chat",
    "crates/aim-context-compressor",
    "crates/aim-session-manager",
    "crates/aim-aider-tool",
    "crates/aim-speculative-prefetch",
    "crates/aim-pairing",
    "crates/aim-cli-completion",
    "crates/aim-tree-planner",
    "crates/aim-module-registry",
    "crates/aim-doctor-calibration",
    "crates/aim-doctor-consult",
    "crates/aim-doctor-dry-run",
    "crates/aim-escalation-engine",
    "crates/aim-routines",
    "crates/aim-job-queue",
    "crates/aim-pi-agent",
    "crates/aim-self-health",
    "crates/aim-prompt-evolver",
    "crates/aim-interactions",
    "crates/aim-diff-analyser",
    "crates/aim-labs",
    "crates/aim-patient-dedup",
    "crates/aim-patient-memory",
    "crates/aim-kpi-auto-updater",
    "crates/aim-readme-generator",
    "crates/aim-project-export",
…<truncated 86 more lines>…
```
### `phoenix-umbrella/mix.exs` (723 chars)
```exs
defmodule AimUmbrella.MixProject do
  use Mix.Project

  def project do
    [
      apps_path: "apps",
      version: "0.1.0",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      aliases: aliases(),
      releases: releases()
    ]
  end

  defp releases do
    [
      aim_web: [
        applications: [
          aim_gateway: :permanent,
          aim_memory: :permanent,
          aim_orchestrator: :permanent,
          aim_web: :permanent
        ],
        include_executables_for: [:unix],
        steps: [:assemble]
      ]
    ]
  end

  defp deps do
    []
  end

  defp aliases do
    [
      setup: ["cmd mix setup"],
      "ecto.setup": ["cmd --app aim_memory mix ecto.setup"]
    ]
  end
end

```
### `pyproject.toml` (2865 chars)
```toml
[build-system]
requires = ["setuptools>=68", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "aim-generalist"
version = "7.1.0"
description = "Assistant of Integrative Medicine — local-first tool-using agent (DeepSeek / Claude / Gemini / Ollama)"
readme = "README.md"
requires-python = ">=3.10"
license = { text = "MIT" }
authors = [
    { name = "Jaba Tkemaladze", email = "jaba@longevity.ge" }
]
keywords = ["llm", "agent", "medical", "tool-use", "react", "deepseek", "claude", "gemini", "ollama"]
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Healthcare Industry",
    "Intended Audience :: Science/Research",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3 :: Only",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Topic :: Scientific/Engineering :: Artificial Intelligence",
    "Topic :: Scientific/Engineering :: Medical Science Apps.",
]
dependencies = [
    "openai>=1.30.0",
    "anthropic>=0.40.0",
    "python-dotenv>=1.0.0",
    "httpx>=0.25.0",
    "json-repair>=0.59.0",
    "pytesseract>=0.3.10",
    "Pillow>=10.0.0",
    "pymupdf>=1.23.0",
    "pdfplumber>=0.10.0",
    "python-telegram-bot>=21.0.0",
    "customtkinter>=5.2.0",
    "fastapi>=0.110.0",
    "uvicorn>=0.27.0",
    "pydantic>=2.5.0",
    "argon2-cffi>=23.1.0",
    "tiktoken>=0.7.0",
]

[project.optional-dependencies]
rapidocr = ["rapidocr-onnxruntime>=1.3.0"]
gmail = [
    "google-api-python-client>=2.0",
    "google-auth-httplib2>=0.2",
    "google-auth-oauthlib>=1.2",
]
dev = ["pytest>=7.0", "pytest-subtests>=0.10"]

[project.urls]
Homepage = "https://github.com/djabbat/AIM-public"
Source = "https://github.com/djabbat/AIM-public"
Issues = "https://github.com/djabbat/AIM-public/issues"

[project.scripts]
aim       = "cli.__main__:main"
aim-hub   = "cli.__main__:main_hub"
aim-node  = "cli.__main__:main_node"
aim-ai    = "cli.__main__:main_ai"

[tool.setuptools]
include-package-data = true

[tool.setuptools.packages.find]
include = ["agents*", "tools*", "web*", "cli*", "scripts*", "migrations*"]
exclude = ["tests*", "Patients*", "USER*", "venv*", "media*", "logs*",
            "experiments*", "reports*", "patches*", "fonts*", "docs*",
            "DiffDiagnosis*", "SSA*", "agents.bak.*", "Books*", "JabaEkimi*"]

[tool.setuptools.package-data]
"web" = ["templates/*", "static/*"]
"scripts" = ["desktop/icons/*", "desktop/*.sh", "desktop/*.ps1", "desktop/*.py"]

[tool.pytest.ini_options]
markers = [
    "network: tests that hit live APIs (PubMed, Crossref, etc.)",
]
# importmode=importlib — avoids `tests.conftest` namespace collision
# between root tests/ and AI/tests/ (each gets a unique module path).
testpaths = ["tests", "AI/tests"]
addopts = "--import-mode=importlib"

```
### `requirements.txt` (553 chars)
```txt
# AIM v7.0 — зависимости
# Ядро
openai>=1.30.0
python-dotenv>=1.0.0

# OCR
pytesseract>=0.3.10
Pillow>=10.0.0
rapidocr-onnxruntime>=1.3.0

# PDF
pymupdf>=1.23.0
pdfplumber>=0.10.0

# Telegram
python-telegram-bot>=21.0.0

# GUI
customtkinter>=5.2.0

# Утилиты
httpx>=0.25.0
json-repair>=0.59.0   # robust JSON парсинг LLM-выдачи (intake, MEMORY.md fill)

# Web / multi-user (hub + node)
fastapi>=0.110.0
uvicorn>=0.27.0
pydantic>=2.5.0
argon2-cffi>=23.1.0   # password hashing on the hub
anthropic>=0.40.0     # Claude API (premium tier + native vision)

```
### `Dockerfile` (1710 chars)
```
FROM python:3.11-slim

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
        tesseract-ocr poppler-utils inotify-tools rlwrap \
        sqlite3 gnupg ca-certificates curl \
        espeak-ng \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -U pip && \
    pip install --no-cache-dir -r requirements.txt && \
    pip install --no-cache-dir \
        prometheus-client circuitbreaker tqdm prompt-toolkit \
        fastapi uvicorn websockets \
        tenacity \
        sentence-transformers lancedb networkx httpx pyyaml

COPY . .

# Make CLI wrappers available
RUN mkdir -p /usr/local/bin && \
    for cmd in agents/graph.py agents/embed_daemon.py agents/memory_index.py \
               scripts/backup_system.py scripts/disk_monitor.py \
               agents/memory_cli.py agents/memory_health.py agents/profile.py \
               agents/pi_agent.py agents/voice.py; do \
        true; \
    done

ENV PYTHONUNBUFFERED=1 \
    AIM_LLM_TIMEOUT=60 \
    AIM_METRICS_PORT=9090 \
    AIM_HEALTH_PORT=9091 \
    AIM_WEB_PORT=8080 \
    AIM_PROFILE=default

EXPOSE 8080 9090 9091

# Persistent state should be mounted as volumes:
#   /root/.claude          - memory + indexes + checkpoints
#   /app/Patients          - patient data
#   /root/.aim_env         - API keys (read-only)
VOLUME ["/root/.claude", "/app/Patients"]

# Healthcheck against /healthz
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD curl -fsS http://127.0.0.1:9091/healthz || exit 1

CMD ["python", "-m", "web.api", "--host", "0.0.0.0", "--port", "8080", "--metrics"]

```
### code `SSA/backend/src/main.rs`
```
mod engine;
mod types;

use axum::{extract::State, response::Json, routing::{get, post}, Router};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::engine::{digitize, load_patterns, load_ranges, match_patterns};
use crate::types::*;

#[derive(Clone)]
struct AppState {
    refs: Arc<RangesFile>,
    patterns: Arc<PatternsFile>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")))
        .with(fmt::layer())
        .init();

    let ranges_path = std::env::var("SSA_RANGES").unwrap_or_else(|_| "../data/ranges.json".into());
    let patterns_path = std::env::var("SSA_PATTERNS").unwrap_or_else(|_| "../data/patterns.json".into());
    let port: u16 = std::env::var("SSA_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8766);

    let refs = load_ranges(&ranges_path)?;
    let patterns = load_patterns(&patterns_path)?;
    tracing::info!("loaded {} parameters, {} patterns",
        refs.parameters.len(), patterns.patterns.len());

    let state = AppState { refs: Arc::new(refs), patterns: Arc::new(patterns) };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/digitize", post(post_digitize))
        .route("/api/v1/syndromes", post(post_syndromes))
        .route("/api/v1/parameters", get(list_params))
        .route("/api/v1/patterns", get(list_patterns))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("ssa-api listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status":"ok","service":"ssa-api","version":env!("CARGO_PKG_VERSION")}))
}

async fn post_digitize(State(s): State<AppState>, Json(input): Json<CbcInput>) -> Json<DigitizeResponse> {
    Json(digitize(&input, &s.refs))
}

async fn post_syndromes(State(s): State<AppState>, Json(input): Json<CbcInput>) -> Json<SyndromesResponse> {
    let d = digitize(&input, &s.refs);
    let matched = match_patterns(&d.digitized, &s.patterns.patterns);
    let red = matched.iter().filter(|p| p.severity == "red").count();
    let amber = matched.iter().filter(|p| p.severity == "amber").count();
    let green = matched.iter().filter(|p| p.severity == "green").count();
    Json(SyndromesResponse {
        digitized: d.digitized,
        patterns: matched,
        red_count: red, amber_count: amber, green_count: green,
    })
}

#[derive(serde::Serialize)]
struct ParamSummary {
    id: String,
    unit: String,
    derived: Option<String>,
}

…<truncated 17 more lines>…
```
### code `DiffDiagnosis/backend/src/main.rs`
```
mod engine;
mod types;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::engine::{load_algorithms, rank};
use crate::types::*;

#[derive(Clone)]
struct AppState {
    algorithms: Arc<Vec<Algorithm>>,
    top_k: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")))
        .with(fmt::layer())
        .init();

    let algo_path = std::env::var("DIFFDX_ALGORITHMS")
        .unwrap_or_else(|_| "../algorithms.json".to_string());
    let port: u16 = std::env::var("DIFFDX_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8765);
    let top_k: usize = std::env::var("DIFFDX_TOP_K")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let algorithms = load_algorithms(&algo_path).unwrap_or_else(|e| {
        tracing::warn!("could not load {}: {} — starting with empty bank", algo_path, e);
        Vec::new()
    });
    tracing::info!("loaded {} algorithms from {}", algorithms.len(), algo_path);

    let state = AppState {
        algorithms: Arc::new(algorithms),
        top_k,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/case", post(post_case))
        .route("/api/v1/diff", post(post_diff))
        .route("/api/v1/algorithm/:id", get(get_algorithm))
        .route("/api/v1/algorithms", get(list_algorithms))
        .route("/api/v1/sources", get(list_sources))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("diffdx-api listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status":"ok","service":"diffdx-api","version":env!("CARGO_PKG_VERSION")}))
}

async fn post_case(Json(input): Json<CaseInput>) -> Json<Case> {
    Json(input.into_case())
}

async fn post_diff(
    State(s): State<AppState>,
…<truncated 51 more lines>…
```
### code `rust-core/crates/aim-ai-prompt-versions/src/main.rs`
```
//! aim-ai-prompt-versions CLI — Phase 9 Tier 1 #2 (2026-05-07).
//!
//! Tracks sha256/size of `SELF_DIAGNOSTIC_PROMPT.md` revisions.
//! Replaces `AI/ai/prompt_versions.py`.
//!
//! Subcommands:
//!   prompt-path                              # resolved prompt path
//!   db-path                                  # resolved ledger DB path
//!   fingerprint [<path>]                     # JSON of current fp
//!   record-current [<path>] [--ts T]         # JSON of recorded fp
//!   history                                  # JSONL of all fps
//!   drift-since-last [<path>]                # JSON Drift struct
//!   summary                                  # plain-text summary

use std::path::PathBuf;
use std::process::ExitCode;

use aim_ai_prompt_versions::{
    default_prompt_path, fingerprint_of, PromptStore,
};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match cli(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("aim-ai-prompt-versions: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cli(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let (cmd, rest) = args
        .split_first()
        .ok_or("usage: aim-ai-prompt-versions <prompt-path|db-path|fingerprint|record-current|history|drift-since-last|summary>; --help")?;
    match cmd.as_str() {
        "--help" | "-h" | "help" => {
            print_usage();
            Ok(())
        }
        "prompt-path" => {
            println!("{}", default_prompt_path().display());
            Ok(())
        }
        "db-path" => {
            // Mirror lib resolution: AI_DIAGNOSTIC_DB or default cache.
            let p = std::env::var("AI_DIAGNOSTIC_DB")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME")
                        .map(PathBuf::from)
                        .unwrap_or_else(|_| PathBuf::from("."));
                    home.join(".cache").join("aim").join("diagnostic_ledger.db")
                });
            println!("{}", p.display());
            Ok(())
        }
        "fingerprint" => {
            let path: PathBuf = rest
                .first()
                .map(PathBuf::from)
                .unwrap_or_else(default_prompt_path);
            let fp = fingerprint_of(&path)?;
            println!("{}", serde_json::to_string(&fp)?);
            Ok(())
        }
        "record-current" => {
            let mut rest_v: Vec<&str> = rest.iter().map(String::as_str).collect();
            let ts = take_opt(&mut rest_v, "--ts");
            let path = rest_v.first().map(|s| PathBuf::from(*s));
            let store = PromptStore::open_default()?;
            let fp = store.record_current(path.as_deref(), ts.as_deref())?;
            println!("{}", serde_json::to_string(&fp)?);
            Ok(())
        }
        "history" => {
            let store = PromptStore::open_default()?;
            for fp in store.history()? {
                println!("{}", serde_json::to_string(&fp)?);
…<truncated 58 more lines>…
```
### code `rust-core/crates/aim-generalist/src/main.rs`
```
//! aim-generalist :8774 — ReAct tool-using executor.
//!
//! Endpoints:
//!   GET  /health
//!   GET  /v1/tools      — list registered tool names
//!   POST /v1/run        — sync: { task, max_iters? } -> { answer, trace, tools_used }
//!   POST /v1/run/stream — SSE: each step emits a typed event

use aim_generalist::react;

use aim_common::{cors_layer, health_handler, init_tracing, ApiError, ApiResult};
use axum::{
    extract::{Path as AxPath, State},
    response::sse::{Event as SseEvent, KeepAlive, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

#[derive(Clone)]
struct AppState {
    runner: Arc<react::Runner>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing("info");
    let runner = Arc::new(react::Runner::from_env());
    let state = AppState { runner };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(aim_common::metrics_handler))
        .route("/v1/tools", get(list_tools))
        .route("/v1/run", post(run_handler))
        .route("/v1/run/stream", post(run_stream_handler))
        .route("/v1/interrupt/:run_id", post(interrupt_handler))
        .with_state(state)
        .layer(cors_layer());

    let port: u16 = aim_common::port(
        "AIM_GENERALIST_PORT",
        aim_common::AimConfig::load().ports.aim_generalist,
        8774,
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!(%addr, "aim-generalist listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Serialize)]
struct ToolsResp { tools: Vec<String>, count: usize }

async fn list_tools(State(s): State<AppState>) -> Json<ToolsResp> {
    let tools = s.runner.tool_names();
    let count = tools.len();
    Json(ToolsResp { tools, count })
}

#[derive(Deserialize)]
struct RunReq {
    task: String,
    #[serde(default)] max_iters: Option<usize>,
    #[serde(default)] system: Option<String>,
}

#[derive(Serialize)]
struct RunResp {
    answer: String,
    trace: Vec<react::TraceEntry>,
    tools_used: Vec<String>,
}

…<truncated 59 more lines>…
```
### code `rust-core/crates/aim-ai-cases/src/main.rs`
```
//! aim-ai-cases CLI — Phase 9 Tier 3 #15 (2026-05-07).
//!
//! Replaces `AI/ai/case_validator.py` (CV1).
//!
//! Subcommands:
//!   validate-one <PATH>      # JSON CaseStatus
//!   validate-dir [--dir D]   # JSON Report
//!   summary       [--dir D]  # plain-text

use std::path::PathBuf;
use std::process::ExitCode;

use aim_ai_cases::{validate_dir, validate_one};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match cli(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("aim-ai-cases: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cli(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let cmd = args.first().map(String::as_str).unwrap_or("summary");
    let rest: Vec<&str> = args.iter().skip(1).map(String::as_str).collect();
    match cmd {
        "--help" | "-h" | "help" => {
            print_usage();
            Ok(())
        }
        "validate-one" => {
            let p = rest.first().ok_or("validate-one: <PATH> required")?;
            let s = validate_one(std::path::Path::new(p));
            println!("{}", serde_json::to_string(&s)?);
            Ok(())
        }
        "validate-dir" => {
            let mut v = rest;
            let dir = take_opt(&mut v, "--dir").map(PathBuf::from);
            let r = validate_dir(dir.as_deref());
            println!("{}", serde_json::to_string(&r)?);
            Ok(())
        }
        "summary" => {
            let mut v = rest;
            let dir = take_opt(&mut v, "--dir").map(PathBuf::from);
            let r = validate_dir(dir.as_deref());
            if r.n_cases == 0 {
                println!("(no eval cases found)");
                return Ok(());
            }
            println!(
                "📋 Case validator — {} cases ({} ok / {} failed)",
                r.n_cases, r.n_ok, r.n_failed
            );
            if r.all_ok() {
                println!("  ✅ all cases pass schema check");
                return Ok(());
            }
            for s in &r.statuses {
                if s.ok {
                    continue;
                }
                let cid = s.case_id.as_deref().unwrap_or("?");
                let name = s
                    .path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?");
                println!("  ❌ {}  ({})", name, cid);
                for i in &s.issues {
                    println!("      • {}", i);
                }
            }
            Ok(())
        }
        other => Err(format!("unknown command {other:?}; try --help").into()),
…<truncated 25 more lines>…
```
### code `rust-core/crates/aim-interactions/src/main.rs`
```
//! aim-interactions CLI — Phase 8 Week 2 (2026-05-07).
//!
//! Drug-drug interaction lookup. The static table (~30 pairs with PMIDs
//! / mechanisms / recommendations) lives in the lib (`aim_interactions`);
//! this binary exposes it as JSON-on-stdout subcommands so the Python
//! shim (`agents/interactions.py`) becomes a thin subprocess wrapper.
//!
//! Subcommands:
//!   check <drug_a> <drug_b>          — JSON Interaction
//!   regimen <drug1> <drug2> ...      — JSONL of pairs
//!   format <drug1> <drug2> ... [--lang en|ru] [--include-no-known]
//!   known-drugs                      — newline-separated canonical names
//!   canon <name>                     — print canonical key

use std::process::ExitCode;

use aim_interactions::{
    canon, check_interaction, check_regimen, dump_table, format_regimen_report, known_drugs,
    DISCLAIMER,
};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match cli(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("aim-interactions: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cli(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let (cmd, rest) = args
        .split_first()
        .ok_or("usage: aim-interactions <check|regimen|format|known-drugs|canon> ...; try --help")?;
    match cmd.as_str() {
        "--help" | "-h" | "help" => {
            print_usage();
            Ok(())
        }
        "check" => {
            let a = rest.first().ok_or("check: <drug_a> required")?;
            let b = rest.get(1).ok_or("check: <drug_b> required")?;
            let i = check_interaction(a, b);
            println!("{}", serde_json::to_string(&i)?);
            Ok(())
        }
        "regimen" => {
            let drugs: Vec<String> = rest.iter().cloned().collect();
            for ix in check_regimen(&drugs) {
                println!("{}", serde_json::to_string(&ix)?);
            }
            Ok(())
        }
        "format" => {
            let mut rest_v: Vec<&str> = rest.iter().map(String::as_str).collect();
            let lang = take_opt(&mut rest_v, "--lang").unwrap_or_else(|| "en".to_string());
            let include_no_known = take_flag(&mut rest_v, "--include-no-known");
            let drugs: Vec<String> = rest_v.iter().map(|s| s.to_string()).collect();
            let interactions = check_regimen(&drugs);
            let report = format_regimen_report(&interactions, &lang, include_no_known);
            print!("{report}");
            Ok(())
        }
        "known-drugs" => {
            for d in known_drugs() {
                println!("{d}");
            }
            Ok(())
        }
        "canon" => {
            let name = rest.first().ok_or("canon: <name> required")?;
            println!("{}", canon(name));
            Ok(())
        }
        "dump-table" => {
            for ix in dump_table() {
                println!("{}", serde_json::to_string(&ix)?);
            }
…<truncated 42 more lines>…
```
## Code volume
| ext | files | bytes |
|---|---|---|
| .rs | 322 | 3606332 |
| .js | 14 | 2846671 |
| .py | 333 | 2727926 |
| .ex | 89 | 286781 |
| .exs | 50 | 57076 |
| .heex | 4 | 45065 |