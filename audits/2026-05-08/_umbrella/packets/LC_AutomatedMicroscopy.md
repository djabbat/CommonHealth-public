# AUDIT PACKET — LC_AutomatedMicroscopy

Path: `/home/oem/Desktop/LongevityCommon/AutomatedMicroscopy`  Date: 2026-05-08

## Size & file counts
```
292K	/home/oem/Desktop/LongevityCommon/AutomatedMicroscopy
```
**Extensions:** .md=12, .pdf=1, .docx=1
## Tree (depth=2, max 200 entries)
```
.
./ROADMAP.md
./PARAMETERS.md
./AGENTS.md
./EVIDENCE.md
./DESIGN.md
./README.md
./CLAUDE.md
./THEORY.md
./OPEN_PROBLEMS.md
./docs
./docs/AUTOMATED_MICROSCOPY_SETUP.pdf
./docs/AUTOMATED_MICROSCOPY_SETUP.md
./docs/AUTOMATED_MICROSCOPY_SETUP.docx
./CONCEPT.md
./JOURNAL.md
./_archive
./_archive/DEEP_AUDIT_2026-04-21.md
```
## Detected stack: **unknown**
## Core files

### `CLAUDE.md` (2460 chars)
```md
# CLAUDE.md — AutomatedMicroscopy

**AutomatedMicroscopy** — infrastructure-подпроект LongevityCommon. Retrofit Zeiss IM 35 inverted microscope с моторизованным XYZ-стейджем, FLIR Blackfly S камерой и LED флуоресцентным иллюминатором ($4,500 BOM, open-source DIY). Сервит CDATA Phase A первым пользователем; в перспективе — все counter-эксперименты.

**Path:** `/home/oem/Desktop/LongevityCommon/AutomatedMicroscopy/`
**Repo:** часть монорепозитория `djabbat/LongevityCommon`.

---

## Source of truth

**`AutomatedMicroscopy/CONCEPT.md`** + `AutomatedMicroscopy/AGENTS.md` (для AI night-shift logic).
Корневой `~/Desktop/LongevityCommon/CLAUDE.md` (parent rules).

---

## Positioning в MCOA framework

**Не** damage counter (нет уравнения Dᵢ). Operational infrastructure — собирает time-series Dᵢ(n,t) для всех биологических counter'ов: CDATA, Telomere, MitoROS, EpigeneticDrift, Proteostasis.

---

## Hardware stack

1. Motorized XY+Z stage (Arduino + steppers)
2. FLIR Blackfly S USB3 scientific camera
3. ThorLabs M470L4 + M565L3 LED fluorescence illuminator
4. DIY environmental chamber (37°C, 5% CO₂, 90% humidity)
5. UPS + NAS backup + WireGuard VPN remote

---

## AI night-shift technician (unique innovation)

Claude Code agent в `/overnight` mode читает natural-language `PROMPT.md` per experiment. Делает routine decisions автономно (focus, ROI, channel switching), эскалирует только strategic decisions человеку.

**Связь с AIM:** не AIM. Использует Claude Code напрямую (per project_aim_roadmap_surpass_cc и AIM AUDIT findings — robotics gap GAP-2 закрыт через `aim-mcp-lab-runner` crate, но E0 / AutomatedMicroscopy всё ещё используют Claude Code напрямую).

---

## Subprojects

- `~/Desktop/PhD/E0/` — experimental Kaede-ablation rig для CDATA (см. memory `project_phd_e0_subproject`)

---

## Правила

1. AGENTS.md — авторитетный для prompts/decisions night-shift agent'а.
2. JOURNAL.md — лабораторный журнал (append-only, не редактировать прошлые записи).
3. Безопасность: agent НЕ выполняет destructive ops без подтверждения (rm/git push/etc); следует principles в Claude Code project rules.
4. PROMPT.md per experiment — каждый эксперимент имеет свой PROMPT.md, описывающий цель/протокол/decision-tree.

---

## Server presence

Нет — это hardware-rig, не webapp. Никаких subdomain'ов или backend'ов.

---

## Тесты

Hardware not testable in CI. Smoke-тесты на calibration runs перед каждой production-сессии. Documented в `JOURNAL.md`.

```
### `README.md` (2212 chars)
```md
# AutomatedMicroscopy — LongevityCommon subproject

**Purpose:** Low-cost ($4,500) AI-operated time-lapse microscopy platform для round-the-clock live-cell imaging, enabling single-PI labs to conduct industrial-grade imaging experiments without human shift overhead.

**Parent ecosystem:** LongevityCommon (longevity research ecosystem)
**Flagship role:** Experimental infrastructure for CDATA Phase A (Impetus Grant 2026-04-25) + future MCOA Counter validation experiments

**Status:** Engineering design complete (2026-04-21). Bill-of-materials ready. Assembly expected Months 1-2 of Phase A Impetus grant (if funded).

**Core innovation:** Claude Code `/overnight` режим управляет микроскопом, интерпретируя естественно-языковой PROMPT (описание целей и задач эксперимента), принимая routine decisions автономно и сигнализируя человека только при стратегически важных событиях.

**Budget target:** $4,500 retrofit (Вариант A DIY) vs $12,700 mid-tier (Вариант B) vs $25-50k turnkey (Вариант C).

## Quick links

- **Theory:** see `THEORY.md`
- **Evidence / references:** see `EVIDENCE.md`
- **Open problems / research questions:** see `OPEN_PROBLEMS.md`
- **Bill of materials / quantitative params:** see `PARAMETERS.md`
- **System architecture / code structure:** see `DESIGN.md`
- **AI agent instructions:** see `AGENTS.md`
- **Changelog / decisions:** see `JOURNAL.md`
- **Future roadmap:** see `ROADMAP.md`

## Контекст в экосистеме LongevityCommon

AutomatedMicroscopy — **инфраструктурный слой** для experimental подпроектов (CDATA, Telomere, MitoROS, EpigeneticDrift, Proteostasis), которые требуют длительного live-cell imaging.

Сравнение с другими подпроектами:
- **CDATA, Telomere, etc.** — scientific hypotheses / damage counters
- **FCLC** — federated data sharing infrastructure
- **MCOA** — theoretical framework
- **AutomatedMicroscopy (this)** — experimental infrastructure for data collection

## Ссылки

- Parent: `~/Desktop/LongevityCommon/CONCEPT.md`
- Related grant: `~/Documents/Grants/LongevityCommon/CDATA/docs/IMPETUS_2026-04-25/`
- External source: `~/Documents/Engineering/AutomatedMicroscopy_2026-04-21/`

## License

MIT (all code + BOM + PROMPT templates released post-Phase A).

```
### `CONCEPT.md` (4573 chars)
```md
# AutomatedMicroscopy — CONCEPT

*(This document is the authoritative CONCEPT.md for AutomatedMicroscopy subproject. Synthesized 2026-04-21 from AUTOMATED_MICROSCOPY_SETUP.md engineering specification.)*

## Parent framework

AutomatedMicroscopy is a **LongevityCommon infrastructure subproject** providing experimental automation for MCOA Counter validation experiments (CDATA Phase A being the first user).

**Positioning in MCOA framework:**
- Not a damage counter itself (no D_i equation)
- Operational infrastructure enabling collection of time-series data D_i(n, t) для all biological counters
- Serves all subprojects requiring live-cell time-lapse imaging: CDATA, Telomere, MitoROS, EpigeneticDrift, Proteostasis

## Core concept

Retrofit of existing Zeiss IM 35 inverted microscope ($4,500 BOM, open-source DIY) with:
1. Motorized XY+Z stage (Arduino-based steppers)
2. FLIR Blackfly S USB3 scientific camera
3. LED fluorescence illuminator (ThorLabs M470L4 + M565L3)
4. DIY environmental chamber (37°C + 5% CO₂ + 90% humidity)
5. UPS + NAS backup + WireGuard VPN remote monitoring

**Unique innovation:** Claude Code agent operating в `/overnight` mode serves as AI night-shift technician, interpreting natural-language PROMPT.md per experiment and making routine decisions (focus adjustment, ROI selection, channel switching) autonomously while signaling human only для strategic decisions.

## Axioms

**M1 (Feasibility):** AI-operated microscopy achieves ≥80% trained-technician supervision quality at <20% capital cost для routine CDATA-class protocols.

**M2 (Interpretability):** Every AI decision links to explicit PROMPT.md line + measurable observations. Full audit trail.

**M3 (Bounded autonomy):** AI acts only within `auto_allow` policy; `require_human_approval` for strategic; `forbidden` for biosafety.

**M4 (Reproducibility):** Complete journals (decisions + rationale + observations) enable post-hoc blind audit.

## Hypothesis

Low-cost retrofit ($4,500) + prompt-driven AI supervision replicates industrial-grade automated microscopy ($25-50k) for class of protocols where:
- Sample stability ≥ 3 weeks
- Imaging frequency ≤ 2/hour
- Environmental stability required ±0.5°C, ±0.5% CO₂
- No physical sample manipulation on-platform (media changes human-performed)

## Primary use case

CDATA Phase A fibroblast experiment (Impetus Grant 2026-04-25):
- BJ-hTERT fibroblasts, 20% vs 3% O₂
- Aim A.1: α vs β discrimination (division vs time accumulation)
- Aim A.2: Parrinello paradox test
- Aim A.3: CCP1/TTLL6-OE causality
- 6 months, 24/7 imaging, >900 GB data, >7,200 AI decisions journaled

## Predictions / success metrics

1. Platform uptime ≥95% over 180 days
2. Claude decisions concordant with trained-technician judgment ≥80% (blind review by 3 external scientists post-hoc)
3. Contamination rate ≤3% per experimental run
4. Cost per 6-month run: $5,020 total ($5,000 amortized equipment + $20 Claude subscription)
5. Bill of materials + policy file + tool function code released open-source concurrent с Phase A preprint

## Falsification conditions

Platform not-suitable если:
- AI decisions deviate >20% from trained-technician judgment
- Hardware uptime <80%
- Contamination rate >10%
- User abandons autonomous mode within 1 month

## Budget

$4,500 (Вариант A DIY) allocated within CDATA Phase A Impetus grant line "Microscope Automation & Upgrade" (reallocation from Consumables).

## Scope exclusions

- Physical cell manipulation (no liquid handling robot in Phase A)
- Chamber opening for media change (human task)
- Novel imaging modalities (only standard epifluorescence)
- Cross-lab federated coordination (that's FCLC)
- Therapeutic intervention decisions (outside AI policy)

## Cross-ecosystem references

- **Uses FCLC**: anonymized imaging data будут contributed to federated learning pool post-Phase A
- **Enables MCOA**: provides empirical substrate for temporal D_i(n, t) dynamics
- **Enables CDATA**: Phase A experiments impossible without 24/7 imaging
- **Enables future counter subprojects** (Telomere, MitoROS, EpigeneticDrift, Proteostasis): same platform reused

## Related documents

- `AUTOMATED_MICROSCOPY_SETUP.md` — detailed engineering specification (BOM, wiring, software stack)
- `THEORY.md` — formal axioms and predictions
- `EVIDENCE.md` — verified refs + internal data
- `OPEN_PROBLEMS.md` — honest list of risks and unresolved questions
- `DESIGN.md` — code architecture + file tree

---

*CONCEPT v1.0, 2026-04-21. Part of LongevityCommon ecosystem per `~/Desktop/LongevityCommon/CONCEPT.md`.*

```
### `THEORY.md` (5327 chars)
```md
# THEORY — AutomatedMicroscopy

## Формальная теоретическая основа

### 1. Проблема — human shift overhead в live-cell microscopy

Traditional time-lapse microscopy требует continuous human oversight: ручная focus adjustment, manual field-of-view selection, visual inspection of cultures, media changes, intervention at anomalies.

В condiciях single-PI labs (as Georgia Longevity Alliance), 24/7 continuous supervision невозможно. Результат: **либо эксперименты ограничены рабочими часами (8-12h/day), либо закупается дорогое автоматизированное оборудование ($25-50k+)**.

### 2. Гипотеза

**Теза:** Low-cost retrofit ($4,500) + AI agent в роли "night-shift lab technician" позволяет достичь industrial-grade 24/7 imaging без capital-intensive hardware.

**Формальная аксиома M1 (Feasibility):**
> Для class CDATA-type experiments (time-lapse polyGlu intensity measurement on mother centrioles в BJ-hTERT fibroblasts), а AI agent (Claude Code в `/overnight` режиме) может выполнять supervisory function eq. quality к trained technician, при условиях:
> - **Well-defined PROMPT** (natural-language protocol)
> - **Bounded autonomy** (pre-authorized routine actions + require-human-approval для strategic decisions)
> - **Full journaling** (every decision logged с rationale, reproducible after-the-fact)

### 3. Prompt-driven supervision model

Formalization of experimenter-AI interaction:

```
PROMPT: natural-language description of experiment goals
       ↓ parsed by Claude Code
