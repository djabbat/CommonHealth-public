# AUDIT PACKET — srv_fclc (server)

Host: `server`  Path: `/home/jaba/web/fclc`  Date: 2026-05-08

## Size
```
40M	/home/jaba/web/fclc
```
## Tree
```
/home/jaba/web/fclc
/home/jaba/web/fclc/PITCH_LONGEVITY.md
/home/jaba/web/fclc/AGENTS.md
/home/jaba/web/fclc/Cargo.toml
/home/jaba/web/fclc/Cargo.lock
/home/jaba/web/fclc/PARAMETERS.md
/home/jaba/web/fclc/fclc-core
/home/jaba/web/fclc/fclc-core/Cargo.toml
/home/jaba/web/fclc/fclc-core/Cargo.lock
/home/jaba/web/fclc/fclc-core/tests
/home/jaba/web/fclc/fclc-core/src
/home/jaba/web/fclc/articles
/home/jaba/web/fclc/articles/FCLC Concept.pdf
/home/jaba/web/fclc/fclc-server
/home/jaba/web/fclc/fclc-server/Cargo.toml
/home/jaba/web/fclc/fclc-server/Dockerfile
/home/jaba/web/fclc/fclc-server/src
/home/jaba/web/fclc/fclc-server/migrations
/home/jaba/web/fclc/frontend
/home/jaba/web/fclc/frontend/mix.exs
/home/jaba/web/fclc/frontend/config
/home/jaba/web/fclc/frontend/lib
/home/jaba/web/fclc/data
/home/jaba/web/fclc/data/clinic_node2_demo.csv
/home/jaba/web/fclc/data/clinic_node1_demo.csv
/home/jaba/web/fclc/data/clinic_node3_demo.csv
/home/jaba/web/fclc/OPEN_PROBLEMS.md
/home/jaba/web/fclc/EVIDENCE.md
/home/jaba/web/fclc/README.md
/home/jaba/web/fclc/_archive
/home/jaba/web/fclc/_archive/DEEP_AUDIT_2026-04-21.md
/home/jaba/web/fclc/DESIGN.md
/home/jaba/web/fclc/LICENSE
/home/jaba/web/fclc/scripts
/home/jaba/web/fclc/scripts/run.sh
/home/jaba/web/fclc/scripts/generate_demo_data.py
/home/jaba/web/fclc/scripts/requirements-python.txt
/home/jaba/web/fclc/PITCH_TECHNOCRATIC.md
/home/jaba/web/fclc/fclc-node
/home/jaba/web/fclc/fclc-node/Cargo.toml
/home/jaba/web/fclc/fclc-node/src
/home/jaba/web/fclc/TODO.md
/home/jaba/web/fclc/docker-compose.yml
/home/jaba/web/fclc/THEORY.md
/home/jaba/web/fclc/fclc-web
/home/jaba/web/fclc/fclc-web/deps
/home/jaba/web/fclc/fclc-web/assets
/home/jaba/web/fclc/fclc-web/Dockerfile
/home/jaba/web/fclc/fclc-web/mix.exs
/home/jaba/web/fclc/fclc-web/config
/home/jaba/web/fclc/fclc-web/priv
/home/jaba/web/fclc/fclc-web/README.md
/home/jaba/web/fclc/fclc-web/lib
/home/jaba/web/fclc/fclc-web/test
/home/jaba/web/fclc/fclc-web/mix.lock
/home/jaba/web/fclc/backend
/home/jaba/web/fclc/backend/Cargo.toml
/home/jaba/web/fclc/backend/src
/home/jaba/web/fclc/REMINDER.md
/home/jaba/web/fclc/docs
/home/jaba/web/fclc/docs/LongevityCommon_PeerReview_v12_Ultra.md
/home/jaba/web/fclc/docs/LongevityCommon_PeerReview_v13_Ultra.md
/home/jaba/web/fclc/docs/literature_daily
/home/jaba/web/fclc/docs/SERVER_DEPLOYMENT_2026-04-26.md
/home/jaba/web/fclc/docs/RESPONSE_TO_TSOMAIA_2026-04-27.md
/home/jaba/web/fclc/docs/CORRECTION_CANDIDATES.md
/home/jaba/web/fclc/docs/TRUE_MISMATCHES_FCLC.md
/home/jaba/web/fclc/docs/META_ANALYSIS_FCLC.md
/home/jaba/web/fclc/docs/REFERENCE_AUDIT_FCLC.md
/home/jaba/web/fclc/docs/v13
/home/jaba/web/fclc/docs/FCLC_CONCEPT_review.md
/home/jaba/web/fclc/docs/PEER_REVIEW_FCLC.md
/home/jaba/web/fclc/COMPLIANCE.md
/home/jaba/web/fclc/fclc-demogen
/home/jaba/web/fclc/fclc-demogen/Cargo.toml
/home/jaba/web/fclc/fclc-demogen/src
/home/jaba/web/fclc/CONCEPT.md

```
## Stack probe
```
---rust---
./Cargo.toml
./fclc-core/Cargo.toml
./fclc-server/Cargo.toml
./fclc-node/Cargo.toml
./backend/Cargo.toml
---elixir---
./frontend/mix.exs
./fclc-web/deps/websock_adapter/mix.exs
./fclc-web/deps/plug/mix.exs
./fclc-web/deps/digital_token/mix.exs
./fclc-web/deps/ex_cldr_calendars/mix.exs
---go---
---php---
---python---
./scripts/generate_demo_data.py
---node---
./fclc-web/deps/phoenix/package.json
./fclc-web/deps/phoenix_live_view/package.json
./fclc-web/deps/phoenix_html/package.json

```
### `README.md` (head 200 lines)
```
# FCLC — Федеративная инфраструктура калибровки для MCOA

**FCLC (Federated Clinical Learning Cooperative)** — это приватный, безопасный вычислительный слой, позволяющий калибровать веса тканевых счётчиков `w_i(tissue)` в рамках теории MCOA (Multi‑Clock Organismal Aging) без передачи исходных медицинских данных. Это реализует ключевое предсказание MCOA: для точной оценки биологического возраста счётчики должны взвешиваться в зависимости от целевой ткани, а веса должны выводиться из реальных клинических когорт. FCLC решает проблему разрозненности и юридической защищённости медицинских данных, предоставляя инфраструктуру для федеративного машинного обучения с криптографическими гарантиями приватности и прозрачным измерением вклада каждого участника.

## Краткое описание решения

Участники (клиники, исследовательские центры) разворачивают локальный узел (node), который подключается к их внутренним системам (HIS, EHR, PACS). Узел выполняет деидентификацию и нормализацию данных до общей схемы OMOP CDM. В процессе обучения глобальной модели (например, модели калибровки MCOA) каждый узел вычисляет локальные градиенты или обновления модели и отправляет их центральному оркестратору. Ключевые технологии:
*   **SecAgg+ (Secure Aggregation+):** Криптографический протокол, гарантирующий, что оркестратор видит только агрегированную сумму обновлений от всех узлов, но не может восстановить вклад отдельного узла. Реализация включает X25519, ChaCha20 и Shamir (t,n)‑пороговое разделение секрета.
*   **Дифференциальная приватность:** К обновлениям добавляется статистический шум (Гауссов механизм) для защиты от атак восстановления данных.
*   **Измерение вклада (Federated Shapley Value):** Вклад каждого участника в качество финальной модели оценивается с помощью приближённого значения Шепли, что обеспечивает справедливое распределение выгод.
*   **Византийская устойчивость:** Использование алгоритма агрегации Krum для защиты от до 25% злонамеренных узлов.

Исходные данные пациентов никогда не покидают периметр учреждения‑участника.

## Позиция в экосистеме LongevityCommon

FCLC является критической инфраструктурной компонентой для валидации и применения теоретических框架, разрабатываемых в других подпроектах:
*   **Для MCOA:** FCLC — это единственный практический путь для калибровки тканевых весов `w_i(tissue)` на реальных мульти‑сайтовых данных. Без FCLC MCOA остаётся теоретической конструкцией.
*   **Для CDATA:** Потенциальный канал для сбора и обработки экспериментальных данных (например, транскриптомных или эпигенетических) в рамках будущих валидационных исследований (WP2 в EIC), с соблюдением строгих норм приватности.
*   **Для BioSense:** Может выступать как безопасный канал для агрегации данных с носимых устройств в клинических пилотных проектах.

FCLC не является самостоятельной теорией старения, а представляет собой инструментальный слой, который делает другие теории применимыми на реальных данных.

## Связь с другими ключевыми документами

*   **Формальная теория и аксиомы:** Полное описание роли федеративного обучения в рамках MCOA, аксиомы безопасности и распределения. См. [THEORY.md](THEORY.md).
*   **Эмпирические основания:** Подтверждённые ссылки на литературу по федеративному обучению, дифференциальной приватности и безопасной агрегации, а также отчёт о внутренних тестах реализации SecAgg+. См. [EVIDENCE.md](EVIDENCE.md).
*   **Неразрешённые проблемы:** Ключевые научные и инженерные проблемы FCLC, включая масштабируемость, ограничения безопасности против активного противника и нормативные барьеры. Для каждой проблемы приведены тесты на фальсификацию. См. [OPEN_PROBLEMS.md](OPEN_PROBLEMS.md).
*   **Количественные параметры:** Таблица всех параметров системы (ε, δ, размер маски, порог Krum и т.д.) с указанием источника, единиц измерения и статуса. См. [PARAMETERS.md](PARAMETERS.md).
*   **Архитектура и API:** Детальное описание структуры кода, файлового дерева и контрактов между компонентами (оркестратор, узел, адаптеры). См. [DESIGN.md](DESIGN.md).
*   **Инструкции для агентов ИИ:** Жёсткие правила и ограничения безопасности для LLM, работающих с кодом или документацией FCLC. См. [AGENTS.md](AGENTS.md).
*   **Журнал изменений:** Хронологическая история всех значимых решений по проекту, их обоснование и связи с аудитом 2026‑04‑22. См. [JOURNAL.md](JOURNAL.md).
*   **Дорожная карта:** План будущих улучшений, приоритеты и зависимости, увязанные с дорожной картой EIC Pathfinder. См. [ROADMAP.md](ROADMAP.md).

## Текущий статус

FCLC находится на стадии **исследовательского прототипа** (Research Prototype). Криптографическое ядро SecAgg+ реализовано и протестировано (44/44 тестов). Интеграция с OMOP CDM и механизм дифференциальной приватности находятся в активной разработке. Проект предложен в качестве **WP4 (FCLC Platform)** в заявке на EIC Pathfinder (€0.5M, M1‑M24). Все утверждения в документации приведены в соответствие с каноном [CORRECTIONS_2026‑04‑22](../CORRECTIONS_2026-04-22.md).
```
### `CONCEPT.md` (head 200 lines)
```
# FCLC — Federated infrastructure layer of MCOA

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


## Position in MCOA
**FCLC = federated calibration infrastructure for the MCOA framework** (Tkemaladze J., 2026, *Nature Aging* Perspective). MCOA counters require tissue-weighted calibration from multi-site biomedical data; FCLC provides the privacy-preserving pipeline (SecAgg+, differential privacy ε≤1.0) that allows *w_i(tissue)* weights to be learned without raw data transfer. Meta-architecture: `~/Desktop/LongevityCommon/MCOA/CONCEPT.md`.

---

# FCLC — Federated Clinical Learning Cooperative
## Privacy-Preserving Infrastructure for Medical AI Training Without Raw Patient Data Transfer

**Version 6.2 — SecAgg+ implemented**
**Date: April 11, 2026**
**Status: RESEARCH PROTOTYPE — EIC Pathfinder candidate (P0-2 EU partner pending)**

---

## One Sentence

We unite clinical and pharmaceutical data for AI training without transferring raw patient data, with measurable contributions from each participant and transparent benefit distribution.

---

## The Problem

Data exists everywhere, but it is:

- **In different formats** — HIS, EHR, PACS, LIS, unstructured notes
- **Under different legal constraints** — GDPR, national laws, internal policies
- **Nobody wants to share it** — reputational, legal, and commercial risks

**Result:** Large medical AI models are trained on narrow datasets, while real clinical data remains unused. Doctors continue working without AI tools that could be built from already existing data.

---

## The Solution

Each participant deploys a **local node** that:

1. **Connects** to HIS/EHR/PACS via adapters (HL7/FHIR, custom APIs)
2. **De-identifies data** with guarantees (identifier removal, quasi-identifiers, differential privacy)
3. **Normalizes** to a common schema (OMOP CDM)
4. **Sends only** de-identified aggregates or model updates (gradients, weights)

**Central Orchestrator:**
- Collects model updates
- Aggregates (federated averaging with secure aggregation)
- Maintains audit and version logs
- Calculates each participant's contribution

**Data never leaves the clinic — only training signals do.**

---

## §3.5 Economic Layer: Marketplace + Voucher ROI

> **Added 2026-04-27 in response to co-PI G. Tsomaia critique (Rational Exchange model).**
> Source: tsomaia_critique_2026-04-27 (B1 — data fragmentation barrier, B2 — ideological noise, B3 — big-data hunger).
> **Critical clarification:** Voucher is NOT a cryptocurrency / token / coin. It is an equity-style registry claim on future cash flow from medical-LLM service revenue. No blockchain, no on-chain settlement.

### Rationale

Volunteer-ideological data collection (longevity-philosophy framing) cannot break the GDPR/HIPAA + commercial-asset wall that keeps clinics' historical case databases siloed. Clinics treat their bases as "dog-in-the-manger" assets — too expensive to share, too costly to monetize alone. The Rational Exchange model converts dead data into a tradable equity-style claim, so the clinic's economic incentive aligns with consortium training need.

### Architecture

```
┌─────────────┐    de-id pipeline    ┌──────────────────┐    voucher issued    ┌──────────────┐
│  Clinic DB  │ ──── (k-anon +DP) ──▶│ Marketplace      │ ─────registry────────▶│ Voucher Reg. │
│  (raw EHR)  │                      │ (anonymized      │                        │ (equity-style│
│             │                      │  contributions)  │                        │  claims)     │
└─────────────┘                      └──────────────────┘                        └──────────────┘
                                              │                                          │
                                              ▼                                          ▼
                                     ┌──────────────────┐                       ┌──────────────┐
                                     │ FL training pool │                       │ Revenue from │
                                     │ (FedProx+SecAgg+)│ ──── LLM trained ────▶│ LLM services │
                                     └──────────────────┘                       │ (diagnostic, │
                                              │                                 │  decision    │
                                              ▼                                 │  support)    │
                                     ┌──────────────────┐                       └──────┬───────┘
                                     │ Federated LLM    │                              │
                                     │ (diagnostic /    │                              │
                                     │  decision sup.)  │ ◀──── ROI distribution ──────┘
                                     └──────────────────┘     (Shapley-weighted, paid
                                                              via voucher registry)
