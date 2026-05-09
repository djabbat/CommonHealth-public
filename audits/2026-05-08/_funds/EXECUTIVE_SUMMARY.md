# EXECUTIVE SUMMARY — Funds-perspective audit LongevityCommon (2026-05-08)

**Аудитор:** DeepSeek-reasoner (deepseek-v4-reasoner) с verifierами PubMed E-utilities + Crossref + DataCite + arXiv API + Zenodo API
**Стандарты:** ERC AdG, EIC Pathfinder Challenges 2026, NIH R01, Wellcome Discovery, Impetus
**Объём:** 14 подпроектов + умбрелла; 67 файлов с 643 ссылками (354 уникальных)

---

## ИТОГ ОДНОЙ ФРАЗОЙ

**WITHDRAW_ALL** (peer-review verdict). После cleanup 2026-05-08 все 40 фабрикаций (21 fake DOI + 19 PMID↔DOI mismatch) **физически удалены** из core docs (заменены на `[REF_NEEDED 2026-05-08]`). Это снимает один из 3 системных blocker'ов, но evidence base всё ещё требует ручного восстановления реальных ссылок. **Не подавать в EIC Pathfinder Challenges 2026 (deadline 2026-10-28).** Минимум 9-12 месяцев на полное исправление перед любой подачей.

## CLEANUP РЕЗУЛЬТАТ (2026-05-08, post-triage)

| Метрика | До | После | Δ |
|---|---|---|---|
| Fake DOIs | 21 | **0** | −21 ✅ |
| PMID↔DOI mismatches | 19 | **0** | −19 ✅ |
| Total fabrications | 40 | **0** | −40 ✅ |
| Total refs in core docs | 643 | 585 | −58 |
| `[REF_NEEDED 2026-05-08]` markers | 0 | 45 | +45 |

**Стратегия cleanup (triage_decisions.json):**
- 37× **REMOVE_BOTH** — text claim не совпал ни с PMID-truth ни с DOI-truth (масштабная фабрикация) → удалены оба, заменены на `[REF_NEEDED 2026-05-08]`
- 2× **KEEP_PMID_REPLACE_DOI** — PMID совпадает с text claim, DOI заменён на canonical из PubMed esummary (Wells DVT, Goetz cilium)
- 2× **KEEP_PMID_REMOVE_DOI** — старые статьи без canonical DOI в PubMed; PMID сохранён, DOI удалён

**Backup:** `~/Desktop/AUDIT_FUNDS_2026-05-08/backup_pre_cleanup/` (16 файлов)
**Лог:** `~/Desktop/AUDIT_FUNDS_2026-05-08/patches/cleanup_log_v3.md`
**Triage decisions:** `~/Desktop/AUDIT_FUNDS_2026-05-08/patches/triage_decisions.json`

**ВАЖНО для PI:** Triage был сделан DeepSeek-chat. Первый pass показал что DeepSeek галлюцинирует canonical DOIs (выдумал DOI вместо тех что в PubMed esummary). Все `KEEP_PMID_REPLACE_DOI` были перепроверены через PubMed esummary + doi.org HEAD probe. Финальные DOI взяты из PubMed.

---

## РАСПРЕДЕЛЕНИЕ ВЕРДИКТОВ (per-subproject)

| Verdict | Count | Подпроекты |
|---|---|---|
| TOXIC_WITHDRAW | 1 | UMBRELLA |
| REJECT | 8 | AIM, EpigeneticDrift, HAP, MCOA, MitoROS, Proteostasis, Telomere, Ze |
| REVISE_MAJOR | 4 | AutomatedMicroscopy, BioSense, CDATA, CytogeneticTree |
| REVISE_MINOR / FUND_AS_IS | 0 | — |

**Средний score экосистемы:** 2.0/5 (баллы Premise/Method/Evidence/Falsif/Deliv/Novelty/Risk → 2-3-2-3-2-3-2). Серьёзные фонды требуют 4+.

---

