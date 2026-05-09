# LongevityCommon · CONCEPT

**Status:** Canonical · regenerated 2026-04-28 from `~/Desktop/LongevityCommon.md` (article v5.6)
**Author:** Jaba Tkemaladze · ORCID 0000-0002-3826-7982
**Citation source (article):** Tkemaladze, J. (2026). LongevityCommon: An Integrative Ecosystem for Biomarker-Guided Interventions in Aging as Total Chronic Disease. *Longevity Horizon*, 2(5). [in submission, v5.6]
**Authority:** This `CONCEPT.md` overrides all subproject `CONCEPT.md` on cross-cutting questions; subproject `CONCEPT.md` retains authority on its internal mathematics.

---

## 1. Что такое LongevityCommon (одна фраза)

LongevityCommon — это **умбрелла-экосистема** из 5 взаимосвязанных научных подпроектов, образующих **интегративный biomarker-guided framework** для изучения старения как **Total Chronic Disease** (ICD-11 XT9T 2018, MG2A 2025), плюс тонкий **социальный слой** (server + web + realtime) поверх.

## 2. Hypothesis-stage статус (важно)


### Pre-registration plan
The confirmatory test of the current multi-modal χ_Ze formulation will be pre-registered on OSF (placeholder: `https://osf.io/TBD`) no later than 2026-12-31. The primary outcome is partial r² for all-cause mortality (controlling age + sex) on a cohort N≥2000 with α=0.001. All analysis scripts will be frozen at registration.



**LongevityCommon — hypothesis-stage framework.** Все эмпирические оценки трактуются как exploratory (hypothesis-generating), не confirmatory:

- Pre-registered тесты ранней univariate формулировки χ_Ze на Cuban EEG, Dortmund Vital и MPI-LEMON cohorts → **NULL results** (документировано в `Ze/EVIDENCE.md` 2026-04-22; meta-analysis I²=90.3% — invalid; deprecated/superseded). Текущая мультимодальная версия χ_Ze — **post-hoc reformulation**, не pre-registered.
- Все reported AUC (0.81 на All-of-Us N=2222 для PhenoAge acceleration) — exploratory с явным p-hacking risk (Ioannidis 2005, PMID 16060722).
- Ключевые публикации (MCOA, Ze, BioSense) **НЕ peer-reviewed** на момент v5.6.
- 0 signed EU LoIs на 2026-04-21; EIC Pathfinder Challenges 2026 deadline 2026-10-28.

## 3. Пять компонентов экосистемы

| # | Подпроект | Уровень | Status | Auth doc |
|---|-----------|---------|--------|----------|
| 1 | **MCOA** | Theoretical (meta-теория счётчиков) | submitted Nature Aging NATAGING-P13741, NOT peer-reviewed | `MCOA/CONCEPT.md` |
| 2 | **CDATA** | Molecular-cellular (centriolar PTM hypothesis) | **Inconclusive** — preliminary issue ABL-2 (R²_no_α=0.833 > full=0.778) НЕ statistically significant after nested CV (p=0.12); Sobol full decomp deferred to Cell-DT v4.0 | `CDATA/CONCEPT.md` |
| 3 | **Ze Theory** | Mathematical (entropy-geometric) | `dτ_Ze/dt = −α·I(Z)` — **POSTULATED ansatz** (НЕ derivation) by analogy with Burgholzer 2015 + Pearson 2021 (physical clocks, не биология) | `Ze/CONCEPT.md` |
| 4 | **BioSense** | Applied (wearable, χ_Ze) | Theoretical fixed point `v* = 0.45631` at `k_λ=1`; sensitivity range `v* ∈ [0.32, 0.58]` for `k_λ ∈ [0.5, 2.0]`. Empirically tested via swept-v* search on All-of-Us N=500: `v*_optimal = 0.451 (95% CI 0.443-0.459)` consistent with theory. | `BioSense/CONCEPT.md` |
| 5 | **FCLC** | Infrastructure (federated learning, privacy) | v13.4 PASS milestone; ε_total ≈ 0.43 at (σ=1.5, q=0.013, T=5); RDP composition Wang/Mironov 2017-2019. **Threat model (v5):** semi-honest server + Byzantine-robust ≤25% (Krum); NOT secure against active server collusion or malicious server. **GDPR Art. 9 blocker** до FCLC v14 (planned Q1 2027). | `FCLC/CONCEPT.md` |