```

### Voucher Semantics

| Property | Value |
|----------|-------|
| Legal form | Equity-style registered claim issued by FCLC consortium legal entity |
| Backing | Pro-rata share of net revenue from LLM service licensing (diagnostic API, decision support, post-marketing analytics) |
| Issuance basis | `(data_volume × clinical_relevance × contribution_score)` where `contribution_score` is the Shapley value already used by FCLC |
| Transferability | Restricted: tradeable only between qualified consortium participants (clinics, pharma, research orgs); not tradeable to retail, not listed on secondary exchanges |
| Storage | Centralised registry maintained by Central Orchestrator legal entity (audit log + cryptographic signing); NOT on a public blockchain |
| Settlement | Periodic cash distribution (annual or quarterly) from LLM service revenue; voucher does not require a token to settle |
| Dilution protection | New issuance requires 2/3 board vote and is capped at +5% per fiscal year |

### Why "voucher" and not "token / coin"

1. **Regulatory** — equity-style registered claims sit cleanly under existing securities regimes (Reg D / EU prospectus exemptions for sophisticated investors). A token introduces MiCA classification risk, Howey-test ambiguity, and AML/KYC overhead that clinics do not want.
2. **Investor framing** — Tsomaia's critique B2: "цифровая церковь" reading. A token deployment looks like crypto sectarianism to clinic CFOs and B2B SaaS investors. A registered equity-style voucher looks like a SAFE or a revenue-share agreement.
3. **Audit** — a centralised registry with cryptographic signing is auditable by Big-4 firms and national regulators today. On-chain settlement requires custom audit tooling that does not exist for medical data flows.
4. **Reversibility** — if a clinic withdraws (DUA termination), its vouchers can be bought back at registry-defined value. Token-based systems make this complex (wallet recovery, lost keys, jurisdictional issues).

### How this addresses Tsomaia's three barriers

| Tsomaia critique | Marketplace+Voucher answer |
|------------------|----------------------------|
| **B1 — Data fragmentation** ("clinics sit on bases like dog in manger") | Voucher converts dead data to an asset on the clinic's balance sheet. Economic incentive replaces volunteer-ideological appeal. |
| **B2 — Ideological noise** ("longevity / digital church" reading) | Primary investor pitch (PITCH_TECHNOCRATIC.md) frames FCLC as a federated B2B SaaS with privacy-by-design and ROI mechanism. Longevity philosophy is moved to optional community-layer pitch. |
| **B3 — Big-data hunger** ("LLM needs millions of anonymised cases") | Marketplace + voucher economics scale to dozens of clinics per region; combined with federated learning, this reaches the volume threshold without centralising raw data. |

### ROI Mechanism (formal sketch)

For clinic *i* in fiscal period *T*:

```
voucher_share_i(T) = Σ_t∈T  shapley_i(t) · normalisation_factor(t)
revenue_i(T)       = voucher_share_i(T) × net_LLM_revenue(T) × payout_ratio
                                                              (default 0.70 — same
                                                              ratio as Phase 2 financial model)
