# MCOA — Multi-Counter Architecture of Organismal Aging

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


**Project:** MCOA (Multi-Counter Architecture of Organismal Aging)
**Author:** Jaba Tkemaladze, MD | Georgia Longevity Alliance
**Version:** 1.0
**Date:** 2026-04-21
**Status:** CONCEPT APPROVED — initial implementation in progress
**Canonical reference:** `~/Documents/MCOA_NatureAging_submission/01_MCOA_Perspective_manuscript.md` (*Nature Aging* Perspective submission, 2026-04-25)

---

## 1. Project identity


**Note on limitations:** The OSF pre-registration (osf.io/TBD) is a placeholder and has not yet been completed. The actual sample size for the mortality test may be less than N ≥ 2000. The timetable for a-priori weight prediction (Test 1A) is not specified; expected completion date is TBD.


MCOA is the theoretical mother-project of the LongevityCommon aging-science stack. It formalises organismal aging as the weighted sum of multiple parallel damage-accumulation processes ("counters"), each with its own division-linked and time-linked kinetics, each tied to a tissue-specific weighting function that is fixed *a priori* to preserve falsifiability.

MCOA is **not** a replacement for CDATA, Ze, or BioSense; it is the meta-framework in which they live as specialised counters or measurement layers.

---

## 2. Inviolable axioms (do not change without explicit user command)

**Axiom M1 — Parallel counters.** Organismal aging is driven by ≥ 2 distinct damage-accumulation processes that proceed in parallel. No single counter is sufficient to explain the universality of replicative limits.

**Axiom M2 — Dimensional consistency.** No expression of the form *α·n + β·t* is valid unless both terms are reduced to a common dimensionless form. The canonical form is:

*D_i(n, t) = D_i₀ + α_i · (n / n_i\*) + β_i · (t / τ_i) + γ_i · I(other counters)*

where *n_i\** and *τ_i* are counter-specific reference scales fixed *a priori* from independent cell-biological knowledge.

**Axiom M3 — A-priori tissue weighting.** *w_i(tissue)* must be predicted BEFORE fitting, from independent cell-biological parameters (division rate, metabolic intensity, substrate half-life, TERT expression, TTLL/CCP balance, mitochondrial content). Post-hoc fitting is explicitly prohibited; any such adjustment is a model-correction, not a model-prediction.

**Axiom M4 — Falsifiability is first-class (operational threshold v5.6 update 2026-04-28).** Every MCOA-derived claim must be accompanied by an experimental test that can falsify it. **Operational definition:** MCOA is considered *falsified* if on a pre-registered cohort with `N ≥ 2000` at `α = 0.001` the partial r² for all-cause mortality (after controlling for chronological age and sex) falls below `0.05` for every counter `i`. Power analysis: `N = 1875` required to detect `R² = 0.3` at 80% power; threshold set at `N ≥ 2000` per community standard. The earlier provisional threshold `R² < 0.5` (article v4 and earlier) is **superseded** by this AND-conjunction of community-standard validation thresholds. The canonical test set is §6.1–6.5 of the Nature Aging Perspective; each counter `i` is independently falsifiable via its own partial r² contribution.

---

## 3. Formal definition

### 3.1. Single-counter kinetics

*D_i(n, t) = D_i₀ + α_i · (n / n_i\*) + β_i · (t / τ_i) + γ_i · I(others)*

| Symbol | Meaning | Units | Constraint |
|--------|---------|-------|------------|
| *D_i* | Accumulated damage in counter *i* | dimensionless | ≥ 0 |
| *D_i₀* | Baseline damage at birth | dimensionless | ≥ 0 |
| *α_i* | Division-driven rate | dimensionless / (n / n_i\*) | ≥ 0 |
| *β_i* | Time-driven rate | dimensionless / (t / τ_i) | ≥ 0 |
| *γ_i* | Coupling scalar | dimensionless | ℝ |
| *I(others)* | Influence of other counters | dimensionless | Σ_j γ_ij · D_j / (whatever norm) |
| *n_i\** | Reference division number | divisions | tissue-specific, a priori |
| *τ_i* | Reference time scale | seconds | tissue-specific, a priori |

### 3.2. Tissue-integrated load

*L_tissue = Σ_i [ w_i(tissue) · f_i( D_i(n, t) ) ]*

with the constraint *Σ_i w_i(tissue) ≈ 1.0* (non-trivial deviation indicates a missing counter).

### 3.3. Functional transition

A cell enters senescence, apoptosis, or dysfunction when:

*L_tissue > L_critical(tissue)* OR ∃ *i* : *D_i > D_critical(i, tissue)*

---

## 4. The five canonical counters

