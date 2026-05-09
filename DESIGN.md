# LongevityCommon · DESIGN

**Status:** Canonical · regenerated 2026-04-28 from CONCEPT v5.6
**Authority:** Cross-cutting architecture; subproject-specific designs in `<subproject>/DESIGN.md`

---

## §1. Repository layout

```
~/Desktop/LongevityCommon/
├── CONCEPT.md · THEORY.md · DESIGN.md (this) · PARAMETERS.md · MAP.md · STATE.md
├── EVIDENCE.md · OPEN_PROBLEMS.md · TODO.md · README.md · CLAUDE.md · LICENSE
├── _archive/v_pre_2026-04-28/ # архив core .md до 2026-04-28
├── _audits/ # cross-subproject audit reports
├── docs/
│ └── EIC_PartB_2026/ # EIC заявка (active grant track)
├── deploy/
│ └── docker-compose-all.yml # production stack
├── server/ # social-layer Rust API (axum + sqlx)
├── web/ # social-layer React+TS PWA
├── realtime/ # social-layer Phoenix Channels
│
├── MCOA/ # subproject: theoretical
├── CDATA/ # subproject: molecular hypothesis
├── Ze/ # subproject: math + simulator (regenerated 2026-04-28)
├── BioSense/ # subproject: applied + simulator + datasets (regenerated 2026-04-28)
├── FCLC/ # subproject: federated infra (server-resident, separate repo)
├── Telomere/ MitoROS/ EpigeneticDrift/ Proteostasis/ # counter modules
├── CytogeneticTree/ AutomatedMicroscopy/ # tooling + demos
└── HAP/ Ontogenesis/ # ❌ TOXIC (failed PMID audits — not in ecosystem v5.6)
```

---

## §2. Two-layer design: scientific layer + social layer

LongevityCommon = **(scientific subprojects) + (thin social layer)**.

### §2.1 Scientific layer (subprojects)

Каждый подпроект автономен:
- Собственный `CONCEPT.md` + `THEORY.md` + `DESIGN.md` + `PARAMETERS.md` + ...
- Собственный simulator / backend / web (где применимо)
- Собственные tests
- Собственный port (если runtime): Ze :4000/4001, BioSense :4100/4101, FCLC :4002/4003

Cross-subproject communication — **через HTTP API** (federated by design, не shared memory).

### §2.2 Social layer (umbrella)

Цель: community-facing platform для пользователей.

| Component | Tech | Port (dev) | Назначение |
|-----------|------|-----------:|------------|
| `server/` | Rust + axum + sqlx + Postgres | 8080 (proposed) | REST API: auth, dashboard, posts, studies, BioSense passthrough, Ze·Guide chat logs, GDPR export |
| `web/` | React + TypeScript + Vite (PWA) | 5173 (Vite default) | UI: Dashboard, Feed, Login, Profile, Settings, Studies |
| `realtime/` | Elixir + Phoenix Channels | 4001 (conflict with Ze!) → 4500 | WebSocket: feed updates, live BioSense samples |
| Postgres | docker | 5432 | основная DB социального слоя (не subproject data) |

⚠ **Port conflict warning:** `realtime/` mix.exs изначально планировался на port 4001 — это конфликтует с `Ze/ze-backend` :4001. Переносим realtime на port **4500** в обновлённой конфигурации (см. `~/Desktop/LongevityCommon/realtime/config/dev.exs` — TODO update).

---

## §3. Что социальный слой делает / НЕ делает

### Делает:
- Auth (email/password, OTP), session management
- User profile with χ_Ze score history (запросы → BioSense API)
- Posts/feed (community discussion)
- Studies enrolment + consent
- Ze·Guide AI chat (с обязательным disclaimer перед каждым ответом, GDPR-compliant logging)
- GDPR data export
- Анти-фрод: DOI verification через Crossref API при создании posts

### НЕ делает (это в подпроектах):
- ❌ Не вычисляет χ_Ze (это в `BioSense/biosense-simulator`)
- ❌ Не делает CHSH optimisation (это в `Ze/ze-simulator`)
- ❌ Не обучает federated models (это в `FCLC/fclc-core`)
- ❌ Не имеет встроенных научных формул в коде (только display + delegate)

---

## §4. API contracts (social layer ↔ subprojects)

### §4.1 Social server → BioSense backend

`POST http://127.0.0.1:4101/api/chi_ze`
Body: `{eeg, hrv, resp, sleep, convention?}`
Response: `{composite, per_modality, v_star, convention}`
Used by: `server/src/handlers/biosense.rs` → `web/src/pages/Dashboard.tsx`