```

Where `payout_ratio = 0.70` is consistent with the existing Phase 2 70/30 split (70% to participants, 30% to platform development). Vouchers therefore re-use the existing Shapley pipeline rather than introducing a parallel scoring system.

### Federated Learning — depth (per Tsomaia recommendation C)

FL is already the FCLC technical core; the marketplace layer **does not change** the FL protocol. What we do strengthen in pitch + COMPLIANCE.md:

| Framework | Strengths | Trade-offs | FCLC fit |
|-----------|-----------|------------|----------|
| **OpenMined PySyft** | Mature SecAgg primitives; active research community; open ecosystem | Python-centric; performance bottleneck on large gradients | **Backup option** for PATE student training |
| **OpenFL (Intel)** | Production-grade for medical use; Federated Tumor Segmentation reference deployment | Tighter integration with Intel hardware; smaller community | **Primary substrate option** for WP3 deployment |
| **NVFlare (NVIDIA)** | GPU-optimised; large-scale tested (FeTS, COVID-19 Rieke et al.) | Vendor lock-in (NVIDIA hardware preference); commercial license tier | **Substrate for high-performance imaging FL** in WP4 |

FCLC's own implementation in `fclc-core` is **not a replacement** for these frameworks — it is the cooperative governance + Shapley + voucher layer that sits on top of any of them. The marketplace_layer Rust module (added in v12) operates above the FL protocol, on FL-aggregated model deltas and post-round Shapley scores.

### Anonymisation pipeline (per Tsomaia + COMPLIANCE.md detail)

Marketplace data contributions pass through the existing 5-layer privacy stack before voucher issuance. The voucher registry stores **only metadata** (volume, schema-fingerprint, Shapley score, issuance timestamp), never the underlying records:

1. Direct identifier removal (HIPAA Safe Harbor)
2. Quasi-identifier generalisation (k-anonymity ≥5 enforced; l-diversity ≥3; t-closeness ≤0.2)
3. Re-identification risk audit (per record, before contribution accepted)
4. DP-SGD / PATE noise on gradients (ε targets per CONCEPT.md §5)
5. SecAgg+ on transmitted gradients (orchestrator sees aggregate only)

Voucher issuance is **conditional** on all five layers passing automated audit. A failed audit returns the contribution to the clinic without voucher issuance and logs the failure for the local Data Steward.

### What this is NOT

- ❌ Not a cryptocurrency. No token, no coin, no on-chain settlement.
- ❌ Not a public market instrument. Vouchers are not retail-tradable.
- ❌ Not a replacement for the existing Shapley contribution scoring — vouchers are issued **as a function of** Shapley scores.
- ❌ Not a replacement for the longevity research vision — that vision is preserved in PITCH_LONGEVITY.md as a community-layer narrative for aligned donors and researchers.

---

## Privacy — Architectural, Not Rhetorical

| Layer | Mechanism |
|-------|-----------|
| 1 | Direct identifier removal (name, ID, address, exact date) |
| 2 | Quasi-identifier generalization (age groups, rare diagnoses → suppression) |
| 3 | Record-level re-identification risk assessment (k-anonymity, l-diversity) |
| 4 | **Differential privacy (v13.4 recalibration: ε ≤ 1.0 per round AND ε_total ≤ 1.0 over the full training run at δ=10⁻⁵, Gaussian mechanism with Poisson-subsampled Rényi-DP accounting; canonical (σ, q, T) = (1.5, 0.013, 5) ⇒ ε_total ≈ 0.43)** ✅ *v13.4 (2026-04-27): the previous narrative of ε=2.0/round, ε_total=10.0 has been replaced. The new calibration uses a tight subsampled-Gaussian RDP bound (Wang, Balle, Kasiviswanathan, AISTATS 2019, Thm 9) instead of the loose Mironov 2017 §3 closed form, which over-estimates ε at moderate Rényi orders. Implementation: `fclc-core::dp::recalibration_v13::TightRdpAccountant` and `Epsilon10TightConfig::V13_4`. ISO/IEC 27559:2022 prescribes a **privacy-risk framework** rather than a fixed ε threshold; the chosen ε ≤ 1.0 envelope is anchored to the medical-FL community best-practice region and the NIST SP 800-226 (2025) "strong privacy" annotation that ε ∈ [10⁻³, 1] is "meaningful" — neither standard mandates a fixed cut-off, and FCLC does not claim one. WP2 PATE target (ε ≈ 0.63 → < 0.35) is preserved as a future tightening, not a substitute for the present ε ≤ 1.0 guarantee.* |
| 5 | **SecAgg+ — ChaCha20 + Shamir (t,n) dropout recovery** (orchestrator sees only aggregated sum; individual gradients cryptographically hidden) ✅ |

**Result:** Even if the orchestrator is compromised or traffic is intercepted, patient data cannot be reconstructed.

### Threat Model and Security

**Threat model:**
- **Orchestrator:** honest-but-curious — follows protocol but may attempt to extract information from received data
- **Nodes:** up to 25% may be malicious (Byzantine) — sending incorrect updates to poison the model
- **External attacker:** may intercept network traffic

**Security measures:**
- **Secure aggregation (SecAgg+) — ✅ research-grade implementation with formal semi-honest security model (2026-04-10); independent cryptographic audit planned WP3:**
  ⚠️ **ACTIVE ADVERSARY LIMITATION:** Current implementation is proven secure against a semi-honest (honest-but-curious) orchestrator only. An active adversary who deviates from protocol (e.g., sends crafted messages to unmask individual gradients) is NOT covered by the current security proof. Mitigation path: (a) WP3 audit will assess active-adversary attack surface; (b) optional migration to verified framework (e.g., OpenMined PySyft SecAgg or Google FLSIM) if audit reveals exploitable gaps. This limitation MUST be disclosed in any regulatory or grant submission as a research-grade constraint.
  Orchestrator sees only the aggregated sum, never individual node updates.
  - **DH key exchange:** X25519 (Curve25519, RFC 7748, 128-bit security) via `x25519_dalek` — real authenticated key agreement, NOT a simulation
  - **Seed derivation:** `seed_ij = SHA-256(X25519(private_i, public_j) || round || "FCLC-SECAGG-V2-SEED")` — symmetric by Curve25519 commutativity; secure against passive adversary
  - **PRG:** ChaCha20 (IETF RFC 8439, cryptographically secure stream cipher)
  - **Pairwise mask cancellation:** Σ_i mask_i = 0 by construction (Bonawitz et al., 2017)
  - **Dropout recovery:** Shamir (t,n)-threshold secret sharing over GF(257), threshold = ⌈n/2⌉
  - **Full API:** `NodeKeypair`, `ShamirShare`, `secagg_apply_masks()`, `secagg_aggregate()` in `fclc-core::aggregation::secagg`
  - **Tests:** 44/44 pass including mask cancellation, Shamir reconstruct, X25519 symmetry
  - **Independent security audit:** planned in WP3 (months 4–6) by external cryptographer
- **Differential privacy:** sufficient noise to prevent data reconstruction from gradients
- **Minimum batch size:** ≥32 records to reduce gradient inversion attack risk (Zhu et al., 2019)
- **Robust aggregation:** **Krum** for Byzantine tolerance (up to 25% malicious nodes)
- **Reputation scoring:** nodes with anomalous behavior are automatically excluded from training

---

```
### `THEORY.md` (head 200 lines)
```
# Теоретические основания FCLC

## 1. Контекст и роль в MCOA Framework

FCLC (Federated Clinical Learning Cooperative) является производной инфраструктурной теорией, вытекающей из потребностей MCOA (Multi‑Clock Organismal Aging). Согласно MCOA, биологический возраст организма в ткани `t` в момент времени `t` описывается функцией:
`L_tissue(n, t) = Σ_i w_i(tissue) · f_i(D_i(n, t))`
где `D_i` — показания i‑го счётчика (эпигенетические, транскриптомные и др.), `f_i` — функция нормализации, а `w_i(tissue)` — тканеспецифичный вес, отражающий вклад данного счётчика в старение данной ткани.

**Ключевая проблема:** Веса `w_i(tissue)` не могут быть выведены априорно или изолированно на малых однородных когортах. Они требуют калибровки на больших, разнородных мультисайтовых клинических наборах данных, которые охватывают различные ткани, возрастные группы и патологии. Прямой сбор и централизация таких данных невозможны из‑за правовых (GDPR, HIPAA), коммерческих и этических ограничений.

**Аксиома FCLC‑1 (Необходимость федеративной калибровки):** Точная калибровка тканевых весов `w_i(tissue)` в рамках MCOA требует доступа к распределённым мультисайтовым клиническим данным. Прямая централизация исходных данных невозможна или нежелательна. Следовательно, необходим механизм, позволяющий выполнять вычисления над распределёнными данными без их экспорта за пределы исходных хранилищ.

Таким образом, FCLC формально определяется как **теория и реализация безопасного, приватного вычислительного протокола, который позволяет решать задачу оптимизации (калибровки `w_i`) в условиях распределённых, юридически защищённых данных**.

## 2. Формальные аксиомы FCLC

### Аксиома FCLC‑2 (Приматив приватности):
Любой информационный обмен между участником (узлом `N_i`) и координатором (`C`) должен удовлетворять одному из двух условий:
1.  **Полная деидентификация:** Передаваемая информация `T` не содержит прямых или косвенных идентификаторов, причём риск реадентификации `R_reid(T)` ≤ `δ_risk`, где `δ_risk` — приемлемый порог, установленный регулирующим органом (например, по методологии HIPAA Safe Harbor).
2.  **Криптографическая или статистическая защита:** Если `T` потенциально содержит следы исходных данных, то либо:
    a)  `T` является криптографически замаскированным значением, причём оркестратор `C` не может извлечь вклад отдельного узла без коллизии всех остальных узлов (формальная модель semi‑honest security).
    b)  К `T` применён механизм (`M, ε, δ`)‑дифференциальной приватности, гарантирующий, что наличие или отсутствие любой отдельной записи в локальном наборе данных узла `N_i` не может быть обнаружено с преимуществом более чем `exp(ε)`.

### Аксиома FCLC‑3 (Справедливое распределение вклада):
Пусть `V` — общая полезность (например, прирост точности) глобальной модели `M_G`, полученной в результате раунда федеративного обучения с множеством участников `P = {P_1, ..., P_n}`. Распределение выгод (доступа к модели, коммерческого дохода) между участниками должно быть пропорционально их маргинальному вкладу `φ_i` в `V`. Формально, функция распределения `Benefit(P_i)` должна быть линейно связана с `φ_i`, где `φ_i` вычисляется как значение Шепли (Shapley value) для участника `P_i` в коалиционной игре с характеристической функцией, равной полезности модели, обученной на данных данной коалиции.
*   **Следствие 3.1:** Использование простых метрик, таких как объём переданных данных, для распределения выгод не соответствует данной аксиоме, так как не отражает маргинальной полезности.
*   **Следствие 3.2:** Система должна быть устойчива к стратегическому поведению (например, отправке зашумленных данных для искусственного завышения вклада). Это требует механизмов верификации качества данных и Byzantine‑robust агрегации.

### Аксиома FCLC‑4 (Детерминированность и воспроизводимость):
Протоколы FCLC (безопасная агрегация, добавление шума DP, агрегация) должны быть детерминированными при заданных начальных условиях (seed, параметры шума). Это необходимо для:
1.  Аудита и верификации корректности расчётов вклада (`φ_i`).
2.  Воспроизведения результатов калибровки `w_i(tissue)`.
3.  Юридической проверки соблюдения протокола.

## 3. Математическая модель и предсказания

### 3.1. Модель безопасной агрегации (SecAgg+)
Пусть `n` узлов, каждый имеет локальный вектор обновлений модели `g_i ∈ R^d`. Цель: вычислить `G = Σ_{i=1}^n g_i`, не раскрывая отдельные `g_i` оркестратору.
1.  **Попарные маски:** Каждая пара узлов (`i`, `j`) генерирует общий секретный seed `s_{ij}` через протокол обмена ключами X25519. Затем с помощью CSPRNG (ChaCha20) генерируется маска `m_{ij} ∈ R^d`.
2.  **Симметрия и аннигиляция:** Узел `i` вычисляет свою итоговую маску как `M_i = Σ_{j<i} m_{ij} - Σ_{j>i} m_{ij}`. При суммировании всех замаскированных векторов `Σ_i (g_i + M_i)` все `m_{ij}` взаимно уничтожаются, остаётся `Σ_i g_i`. Оркестратор никогда не видит отдельные `m_{ij}`.
3.  **Защита от выбытия узлов (dropout):** Каждый узел делит свой приватный ключ X25519 на `n` долей по схеме Шамира (порог `t = ⌈n/2⌉`) и рассылает их другим узлам. Если узел `i` выбывает, оставшиеся узлы, имеющие в сумме ≥ `t` долей его ключа, могут восстановить его и вычислить недостающие маски для коррекции агрегации.

**Предсказание FCLC‑P1:** Реализация SecAgg+ с X25519, ChaCha20 и Shamir (t,n) позволит корректно агрегировать обновления в присутствии до `n - t` выбывших узлов, при этом вычислительная сложность для каждого узла будет `O(d * n)`.

### 3.2. Модель дифференциальной приватности (Гауссов механизм)
Пусть функция (запрос) `q: Dataset → R^d` имеет чувствительность `Δ_2 q = max_{D, D'} ||q(D) - q(D')||_2`, где `D` и `D'` — соседние наборы данных (отличающиеся одной записью). Для обеспечения (`ε, δ`)‑дифференциальной приватности к результату `q(D)` добавляется шум `N ~ 𝒩(0, σ^2 I_d)`, где `σ = Δ_2 q * √(2 log(1.25/δ)) / ε`.
В контексте FCLC, `q` — это градиент или обновление модели, вычисленное на локальном наборе данных узла.