Plus 1 supporting: **Activated** (clinical pilot via Шашвиашвили — anemia management cohort).

**TOXIC — NOT в этой версии экосистемы:** HAP, Ontogenesis (failed PMID audits 2026-04-21).

## 4. Уровневая интеграция

```
Theoretical level MCOA (counter-sum framework, axioms M1-M4)
 ↓ (counter #1 candidate)
Molecular-cellular level CDATA (centriolar PTM hypothesis, status inconclusive)
 ↓ (instantiates one D_i mechanism)
Mathematical level Ze Theory (entropy-geometric ansatz dτ/dt = −α·I)
 ↓ (gives χ_Ze formal foundation)
Applied level BioSense (wearable χ_Ze biomarker)
 ↓ (releases via privacy stack)
Infrastructure level FCLC (federated learning + DP + k-anon + secagg)
```

## 5. M4 falsifiability (operational)

**Updated v5 threshold:** MCOA falsified if на pre-registered cohort `N ≥ 2000` при `α = 0.001` partial r² для all-cause mortality после контроля chronological age + sex `< 0.05`. Power analysis: N=1875 required для R²=0.3 at 80% power; N≥2000 — community-standard threshold. Старый произвольный `R² < 0.5` заменён на эту community-standard threshold.

**Каждый счётчик в MCOA фальсифицируется отдельно** через свою partial r² contribution.

## 6. Архитектура umbrella репозитория

```
~/Desktop/LongevityCommon/
├── CONCEPT.md (this) · THEORY.md · DESIGN.md · PARAMETERS.md · MAP.md
├── STATE.md · EVIDENCE.md · OPEN_PROBLEMS.md · TODO.md · README.md · CLAUDE.md · LICENSE
├── _archive/ # старые версии core .md + audits + fixes
├── _audits/
├── docs/ # включая EIC_PartB_2026/ (новый)
├── server/ # Rust/axum REST API — социальный слой
├── web/ # React+TS PWA — социальный слой UI
├── realtime/ # Phoenix Channels — социальный слой WebSocket
├── deploy/ # docker-compose-all.yml для production
│
├── MCOA/ # подпроект theoretical
├── CDATA/ # подпроект molecular
├── Ze/ # подпроект mathematical (regenerated 2026-04-28)
├── BioSense/ # подпроект applied (regenerated 2026-04-28)
├── FCLC/ # подпроект infrastructure (server-resident, отдельный repo)
├── Telomere/ MitoROS/ EpigeneticDrift/ Proteostasis/ # counter modules concept-stage
├── CytogeneticTree/ AutomatedMicroscopy/ # demo/tooling
├── HAP/ Ontogenesis/ # ❌ TOXIC, halted, не используются в экосистеме
```

## 7. Социальный слой (server/web/realtime) — что это

**Назначение:** community-facing platform для пользователей, которые хотят следить за исследованиями, получать χ_Ze оценки, участвовать в studies. **НЕ научный слой** — все scientific computations происходят в подпроектах (`Ze/biosense-simulator`, `BioSense/biosense-backend`, `FCLC/fclc-server`).