## КРИТИЧЕСКИЕ НАХОДКИ (P0)

### 1. Системная фабрикация ссылок
- **218/219 PMID валидны** ✓ (одна не найдена: PMID 27249423)
- **104/129 DOI в Crossref**, +3 Zenodo (DataCite), +1 Longevity Horizon internal, **+21 FAKE** (нет ни в одном реестре, doi.org=404)
- **🔴 19 PMID↔DOI mismatches** — реальный PMID указывает на статью, не имеющую отношения к описанию в тексте
- Локализация: **AIM/docs/diffdiagnosis/EVIDENCE.md (12 mismatch + 4 fake)**, AutomatedMicroscopy/EVIDENCE.md (3+1), CDATA/EVIDENCE.md (0+6), Proteostasis/EVIDENCE.md (0+2)

### 2. Подозрительный DOI prefix
`10.64898/2026.03.26.714124v1` (BrainYears bioRxiv 2026) в Ze/CONCEPT.md и Ze/KNOWLEDGE.md — bioRxiv использует prefix `10.1101`, не `10.64898`. **Скорее всего выдуман.**

### 3. Все эмпирические результаты — hypothesis-stage / post-hoc / без confirmatory
- CDATA Sobol p=0.12 (inconclusive)
- Ze v1 NULL на 3 cohort'ах (Cuban EEG / Dortmund Vital / MPI-LEMON), v2 χ_Ze — post-hoc reformulation
- BioSense χ_Ze MCID на N=12, AUC=0.81 на All-of-Us N=2222 — exploratory с p-hacking risk
- Ни один pre-registered протокол с α=0.001 не существует

### 4. Нулевая институциональная готовность
- 0 signed EU LoIs (для EIC Pathfinder требуется ≥3 EU MS partners)
- Single-PI экосистема, h-index неизвестен, последняя публикация 2023, Mol Biol Reports IF≈2
- Social server / BioSense backend / FCLC v14 не развёрнуты на сервере — экосистема существует только на бумаге

---

## VERDICT по фондам

| Фонд | Verdict | Минимальное время до подачи |
|---|---|---|
| ERC AdG | **WITHDRAW** | >24 мес (нужны publications, h-index, лидерство) |
| EIC Pathfinder Challenges 2026 (deadline 2026-10-28) | **WITHDRAW / DEFER до 2027** | 12-18 мес |
| NIH R01 | **WITHDRAW** | >18 мес (нужны preliminary data, environment) |
| Wellcome Discovery | **WITHDRAW** | >12 мес |
| Impetus Round 4 | **DEFER** | 9-12 мес (единственный realistic путь) |

---

## 4-WEEK EMERGENCY PLAN (8 мая — 5 июня 2026)

1. **Удалить 21 fabricated DOI и исправить 19 PMID↔DOI mismatches** во ВСЕХ файлах (см. `refs/REFERENCE_AUDIT_FINAL.md`). Не обращать в reverse через автоматический patcher (он часто меняет один fake на другой fake) — ручная проверка по PubMed для каждого случая.
2. **Переместить HAP/, Ontogenesis/ в `_archive/`** с пометкой TOXIC. Очистить корневой CLAUDE.md.
3. **Создать недостающие CLAUDE.md** для подпроектов: MCOA, CDATA, AutomatedMicroscopy, Telomere, MitoROS, EpigeneticDrift, Proteostasis, FCLC.
4. **Зарегистрировать на OSF.io** протоколы для BioSense (replication v*_active), CDATA (Sobol S1+S2 на GTEx), Ze (test-retest reliability χ_Ze).
5. **Развернуть social server (Rust/Axum) и BioSense backend** на сервере — иначе все утверждения о работающей платформе технически некорректны.

## 12-WEEK PLAN (до 31 июля 2026)

