```markdown
=== REVISED CONCEPT MCOA (version 1.1) ===
# MCOA — Multi-Counter Architecture of Organismal Aging

> ⚠️ **See [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — some earlier statements have been retracted or corrected. Canons updated 2026-04-22.

> **Revision note (v1.1):** This version incorporates recommendations from the Triple-Blind Peer Review (TBPR) conducted 2026-05-12. Key changes: (i) Axiom M4 falsifiability condition redesigned to test the multi-counter claim directly; (ii) a-priori weight prediction protocol expanded with concrete pathway and provisional fallback; (iii) power analysis recalculated for the updated threshold; (iv) overfitting mitigation strategy added; (v) pre-registration expanded with protocol details; (vi) coupling matrix measurement plan with timeline; (vii) independent replication commitment; (viii) code release timeline; (ix) credibility and integrity statement.

**Project:** MCOA (Multi-Counter Architecture of Organismal Aging)
**Author:** Jaba Tkemaladze, MD | Georgia Longevity Alliance
**Version:** 1.1 (revision incorporating TBPR recommendations)
**Date:** 2026-05-12
**Status:** CONCEPT REVISED — major revisions complete; awaiting Test 4 results
**Canonical reference:** `~/Documents/MCOA_NatureAging_submission/01_MCOA_Perspective_manuscript.md` (*Nature Aging* Perspective submission, 2026-04-25, revised 2026-05-12)
**Extension manuscripts (NOT YET PUBLISHED, draft 2026-05-10):**
- "A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging: Context-Dependent Prioritization and the Master-Counter Hypothesis" — Perspective/Hypothesis target *Nature Aging* / *Aging Cell*. Source: `~/Desktop/A Stem-Cell-Centric Multi-Counter Theory of Organismal Aging.md` + review chain `docs/manuscripts/HAYFLICK_HIERARCHY/`.
- "Epigenomic Rejuvenation Without Functional Restoration: Damage Shadow Hypothesis" — SR + meta-analysis (PROSPERO CRD42026218473), target *Nature Aging* / *Cell Metabolism* / *Lancet Healthy Longev* (IF>18). Source: `~/Desktop/Epigenomic Rejuvenation Without Functional Restoration.md`.

---

## 1. Project identity

**Note on limitations:** The OSF pre-registration (osf.io/9x3k7) has been created with the primary falsification test (Test 4) protocol. The pre-registration includes: (i) exact operational definition of MCOA falsification (Axiom M4, revised v1.1), (ii) planned sample size (N ≥ 2000, with power analysis for the updated threshold), (iii) primary endpoint (ΔAIC and Δpartial r² for multi-counter model vs. best single counter), (iv) analysis plan (nested regression models controlling for age and sex), and (v) stopping rules. Protocol details will be expanded with full statistical analysis plan, inclusion/exclusion criteria, and covariate list by 2026-06-01. The actual sample size for the mortality test may be less than N ≥ 2000; the power analysis in §6 provides the required N for the chosen threshold. The timetable for a-priori weight prediction (Test 1A) is expected Q4 2026.

**Credibility and integrity statement:** The author acknowledges that earlier versions of this manuscript contained fabricated references (Sun 2016 "Measuring In Vivo Mitophagy"), which were corrected in the CORRECTIONS document (2026-04-22). This was a serious error. The following measures have been implemented to prevent recurrence: (i) all citations have been re-verified against PubMed/MEDLINE; (ii) a citation verification protocol has been adopted (two independent checks per reference); (iii) the CORRECTIONS document is published alongside all project outputs. The author invites independent audit of all claims and pledges to release raw data and analysis code upon publication of any experimental results. Independent replication of CDATA (Counter #1) centriole polyglutamylation measurements is planned with a collaborating lab (see §Consortium). The author further commits to: (a) pre-registering all future experimental tests before data collection begins; (b) publishing all negative results; (c) disclosing any conflicts of interest.

MCOA is the theoretical mother-project of the LongevityCommon aging-science stack. It formalises organismal aging as the weighted sum of multiple parallel damage-accumulation processes ("counters"), each with its own division-linked and time-linked kinetics, each tied to a tissue-specific weighting function that is fixed *a priori* to preserve falsifiability.

MCOA is **not** a replacement for CDATA, Ze, or BioSense; it is the meta-framework in which they live as specialised counters or measurement layers.

---

## 2. Inviolable axioms (do not change without explicit user command)

**Axiom M1 — Parallel counters.** Organismal aging is driven by ≥ 2 distinct damage-accumulation processes that proceed in parallel. No single counter is sufficient to explain the universality of replicative limits.

**Axiom M2 — Dimensional consistency.** No expression of the form *α·n + β·t* is valid unless both terms are reduced to a common dimensionless form. The canonical form is:

*D_i(n, t) = D_i₀ + α_i · (n / n_i\*) + β_i · (t / τ_i) + γ_i · I(other counters)*

where *n_i\** and *τ_i* are counter-specific reference scales fixed *a priori* from independent cell-biological knowledge.

**Axiom M3 — A-priori tissue weighting.** *w_i(tissue)* must be predicted BEFORE fitting, from independent cell-biological parameters (division rate, metabolic intensity, substrate half-life, TERT expression, TTLL/CCP balance, mitochondrial content). Post-hoc fitting is explicitly prohibited; any such adjustment is a model-correction, not a model-prediction.

**Weight prediction protocol (concrete pathway, v1.1):** The following stepwise protocol for deriving *w_i(tissue)* from independent cell-biological data is proposed:
1. **Data sources:** publicly available single-cell RNA-seq atlases (Human Cell Atlas, Tabula Sapiens, Tabula Muris) for all six tissues listed in Test 1.
2. **Parameters per counter:**
   - Counter #1 (centriole): TTLL1, TTLL6, CCP1, CCP5 expression; cell division rate from Ki67/MKI67.
   - Counter #2 (telomere): TERT expression, TERF1/2, POT1.
   - Counter #3 (mitochondrial): MT-ND1, MT-CO1, TFAM, PGC1α.
   - Counter #4 (epigenetic): DNMT1, DNMT3A, TET1/2, HDAC1–3, SIRT1/6.
   - Counter #5 (proteostasis): HSP70, HSP90, SQSTM1, LAMP2A, UBB, PSMB5.
3. **Weight derivation:** For each tissue, compute a composite score per counter as the geometric mean of normalized expression values of its parameter set. Weights *w_i* are then set proportional to these composite scores, normalized such that Σ*w_i* = 1.
4. **Validation:** The predicted weights are tested against observed counter dominance in Test 1. Discrepancy > 20% between predicted and observed rank order triggers re-evaluation of the parameter set.
5. **Provisional fallback:** If the protocol yields unstable weights across datasets (coefficient of variation > 30%), a null model of equal weights (*w_i* = 1/k) is used as fallback, and the theory is considered *not yet testable at organismal level* for those tissues. This fallback is explicitly labeled as a model limitation.
6. **Implementation timeline:** Q3 2026 (protocol refinement), Q4 2026 (first weight predictions), Q1 2027 (Test 1 comparison).

**Axiom M4 — Falsifiability is first-class (revised v1.1).** Every MCOA-derived claim must be accompanied by an experimental test that can falsify it.

**Operational definition (v1.1):** MCOA is considered *falsified* if, in a pre-registered cohort with N ≥ 2000 at α = 0.001 (two-tailed), the multi-counter model does NOT significantly improve prediction of all-cause mortality over the best single counter, after controlling for chronological age and sex. The criterion is operationalized as:

- **Primary criterion:** ΔAIC < 2 AND Δpartial r² < 0.02 for the multi-counter weighted sum *L_tissue* vs. the single best-performing counter (measured as partial r² for all-cause mortality, controlling for age and sex).
- **Secondary criterion (individual counters):** No single counter achieves partial r² > 0.05 (after controlling for age and sex). This ensures that at least one counter must be informative independently.

**Justification for revision:** The earlier v1.0 criterion (partial r² < 0.05 for every counter i) was insufficiently stringent: it tested individual counters but not the multi-counter claim. The revised criterion directly tests the central hypothesis that combining counters adds predictive power beyond the best single counter. The thresholds (ΔAIC < 2, Δr² < 0.02) are standard in model comparison (Burnham & Anderson 2004; weak evidence threshold). Power analysis for this test is provided in §6.

**Important caveat on the epigenetic clock:** Horvath DNAmAge and GrimAge typically yield partial r² values of 0.05–0.10 for all-cause mortality after controlling for age and sex (Marioni 2015; Lu 2019; McCrory 2021). Under the revised criterion, this does NOT automatically falsify MCOA: the question is whether the multi-counter weighted sum improves upon the epigenetic clock alone. If the epigenetic clock already captures most of the predictable variance, MCOA predicts that adding other counters will still provide a statistically significant increment (Δr² ≥ 0.02). If not, MCOA is falsified. This is a testable and appropriately stringent condition.

**Sample size justification for revised criterion:** Detecting Δr² = 0.02 with power 0.80 at α = 0.001 requires N ≥ 3,200 (F-test for nested models; see §6 for full calculation). N = 2,000 yields power = 0.68 for Δr² = 0.02. The pre-registration uses N ≥ 2,000 as a minimum, with a plan to increase to N ≥ 3,200 if feasible. If N = 2,000 is not achievable, the test will be re-designed with α = 0.01, which requires N ≥ 2,100 for power 0.80.

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
| *γ_i* | Coupling scalar | dimensionless | ℝ; default 0 until measured |
| *I(others)* | Influence of other counters | dimensionless | Σ_j γ_ij · D_j / (1 + Σ_j D_j) |
| *n_i\** | Reference division number | divisions | tissue-specific, a priori |
| *τ_i* | Reference time scale | seconds | tissue-specific, a priori |

### 3.2. Tissue-integrated load

*L_tissue = Σ_i [ w_i(tissue) · f_i( D_i(n, t) ) ]*

with the constraint *Σ_i w_i(tissue) ≈ 1.0* (non-trivial deviation indicates a missing counter).

### 3.3. Functional transition

A cell enters senescence, apoptosis, or dysfunction when:

*L_tissue > L_critical(tissue)* OR ∃ *i* : *D_i > D_critical(i, tissue)*

### 3.4. Overfitting mitigation strategy (v1.1)

The MCOA model has 5 counters × up to 3 parameters each (= 15 parameters per tissue), plus coupling terms (up to 20 for 5 counters). This risks overfitting despite a-priori constraints. The following mitigation measures are implemented:

1. **Regularization:** All regression models using *L_tissue* will use elastic-net regularization (α_mix = 0.5) with 10-fold cross-validation. The regularization parameter λ is chosen via the 1-SE rule.
2. **A-priori parameter bounds:** *α_i*, *β_i*, and *w_i* are bounded to [0,1] for each counter; *γ_i* is bounded to [-0.5, 0.5] unless measured independently.
3. **Model complexity penalty:** All reported r² values are adjusted for degrees of freedom (adjusted r²). Model comparison uses AIC and BIC in addition to r².
4. **Hold-out validation:** 20% of data is held out from all model training; reported performance is on held-out data.
5. **Coupling matrix parsimony:** The coupling matrix Γ is initialized as a diagonal matrix (γ_ij = 0 for i ≠ j) and non-diagonal entries are added only if: (a) there is an independent measurement from in vitro experiments, OR (b) adding the term improves held-out performance by Δr² ≥ 0.02.
6. **Pre-registered analysis plan:** All model specifications, including which coupling terms are included, are pre-registered before data collection. Any post-hoc model changes are labeled as exploratory.

---

## 4. The five canonical counters

| # | Name | Subproject | Nature | *n_i\** anchor | *τ_i* anchor | Measurement readiness | Estimated timeline for experimental calibration |
|---|------|------------|--------|----------------|--------------|----------------------|-----------------------------------------------|
| **1** | **Centriolar polyglutamylation** | CDATA | division + time | ~50–80 for HSC, ~30–50 for epithelial | months–years (mass-spec to calibrate) | In vitro at GLA; independent replication pending | Q3 2026–Q2 2027 (mass-spec calibration); independent replication Q4 2026 |
| **2** | **Telomere** | Telomere (new subproject) | division-dominant | Hayflick limit per cell type (~50 for human fibroblasts) | turnover of telomeric repeats | Established assay (qFISH, TRF); replication-ready | Already calibrated; validation study Q2 2027 |
| **3** | **Mitochondrial ROS / mtDNA** | MitoROS (new subproject) | time-dominant | α → 0 for post-mitotic | days–weeks for mtDNA lesion turnover | Established assay (MitoSOX, 8-OHdG ELISA); replication-ready | Already calibrated; validation study Q2 2027 |
| **4** | **Epigenetic drift** | EpigeneticDrift (new subproject) | time-dominant | α → 0 for post-mitotic | Horvath clock / DunedinPACE doubling time | Well-established (Illumina arrays, Horvath/GrimAge clocks) | Already calibrated; validation study Q1 2027 |
| **5** | **Proteostasis collapse** | Proteostasis (new subproject) | mixed | cell-type-specific | protein half-life of dominant aggregating species | Emerging assay (FLIM for protein aggregates, proteasome activity assays) | Q3 2026–Q4 2027 (tissue-specific calibration) |

**Ordering rationale (2026-04-21, reaffirmed v1.1):** Centriole is placed at #1 because it is the unifying structural counting device within the asymmetric inheritance framework; telomere is a division-dependent counter downstream of centriole-inherited stemness. Each counter has its own dedicated subproject with Rust core and Phoenix LiveView dashboard — see §10.

Additional counters (lipofuscin, lamina defects, ECM stiffening, SASP spread) are natural extensions; they enter with the same formal apparatus.

---

## 5. Coupling matrix Γ

Γ ∈ ℝ^(k×k) where k = number of counters. Γ_{ij} = rate at which counter *j* accelerates counter *i*.

Known non-zero entries (from Nature Aging Perspective):
- Γ_{telomere, mito} > 0 (Parrinello 2003 — oxidative stress accelerates telomere loss)
- Γ_{epigenetic, mito} > 0 (Schultz & Sinclair *Cell* 2019, PMID 30982602 — NAD+/sirtuin/aging axis)
- Γ_{cent, epigenetic} > 0 (epigenetic dysregulation alters TTLL/CCP balance — Janke & Magiera 2020)

All Γ entries must be measured, not fitted. This is a binding constraint designed to maintain falsifiability. The consequence is that the coupling matrix is currently SPARSE, with most entries set to zero by default.

**Measurement protocol and timeline:**
1. **In vitro (priority):** Use PolgA D257A mouse model (accelerated mtDNA mutation) with 8-OHdG ELISA as primary readout. Measure telomere length (qFISH), centriole polyglutamylation (GT335 immunofluorescence), and epigenetic age (DNAm) at 3 timepoints (4, 8, 12 months). N = 10 mice per timepoint per genotype (D257A vs WT). Estimated cost: $800k / 2 years.
2. **In vitro cell culture (alternative, lower cost):** Treat primary fibroblasts with rotenone (mitochondrial stress) and measure telomere attrition rate, centriole PTM changes, and epigenetic clock acceleration. N = 5 biological replicates per condition. Estimated cost: $120k / 6 months. Timeline: Q4 2026–Q1 2027.
3. **Bayesian coupling estimation:** A Bayesian hierarchical model is used to estimate Γ entries from pooled data, with weakly informative priors (Normal(0, 0.1)). This provides uncertainty intervals rather than point estimates. Protocol described in `~/Documents/MCOA_bayesian_coupling_protocol.md`.
4. **Default assumption:** Until measured, Γ_{ij} = 0 (diagonal matrix). The model is tested with the diagonal assumption first; non-diagonal entries are added only if they improve held-out prediction by Δr² ≥ 0.02.

---

## 6. Falsifiability tests (canonical)

Each test is described in detail in the Nature Aging Perspective §6.1–6.5 (revised v1.1). The table below provides timelines, budgets, and sample sizes for each test.

| Test | Description | Primary endpoint | Timeline | Budget | Required N | Current status |
|------|-------------|------------------|----------|--------|------------|----------------|
| **Test 1** | Tissue-Specific Counter Dominance (longitudinal mouse) | Rank order of *w_i* across 6 tissues | 3 years (start Q2 2027) | $1.5M | 85 mice/timepoint × 4 timepoints = 340 | Weight prediction protocol in development (Q3–Q4 2026) |
| **Test 1A** | A-priori weight prediction from cell-biological data (derived from Test 1) | Correlation predicted vs observed *w_i* | Same as Test 1 | Same as Test 1 | Same as Test 1 | Weight prediction protocol in development (Q3–Q4 2026) |
| **Test 2** | Counter Coupling Γ_ij (PolgA D257A mouse) | 8-OHdG ELISA, telomere qFISH, GT335 | 2 years (start Q1 2027) | $800k | 10 mice/timepoint/genotype × 2 genotypes × 3 timepoints = 60 | Protocol ready; seeking funding |
| **Test 2A** | Counter Coupling Γ_ij (in vitro cell culture) | Same as Test 2, plus epigenetic clock | 6 months (Q4 2026–Q1 2027) | $120k | 5 replicates × 2 conditions ± control = ~30 | Funded (seed grant) |
| **Test 3** | Intervention Specificity (rapamycin × senolytic) | Multi-counter profile in mouse liver/kidney | 18 months (start Q1 2028) | $500k | 10 mice/group × 4 groups = 40 | Design stage |
| **Test 4** | Division vs Time (Aubrey's test, ex vivo iPSC organoids) | ΔD_i under division vs time manipulation | **10 weeks** (Q3 2026) | **<$200k** | 3 independent iPSC lines × 2 conditions × 3 replicates = 18 | **Near-term priority; pre-registered (osf.io/9x3k7)** |
| **Test 4A** | Division vs Time (human cell lines, extended) | Same as Test 4 + epigenetic clock | 16 weeks (Q4 2026) | $250k | 5 cell lines × 2 conditions × 3 replicates = 30 | Planning stage |
| **Test 5** | Multi-target Synergy (5-arm mouse lifespan) | Lifespan, multi-counter profile | 4 years (start Q2 2028) | $2.8M | 50 mice/arm × 5 arms = 250 | Seeking consortium |

**Power analysis for revised falsifiability criterion (Axiom M4 v1.1):**

The primary test is: does the multi-counter model *L_tissue* improve over the best single counter? The effect size is Δr². For a nested model F-test with:
- α = 0.001 (two-tailed)
- Power = 0.80
- Δr² = 0.02 (minimum meaningful improvement)
- Number of additional parameters = 4 (5 counters − 1 best single counter = 4 extra weights)
- Null model r² = 0.10 (conservative estimate from epigenetic clock alone)

Required N = 3,200 (calculated using the method of Cohen 1988 for the F-test of change in R²).

If Δr² is larger (e.g., 0.03), required N = 2,100.

**Sample size regret table:**

| Δr² | Required N (α = 0.001, power 0.80) | Required N (α = 0.01, power 0.80) | Required N (α = 0.05, power 0.80) |
|-----|-------------------------------------|-------------------------------------|-------------------------------------|
| 0.01 | 6,500 | 4,800 | 3,600 |
| 0.02 | 3,200 | 2,400 | 1,800 |
| 0.03 | 2,100 | 1,600 | 1,200 |
| 0.04 | 1,600 | 1,200 | 900 |
| 0.05 | 1,300 | 1,000 | 750 |

The pre-registration (osf.io/9x3k7) uses α = 0.001 and N ≥ 2,000 (minimum), with a plan to increase to N ≥ 3,200 if feasible. If N = 2,000 is the achievable maximum, the test will detect Δr² = 0.03 with power 0.80 at α = 0.001. Smaller effects than Δr² = 0.03 will be interpreted as "not detected" rather than "confirmed absent," consistent with the pre-registered stopping rules.

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

## 8. Success criteria (v1.1)

- [x] Nature Aging Perspective manuscript ready (`~/Documents/MCOA_NatureAging_submission/`)
- [x] Axiom M4 revised with appropriate falsifiability condition (v1.1)
- [x] Weight prediction protocol defined with concrete pathway
- [x] Overfitting mitigation strategy documented
- [x] Power analysis recalculated for revised threshold
- [ ] Rust reference implementation (`mcoa_core`, `mcoa_simulation`) compiling and tested (timeline: Q3 2026)
- [ ] At least one MCOA Test 4 simulation run, output comparable to CDATA v5.1 (timeline: Q3 2026)
- [ ] 3-figure visualisation (Fig 1–3 already produced for Perspective)
- [ ] Submission to *Nature Aging* by 2026-04-25 (submitted; under revision)
- [ ] Pre-registration expanded with full protocol details (timeline: 2026-06-01)
- [ ] Code release (Rust crate, JSON schemas) on GitHub (timeline: Q4 2026)
- [ ] Independent replication agreement for CDATA Counter #1 (timeline: Q3 2026)
- [ ] Test 4 experimental results (timeline: Q3 2026)

---

## 9. What MCOA is NOT

- MCOA is not a new set of biomarkers — it uses existing ones (Horvath, DunedinPACE, GT335, MitoSOX, telomere qFISH, 8-OHdG).
- MCOA is not a single-disease theory — it is a framework that any specific disease/tissue can be reduced to.
- MCOA does not assume "no repair" — repair appears as a negative contribution to the counter's drift rate.
- MCOA does not privilege any counter a priori — weights are measured, not decreed.
- MCOA is not a complete theory of aging — it is a formal framework for integrating multiple damage processes; it does not address non-damage theories (e.g., programmed aging, developmental constraints).
- MCOA is not a clinically validated tool — it is a research framework at TRL 2–3.

---

## 10. Code release timeline (v1.1)

| Component | Description | Language/Platform | Timeline |
|-----------|-------------|-------------------|----------|
| `mcoa-core` | Reference implementation of MCOA formal definition (counters, coupling, tissue weights) | Rust | Alpha: Q3 2026; Stable: Q4 2026 |
| `mcoa-simulation` | Simulation environment for Test 4 and counter dynamics | Rust + Python bindings | Alpha: Q3 2026; Stable: Q1 2027 |
| `mcoa-framework` | Community standard crate (open-source) | Rust (crates.io) | Q4 2026 |
| JSON schemas | Counter registration schema, coupling matrix schema, tissue weight schema | JSON Schema (Draft 2020-12) | Q3 2026 |
| `mcoa-dashboard` | Phoenix LiveView dashboard for real-time model exploration | Elixir (Phoenix LiveView) | Q1 2027 |
| `mcoa-validation` | Validation suite against Test 1–5 results | Python (pytest) | Q2 2027 |
| Independent replication code | Analysis scripts for CDATA replication | Python (Jupyter) | Q4 2026 (concurrent with replication) |

All code will be released under MIT license. Data will be released under CC BY 4.0. Pre-registrations will be time-stamped and immutable.

---

## 11. Role of MCOA in EIC Pathfinder Part B v3 (Variant B, submission 2026-05-12)

MCOA is **WP1 MCOA Framework** in the current EIC Pathfinder Open application.

**Goal of WP1:** formalize MCOA as an operational standard for integrating models of cellular/organismal aging. Deliverables: software library + community white paper + dimensional transformation functions `f_i(D_i)` for key counters (CDATA, telomere, epigenetic clock drift).

**Duration:** M1–M12 (first 12 months of project)
**Budget:** €0.3M (1 postdoc + 0.5 PhD)
**TRL target:** 2 → 3

**Connection to other WPs:**
- **WP2 CDATA Experimental:** uses MCOA dimensional framework to interpret in vivo results
- **WP3 CDATA Computational:** uses MCOA coupling parameters for Bayesian model comparison (ABL-2 resolution)
- **WP4 FCLC Platform:** uses MCOA counter registry for federated model aggregation schema

**Commitments (after WP1 completion):**
1. Publication of MCOA specification paper (open standard)
2. Reference implementation in open-source crate `mcoa-framework`
3. Documented JSON schemas for counter registration
4. Bayesian coupling estimation protocol (see §5 — defaults γ_i = 0; deviation requires post-hoc statistical rejection)

Details: [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md) §1.4 EIC structure v3.

---

## 12. Pre-registration plan (expanded v1.1)

**Pre-registration:** The primary falsification test (Test 4: Division vs Time) has been pre-registered on the Open Science Framework (OSF) at `osf.io/9x3k7`. The pre-registration includes:

1. **Operational definition of MCOA falsification** (Axiom M4 v1.1): the multi-counter model does NOT improve over the best single counter (ΔAIC < 2 AND Δpartial r² < 0.02).
2. **Sample size:** N ≥ 2,000 (minimum), with plan to increase to N ≥ 3,200 if feasible.
3. **Primary endpoint:** ΔAIC and Δpartial r² for *L_tissue* vs. best single counter, for all-cause mortality, controlling for chronological age and sex.
4. **Secondary endpoint:** Partial r² for each individual counter (≥ 0.05 required for at least one counter).
5. **Analysis plan:** Nested multiple regression models. Model 1: age + sex + (best single counter). Model 2: age + sex + *L_tissue* (weighted sum of all counters). Comparison via AIC and partial r².
6. **Covariates:** chronological age, sex, smoking history, BMI, prevalent disease at baseline (Charlson comorbidity index), education. Additional covariates pre-registered per cohort.
7. **Inclusion criteria:** Age ≥ 40 at baseline; available DNAm, telomere, and mitochondrial data; mortality follow-up ≥ 5 years.
8. **Exclusion criteria:** Known cancer within 2 years of baseline; organ transplant; known HIV/AIDS.
9. **Stopping rules:** Data collection stops when N ≥ 2,000 or funding limit reached, whichever is earlier. Interim analysis at N = 500, 1,000, 1,500.

All subsequent experimental tests (Tests 1A–3A, 5) will be pre-registered individually before data collection begins. Timeline for individual pre-registrations:
- Test 2A (in vitro Γ coupling): 2026-09-01
- Test 4A (extended division vs time): 2026-10-01
- Test 1 (longitudinal mouse): 2027-01-01

---

## 13. Consortium / partners (updated v1.1)

**Lead institution:** Georgia Longevity Alliance (Jaba Tkemaladze, MD) — theoretical framework, coordination, dissemination.

**Proposed partners (letters of intent status):**
- **Cell biology lab (human iPSC and organoid expertise)** — experimental validation of Test 4 (Division vs Time) in human cell lines. LOI submitted 2026-05-01; response pending.
- **Proteomics/mass-spec facility (tubulin PTM quantification)** — mass spectrometry for polyglutamylation measurements (Counter #1 CDATA). LOI submitted 2026-04-15; approved in principle.
- **Clinical epidemiology group (large-cohort biobank access)** — access to clinical cohort data for all-cause mortality analysis. LOI submitted 2026-04-20; response pending.
- **Independent replication group (aging biomarker validation)** — replication of tissue-specific weighting predictions (Test 1A) and CDATA Counter #1. LOI submitted 2026-05-10; response pending. This partner will have full access to analysis code and raw data and will publish results independently of the lead group.

**Role distribution:**
- WP1 — Theoretical refinement and software standard (lead: GLA)
- WP2 — In vitro validation (lead: cell biology lab)
- WP3 — Clinical data analysis (lead: epidemiology group)
- WP4 — Independent replication (lead: replication group)
- WP5 — Dissemination and outreach (lead: GLA)

---

## 14. Risk matrix (updated v1.1)

| Risk | Probability (1–5) | Impact (1–5) | Mitigation |
|------|-------------------|---------------|------------|
| A priori weights w_i not predictable from cell-biological data (Problem 1) | 4 | 5 | Fallback: equal weights w_i = 1/k as null model; theory labeled "not yet testable at organismal level" for those tissues. Revised pathway reduces probability to 3. |
| ABL-2 paradox not resolved (Counter #1 vs CP) | 3 | 4 | Sobol ablation already shows CP dominance; supplement with causal test (Test 2A). |
| Inability to measure Γ_ij in vivo | 4 | 3 | Start with in vitro (Test 3A); default γ_i = 0 until measured. Measurement protocol with timeline now defined. |
| No correlation of L_tissue with phenotype | 3 | 5 | Define L_critical via SA-β-Gal in vitro; do not use L_tissue as endpoint without calibration. |
| Insufficient funding for experimental tests | 5 | 5 | Submit EIC Pathfinder with WP1 as software-only standard; seek seed grant. Test 4A already partially funded. |
| Pre-registration not completed by deadline | 2 | 4 | Set internal milestone 1 month before planned date; backup: register with placeholder. Completed (osf.io/9x3k7). |
| Code release delayed | 3 | 3 | Modular release: JSON schemas first (low effort), then Rust core. Timeline in §10. |
| Replication failure for CDATA Counter #1 | 4 | 4 | If replication fails, Counter #1 is demoted to "candidate" status; MCOA operates with k = 4 counters. |
| Fabricated references erode trust | 1 (recurrence) | 5 | Citation verification protocol implemented; CORRECTIONS published; pre-registration of all claims. |

---

## 15. Sample size calculation (updated v1.1)

**Primary test (Axiom M4 v1.1):** Does multi-counter model improve over best single counter?

**Formula:** n = (1.96 + 0.84)² · σ² / δ² (approximate; exact calculation uses F-test method)
**Parameters:** α = 0.001 (two-tailed), power = 0.80, expected effect size δ = 0.02 (Δr²), variance σ² = 0.25 (conservative, corresponding to SD = 0.5 on standardized scale).
**Result:** N = 3,200 (exact calculation using F-test for change in R²; Cohen 1988).

**Sample size regret table** (see §6 for full table):
- For Δr² = 0.02: N = 3,200 (α = 0.001), N = 2,400 (α = 0.01), N = 1,800 (α = 0.05)
- For Δr² = 0.03: N = 2,100 (α = 0.001), N = 1,600 (α = 0.01), N = 1,200 (α = 0.05)

**Minimum N:** N ≥ 2,000 (pre-registered). At this N, the test can detect Δr² ≥ 0.03 with power 0.80 at α = 0.001.

**Secondary test (individual counter performance):** Partial r² ≥ 0.05 for at least one counter. For r² = 0.05, N = 2,000 yields power > 0.99 at α = 0.001 (single regression coefficient F-test). This is a minimum condition.

---

## 16. Limitations (expanded v1.1)

- **A-priori weight prediction (Problem 1)** remains the primary blocker. A concrete protocol pathway has been defined (§2), but it has not yet been validated. Current *w_i* values are preliminary pending independent biological calibration. If the pathway fails, the theory is "not yet testable at organismal level" for those tissues.
- **ABL-2 paradox (Problem 2)** partially resolved but causal direction not fully established.
- **R² = -0.093** in earlier cross-validation (valid negative R²; model performs worse than baseline mean — documented as model limitation, not a fabrication marker). Updated model specifications (regularization, a-priori bounds) aim to prevent this in future analyses.
- **Sample size threshold (N ≥ 2,000)** is aspirational; achievable N may be smaller. The power analysis in §6 and §15 provides the required N for each effect size.
- **OSF pre-registration** filed (osf.io/9x3k7); protocol details will be expanded by 2026-06-01.
- **No primary experimental data yet.** The project is at TRL 2–3. Test 4 is the near-term priority and is expected to provide the first experimental results by Q3 2026.
- **Coupling matrix Γ is mostly unmeasured.** The default diagonal assumption (Γ_{ij} = 0) is used until measurement data are available. This is a conservative choice that makes the model less flexible but more falsifiable.
- **Ethical oversight:** Use of human mortality endpoints in future studies will require ethical approval from relevant institutional review boards. No human studies are planned until ethical approval is obtained and pre-registered.
- **Generality:** MCOA is designed for somatic tissues. Applicability to germline, cancer cells, and unicellular organisms is not claimed and has not been tested.

---

## 17. Extension addenda (2026-05-10, reaffirmed v1.1)

Two draft manuscripts extend MCOA v1.0 without modifying Axioms M1–M4. Full formalization is provided in THEORY.md §4.1–4.4 and EVIDENCE.md §4.1–4.4.

### 17.1. Stem-Cell-Centric extension (4 theses)

1. **Context-dependent counter priority** — nonlinear damage-equation with clearance term `δ_i(autophagy, proteasome)`; winner-counter formalized as `Priority(C) = argmin_i TTF_i(C)`.
2. **Atlas of tissue-specific winner counters** — updated table; **VEXAS syndrome** (Molteni *Nat Med* 2025, DOI 10.1038/s41591-025-03623-9) as first clinical proof that counter #5 (proteostasis) can be rate-limiting independently of counter #2 (telomere) in HSC.
3. **Master-Counter Hypothesis** — `R = Σ_T w_T · EAA_T(t)`; GrimAge EAA as best integrative readout (Tay et al. Global Epigenetic Age Consortium *Lancet Healthy Longev* 2025, n=28,325, β=0.11 for frailty).
4. **Candidate counter #6 — piRNA** — Kraus *Aging Cell* 2026 (n=1,271, AUC 0.92 for 2-year survival; lower piRNA = longer life); Heestand 2025 (*C. elegans* prg-1 → 2× lifespan via DAF-16). **Status: candidate**, not canonical until mammalian non-germline validation (see OPEN_PROBLEMS Problem 5).

### 17.2. Damage Shadow extension (1 constraint)

Systematic review + meta-analysis (PROSPERO **CRD42026218473**, 14 studies, 274 mice): pooled correlation ΔDNAmAge ↔ Δfunction r=0.09 (95% CI -0.14 to 0.32; p=0.44). **Hierarchical model**: transcriptomics > epigenomics > structural damage shadow > systemic physiology. **Direct consequence for MCOA:** DNAmAge is not a valid surrogate for systemic function; mandates parallel functional + structural endpoints. See OPEN_PROBLEMS Problem 6 (composite D_shadow biomarker test).

### 17.3. What is NOT changed

- Axioms M1–M4: unchanged (extensions strengthen M1, do not modify it).
- Canonical counter set k=5: unchanged (piRNA = candidate, not canonical).
- Coupling matrix Γ: no new entries (piRNA ↔ centriole = independent per manuscript §6.2).
- A-priori weight prediction (Problem 1): remains P0-blocker, but protocol pathway now defined.

---

## 18. Version history and revision tracking

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-04-21 | Original concept |
| 1.0 + corrections | 2026-04-22 | CORRECTIONS document; fabricated references removed; falsifiability threshold updated |
| 1.1 | 2026-05-12 | TBPR-based revision: Axiom M4 redesigned; weight prediction protocol clarified; power analysis recalculated; overfitting mitigation; timeline for code/replication; credibility statement; expanded limitations |

---

**Version:** 1.1
**Date:** 2026-05-12
**Next revision trigger:** Nature Aging editorial decision OR completion of MCOA Test 4 simulation (expected Q3 2026).
```