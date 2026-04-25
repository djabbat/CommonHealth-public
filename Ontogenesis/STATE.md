# STATE — Ontogenesis

---

## Current status (2026-04-25)

- **Status:** halted 2026-04-21 после audit
- **Reason:** 6/6 KNOWLEDGE.md PMIDs fabricated (см. umbrella audit)
- **Recovery plan:** rebuild EVIDENCE.md from verified PubMed before 2026-09
- **EIC Pathfinder:** deferred to 2027

---

## Active TODOs

- [ ] Rebuild EVIDENCE.md полностью из верифицированных PubMed (2026-05 … 2026-09)
- [ ] Audit и rewrite affected sections в CONCEPT/THEORY
- [ ] Restore правило: zero hallucinations PMID/DOI
- [ ] Подготовить к 2027-Q1 EIC Pathfinder Open

---

## Milestones

### v9-file core ✅ 2026-04-25
- [x] CLAUDE.md создан
- [x] STATE.md создан

### Code baseline ✅ 2026-04-25 (overnight #5 verified)
- [x] cargo build --release: success
- [x] cargo test --release --lib: **21/21 tests pass** (metamorphosis + data::ingestion suites)
- [x] Backend (отдельный) — не проверен, низкий приоритет (halted после PMID audit)

---

## Decision Log

### 2026-04-25 — 9-file core scheme
Добавлены CLAUDE + STATE. Существующие 7 файлов соответствуют новой схеме.

### 2026-04-21 — Halted после audit
6/6 PMIDs в KNOWLEDGE.md (старая схема) оказались fabricated. Проект halted до полного rebuild evidence base.

---

## Что НЕ делать

- Не цитировать ничего без PubMed-верификации (как минимум до полного rebuild)
- Не подавать на гранты до завершения recovery (целевой срок 2027-Q1)
- Не возвращать старый KNOWLEDGE.md

## Startup checklist

1. Прочитать OPEN_PROBLEMS — там список того, что было fabricated
2. Если работаем над EVIDENCE — проверять КАЖДУЮ ссылку
3. Спросить пользователя