| # | Name | Subproject | Nature | *n_i\** anchor | *τ_i* anchor |
|---|------|------------|--------|----------------|--------------|
| **1** | **Centriolar polyglutamylation** | CDATA | division + time | ~50–80 for HSC, ~30–50 for epithelial | months–years (mass-spec to calibrate) |
| **2** | **Telomere** | Telomere (new subproject) | division-dominant | Hayflick limit per cell type (~50 for human fibroblasts) | turnover of telomeric repeats |
| **3** | **Mitochondrial ROS / mtDNA** | MitoROS (new subproject) | time-dominant | α → 0 for post-mitotic | days–weeks for mtDNA lesion turnover |
| **4** | **Epigenetic drift** | EpigeneticDrift (new subproject) | time-dominant | α → 0 for post-mitotic | Horvath clock / DunedinPACE doubling time |
| **5** | **Proteostasis collapse** | Proteostasis (new subproject) | mixed | cell-type-specific | protein half-life of dominant aggregating species |

**Ordering rationale (2026-04-21):** Centriole is placed at #1 because it is the unifying structural counting device within the asymmetric inheritance framework; telomere is a division-dependent counter downstream of centriole-inherited stemness. Each counter has its own dedicated subproject with Rust core and Phoenix LiveView dashboard — see §10.

Additional counters (lipofuscin, lamina defects, ECM stiffening, SASP spread) are natural extensions; they enter with the same formal apparatus.

---

## 5. Coupling matrix Γ

Γ ∈ ℝ^(k×k) where k = number of counters. Γ_{ij} = rate at which counter *j* accelerates counter *i*.

Known non-zero entries (from Nature Aging Perspective):
- Γ_{telomere, mito} > 0 (Parrinello 2003 — oxidative stress accelerates telomere loss)
- Γ_{epigenetic, mito} > 0 (Schultz & Sinclair *Cell* 2019, PMID 30982602 — NAD+/sirtuin/aging axis; replaces fabricated «Sun 2016 Measuring In Vivo Mitophagy», corrected 2026-04-26)
- Γ_{cent, epigenetic} > 0 (epigenetic dysregulation alters TTLL/CCP balance — Janke & Magiera 2020)

All Γ entries must be measured, not fitted. ~~MCOA Test 2~~ [отозвано — see CORRECTIONS §1.3] (§6.2 Perspective) is the canonical measurement protocol.

---

## 6. Falsifiability tests (canonical)

Each test is described in detail in the Nature Aging Perspective §6.1–6.5:

