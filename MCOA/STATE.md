# STATE — MCOA

**Назначение:** волатильное состояние.

---

## Current status (2026-05-10)

- **Submission #1:** Nature Aging NATAGING-P13741 (MCOA v5 Perspective), поданo 2026-04-19, статус review
- **Manuscript #2 (NOT YET PUBLISHED, draft):** "A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging" — extension paper, 3 core principles + потенциальный 6-й счётчик (piRNA), VEXAS как доказательство независимости #5 от #2. Source: `~/Desktop/A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging.md` + review chain `docs/manuscripts/HAYFLICK_HIERARCHY/01–15`. Готов к финальной TBPR-сверке.
- **Manuscript #3 (NOT YET PUBLISHED, draft):** "Epigenomic Rejuvenation Without Functional Restoration" — systematic review + meta-analysis, PROSPERO **CRD42026218473**, n=14 studies, 274 animals; **damage shadow hypothesis**. Target: *Nature Aging* / *Cell Metabolism* / *Lancet Healthy Longevity* (IF>18). Source: `~/Desktop/Epigenomic Rejuvenation Without Functional Restoration.md`.
- **Counters:** 5 канонических + χ_Ze (S-counter) + **#6 candidate: piRNA-counter** (placeholder, не входит в v5; см. THEORY.md §4.1)
- **Tissue weights:** w_HSC, w_skin, w_neural, w_muscle (см. PARAMETERS.md)

---

## Active TODOs

- [ ] Дождаться решения Nature Aging editorial decision (рукопись #1)
- [ ] Подготовить response к reviewer comments (если будут)
- [ ] Backup: secondary target (npj Aging, eLife) если reject
- [ ] Sobol ABL-2 paradox для Counter #1 — закрыть в координации с CDATA L1
- [ ] Tissue-specific weights калибровка против реальных данных HSC/skin/neural
- [ ] **Stem-Cell-Centric extension:** финальная TBPR-сверка, verify VEXAS PMID, JAK/NLRP3 therapeutic refs, формализовать D_pi (piRNA counter) кинетику для §4.1 THEORY.md
- [ ] **Damage Shadow review:** перенести черновик в `docs/manuscripts/DAMAGE_SHADOW/`, сверить PROSPERO record, добавить EpigeneticDrift subproject EVIDENCE.md cross-link
- [ ] piRNA-counter (#6): поиск млекопитающих-специфичных данных (валидация вне germline) — блокирует включение в canonical set

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

### 2026-05-10 — Two new draft manuscripts integrated into MCOA roadmap
1. **Stem-Cell-Centric extension (HAYFLICK_HIERARCHY v12+).** Заявлены три новых тезиса (контекст-зависимый приоритет счётчиков, ткань-специфичный winner-counter atlas, приоритет лимита Хейфлика у стволовых). VEXAS-синдром (UBA1 mutation) introduced как clinical evidence: counter #5 (proteostasis) может быть rate-limiting **независимо от теломер**, что укрепляет M1 (parallel counters). Введён candidate **counter #6 — piRNA**; не canonical до validation в mammalian non-germline tissue.
2. **Damage Shadow systematic review (PROSPERO CRD42026218473).** Pooled correlation между ΔDNAmAge и Δfunction r=0.09 (NS); threshold ΔDNAmAge ≈ −2.4 yrs-equiv до появления modest tissue-specific gain; Lu 2020 (RGC) и Berdugo-Vega 2026 (engram neurons) refine, не refute, общий тезис. Hierarchical model: транскриптомика > эпигеномика > структурный damage shadow. **Прямое следствие для MCOA:** оправдывает M1 (single-counter эпигенетический reset недостаточен) и формализует concept of **structural counters** (collagen cross-links, mtDNA, ECM stiffening) как epigenetically-independent. Mesenchymal drift (Li & Tay 2026) — кандидат на operationalisation для счётчика #4 (epigenetic drift) как reversible vs irreversible component.

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