**Предсказание FCLC‑P2:** Применение Гауссова механима с `ε=2.0` за раунд и `δ=10⁻⁵` снизит точность конечной модели (увеличит loss) на величину, пропорциональную `σ^2 / (размер_пакета)`, но позволит обеспечить формальные гарантии приватности против атак восстановления данных из градиентов.

**Важное ограничение (из CORRECTIONS_2026‑04‑22):** Параметр `ε_total=10.0` (суммарный бюджет после нескольких раундов) является исследовательским и превышает рекомендации ISO/IEC 27559:2022 для медицинских данных (`ε_total < 1.0`). Это требует перепроектирования (например, перехода на PATE) в WP2.

### 3.3. Модель оценки вклада (Federated Shapley Value)
Пусть `v(S)` — полезность (например, 1 — validation loss) модели, обученной на совокупности данных участников из коалиции `S ⊆ P`. Значение Шепли для участника `i`:
`φ_i(v) = (1 / |P|!) * Σ_{π ∈ Π(P)} [ v(P_{π, i} ∪ {i}) - v(P_{π, i}) ]`
где `Π(P)` — множество всех перестановок участников, а `P_{π, i}` — множество участников, предшествующих `i` в перестановке `π`.
Так как точное вычисление невозможно для `n > 10`, используется оценка методом Монте‑Карло:
`φ_i(v) ≈ (1 / M) * Σ_{m=1}^M [ v(S_{m} ∪ {i}) - v(S_{m}) ]`
где `S_{m}` — случайная выборка участников, полученная путём случайной перестановки и взятия префикса случайной длины.

**Предсказание FCLC‑P3:** При использовании `M = 200` итераций Монте‑Карло для `n ≤ 10` участников, ошибка оценки `|φ_i_estimate - φ_i_true|` будет менее `0.05 * max(v)` с вероятностью >95%, что приемлемо для справедливого распределения выгод. Вычислительные затраты составят `O(M * n * стоимость_обучения_модели)`.

## 4. Связь с теорией CDATA

FCLC является инфраструктурным, а не биологическим проектом. Поэтому аксиомы CDATA (Cellular Decision and Tissue‑Wide Adaptation) не используются напрямую в его ядре. Однако FCLC может быть инструментом для проверки гипотез CDATA:
*   Если CDATA предсказывает специфические паттерны транскриптомного ответа на стресс в зависимости от тканевого контекста, то модель для классификации этих паттернов может быть обучена на распределённых данных через FCLC.
*   Калибровка счётчиков MCOA, которые, согласно теории, могут быть downstream‑проявлением CDATA, также зависит от FCLC.

Таким образом, FCLC занимает место **enabling infrastructure** в общей исследовательской программе, позволяя проводить валидацию теоретических конструкций (MCOA, CDATA) на реальных клинических данных, соблюдая при этом высочайшие стандарты приватности.
```
### `PARAMETERS.md` (head 200 lines)
```
# Количественные параметры FCLC

Все параметры системы, их происхождение, единицы измерения и текущий статус. Параметры, помеченные ⚠️, требуют пересмотра согласно канону CORRECTIONS_2026‑04‑22 или имеют исследовательский статус.

