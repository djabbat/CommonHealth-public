# STATE — MCOA

**Назначение:** волатильное состояние.

---

## Current status (2026-04-25)

- **Submission:** Nature Aging NATAGING-P13741, поданo 2026-04-19, статус review
- **Версия рукописи:** MCOA v5
- **Counters:** 5 + χ_Ze (S-counter)
- **Tissue weights:** w_HSC, w_skin, w_neural, w_muscle (см. PARAMETERS.md)

---

## Active TODOs

- [ ] Дождаться решения Nature Aging editorial decision
- [ ] Подготовить response к reviewer comments (если будут)
- [ ] Backup: secondary target (npj Aging, eLife) если reject
- [ ] Sobol ABL-2 paradox для Counter #1 — закрыть в координации с CDATA L1
- [ ] Tissue-specific weights калибровка против реальных данных HSC/skin/neural

---

## Milestones

### v5 — Nature Aging submission ✅ 2026-04-19
- [x] MCOA_v5_NatureAging_2026-04-21.pdf готов
- [x] Cover letter
- [x] Submission через editorial system
- [x] 2 follow-up correspondence (2026-04-21)

### v9-file core ✅ 2026-04-25
- [x] CLAUDE.md создан
- [x] STATE.md создан

### Code baseline ✅ 2026-04-25 (overnight #5 fixed)
- [x] cargo build --release: success
- [x] mcoa_core: 6/6 unit tests pass (было 3 → +3 новых)
- [x] **NEW:** `aging_rate_is_weighted_sum` — формула `Σ w_i · C_i = 0.42` на тестовых значениях
- [x] **NEW:** `null_gamma_yields_zero_influence` — γ=0 default per CORRECTIONS-2026-04-22
- [x] **NEW:** `identity_gamma_yields_self_value` — γ identity = self-value
- [x] mcoa_tests crate (workspace integration tests) — пуст, оставлен на будущее
- [x] mcoa_cli, mcoa_api — компилируются

### Python scripts → Rust port ✅ 2026-04-25 (overnight)

Created `crates/mcoa_compare/`:
- [x] `mcoa-compare-cdata` binary — replaces `scripts/compare_mcoa_cdata.py` (markdown report без plot)
- [x] `mcoa-compare-all` binary — replaces `scripts/compare_all.py` (pairwise Δ matrix)
- [x] `mcoa_compare` lib — `read_csv`, `delta_stats`, `compare_mcoa_cdata`. **3/3 tests pass.**
- [x] cargo build --release: success
- [x] Plot generation вынесен из scope Rust port (можно добавить через `plotters` crate позже)
- [x] Старые Python скрипты остаются в `scripts/` для cross-validation

---

## Decision Log

### 2026-04-25 — 9-file core scheme
Добавлены CLAUDE + STATE. Существующие 7 файлов (CONCEPT/DESIGN/EVIDENCE/OPEN_PROBLEMS/PARAMETERS/README/THEORY) уже соответствуют новой схеме.

### 2026-04-19 — Nature Aging submission
MCOA v5 поданa в Nature Aging как flagship мета-теория LongevityCommon. Включает Counter #1 (CDATA), и формализует общую multi-counter архитектуру.

---

## Что НЕ делать

- Не публиковать препринт MCOA до решения Nature Aging
- Не добавлять новые counters без явной интеграции с CONCEPT.md
- Не путать "5 counters" со "5 hallmarks" (Counter ≠ hallmark)

## Startup checklist

1. Прочитать CONCEPT + STATE Decision Log
2. Проверить ответ Nature Aging
3. Спросить пользователя
