# STATE — HAP

---

## Current status (2026-04-25)

- **Status:** halted 2026-04-21 (все 10 EVIDENCE PMIDs fabricated)
- **Recovery plan:** rebuild EVIDENCE.md с верифицированными PMID 2026-05 … 2026-09
- **Опубликовано:** *HAP Theory* в Longevity Horizon 2(4), DOI 10.65649/d76f6c48 (2026-04-15)
- **EIC Pathfinder:** deferred to 2027

---

## Active TODOs

- [ ] Rebuild EVIDENCE.md полностью из верифицированных PubMed
- [ ] Связь с CDATA/MCOA: HAP может быть Counter #6 (печёночно-аффективный)?
- [ ] Подготовить к peer-reviewed журналу (Hepatology / Nature Mental Health)
- [ ] Bradford-Hill анализ HAP (~20/27 ожидается, Class II)

---

## Milestones

### Publication ✅ 2026-04-15
- [x] HAP Theory опубликована в Longevity Horizon 2(4)

### v9-file core ✅ 2026-04-25
- [x] CLAUDE.md создан
- [x] STATE.md создан
- [x] EVIDENCE.md создан как stub (rebuild pending)

### Code baseline ⚠️ 2026-04-25 (overnight #2 partial)
- [x] **Fixed:** sqlx "offline" feature removed (replaced by SQLX_OFFLINE env var в 0.7)
- [x] **Fixed:** добавлен `src/lib.rs` (модули config/db/error/models/routes), Cargo.toml `[lib]`
- [x] **Fixed:** `axum::Server::bind` → `axum::serve` + tokio TcpListener (axum 0.7 API)
- [ ] **Remaining:** 182 sqlx query type errors — нужны feature flags `chrono`/`uuid` для query!() macros, либо `cargo sqlx prepare` для offline cache
- Status: проект halted из-за fabricated PMIDs; полный fix низкого приоритета до 2026-09 PMID rebuild

---

## Decision Log

### 2026-04-25 — 9-file core scheme
Добавлены CLAUDE + STATE + EVIDENCE (stub). Существующие 6 файлов сохранены.

### 2026-04-21 — Halted
10/10 PMIDs в EVIDENCE.md fabricated. Halted до rebuild.

### 2026-04-15 — Publication
HAP Theory опубликована в Longevity Horizon.

---

## Что НЕ делать

- Не использовать старые EVIDENCE PMIDs (помечены как fabricated)
- Не переподавать в peer-reviewed журнал до rebuild
- Не позиционировать HAP как замену CDATA — это параллельный модуль

## Startup checklist

1. OPEN_PROBLEMS первым
2. Если работаем над EVIDENCE — каждая ссылка через PubMed
3. Спросить пользователя