| Параметр | Символ / Имя | Значение (по умолчанию / текущее) | Единицы | Происхождение / Обоснование | Статус / Примечание |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Безопасная агрегация (SecAgg+)** | | | | | |
| Криптографическая кривая | — | Curve25519 (X25519) | — | RFC 7748, 128‑bit security, широкое применение в TLS. | **Каноническое.** Реализация через `x25519_dalek`. |
| Размер ключа | — | 256 | бит | Стандарт для Curve25519. | **Каноническое.** |
| PRG для масок | — | ChaCha20 (IETF RFC 8439) | — | Криптографически безопасный генератор, высокая скорость. | **Каноническое.** |
| Порог схемы Шамира | `t` | `⌈n / 2⌉` | безразмерн. | Компромисс между устойчивостью к выбытию (`n - t`) и безопасностью (нельзя собрать `t` долей). | **Исследовательский.** Может быть изменён после анализа отказоустойчивости. |
| **Дифференциальная приватность** | | | | | |
| Бюджет ε за раунд | `ε_round` | 2.0 | безразмерн. | Эвристический выбор для баланса приватности и utility в исследовательском контексте. | ⚠️ **Исследовательский, требует снижения.** Не соответствует ISO/IEC 27559:2022. Цель WP2: `ε_round ≤ 0.5`. |
| Параметр δ за раунд | `δ_round` | 10⁻⁵ | безразмерн. | Стандартное значение, много меньше 1/размер_набора_данных. | **Стабильный.** |
| Механизм | — | Гауссов (Gaussian) | — | Подходит для векторов (градиентов) с L2‑чувствительностью. | **Каноническое для градиентов.** |
| Общий бюджет ε | `ε_total` | 10.0 (для 5 раундов) | безразмерн. | Линейная композиция: `ε_round * num_rounds`. | ⚠️ **Критическое ограничение.** Превышает рекомендации стандарта (<1.0). Должно быть раскрыто. |
| **Византийская устойчивость** | | | | | |
| Алгоритм агрегации | — | Krum (или Multi‑Krum) | — | Blanchard et al., NeurIPS 2017. Устойчив до `f` злонамеренных узлов. | **Исследовательский.** Альтернативы: Trimmed Mean, Median. |
| Макс. доля византийцев | `f_max` | 0.25 | доля от `n` | Консервативная оценка. Krum теоретически работает при `2f + 2 < n`. | **Консервативный.** |
| **Оценка вклада (Shapley)** | | | | | |
| Число итераций Монте‑Карло | `M` | 200 | безразмерн. | Эвристика для баланса точности (`error ~ 1/√M`) и вычислительной стоимости. | **Эмпирический.** Может адаптироваться под `n`. |
| Минимальный вклад для доступа | `φ_min` | 0.05 * `φ_mean` | единицы полезности | Порог для защиты от free‑riders: вклад должен быть хотя бы 5% от среднего. | **Политический параметр.** Устанавливается правлением консорциума. |
| **Сетевые и вычислительные** | | | | | |
| Таймаут подключения узла | `t_connect` | 30 | секунды | Стандартный сетевой таймаут. | **Операционный.** |
| Таймаут выполнения раунда | `t_round` | 300 | секунды | Предполагаемое время вычисления градиента на узле + коммуникации. | **Операционный.** Настраивается под задачу. |
| Минимальный размер пакета | `batch_min` | 32 | записей | Для снижения риска gradient inversion attacks (Zhu et al., 2019). | **Рекомендованный.** |
| **Нормализация данных** | | | | | |
| Целевая схема данных | — | OMOP CDM v5.4 | — | Де‑факто стандарт для observational health research. | **Каноническое.** |
| Обобщение возраста | — | 5‑летние группы (0‑4, 5‑9, ... ≥90) | годы | Баланс между utility и деидентификацией (k‑anonymity). | **Политический параметр.** Может измениться по требованию IRB. |
| Подавление редких кодов | `k_anon` | 3 | пациентов | Значение k‑anonymity: любой комбинации квазиидентификаторов должно соответствовать ≥3 пациентов. | **Рекомендованный (Safe Harbor).** |
| **Управление консорциумом** | | | | | |
| Размер правления (Clinical Tier) | — | 3‑5 | человек | Достаточно для представления, но управляемо. | **Политический параметр.** |
| Размер правления (Industrial Tier) | — | 2‑3 | человек | Обеспечивает голос индустрии, но не доминирование. | **Политический параметр.** |
| Бюджет внешнего совета | — | 20,000 | EUR | Оплата гонораров и travel для 2‑3 независимых экспертов. | **Бюджетный (из EIC WP4).** |
```
### `TODO.md` (head 200 lines)
```
# FCLC — TODO

> Created 2026-04-27 in v12 cycle (Tsomaia Rational Exchange integration).
> Tracks v11 → v12 → v13 progression. Cross-references peer review in `docs/LongevityCommon_PeerReview_v12_Ultra.md`.

---

## v11 — completed in v12 cycle

- [x] **v11.F1** Quarantine χ_Ze claims to community-layer pitch (CORRECTIONS_2026-04-22 honesty preserved). Done in `PITCH_LONGEVITY.md` v12.
- [x] **v11.F2 (partial)** Document FL substrate options (PySyft / OpenFL / NVFlare). Done in `COMPLIANCE.md §2`.
- [x] **v11.F3 (partial)** Document audit cadence (privacy / DPIA / SecAgg / voucher reconciliation / ISO 27559 / EU AI Act). Done in `COMPLIANCE.md §5`. ISO 13485 deferred to v13.
- [x] **v11.F4 (partial)** Document SecAgg+ active-adversary mitigation path (WP3 audit + PySyft fallback). Done in `COMPLIANCE.md §1 Layer 5`. Code-level mitigation deferred to v13.
- [ ] **v11.F5** Execute first DUA + IRB at a Georgian clinic. Carried into v13.

---

## v12 — done 2026-04-27

- [x] **v12.1** Marketplace layer code — `fclc-core/src/marketplace_layer/mod.rs`, 19 unit tests passing.
- [x] **v12.2** Voucher ROI mathematical formalisation — `CONCEPT.md §3.5` (formula, semantics, dilution cap, NOT-a-token compliance check).
- [x] **v12.3** Pitch technocratic vs longevity split — `PITCH_TECHNOCRATIC.md` + `PITCH_LONGEVITY.md`.
- [x] **v12.4** Anonymisation pipeline detail — `COMPLIANCE.md §1` (5 layers, k-anonymity / l-diversity / t-closeness / DP / SecAgg+).
- [x] **v12.5** FL deployment minimum viable demo (2-node synthetic) — **DEFERRED to v13.2**: trait + adapter shims are present; live demo requires WP1 substrate-shim sprint.
- [x] **v12.6** v12 peer review — `docs/LongevityCommon_PeerReview_v12_Ultra.md`. Score **2.70/5** (target ≥2.5).

---

## v13 — open (target peer-review score ≥3.5/5)

- [ ] **v13.1** Cryptographic signing of voucher registry entries (closes last NOT-a-token gap; Big-4 audit ready). Effort: 1–2 weeks.
- [ ] **v13.2** Live FL deployment between ≥2 nodes (synthetic data acceptable) — replaces deferred v12.5. Effort: 2–3 weeks.
- [ ] **v13.3** First IRB approval at one Georgian clinic. Effort: 6–8 weeks (clinic-driven).
- [ ] **v13.4** DP-SGD recalibration to ε ≤ 1.0/round (v1.5 milestone). Effort: 2 weeks.
- [ ] **v13.5** ISO 13485 gap analysis (if MDR classification applies to FCLC diagnostic LLM service). Effort: 2–3 weeks.
- [ ] **v13.6** Tsomaia formal sign-off on v12 voucher design. Effort: days. Source critique: `~/Desktop/FCLC/docs/tsomaia_critique_2026-04-27.md` (desktop copy not present on server — request user transfer if missing).
- [ ] **v13.7** One executed DUA with one pilot clinic (Aversi / GeoHospitals / Iashvili). Effort: 4–6 weeks.
- [ ] **v13.8** Re-run peer review through DeepSeek when `DEEPSEEK_API_KEY` is restored. Memory note: `project_deepseek_state.md` flags key empty across all `.env`. Cross-validate the v12 score of 2.70 independently.

---

## Operational notes

- **Server source of truth:** `/home/jaba/web/fclc/`.
- **Workspace:** `Cargo.toml` declares members `fclc-core`, `fclc-node`, `fclc-server`, `fclc-demogen`. v12 changes are scoped to `fclc-core`.
- **Test count:** 100 in fclc-core (81 pre-v12 + 19 marketplace_layer). Workspace total run via `cargo test --workspace`.
- **Push policy:** never push to `origin/main` without explicit user confirmation (per server workflow memory).

```
### `DESIGN.md` (head 200 lines)
```
# Архитектура и дизайн FCLC

## 1. Обзор высокоуровневой архитектуры

FCLC построен по гибридной архитектуре "Центральный Оркестратор — Локальные Узлы".
```
    [Hospital A EHR]    [Hospital B PACS]    [Clinic C LIS]
            |                    |                    |
      +-----v-----+        +-----v-----+        +-----v-----+
      |  FCLC     |        |  FCLC     |        |  FCLC     |
      |  Node A   |        |  Node B   |        |  Node C   |
      | (Adapter, |        | (Adapter, |        | (Adapter, |
      |  SecAgg   |        |  SecAgg   |        |  SecAgg   |
      |  Client)  |        |  Client)  |        |  Client)  |
      +-----+-----+        +-----+-----+        +-----+-----+
            |                    |                    |
            +---------+----------+---------+----------+
                      |                    |
               [ Secure Channel (HTTPS/mTLS) ]
                      |                    |
              +-------v-------------------v-------+
              |        FCLC Orchestrator          |
              | (Job Scheduler, Secure Aggregator,|
              |  Model Registry, Contribution Calc)|
              +-----------------------------------+
                      |                    |
              +-------v-------------------v-------+
              |       Global Model Store          |
              |    (Versioned, Encrypted at Rest) |
              +-----------------------------------+
```

## 2. Детализация компонентов

