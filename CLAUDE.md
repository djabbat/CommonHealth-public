# CLAUDE.md — LongevityCommon Ecosystem

> ⚠️ **См. [_archive/audits/CORRECTIONS_2026-04-22.md](_archive/audits/CORRECTIONS_2026-04-22.md)** — некоторые утверждения в этом файле могут быть отозваны. Каноны обновлены 2026-04-22.


## Project Identity

**LongevityCommon** — центральная платформа экосистемы долголетия + **umbrella для EIC Pathfinder заявки**.
**Подпроекты (подпапки):** MCOA · Ze · CDATA · BioSense · Ontogenesis · HAP (+ Activated из Iqalto как WP5). **MCOA** (добавлен 2026-04-21) — мета-теоретический фундамент; остальные подпроекты встраиваются в MCOA как счётчики или измерительные слои.
**Версия:** CONCEPT v4.0 (Ecosystem Edition) | **Status: CONCEPT APPROVED**
**Location:** `~/Desktop/LongevityCommon/`

**EIC Pathfinder — ✅ ACTIVE TARGET: Pathfinder Challenges 2026 — "Biotechnology for Healthy Ageing"** (verified 2026-04-28 on eic.ec.europa.eu):

- **Deadline:** 28 October 2026, 17:00 Brussels time (CET/CEST). 6 months runway.
- **Budget:** up to €4M per project (challenge budget €32M; ~8 projects funded).
- **TRL target:** TRL3-4 (proof of principle + scientific validation).
- **NOT** Pathfinder Open 2026 (12 May 2026) — withdrawn 2026-04-21 after internal peer review v10 (1.86/5 REJECT).
- **NOT** Pathfinder Open 2027 — superseded by the Challenges 2026 pivot (decision 2026-04-17, Variant C).

**Why Challenges 2026 instead of Open 2027:**
The "Biotechnology for Healthy Ageing" topic in Challenges 2026 is a closer scientific fit than the generic Open call. Higher topical alignment → higher Excellence score for the same content.