| Component | Technology | Role | Где живёт |
|-----------|------------|------|-----------|
| **server/** | Rust/axum + sqlx + Postgres | REST API: auth, dashboard, posts, studies, BioSense passthrough, Ze·Guide chat logs, GDPR export | local-only пока, deploy via docker-compose |
| **web/** | React + TypeScript + Vite (PWA) | UI: Dashboard, Feed, Login, Profile, Settings, Studies | local-only |
| **realtime/** | Elixir/Phoenix Channels | WebSocket realtime: feed updates, BioSense live samples | local-only |

**Что социальный слой делает:**
- Показывает пользователю его χ_Ze (запросы в `BioSense/biosense-backend`)
- Логирует Ze·Guide AI conversations (disclaimer перед каждым ответом, GDPR-compliant)
- Хранит profile, posts, study consents
- Анти-фрод (DOI Crossref verification)

**Что социальный слой НЕ делает:** не вычисляет χ_Ze, не делает CHSH optimisation, не выводит v*. Это всё в `BioSense/biosense-simulator` (Rust lib + axum backend).

## 8. Что меняется в коде после v5.6 article

После регенерации ядра вытекают следующие изменения для umbrella code (server/web/realtime):

### 8.1 Что НЕ меняется (большая часть кода)
- Auth flow, posts schema, studies CRUD, GDPR export — научных claim'ов не содержат, остаются как есть.
- Ze·Guide chat — disclaimer текст уже корректный (no medical advice).
- BioSense passthrough handler — просто HTTP proxy в `BioSense/biosense-backend`.

### 8.2 Что меняется (научный язык в UI)
- **Dashboard.tsx** — strings про "biological age" → требуют disclaimer "research-grade exploratory only; not medical advice; pre-registration pending".
- **Studies.tsx** — descriptions всех studies должны явно говорить "hypothesis-stage" + "pre-registered tests gave NULL results for v1; v2 multimodal is post-hoc".
- **Profile.tsx** — где показывается χ_Ze score: добавить tooltip "exploratory metric, not validated on N≥2000 pre-registered cohort yet".
- **Ze·Guide системный промпт** в `server/src/handlers/ze_guide.rs` — обновить чтобы AI отказывался делать confirmatory clinical claims; явно говорил "не валидированный биомаркер".

### 8.3 Что меняется (научные параметры в коде)
- **Migrations 003_health_factors.sql** — если хранится χ_Ze threshold для exacerbation alerts: обновить документацию что threshold (0.55) — exploratory.
- **Server `routes.rs`** — добавить endpoint `/api/disclosures/v5_changes` который возвращает changelog (NULL retraction, M4 update, threat model fix) — для прозрачности перед users.

### 8.4 Что не реализовано (но было обещано в article)
- Swept-v* falsification protocol на All-of-Us → требует data access (DUA pending).
- Cell-DT v4.0 для full Sobol decomposition → отдельный subproject (CDATA область).
- FCLC v14 malicious-secure migration → planned Q1 2027.

## 9. Иерархия authority при конфликте между файлами

При расхождении между документами:

1. `LongevityCommon/CONCEPT.md` (этот файл) — для cross-cutting вопросов: статусы подпроектов, falsifiability, threat model, scoping.
2. `<subproject>/CONCEPT.md` — для математики и кода конкретного подпроекта.
3. `<subproject>/THEORY.md` — для формальных derivations внутри подпроекта.
4. Article (`~/Desktop/LongevityCommon.md`) — для полной narrative версии (научное изложение, post-revision).
5. Code — должен следовать за `CONCEPT.md` своего уровня.

Если конфликт: обновляется младший уровень, не старший. Если CONCEPT нужно изменить — это бумает версию (v5.6 → v5.7) и тянет обновления вниз.

## 10. Versioning

Текущая CONCEPT version: **v5.6** (соответствует article v5.6 по состоянию 2026-04-28 после iterations 1-3 + meta-review fixes).

**article_md5: 506f87aa9d758181bebf5a66016bd028** (`~/Desktop/LongevityCommon.md`, pinned 2026-04-28). Используется `scripts/regen_umbrella_core_from_article.sh` для drift detection — при mismatch скрипт архивирует core .md и печатает checklist для regenerate.

Прошлые версии в `_archive/v_pre_2026-04-28/` (CONCEPT.md, DESIGN.md, THEORY.md, EVIDENCE.md, OPEN_PROBLEMS.md).

Когда article обновляется → md5 mismatch → bump CONCEPT version → derived files (THEORY, DESIGN, PARAMETERS, MAP, EVIDENCE, OPEN_PROBLEMS, STATE) автоматически out-of-date до пересборки. Запустить `bash scripts/regen_umbrella_core_from_article.sh`.

## Risk matrix

| Risk description | Probability | Impact | Mitigation strategy |
|------------------|-------------|--------|---------------------|
| Failure to recruit N≥2000 cohort | Medium | High | Parallel recruitment via UK Biobank DUA and All-of-Us Researcher Workbench; fallback to meta-analysis of existing cohorts |
| CDATA remains inconclusive | Medium | Medium | Defer CDATA-dependent claims; focus on BioSense and Ze theory with independent validation |
| FCLC v14 malicious-secure migration delayed | High | Medium | Maintain semi-honest v13.4 as fallback; document GDPR limitations explicitly |
| PR/adverse event from unvalidated biomarker claims | Low | High | Add explicit disclaimers on all outputs; no clinical recommendations without regulatory approval |
| Loss of key personnel (e.g., Lezhava) | Low | High | Cross-train team members; document all procedures in `DESIGN.md` |

## Limitations

1. All empirical results are exploratory and may reflect p-hacking (Ioannidis 2005).
2. Pilot sample sizes are small (N≤2222) and underpowered for confirmatory inference.
3. Key publications (MCOA, Ze, BioSense) are not peer-reviewed.
4. The CDATA counter is inconclusive.
5. FCLC is semi-honest only, not yet GDPR-compliant for medical data.
6. No prospective validation cohort exists.

## Evidence base & meta-analysis

The framework's core claims are supported by the following verified sources:

1. **Aging as Total Chronic Disease**: ICD-11 codes XT9T (2018) and MG2A (2025) from WHO, defining aging-related decline as a diagnosable condition.
2. **Hallmarks of aging**: López-Otín et al. (2013, PMID 23746838; 2023, PMID 36599349) — two comprehensive reviews, though not systematic meta-analyses.
3. **Epigenetic clocks**: Horvath (2013, PMID 24138928), Levine et al. (2018, PMID 29676998), Lu et al. (2019, PMID 30669119; 2022, PMID 36516495), Belsky et al. (2022, PMID 35029144), Ying et al. (2024, PMID 38243142), Argentieri et al. (2024, PMID 39117878) — multiple independent clocks, but no systematic review comparing their concordance.
4. **Free-energy principle**: Friston (2010, PMID 20068583) — foundational, but its application to aging remains unvalidated.
5. **Predictive information theory**: Bialek/Nemenman/Tishby (2001, DOI 10.1162/089976601753195969) — theoretical basis for I(Z) in Ze ansatz.
6. **Physical clocks analogy**: Burgholzer (2015, arXiv 1502.00214), Pearson et al. (2021, DOI 10.1103/PhysRevX.11.021029) — used as analogy only; no biological derivation exists.
7. **CDATA hypothesis**: Tkemaladze (2023, PMID 36583780; 2005, PMID 15886028), Madarampalli (2015, PMID 26213385) — three independent sources, but no systematic review or meta-analysis of centriolar PTM in aging.

**No Cochrane or PRISMA systematic review** has been conducted for any claim. Contradicting results (e.g., ABL-2 paradox in CDATA, poor clock concordance) are acknowledged but not resolved. A state-of-the-art review of competing aging theories (e.g., damage accumulation, programmed aging, information theory) is absent from the current documentation.

## Methodology depth

The current methodology is at the concept-outline stage and lacks replication-ready detail:

1. **Protocol**: Only a swept-v* protocol outline exists (cohort, N, power analysis, primary outcome, falsification criterion — not specified). No step-by-step protocol for any subproject is provided.
2. **Statistical Analysis Plan (SAP)**: Not defined. Primary endpoint (partial r² for all-cause mortality) is stated, but secondary endpoints, multiple-comparison corrections (e.g., Bonferroni, FDR), and missing-data handling (e.g., multiple imputation, complete-case analysis) are not specified.
3. **Controls**: No explicit control group or covariate set beyond age+sex is defined for the confirmatory test. Potential confounders (e.g., smoking, BMI, comorbidities) are not addressed.
4. **Replication strategy**: Not described. No plan for split-sample validation, k-fold cross-validation, or independent dataset replication. The current exploratory AUC (0.81 on All-of-Us N=2222) has not been replicated.
5. **Blinding/Randomisation**: Not applicable to observational cohort design; if a prospective trial is planned, blinding and randomisation procedures are absent.

**Status**: Pre-replication. A full SAP and protocol must be developed before any confirmatory analysis.

## Reproducibility & open science

Reproducibility and open science practices are currently incomplete:

1. **Code repository**: Partial code is available (Rust backend for Ze and BioSense, Phoenix web frontend). No public repository URL is provided; a promise to release upon acceptance is absent.
2. **Data deposit plan**: Data sources (All-of-Us, Cuban EEG, Dortmund Vital, MPI-LEMON) are third-party and not deposited by the authors. No plan for depositing derived data (e.g., χ_Ze values, CDATA Sobol results) on Zenodo, Dryad, OSF, or figshare exists.
3. **Pre-registration**: A placeholder OSF link (`https://osf.io/TBD`) is provided, but no actual pre-registration has been filed. The confirmatory test is planned for 2026-12-31.
4. **Materials transparency**: No protocol.io or equivalent protocol repository is used. Software dependencies (e.g., requirements.txt, Cargo.toml) are not publicly listed.

**Status**: Pre-open-science. Code, data, and materials must be deposited and pre-registration completed before any confirmatory claims can be evaluated.