1. Полный аудит всех ссылок через API (PubMed/Crossref/DataCite) → REFERENCE_AUDIT_REPORT.md.
2. Один экспериментальный proof-of-concept (например, CDATA C1 на HSC мышах: n=3, 3 возраста, sortinp + IF, blinded). Pre-registration, препринт.
3. Заключить ≥2 signed EU LoIs (MPI Leipzig, Karolinska, CNRS).
4. CI/CD для umbrella (GitHub Actions): integration tests social layer + BioSense + Ze loopback.
5. Унифицировать терминологию (POSTULATE, v*, hypothesis-stage маркеры) между подпроектами.
6. Подать малый грант (AFAR, Glenn Foundation) на pilot эксперимент → внешняя валидация.

---

## ЧТО УЖЕ FUNDED-READY (при условии устранения fabrication)

- **Ze математический аппарат** (derivation v*_passive, Minkowski metric, Rust core)
- **BioSense Rust core + biosense-web** (компилируется, нужен deploy)
- **Cell-DT симулятор (CDATA)** — 161/161 тестов, Apache 2.0
- **FCLC v13** (semi-honest) — после v14 (Q1 2027) пригоден для GDPR

Ничего из этого не подаётся самостоятельно без устранения проблем.

---

## ARTIFACTS (структура аудита)

```
~/Desktop/AUDIT_FUNDS_2026-05-08/
├── refs/
│   ├── refs.json                      ← 643 извлечённых refs (354 уникальных)
│   ├── verified.json                  ← результаты PubMed/Crossref/DataCite
│   ├── pmid_doi_mismatch.json         ← 19 mismatches
│   ├── recheck.json                   ← повторная проверка через DataCite + doi.org HEAD
│   └── REFERENCE_AUDIT_FINAL.md       ← главный отчёт по ссылкам
├── packets/                           ← 13 review packets + 2 meta packets
├── reviews_v1/                        ← 13 per-subproject reviews (до safe patches)
├── reviews_v2/                        ← 13 reviews после safe patches
├── reports/
│   ├── ECOSYSTEM_META_REVIEW_v1.md    ← 17K — meta v1 (до safe patches)
│   └── ECOSYSTEM_META_REVIEW_v2_FINAL.md  ← 16K — ФИНАЛЬНЫЙ meta-обзор ⭐
├── patches/
│   ├── PATCHES_PLAN.md                ← план патчей (auto + manual)
│   ├── auto_patches.json              ← 7 опасных авто-патчей (НЕ применены)
│   ├── manual_flags.json              ← 34 ручных решения
│   └── SAFE_PATCHES_PLAN.md           ← план безопасных аннотаций (применён)
├── logs/                              ← логи всех DeepSeek call'ов
├── INTERIM_PROGRESS.md                ← прогресс (промежуточный)
└── EXECUTIVE_SUMMARY.md               ← этот документ
```

## ПРИМЕНЁННЫЕ ИЗМЕНЕНИЯ В РЕПОЗИТОРИИ (safe patches)

В 12 файлах добавлен `<!-- REF_AUDIT_2026-05-08 -->` баннер и 36 inline маркеров рядом с проблемными строками. Это reversible:
- AIM/docs/diffdiagnosis/EVIDENCE.md (16 markers)
- CDATA/EVIDENCE.md (6)
- AutomatedMicroscopy/EVIDENCE.md (4)
- Proteostasis/EVIDENCE.md (2)
- Ze/UPGRADE.md, Ze/MEMORY.md, Ze/CONCEPT.md, Ze/KNOWLEDGE.md (4)
- CytogeneticTree/KNOWLEDGE.md, CDATA/CONCEPT.md, HAP/LINKS.md, AIM/docs/ssa/EVIDENCE.md (4)

**НЕ применены auto patches** (опасны: часто меняют fake DOI на DOI другой статьи).

---

## РЕКОМЕНДАЦИЯ PI ОДНОЙ ФРАЗОЙ

Прекратить все подачи на 9-12 месяцев. Вычистить fabrications. Провести **один** эксперимент с pre-registration. Опубликовать в peer-reviewed журнале. Только тогда подавать в Impetus, через 18-24 мес — в более серьёзные фонды.