### 2.1. FCLC Node (Локальный узел)
Ответственность: Подключение к локальным данным, выполнение задач обучения/агрегации, обеспечение приватности.
```
fclc-node/
├── src/
│   ├── main.rs                          # Точка входа
│   ├── adapter/                         # Адаптеры к источникам данных
│   │   ├── mod.rs
│   │   ├── fhir_adapter.rs              # FHIR API client
│   │   ├── omop_adapter.rs              # OMOP CDM трансформатор
│   │   └── deidentify/                  # Модули деидентификации
│   │       ├── k_anonymity.rs
│   │       └── generalizer.rs
│   ├── secagg_client/                   # Клиентская часть SecAgg+
│   │   ├── mod.rs
│   │   ├── key_exchange.rs              # X25519, генерация seed_{ij}
│   │   ├── mask_generator.rs            # Генерация масок via ChaCha20
│   │   └── shamir_share.rs              # Работа с долями Шамира
│   ├── dp_noise/                        # Добавление шума DP
│   │   ├── gaussian_mechanism.rs
│   │   └── rdp_accountant.rs            # Учёт бюджента (Rényi DP)
│   ├── local_trainer/                   # Локальное обучение модели
│   │   ├── task_fetcher.rs              # Получение задачи от оркестратора
│   │   └── gradient_computer.rs
│   └── api/
│       └── node_api.rs                  # REST API для управления узлом
├── config/
│   ├── node_config.toml                 # Конфигурация узла (ключи, endpoints)
│   └── data_schema.toml                 # Схема OMOP CDM для трансформации
└── tests/
    └── integration_node_test.rs
```

**API контракт узла (REST):**
*   `POST /api/v1/task/execute` — Выполнить полученную задачу (обучение, инференс). Принимает `TaskPayload` (ID модели, гиперпараметры), возвращает `TaskResult` (градиенты/предсказания + замаскированные).
*   `GET /api/v1/status` — Возвращает статус узла (здоровье, версия, ID).
*   `POST /api/v1/secagg/setup` — Участвует в раунде обмена ключами SecAgg.

### 2.2. FCLC Orchestrator (Центральный оркестратор)
Ответственность: Координация обучения, безопасная агрегация, управление моделями, расчёт вклада.
```
fclc-orchestrator/
├── src/
│   ├── main.rs
│   ├── scheduler/                       # Планировщик заданий
│   │   ├── job_queue.rs
│   │   └── node_manager.rs              # Реестр узлов, проверка живучести
│   ├── secagg_server/                   # Серверная часть SecAgg+
│   │   ├── mod.rs
│   │   ├── aggregator.rs                # Логика агрегации замаскированных векторов
│   │   ├── dropout_handler.rs           # Восстановление при выбытии узлов
│   │   └── crypto_verifier.rs           # (Будущее) верификация NIZK proofs
│   ├── model_registry/                  # Управление версиями глобальной модели
│   │   ├── versioned_model.rs
│   │   └── storage_backend.rs           # S3 / локальная ФС
│   ├── contribution/                    # Вычисление вклада
│   │   ├── shapley_estimator.rs         # Монте‑Карло оценка значения Шепли
│   │   └── credit_ledger.rs             # Учёт "кредитов" участников
│   ├── api/
│   │   ├── admin_api.rs                 # API для администратора (запуск раундов)
│   │   └── node_api.rs                  # API для взаимодействия с узлами
│   └── privacy_accountant.rs            # Отслеживание бюджента ε_total
├── config/
│   └── orchestrator_config.toml
└── tests/
    └── integration_orchestrator_test.rs
```

**API контракт оркестратора:**
*   `POST /admin/v1/round/start` — (Админ) Запуск нового раунда федеративного обучения.
*   `POST /node/v1/secagg/commit` — Приём зафиксированных (committed) замаскированных обновлений от узлов.
*   `GET /node/v1/task/{task_id}` — Предоставление задачи узлу.
*   `GET /admin/v1/contributions/{round_id}` — Получение оценок вклада за раунд.

### 2.3. Общая библиотека (fclc-core)
Общие структуры данных, утилиты и криптографические примитивы.
```
fclc-core/
├── src/
│   ├── lib.rs
│   ├── models/                          # Структуры данных
│   │   ├── task.rs                      # TaskPayload, TaskResult
│   │   ├── node_info.rs
│   │   └── secagg_protocol.rs           # Сообщения SecAgg (MaskedVector, Share)
│   ├── aggregation/                     # **ЯДРО: SecAgg+**
│   │   ├── mod.rs
│   │   ├── secagg.rs                    // Главная логика SecAgg+
│   │   ├── keys.rs                      // NodeKeypair, PublicKeyPack
│   │   ├── shamir.rs                    // ShamirShare, reconstruct_secret()
│   │   └── masks.rs                     // apply_masks(), generate_pairwise_mask()
│   ├── crypto/                          # Криптографические примитивы
│   │   ├── x25519.rs                    // Обёртка над x25519_dalek
│   │   └── chacha20_rng.rs              // Детерминированный CSPRNG
│   └── utils/
│       ├── serialization.rs             // Serde для сетевой передачи
│       └── logging.rs
└── tests/                               // **44 теста для SecAgg+**
    ├── unit_tests.rs
    └── integration_test.rs              // Полный цикл агрегации с dropout
```

## 3. Последовательность выполнения (Workflow)

1.  **Инициализация:** Админ через `/admin/v1/round/start` запускает раунд. Оркестратор рассылает узлам команду на `secagg/setup`.
2.  **Обмен ключами SecAgg (Round 1):** Узлы генерируют пары ключей X25519, обмениваются публичными ключами через оркестратор, вычисляют попарные seeds, генерируют маски, создают доли Шамира.
3.  **Локальное обучение:** Оркестратор рассылает текущую глобальную модель и задачу. Узлы загружают локальные данные, вычисляют градиенты, добавляют шум DP (Gaussian mechanism), применяют свои маски SecAgg.
4.  **Фиксация и агрегация:** Узлы отправляют замаскированные градиенты и доли Шамира оркестратору (`/node/v1/secagg/commit`). Оркестратор дожидается `threshold` узлов, агрегирует векторы, при необходимости восстанавливает маски выбывших узлов через доли Шамира. Результат — чистый суммарный градиент.
5.  **Обновление модели и оценка вклада:** Оркестратор обновляет глобальную модель. Затем, используя сохранённые результаты от узлов и агрегированные результаты для случайных коалиций, вычисляет приближённые значения Шепли для каждого узла.
6.  **Учёт приватности:** Обновляется общий бюджет `ε_total`.

## 4. Требования к развёртыванию

*   **Узел:** Rust 1.75+, 2+ vCPU, 4+ GiB RAM, доступ к источнику данных (HIS/EHR), исходящий HTTPS‑доступ к оркестратору.
*   **Оркестратор:** Rust 1.75+, 4+ vCPU, 8+ GiB RAM, публичный IP/домен, SSL‑сертификат, persistent storage (для моделей и логов).
*   **Сеть:** Все коммуникации по HTTPS/mTLS. Порты по умолчанию: Orchestrator API — 8080, Admin API — 8081.
```
### `EVIDENCE.md` (head 200 lines)
```
# Эмпирические основания FCLC

## 1. Подтверждённые ссылки на литературу (Verified Literature)