### §4.2 Social server → Ze backend (если нужно)

`POST http://127.0.0.1:4001/api/chsh` etc. — для теоретических визуализаций в `web/src/pages/Studies.tsx`.

### §4.3 Social server → FCLC backend

Через `FCLC/fclc-server` HTTP API на server :4002 — для отображения federated round metrics в Studies.

---

## §5. Что меняется после CONCEPT v5.6 (action items)

После регенерации CONCEPT и article v5.6 social-layer code требует точечных правок (не переписки):

### §5.1 Server-side (Rust)

- [ ] `src/handlers/biosense.rs` — добавить header в response: `X-LongevityCommon-Status: hypothesis-stage-exploratory`
- [ ] `src/handlers/dashboard.rs` — strings про "biological age" → "exploratory aging activity index (research only)"
- [ ] `src/handlers/ze_guide.rs` — обновить системный prompt для AI: **"χ_Ze is a research-grade exploratory metric, not a validated medical biomarker. Pre-registered tests of v1 yielded NULL results; current v2 is post-hoc. Do not give medical advice. Decline confirmatory clinical claims."**
- [ ] Добавить новый endpoint `GET /api/disclosures/v5_changes` — возвращает changelog (NULL retraction, M4 update, FCLC threat model fix) — публичная transparency
- [ ] `migrations/003_health_factors.sql` — комментарии добавить "thresholds are exploratory, see CONCEPT v5.6 §2"

### §5.2 Web-side (React/TS)

- [ ] `web/src/pages/Dashboard.tsx` — добавить banner "⚠ Hypothesis-stage research platform. Metrics shown are exploratory, not clinical advice."
- [ ] `web/src/pages/Studies.tsx` — каждый study card: "v1 NULL results (deprecated/superseded); v2 multimodal post-hoc" disclosure
- [ ] `web/src/pages/Profile.tsx` — tooltip на χ_Ze score: "exploratory metric; not validated on N≥2000 pre-registered cohort yet"
- [ ] `web/src/components/feed/PostComposer.tsx` — DOI verification flow (existing); добавить warning если DOI relates to Longevity Horizon journal (not PubMed-indexed)

### §5.3 Realtime (Elixir/Phoenix)

- [ ] `realtime/config/dev.exs` — port move из 4001 → 4500 (избежать конфликта с Ze :4001)
- [ ] `lib/longevitycommon_realtime_web/channels/feed_channel.ex` — без изменений
- [ ] BioSense live stream channel — добавить metadata `{disclosure: "exploratory"}` к каждому sample

### §5.4 Deploy

- [ ] `deploy/docker-compose-all.yml` — обновить service names + порты согласно §2.2 + §5.3
- [ ] Environment variable `LONGEVITYCOMMON_VERSION=v5.6` для tracking

---

## §6. Что НЕ требует изменений

- Auth flow, session management, OTP — ortogonal к научным claim'ам
- Posts/feed CRUD — community feature, не зависит от scientific content
- Studies CRUD schema (sans descriptions text) — schema валидна
- GDPR export endpoint — без изменений
- Anti-fraud DOI Crossref check — работает корректно

---

## §7. Build / run

### §7.1 Local dev (full stack)

```bash
# Subprojects (backends)
cd ~/Desktop/LongevityCommon/Ze && ./run.sh # :4000/:4001
cd ~/Desktop/LongevityCommon/BioSense && ./run.sh # :4100/:4101

# Social layer
cd ~/Desktop/LongevityCommon/server && cargo run --release # :8080
cd ~/Desktop/LongevityCommon/web && npm run dev # :5173
cd ~/Desktop/LongevityCommon/realtime && mix phx.server # :4500 (after fix)
```

### §7.2 Production

```bash
cd ~/Desktop/LongevityCommon/deploy && docker compose -f docker-compose-all.yml up -d
```

---

## §8. Subproject autonomy contract

Каждый подпроект должен:
- Иметь собственный `CONCEPT.md` (overridable by umbrella `CONCEPT.md` on cross-cutting points)
- Не зависеть от umbrella server/web/realtime для тестов / релизов
- Опубликовать свои API endpoints для umbrella consumption
- Отдельные tests (cargo test / mix test) запускаются автономно

При conflict между umbrella `CONCEPT.md` и subproject `CONCEPT.md`: umbrella > subproject на cross-cutting вопросах (статусы, falsifiability, threat model); subproject > umbrella на internal math.