CRITERIA: concrete thresholds, metrics, conditions
       ↓ continuous monitoring every 30 min
OBSERVATIONS: image data, environmental sensors
       ↓ comparison to CRITERIA
SIGNAL: INFO / WARN / CRIT → human
     OR continue_schedule autonomously
```

Bayesian decision-theoretic framing:

```
P(action | observation, prompt) ∝ P(observation | action, prompt) · P(action | prompt)
```

где:
- **prior P(action|prompt)** = "what would a trained technician do here"
- **likelihood P(observation|action, prompt)** = expected outcome given protocol compliance
- Decision: select action maximizing expected reward (experiment success ∩ biosafety ∩ human trust)

### 4. Аксиомы subproject

**M1 (Feasibility):** AI-operated microscopy achieves ≥80% of trained-technician supervision quality для routine protocols, at <20% cost.

**M2 (Interpretability):** Every AI decision must link к explicit PROMPT.md line + measurable observations. No "black-box" автономных actions без traceable rationale.

**M3 (Bounded autonomy):** AI acts only within `auto_allow` policy list; `require_human_approval` gates preserve human strategic control; `forbidden` gates preserve biosafety.

**M4 (Reproducibility):** Complete journals (decisions + rationale + observations) enable post-hoc audit of any experimental run by human reviewer.

### 5. Scope

**In scope:**
- Live-cell fluorescence imaging (BF + FITC + TRITC + DAPI channels)
- Z-stack acquisition (up to 20 μm range, 2 μm steps)
- Environmental chamber monitoring (37°C + 5% CO₂ + humidity)
- Autonomous autofocus, channel switching, stage positioning
- Image analysis pipeline (CellPose segmentation, ImageJ measurements)
- Signal generation to human experimenter per PROMPT.md

**Out of scope (for Phase A):**
- Physical cell manipulation (no liquid handling robot в Phase A)
- Chamber opening для media change (manual, human task)
- Novel imaging modalities (only standard epifluorescence)
- Cross-lab federated coordination (that's FCLC scope)
- Therapeutic intervention decisions (outside AI policy)

### 6. Interfaces с другими подпроектами LongevityCommon

| Subproject | Interface |
|------------|-----------|
| **CDATA** | Primary user — Phase A experiments run on this platform |
| **FCLC** | Future: anonymized imaging data contribution to federated learning pool |
| **MCOA** | Future: multi-counter experiments (Telomere, MitoROS) reuse same infrastructure |
| **BioSense** | Potential: shared signal-processing pipelines (cross-domain aging markers) |

### 7. Predictions

1. **Data yield:** 6 months `/overnight` operation → ~900 GB imaging data, ~40 decisions/night journaled = 7,200 logged decisions total
2. **Efficiency:** experiments complete 2-3× faster than with 9-5 human oversight (continuous vs 40-hour weeks)
3. **Cost per experiment:** ~$5k equipment amortization + ~$20 AI subscription per 6-month run = ~$5,020 per experimental cycle
4. **Reliability:** 95%+ uptime target (UPS + redundant sensors + fail-safe policies)

### 8. Falsification conditions

Platform is **falsified / not-suitable** если:
- Claude Code decisions deviate from trained-technician judgment >20% случаев (measured post-hoc blind review by independent scientist)
- Hardware uptime <80% over first 60 days
- Contamination rate >10% per experimental run (vs typical 1-3% in standard microscopy)
- User (Jaba) abandons autonomous mode after 1 month (too stressful, too much supervision needed)

### 9. Связь с MCOA framework

AutomatedMicroscopy — **instrumental layer** не theoretical counter. Но сам факт его существования enables MCOA framework operationally: без 24/7 imaging infrastructure невозможно собрать данные для temporal dynamics D_i(n, t) разных counter'ов.

Без AutomatedMicroscopy → MCOA остаётся теоретической абстракцией.
С AutomatedMicroscopy → MCOA получает эмпирический substrate.

```
### `PARAMETERS.md` (435 chars)
```md
# PARAMETERS — AutomatedMicroscopy

*Stub (created 2026-04-21). Will be regenerated by DeepSeek orchestrator as part of ecosystem-wide core file refresh.*

See `README.md` для overview; `THEORY.md` для formal theoretical framework; `EVIDENCE.md` для verified references.

Source content pending: `AUTOMATED_MICROSCOPY_SETUP.md` в этой же папке содержит полную engineering спецификацию и является материнским документом для generation.

```
### `DESIGN.md` (431 chars)
```md
# DESIGN — AutomatedMicroscopy

*Stub (created 2026-04-21). Will be regenerated by DeepSeek orchestrator as part of ecosystem-wide core file refresh.*

See `README.md` для overview; `THEORY.md` для formal theoretical framework; `EVIDENCE.md` для verified references.

Source content pending: `AUTOMATED_MICROSCOPY_SETUP.md` в этой же папке содержит полную engineering спецификацию и является материнским документом для generation.

```
### `EVIDENCE.md` (5769 chars)
```md
# EVIDENCE — AutomatedMicroscopy

Верифицированные facts, references и internal data, поддерживающие design choices в этом subproject.

---

## Verified Literature

### Foundational — imaging hardware

| Claim | Source | PMID / DOI | Verified | Strength |
|-------|--------|------------|----------|----------|
| Zeiss IM 35 inverted microscope has C-mount port for digital camera adaptation | Zeiss product manual 1985 | — (manufacturer spec) | ✅ 2026-04-21 (confirmed by Jaba's personal unit) | Strong |
| FLIR Blackfly S BFS-U3-63S4M-C: 2448×2048 mono, 74 FPS, USB3 scientific camera | FLIR product datasheet | flir.com/products/blackfly-s-usb3 | ✅ 2026-04-21 | Strong |
| Arduino-based motorized XY stage achievable с ±5μm accuracy using linear rails + NEMA-17 steppers | OpenFlexure Microscope project | 10.1364/BOE.10.002807 | ✅ 2026-04-21 (Sharkey et al. 2019) | Strong |
| Micro-Manager 2.0 open-source acquisition supports pymmcore-plus Python bindings | micro-manager.org | — | ✅ 2026-04-21 | Strong |

### Foundational — live-cell imaging environmental control

| Claim | Source | PMID / DOI | Verified | Strength |
|-------|--------|------------|----------|----------|
| 37°C + 5% CO₂ environment необходимо для BJ-hTERT fibroblast long-term culture | Hayflick 1965; standard ATCC protocols | 10.1016/0014-4827(65)90211-9 | ✅ 2026-04-21 (Hayflick 1965 PMID 14315085) | Strong |
| Humidity 80-95% RH prevents media evaporation over 3-week contact-inhibition protocol | standard cell culture practice | — | ✅ 2026-04-21 | Moderate |
| Peltier heater + PID controller achieves ±0.3°C stability | Inkbird ITC-100 spec; DIY community | — | ✅ 2026-04-21 | Moderate |

### Foundational — image analysis

| Claim | Source | PMID / DOI | Verified | Strength |
|-------|--------|------------|----------|----------|
| CellPose v2 segments cells in brightfield and fluorescence with generalist model | Stringer et al. 2021 Nat Methods | 10.1038/s41592-020-01018-x | ✅ 2026-04-21 (PMID 33318659) | Strong |
| ImageJ/Fiji batch processing pipelines standard in centrosomal research | Schindelin et al. 2012 | 10.1038/nmeth.2019 | ✅ 2026-04-21 (PMID 22743772) | Strong |
| GT335 antibody recognizes polyglutamylated tubulin (ammonium sulfate precipitated cells) | Wolff et al. 1992 Eur J Cell Biol | PMID: 1385210 | ✅ 2026-04-21 | Strong |
| Ninein antibody marks mother centriole distal appendage complex | Delgehyr et al. 2005 J Cell Sci | 10.1242/jcs.02302 | ✅ 2026-04-21 (PMID 15972319) | Strong |

### AI-operated experimental science — precedents

| Claim | Source | PMID / DOI | Verified | Strength |
|-------|--------|------------|----------|----------|
| Autonomous lab robots for chemistry synthesis (Burger et al. 2020 Nature) | Burger et al. 2020 | 10.1038/s41586-020-2442-2 | ✅ 2026-04-21 (PMID 32641820) | Strong |
| GPT-4 driving chemical synthesis planning (Boiko et al. 2023 Nature) | Boiko et al. 2023 | 10.1038/s41586-023-06792-0 | ✅ 2026-04-21 (PMID 38123808) | Strong |
| ChemCrow — LLM with chemistry tools (Bran et al. 2024 Nat Machine Intell) | Bran et al. 2024 | 10.1038/s42256-024-00832-8 | ✅ 2026-04-21 | Strong |

**Note:** до настоящего момента no published precedent of **LLM agent (Claude-class) operating microscopy в `/overnight` mode для aging biology experiments**. This subproject would be among first. Novel, but not unprecedented (follows chemistry lab automation paradigm).

---

## Internal Data / Artifacts

- `AUTOMATED_MICROSCOPY_SETUP.md` — full engineering specification (this subproject)
- `~/Documents/Engineering/AutomatedMicroscopy_2026-04-21/` — source material (pre-LongevityCommon integration)
- Future: PROMPT.md templates for each Aim
- Future: Claude Code policy file `microscope-operator.md`
- Future: bill-of-materials spreadsheet с актуальными 2026 prices

---

## Refuting / Cautionary Evidence (honest)

- **DIY stage accuracy limitations:** Prior 3rd-party commercial motorized stages (Prior ProScan, Märzhäuser) achieve ±0.1μm; DIY Arduino-based will not match. This may limit reproducibility of subcellular localization measurements (centriole is ~500nm diameter). Mitigation: use only relative measurements (intensity ratios), not absolute positional assays.

- **Long-term calibration drift:** Belt-driven stepper actuators can drift 10-50μm over days. Mitigation: daily autofocus pass + fiducial markers + recalibration every 48h.

- **LED bleaching:** Continuous LED illumination over 3 weeks may degrade sample (phototoxicity, photobleaching). Mitigation: exposure ≤500ms, imaging interval ≥30min, low LED intensity (50% max).

- **AI hallucination risk:** Claude Code может misinterpret image features и принять неверное routine decision. Mitigation: `auto_allow` список узкий; `require_human_approval` для strategic; all decisions journaled for post-hoc audit.

- **Biosafety blind spot:** AI cannot detect contamination visually as reliably as trained human (microbial turbidity subtle in early stages). Mitigation: Claude flags `cell_density_drop` as WARN, human checks visually at 8 AM daily.

- **No precedent in aging biology:** First project using Claude-class LLM for continuous microscopy supervision. Unknown failure modes. Mitigation: 48h validation period в Phase A Month 1 before full autonomy.

---

## Cross-references

- Parent theory: `THEORY.md` §2 hypothesis, §3 prompt-driven supervision
- Related open problems: `OPEN_PROBLEMS.md` §1 AI judgment quality, §2 hardware reliability
- Parameter provenance: `PARAMETERS.md`
- External: Impetus LOI v24 §Methods section cites automation (`~/Documents/Grants/LongevityCommon/CDATA/docs/IMPETUS_2026-04-25/LOI_Impetus_v24_MCOA_2026-04-21.pdf`)

---

*Last verified: 2026-04-21. Literature refs checked via PubMed esummary API on this date.*

```
### `OPEN_PROBLEMS.md` (438 chars)
```md
# OPEN_PROBLEMS — AutomatedMicroscopy

*Stub (created 2026-04-21). Will be regenerated by DeepSeek orchestrator as part of ecosystem-wide core file refresh.*

See `README.md` для overview; `THEORY.md` для formal theoretical framework; `EVIDENCE.md` для verified references.

Source content pending: `AUTOMATED_MICROSCOPY_SETUP.md` в этой же папке содержит полную engineering спецификацию и является материнским документом для generation.

```
### `AGENTS.md` (431 chars)
```md
# AGENTS — AutomatedMicroscopy

*Stub (created 2026-04-21). Will be regenerated by DeepSeek orchestrator as part of ecosystem-wide core file refresh.*

See `README.md` для overview; `THEORY.md` для formal theoretical framework; `EVIDENCE.md` для verified references.

Source content pending: `AUTOMATED_MICROSCOPY_SETUP.md` в этой же папке содержит полную engineering спецификацию и является материнским документом для generation.

```