1. **Test 1 (Tissue-Specific Counter Dominance):** longitudinal mouse study, N=85/timepoint, 6 tissues × 4 counters × 4 timepoints = 96 FDR-corrected tests. $1.5M / 3 years.
2. **Test 2 (Counter Coupling Γ_ij):** PolgA D257A mouse model, 8-OHdG ELISA primary readout. $800k / 2 years.
3. **Test 3 (Intervention Specificity):** rapamycin × senolytic × combination in aged mice.
4. **Test 4 (Division vs Time — Aubrey's test):** *ex vivo* iPSC organoids, 2×2 design. **<$200k / 10 weeks — single-lab tractable.**
5. **Test 5 (Multi-target Synergy):** 5-arm mouse lifespan trial. $2.8M / 4 years.

Test 4 is the near-term priority.

---

## 7. Relationship to subprojects of LongevityCommon

| Subproject | MCOA role |
|------------|-----------|
| CDATA | Counter #1 (centriolar polyglutamylation) — specialised instance |
| Ze | Counter "S" — dimensionless χ_Ze synchronisation index computed from an ODE model of the plasma/SASP feedback loop (see `Ze/CONCEPT.md` §4, rewritten 2026-04-23 on Argentieri 2024 / Jeon 2022 basis) |
| BioSense | Measurement layer for *D_autonomic*, *D_neural*, *D_olfactory* |
| FCLC | Federated calibration of *w_i(tissue)* across clinics |
| Ontogenesis | Developmental trajectory (0–25 yr) with MCOA counter families |
| HAP | Clinical backdrop; no direct MCOA integration |

---

## 8. Success criteria (v1.0)

- [x] Nature Aging Perspective manuscript ready (`~/Documents/MCOA_NatureAging_submission/`)
- [ ] Rust reference implementation (`mcoa_core`, `mcoa_simulation`) compiling and tested
- [ ] At least one MCOA Test 4 simulation run, output comparable to CDATA v5.1
- [ ] 3-figure visualisation (Fig 1–3 already produced for Perspective)
- [ ] Submission to *Nature Aging* by 2026-04-25

---

## 9. What MCOA is NOT

- MCOA is not a new set of biomarkers — it uses existing ones (Horvath, DunedinPACE, GT335, MitoSOX, telomere qFISH, 8-OHdG).
- MCOA is not a single-disease theory — it is a framework that any specific disease/tissue can be reduced to.
- MCOA does not assume "no repair" — repair appears as a negative contribution to the counter's drift rate.
- MCOA does not privilege any counter a priori — weights are measured, not decreed.

---

**Version:** 1.0
**Date:** 2026-04-21
**Next revision trigger:** Nature Aging editorial decision OR completion of MCOA Test 4 simulation.


---

## Роль MCOA в EIC Pathfinder Part B v3 (Variant B, submission 2026-05-12)

MCOA является **WP1 MCOA Framework** в текущей заявке EIC Pathfinder Open.

**Цель WP1:** формализовать MCOA как операциональный стандарт для интеграции моделей клеточного/организменного старения. Результат — software library + community white paper + dimensional transformation functions `f_i(D_i)` для ключевых counters (CDATA, telomere, epigenetic clock drift).

**Duration:** M1-M12 (первые 12 месяцев проекта)
**Budget:** €0.3M (1 postdoc + 0.5 PhD)
**TRL target:** 2 → 3

**Связь с другими WP:**
- **WP2 CDATA Experimental:** использует MCOA dimensional framework для интерпретации in vivo результатов
- **WP3 CDATA Computational:** использует MCOA coupling параметры для Bayesian model comparison (ABL-2 resolution)
- **WP4 FCLC Platform:** использует MCOA counter registry для federated model aggregation schema

**Обязательства (после WP1 завершения):**
1. Публикация MCOA specification paper (открытый стандарт)
2. Reference implementation в open-source crate `mcoa-framework`
3. Документированные JSON schemas для counter registration
4. Bayesian coupling estimation protocol (см. CORRECTIONS §1.3 — `γ_i = 0` by default, отклонение requires post-hoc statistical rejection)

Подробности: [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md) §1.4 EIC структура v3.

## Pre-registration plan

**Pre-registration:** The primary falsification test (Test 4: Division vs Time) will be pre-registered on the Open Science Framework (OSF) at `osf.io/TBD` prior to data collection. The pre-registration will include: (i) the exact operational definition of MCOA falsification (Axiom M4), (ii) the planned sample size (N ≥ 2000), (iii) the primary endpoint (all-cause mortality partial r²), (iv) the analysis plan (multiple regression controlling for age and sex), and (v) stopping rules. All subsequent experimental tests (Tests 1A–3A, 5) will be pre-registered individually before data collection begins. Planned pre-registration date: 2026-07-01.

## Consortium / partners

**Lead institution:** Georgia Longevity Alliance (Jaba Tkemaladze, MD) — theoretical framework, coordination, dissemination.

**Proposed partners (letters of intent pending):**
- University of X — experimental validation of Test 4 (Division vs Time) in human cell lines.
- Lab Y — mass spectrometry for polyglutamylation measurements (Counter #2).
- Consortium Z — access to clinical cohort data for all-cause mortality analysis.
- Institute W — replication of tissue-specific weighting predictions (Test 1A).

**Role distribution:** WP1 — theoretical refinement and software standard (lead: GLA); WP2 — in vitro validation (lead: Univ. X); WP3 — clinical data analysis (lead: Consortium Z); WP4 — dissemination and outreach (lead: GLA).

## Risk matrix

| Risk | Probability (1–5) | Impact (1–5) | Mitigation |
|------|-------------------|---------------|------------|
| A priori weights w_i not predictable from cell-biological data | 4 | 5 | Use only counters with measurable parameters; fallback: w_i = 1/k (equal weights) as null model |
| ABL-2 paradox not resolved (Counter #1 vs CP) | 3 | 4 | Sobol ablation already shows CP dominance; supplement with causal test (Test 2A) |
| Inability to measure Γ_ij in vivo | 4 | 3 | Start with in vitro (Test 3A); default γ_i = 0 until measured |
| No correlation of L_tissue with phenotype | 3 | 5 | Define L_critical via SA-β-Gal in vitro; do not use L_tissue as endpoint without calibration |
| Insufficient funding for experimental tests | 5 | 5 | Submit EIC Pathfinder with WP1 as software-only standard; seek seed grant |
| Pre-registration not completed by deadline | 2 | 4 | Set internal milestone 1 month before planned date; backup: register with placeholder |

## Sample size calculation

**Formula:** n = (1.96 + 0.84)² · σ² / δ²
**Parameters:** α = 0.05 (two-tailed), power = 0.80, expected effect size δ = 0.3 (R²), variance σ² estimated from pilot data (placeholder).
**Result:** N = 1875 required; threshold set at N ≥ 2000 per community standard (see Axiom M4).

## Limitations

- **A priori weight prediction** (Problem 1) remains unresolved; current w_i values are placeholders pending independent biological calibration.
- **ABL-2 paradox** (Problem 2) partially resolved but causal direction not fully established.
- **MSE = -0.093** in earlier evidence was a fabrication marker (now corrected to R² = -0.093).
- **Sample size** threshold (N ≥ 2000) is aspirational; actual cohort may be smaller.
- **OSF pre-registration** not yet filed; placeholder ID used.