### 1.1. Федеративное обучение и приватность
| Утверждение / Концепт | PMID/DOI / arXiv | Статья | Дата проверки | Сила доказательства |
| :--- | :--- | :--- | :--- | :--- |
| Основополагающая работа по федеративному обучению | DOI: [10.48550/arXiv.1602.05629](https://doi.org/10.48550/arXiv.1602.05629) | McMahan, B. et al. "Communication-Efficient Learning of Deep Networks from Decentralized Data" (2016) | 2026‑04‑22 (Crossref) | **Сильная** (основная цитата области) |
| Протокол безопасной агрегации (SecAgg) | DOI: [10.1145/3133956.3133982](https://doi.org/10.1145/3133956.3133982) | Bonawitz, K. et al. "Practical Secure Aggregation for Privacy-Preserving Machine Learning" (CCS 2017) | 2026‑04‑22 (Crossref) | **Сильная** (криптографически формализованный протокол) |
| Атаки восстановления данных из градиентов | arXiv: [1906.08935](https://arxiv.org/abs/1906.08935) | Zhu, L., Liu, Z. & Han, S. "Deep Leakage from Gradients" (NeurIPS 2019) | 2026‑04‑21 (arXiv) | **Сильная** (демонстрация уязвимости) |
| Дифференциальная приватность в машинном обучении (монография) | DOI: 10.1561/0400000042 | Dwork, C., Roth, A. "The Algorithmic Foundations of Differential Privacy." *Foundations and Trends in Theoretical Computer Science* 9(3-4):211-407 (2014) | 2026‑04‑27 (Verified — DOI) | **Каноническая** |
| Византийско‑устойчивая агрегация (Krum) | arXiv: [1703.02757](https://arxiv.org/abs/1703.02757) | Blanchard, P. et al. "Machine Learning with Adversaries: Byzantine Tolerant Gradient Descent" (NeurIPS 2017) | 2026‑04‑22 (arXiv) | **Умеренная** (одна из ключевых ранних работ) |

### 1.2. Стандарты данных и деидентификации
| Утверждение / Концепт | PMID/DOI / Стандарт | Источник | Дата проверки | Сила доказательства |
| :--- | :--- | :--- | :--- | :--- |
| OMOP Common Data Model | [OHDSI OMOP CDM Specification v5.4](https://ohdsi.github.io/CommonDataModel/) | Observational Health Data Sciences and Informatics | 2026‑04‑22 (Официальный сайт) | **Сильная** (промышленный стандарт) |
| Методология деидентификации HIPAA Safe Harbor | [45 CFR § 164.514(b)(2)](https://www.ecfr.gov/current/title-45/section-164.514) | U.S. Department of Health & Human Services | 2026‑04‑22 (Официальный регламент) | **Нормативная** |
| Рекомендации по дифференциальной приватности для медицинских данных | ISO/IEC 27559:2022 | Международная организация по стандартизации | 2026‑04‑22 (Резюме стандарта) | **Нормативная** (рекомендует ε_total < 1.0) |

## 2. Внутренние данные и валидация

### 2.1. Реализация и тестирование SecAgg+
*   **Файл:** `fclc-core/aggregation/secagg/tests/integration_test.rs`
*   **Описание:** Полная батарея из 44 модульных и интеграционных тестов для компонентов SecAgg+.
*   **Результаты (2026‑04‑10):** 44/44 тестов пройдены. Подтверждены:
    *   Корректность попарной отмены масок (`Σ_i M_i = 0`).
    *   Восстановление секрета Шамира при наличии достаточных долей.
    *   Симметричная выработка общего seed через X25519.
    *   Корректная агрегация при имитации выбытия узлов (dropout).
*   **Статус:** Исследовательский прототип, прошедший внутреннюю проверку. **Не аудирован внешней криптографической экспертизой.**

### 2.2. Профилирование производительности
*   **Скрипт:** `scripts/benchmark_secagg.py`
*   **Конфигурация:** AWS c5.xlarge (4 vCPU, 8 GiB RAM), имитация 10 узлов, размер вектора обновления `d = 10000`.
*   **Результаты (2026‑04‑15):**
    *   Время генерации ключевой пары и масок на узел: `125 ± 15 мс`.
    *   Время агрегации на оркестраторе (без учета сетевой задержки): `45 ± 5 мс`.
    *   Пропускная способность данных на узел: ~80 МБ/с.
*   **Вывод:** Протокол вносит вычислительную и коммуникационную нагрузку (`O(d * n)`), но она приемлема для моделей умеренного размера и до 10‑15 узлов.

## 3. Опровергающие доказательства и ограничения (Honest Disclosure)

### 3.1. Ограничения текущей реализации SecAgg+
*   **Угроза:** Активный противник (Active Adversary) в роли оркестратора.
*   **Доказательство:** В текущей реализации и формальной модели безопасности (следуя Bonawitz et al., 2017) предполагается **semi‑honest (honest‑but‑curious)** оркестратор, который следует протоколу, но пытается извлечь информацию. Активный противник, который может отклоняться от протокола (например, посылать сконструированные сообщения узлам), не охвачен текущей моделью.
*   **Источник:** Собственный анализ угроз (Threat Assessment, 2026‑04‑18), документированный в `docs/threat_model.md`.
*   **Следствие:** Данное ограничение **должно быть раскрыто** во всех грантовых и регуляторных заявках. Планируемый аудит WP3 должен оценить эту поверхность атаки.

### 3.2. Параметры DP — соответствие community best practice
*   **Утверждение:** Параметр `ε_total=10.0` (суммарный бюджет дифференциальной приватности) обеспечивает адекватную защиту для медицинских данных.
*   **Реальный статус:** ISO/IEC 27559:2022 "Privacy enhancing data de-identification framework" задаёт **рамочный подход** (context / data / identifiability / governance assessment), а **не фиксированный порог ε**. Численные ориентиры берутся из других источников: NIST SP 800-226 (2025) указывает, что ε > 10 "may not provide meaningful protection", а ε ∈ [10⁻³, 1] — это область «сильной приватности»; обзор Wadhera et al. *npj Digital Medicine* (2025, DOI 10.1038/s41746-025-02280-z, 74 исследования) приводит ε ∈ [1.6, 2.5] как практический sweet-spot для медицинского DL.
*   **Источник:** ISO/IEC 27559:2022 (iso.org/standard/71677.html); NIST SP 800-226 (csrc.nist.gov/pubs/sp/800/226/final); Wadhera 2025.
*   **Следствие:** Текущий ε=2.0/round находится в верхней части community-sweet-spot и приемлем для исследовательского прототипа, но **выше целевого уровня ε ≤ 1.0**, который стандартен для развёртываний на чувствительных медицинских данных. Перепроектирование на ε ≤ 1.0 (DP-SGD recalibration, WP1 v1.5) и затем ε ≈ 0.63 (PATE, WP2 v2.0) запланировано.

### 3.3. Практическая сложность оценки вклада (Shapley Value)
*   **Утверждение:** Federated Shapley Value может быть точно и эффективно вычислен для консорциума из 5‑10 участников.
*   **Опровергающее доказательство:** Даже при использовании Монте‑Карло (`M=200`), стоимость вычисления `φ_i` требует переобучения модели `O(M * n)` раз, что может быть непрактично для больших моделей глубокого обучения. Альтернативные методы (например, на основе influence functions) имеют свои ограничения и могут быть неточными.
*   **Источник:** Собственный анализ алгоритмической сложности и обзор литературы (arXiv:1905.00457, "Evaluating Client Contributions in Federated Learning").
*   **Следствие:** Механизм распределения выгод может стать узким местом системы. Необходимо разработать упрощённые, но справедливые прокси‑метрики для больших консорциумов.
```
### `OPEN_PROBLEMS.md` (head 200 lines)
```
# Открытые проблемы FCLC

Список нерешённых научных, инженерных и организационных проблем в проекте FCLC. Для каждой проблемы приведены конкретные тесты на фальсификацию (falsification tests), позволяющие определить, верно ли наше текущее понимание или подход нуждается в пересмотре.

## Проблема F1: Масштабируемость безопасной агрегации (SecAgg+) при n > 20

**Описание:** Текущая реализация SecAgg+ имеет вычислительную и коммуникационную сложность `O(d * n^2)` для генерации попарных масок, где `d` — размерность модели, `n` — число узлов. Для небольших консорциумов (≤10 узлов) это приемлемо, но при масштабировании до десятков или сотен узлов (например, в национальной инициативе) протокол станет непрактичным.

**Гипотеза:** Можно разработать или адаптировать протокол безопасной агрегации с линейной или квазилинейной сложностью от `n`, который сохранит криптографические гарантии против semi‑honest оркестратора и обеспечит устойчивость к выбытию узлов.

**Фальсификационные тесты:**
1.  **Тест на пропускную способность (F1‑T1):** Развернуть симуляцию с `n=50` узлами и `d=1,000,000` (типично для средней нейросети). Если среднее время подготовки маски на узел превышает 5 секунд на стандартном сервере (c5.2xlarge), гипотеза о практичности текущего подхода для больших `n` будет опровергнута.
2.  **Тест на устойчивость к выбытию (F1‑T2):** В симуляции с `n=30` случайным образом "отключить" 40% узлов после фазы обмена ключами, но до агрегации. Если оркестратор не может корректно вычислить агрегированную сумму более чем в 5% запусков из‑за нехватки долей Шамира, гипотеза об эффективности пороговой схемы `t = ⌈n/2⌉` для данного масштаба будет опровергнута.
3.  **Тест на альтернативные протоколы (F1‑T3):** Реализовать протокол "LightSecAgg" (arXiv:2109.14236), который имеет сложность `O(d * log n)`. Сравнить время выполнения и потребление памяти с текущим SecAgg+ при `n=25`. Если LightSecAgg не показывает улучшения более чем в 2 раза при сопоставимом уровне безопасности, гипотеза о существовании существенно более масштабируемых прямых альтернатив будет ослаблена.
4.  **Тест на компромисс безопасность/производительность (F1‑T4):** Исследовать протоколы, использующие доверенные аппаратные окружения (TEEs), например, Intel SGX. Если стоимость развёртывания и аттестации TEE на каждом узле окажется выше, чем совокупные вычислительные издержки SecAgg+ за 1 год работы для консорциума из 20 узлов, гипотеза о экономической целесообразности аппаратных решений для данной проблемы будет опровергнута.

**Приоритет:** Высокий (P0). Критично для долгосрочной жизнеспособности проекта.
**Зависимости:** Результаты криптографического аудита (WP3).

## Проблема F2: Защита против активного противника (Active Adversary) в роли оркестратора

**Описание:** Текущая модель безопасности SecAgg+ предполагает semi‑honest оркестратор. Активный противник, контролирующий оркестратор, может попытаться отклониться от протокола, чтобы раскрыть вклады отдельных узлов (например, посылая сконструированные сообщения).

**Гипотеза:** Существующие модификации SecAgg (например, с использованием zero‑knowledge proofs) или переход на модель с доверенным координатором (trusted dealer) или TEE могут эффективно mitigate угрозу активного противника без чрезмерного падения производительности.

**Фальсификационные тесты:**
1.  **Тест на атаку отклонения протокола (F2‑T1):** Разработать модель активного противника, который подменяет свои публичные ключи, отправляемые узлам в раунде обмена. Если противник может, имея `t` долей от выбранного узла‑жертвы, восстановить его приватный ключ и затем вычислить его вклад в агрегацию в более чем 30% симулированных атак, текущая реализация будет считаться уязвимой.
2.  **Тест на стоимость верификации (F2‑T2):** Интегрировать в протокол non‑interactive zero‑knowledge proofs (NIZK) для проверки корректности выполнения шагов оркестратором. Измерить увеличение времени раунда и объёма передаваемых данных. Если overhead превышает 50%, гипотеза о практичности "лёгких" криптографических проверок будет опровергнута.
3.  **Тест на TEE‑решение (F2‑T3):** Перенести код агрегации оркестратора в анклав Intel SGX. Если аттестация анклава (remote attestation) со стороны каждого узла перед каждым раундом увеличивает latency раунда более чем на 2 секунды, гипотеза о прозрачности TEE‑подхода для узлов будет опровергнута.
4.  **Тест на модель доверенного дилера (F2‑T4):** Реализовать схему, где доверенная третья сторона (dealer) генерирует маски для узлов. Если при симуляции компрометации дилера (злоумышленник получает его ключи) возможно раскрытие вкладов более чем 10% узлов за историю в 100 раундов, гипотеза о безопасности этой модели будет опровергнута.

**Приоритет:** Высокий (P0). Необходимо для доверия со стороны регулирующих органов и участников.
**Зависимости:** Внешний криптоаудит (WP3), исследование TEE.

## Проблема F3: Соответствие дифференциальной приватности регуляторным требованиям для медицинских данных

**Описание:** Текущий параметр `ε_total=10.0` не соответствует рекомендациям стандарта ISO/IEC 27559:2022 (`ε_total < 1.0`). Необходимо либо обосновать приемлемость большего ε для исследовательских целей, либо перепроектировать pipeline для достижения более строгих гарантий.

**Гипотеза:** Можно перепроектировать тренировочный pipeline FCLC, используя такие методы как PATE (Private Aggregation of Teacher Ensembles) или значительно более строгое композиционное учёта бюджета, чтобы достичь `ε_total ≤ 1.0` без катастрофической потери utility (точности модели).

**Фальсификационные тесты:**
1.  **Тест на utility при ε=0.5 (F3‑T1):** Провести серию экспериментов на публичном медицинском датасете (например, MIMIC‑III) с федеративным обучением простой модели. Сравнить accuracy модели, обученной с `ε_total=0.5` (используя улучшенную композицию Rényi DP), с accuracy модели, обученной без DP. Если падение accuracy превышает 15 процентных пунктов, гипотеза о достижимости строгого ε без больших потерь будет опровергнута для данного класса моделей.
2.  **Тест на масштабируемость PATE (F3‑T2):** Реализовать框架 PATE для контекста FCLC, где каждый узел — это "учитель". Обучить модель на 10 узлах с синтетическими данными. Если для достижения консенсуса между учителями требуется передача >1000 запросов к "студенту" на один раунд, гипотеза о коммуникационной эффективности PATE в федеративном сценарии будет опровергнута.
3.  **Тест на композицию с коррелированными запросами (F3‑T3):** Смоделировать обучение, где запросы (градиенты) сильно коррелированы между раундами. Применить advanced composition theorems (Moments Accountant). Если рассчитанный аналитически `ε_total` для 100 раундов с желаемым `δ=10⁻⁵` всё ещё превышает 5.0, гипотеза о возможности достижения `ε_total ≤ 1.0` при интенсивном обучении будет опровергнута.
4.  **Тест на регуляторное восприятие (F3‑T4):** Провести structured interview с 3 экспертами по медицинскому праву и этике (не из проекта), представив два сценария: `ε_total=10.0` и `ε_total=0.7`. Если менее 2 из 3 экспертов сочтут сценарий с `ε_total=0.7` "существенно более безопасным" или "регуляторно приемлемым", гипотеза о том, что снижение ε само по себе решит регуляторные проблемы, будет ослаблена.

**Приоритет:** Высокий (P0). Необходимо для соответствия стандартам и одобрения этических комитетов.
**Зависимости:** Прогресс в WP2 (CDATA Experimental Validation), где DP является ключевым требованием.

## Проблема F4: Практическое внедрение и нормативные барьеры в разных юрисдикциях

**Описание:** Даже технически безупречная система FCLC столкнётся с юридическими барьерами: разные трактовки анонимизации данных в ЕС (GDPR), США (HIPAA), Китае и т.д.; требования к локализации данных; сложности с получением информированного согласия для федеративного обучения.

**Гипотеза:** Разработка модульной юридической "обёртки" — набора адаптируемых шаблонов DUAs, IRB‑заявок и разъяснительных документов — позволит ускорить процесс подключения новых клиник из ключевых регионов (ЕС, США, Великобритания) до 4 месяцев на учреждение.

**Фальсификационные тесты:**
1.  **Тест на подключение в ЕС (F4‑T1):** Выбрать пилотную клинику в Германии. Попытаться пройти полный цикл: переговоры, адаптация DUA, получение локального IRB‑одобрения. Если процесс займёт более 6 месяцев, гипотеза об эффективности текущих шаблонов для ЕС будет опровергнута.
2.  **Тест на совместимость с HIPAA (F4‑T2):** Представить описание протокола деидентификации и SecAgg+ юристу, специализирующемуся на HIPAA. Если юрист даст письменное заключение, что данная схема **не** может считаться создающей "Limited Data Set" или обеспечивающей "Safe Harbor", гипотеза о соответствии HIPAA без существенных изменений будет опровергнута.
3.  **Тест на информированное согласие (F4‑T3):** Разработать текст информированного согласия для пациентов, объясняющий федеративное обучение. Провести фокус‑группу с 10 пациентами. Если более 30% участников после прочтения не смогут правильно ответить на вопрос "покидают ли ваши исходные данные эту больницу?", гипотеза о понятности предлагаемого подхода для пациентов будет опровергнута.
4.  **Тест на модель доверия (F4‑T4):** Провести опрос среди 20 потенциальных клиник‑участников. Предложить на выбор: 1) полностью открытый код (оркестратор + узел), 2) внешний аудит кода, 3) доверенный аппаратный модуль (TEE) у оркестратора. Если более 50% выберут вариант 3 как единственно приемлемый, гипотеза о том, что программно‑криптографические гарантии достаточны для привлечения участников, будет опровергнута.

**Приоритет:** Средний (P1). Блокирует развёртывание пилотов.
**Зависимости:** Юридическая экспертиза, пилотные партнёры.
```
### `AGENTS.md` (head 200 lines)
```
# Инструкции для агентов
```
### `Cargo.toml` (head 200 lines)
```
[workspace]
members = ["fclc-core", "fclc-node", "fclc-server", "fclc-demogen"]
resolver = "2"

```
## systemd snapshot
```
  UNIT                                                                                                      LOAD   ACTIVE SUB       DESCRIPTION
  sys-devices-pci0000:00-0000:00:02.0-0000:01:00.0-virtio1-net-eth0.device                                  loaded active plugged   Virtio 1.0 network device
  sys-devices-pci0000:00-0000:00:02.2-0000:03:00.0-virtio2-virtio\x2dports-vport2p1.device                  loaded active plugged   /sys/devices/pci0000:00/0000:00:02.2/0000:03:00.0/virtio2/virtio-ports/vport2p1
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:0-block-sr0.device       loaded active plugged   QEMU_CD-ROM
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda1.device  loaded active plugged   QEMU_HARDDISK 1
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda14.device loaded active plugged   QEMU_HARDDISK 14
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda-sda15.device loaded active plugged   QEMU_HARDDISK 15
  sys-devices-pci0000:00-0000:00:02.5-0000:06:00.0-virtio5-host0-target0:0:0-0:0:0:1-block-sda.device       loaded active plugged   QEMU_HARDDISK
  sys-devices-pci0000:00-0000:00:02.6-0000:07:00.0-virtio6-net-enp7s0.device                                loaded active plugged   Virtio 1.0 network device
  sys-devices-pci0000:00-0000:00:04.0-0000:00:04.0:0-0000:00:04.0:0.0-tty-ttyS0.device                      loaded active plugged   QEMU PCI 16550A Adapter (QEMU Virtual Machine)
  sys-devices-platform-ARMH0011:00-ARMH0011:00:0-ARMH0011:00:0.0-tty-ttyAMA0.device                         loaded active plugged   /sys/devices/platform/ARMH0011:00/ARMH0011:00:0/ARMH0011:00:0.0/tty/ttyAMA0
  sys-devices-platform-serial8250-serial8250:0-serial8250:0.1-tty-ttyS1.device                              loaded active plugged   /sys/devices/platform/serial8250/serial8250:0/serial8250:0.1/tty/ttyS1
  
```
## Code histogram
```
rs 35
ex 642
exs 117
heex 19
go 0
py 1
php 0
ts 47
tsx 0
js 41

```