**Architecture (Variant C, decided 2026-04-17, ACTIVE):**
- WP1 FCLC (€0.6M, 12 mo) — federated learning
- WP2 Ze (€0.5M, 12 mo) — biomarker
- WP3 CDATA (€0.8M, 24 mo) — biology / centriolar damage (partner: Institut Curie via Janke's co-PI, in negotiation 2026-04-28)
- WP4 BioSense (€0.6M, 18 mo) — wearable hardware
- WP5 Aqtivirebuli/Korkoti (€0.5M, 12 mo) — clinical pilot anaemia
- **Total: €3.0M, 36 mo** (revisable up to €4M ceiling)
- **Host:** NGO Georgia Longevity Alliance (reg. №404506520)

**Consortium status (2026-04-28):**
- ✅ Geiger (Ulm DE) — Phase B Co-PI, LoS signed 2026-04-23
- 🟡 Miguel A. Gonzalez Ballester (UPF Barcelona ES) — meeting 2026-04-28, awaiting reply ~2026-05-01-05
- 🟡 Janke (Curie FR) — declined personally (CoI), introducing his co-PI as real partner lab
- ⏳ Need ≥2 confirmed EU-MS signed LoIs by ~2026-09 to be safe

**Outstanding blockers from 2026-04-21 audit (must resolve before submission):**
1. PATE demo implementation (ε≈0.63 path) — target Sep 2026
2. CDATA ABL-2 Sobol S1 paradox — extended global sensitivity analysis — target Aug 2026
3. HAP/Ontogenesis fabricated PMIDs — already halted; rebuilt before any inclusion in proposal

**Concept Note preprint:** Tkemaladze, J. (2026) "LongevityHealth", Zenodo v3, (28 April 2026; supersedes earlier "LongevityCommon" preprint 19546679).

*Deep audit file (still relevant):* `~/Desktop/LongevityCommon/FCLC/DEEP_AUDIT_2026-04-21.md`
*Variant C decision record:* `~/Desktop/LongevityCommon/_archive/EIC_CONSORTIUM_STRUCTURE_2026-04-21.md`
*Canonical deferral record (DEFERRAL CANCELLED 2026-04-28; kept for history):* same path

---

## Ecosystem Structure

```
LongevityCommon/ ← этот проект (социальный слой)
├── MCOA/ ← Multi-Counter Architecture of Organismal Aging — мета-теория (добавлен 2026-04-21)
├── Ze/ ← Ze Theory — Entropic-Geometric Theory of Everything. Канон: Ze Theory.pdf + Ze Теория.pdf (2026-04-25). Rust simulator покрывает 3 блока: импеданс-ODE, CHSH-деформация, cheating autowaves.
├── CDATA/ ← теория повреждения центриолей, MCAI (Counter #2 в MCOA)
├── BioSense/ ← EEG+HRV+обоняние (измерительный слой MCOA)
├── Ontogenesis/ ← платформа онтогенеза 0–25 лет
├── HAP/ ← Hepato-Affective Primacy Theory (нейро-гепатология)
├── server/ ← Rust/Axum REST API
├── web/ ← React TypeScript PWA
└── realtime/ ← Elixir/Phoenix Channels
```

**Правило:** LongevityCommon — thin social layer over Ze+CDATA+BioSense+Ontogenesis+HAP. Никакой новой науки, никакой новой privacy-инфраструктуры. Новое: UX сообщества, ранжирование ленты, Ze·Guide AI.

**FCLC — отдельный проект** (extracted 2026-04-26). Server-resident: `jaba@server:/home/jaba/web/fclc/`. Repos: `djabbat/FCLC` (private) + `djabbat/FCLC-public`. См. `~/.claude/projects/-home-oem/memory/project_fclc_server_workflow.md`.

---

## Source of Truth

**CONCEPT.md is the authoritative document.**
Все подпроекты имеют собственные CONCEPT.md — авторитет на уровне подпроекта.
При конфликте: LongevityCommon CONCEPT.md > субпроект CONCEPT.md.

---

## Language Defaults

- Backend API: **Rust** (Axum, sqlx)
- Frontend: **React + TypeScript** (Vite, PWA)
- Realtime: **Elixir/Phoenix** (Channels, LiveView)
- Subproject specifics: см. CLAUDE.md каждого подпроекта

---

## Critical Rules

### Четыре фактора здоровья (обязательно в UI и API)
Здоровье = ОРГАНИЗМ + ПСИХИКА + СОЗНАНИЕ + СОЦИУМ
- Ze·Profile отображает все 4 фактора
- Ze·Guide отвечает на вопросы по всем 4 доменам
- Таблица `health_factors` хранит психика/сознание/социум (организм — в ze_samples)
- ~~Интегральный Health Score: `0.40*organism + 0.25*psyche + 0.20*consciousness + 0.15*social`~~ **УДАЛЕНО 2026-04-22** — веса не имели вывода из MCOA L_tissue; используется напрямую L_tissue с tissue-specific w_i (см. CONCEPT.md §A.2)

### Ze·Guide
1. **Disclaimer перед КАЖДЫМ ответом** — без исключений
2. **Логировать ВСЁ** в `ze_guide_logs` (disclaimer_sent = true)
3. **Не давать медицинских советов** — только научный контекст
4. **Цитировать источники** — DOI, файлы, датасеты

### Биологический возраст
- Всегда: point estimate + 95% CI + stability label
- Никогда: «Ваш возраст улучшился на 2 года за ночь»
- stability: high (<3y CI) / medium (<7y) / low

### База данных
- Схема: `server/migrations/001_initial.sql`
- ORM: sqlx (compile-time queries)
- Параметры: `$1, $2, ...` — никогда строковая интерполяция
- GDPR: soft delete через `deleted_at`, экспорт через `GET /api/data/export`

### Антифрод
- DOI → verify через Crossref API при создании поста
- Неверный DOI → `rank_penalty += 2.0` (не блокировать пост)

### API responses
```rust
// Успех: Json(value)
// Ошибка: (StatusCode::XXX, String)
// Никогда: .unwrap в handlers
```

---

## Приоритеты разработки

1. **Безопасность** — no SQL injection, параметры везде
2. **Корректность** — Ze compute с CI
3. **Юридическая защита** — Ze·Guide logs, consent, GDPR export
4. **Производительность** — индексы на ze_samples, posts; pagination

---

## DeepSeek Rule

**Код — Claude. Всё остальное (статьи, тексты, переводы, гранты) — DeepSeek API.**
Ключ: `~/.aim_env → DEEPSEEK_API_KEY`
Модели: `deepseek-chat` (быстро) · `deepseek-reasoner` (научные рассуждения)

---

## Core .md Files

Все .md кроме README.md — файлы ядра.
Генерируются из CONCEPT.md. Обновляются при каждом значимом изменении.
ARCHITECTURE не существует отдельно — его содержимое в CONCEPT.md.

**Файлы ядра (полный список — в .gitignore для public):**
`CONCEPT.md` · `KNOWLEDGE.md` · `PARAMETERS.md` · `MAP.md` · `MEMORY.md` · `LINKS.md` · `UPGRADE.md` · `TODO.md` · `CLAUDE.md` · `STRATEGY.md` · `REMINDER.md`

**`STRATEGY.md`** — гибридная грантовая стратегия (4 трека: CDATA/Ze/BioSense/Ontogenesis; FCLC — отдельный проект).
Читать первым делом в каждой сессии перед работой с любым подпроектом.

**Git (монорепозиторий):**
- **Единый репозиторий:** `djabbat/LongevityCommon` (объединяет LongevityCommon + Ze + CDATA + BioSense + Ontogenesis + HAP). FCLC = отдельный repo `djabbat/FCLC`.
- Private: все файлы включая .md ядра
- Public: только код + README (core .md в .gitignore)

---

## Subproject References

| Подпроект | CLAUDE.md | Авторитетный документ |
|-----------|-----------|----------------------|
| ~~FCLC~~ | extracted 2026-04-26 | server-resident, `djabbat/FCLC` repo |
| Ze | `Ze/CLAUDE.md` | `Ze/CONCEPT.md` |
| CDATA | `CDATA/CLAUDE.md` | `CDATA/CONCEPT.md` |
| BioSense | `BioSense/CLAUDE.md` | `BioSense/CONCEPT.md` |
| Ontogenesis | `Ontogenesis/CLAUDE.md` | `Ontogenesis/CONCEPT.md` |
| HAP | `HAP/CLAUDE.md` | `HAP/CONCEPT.md` |
