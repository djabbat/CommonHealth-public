# TBPR for Project: MCOA

**Project:** MCOA (Multi-Counter Architecture of Organismal Aging)  
**Version:** 1.0 (extension addenda 2026-05-10)  
**Author:** Jaba Tkemaladze, MD  
**Date of Review:** 2026-05-13  
**Review Type:** Triple-Blind Peer Review  

---

## Executive Summary

This project proposes a formal meta-framework for aging research called MCOA (Multi-Counter Architecture of Organismal Aging). It defines aging as a weighted sum of parallel damage-accumulation processes (“counters”) with fixed a-priori tissue-weighting and falsifiability thresholds. The framework is supported by five canonical counters (centriolar polyglutamylation, telomere, mitochondrial ROS/mtDNA, epigenetic drift, proteostasis collapse) and includes coupling terms, a pre-registration plan (OSF), and extension manuscripts (stem-cell-centric and damage-shadow hypotheses). The document is well-structured, self-critical (lists limitations, acknowledges unresolved problems), and commits to high standards of reproducibility (pre‑registration, sample size calculations, multiple falsification tests). However, several foundational issues remain: (i) a‑priori weight prediction (Problem 1) is still uncalibrated; (ii) the coupling matrix Γ is largely assumed zero until measured; (iii) the “ABL‑2 paradox” is only partially resolved; (iv) the extension addenda introduce a sixth candidate counter (piRNA) without mammalian non‑germline validation, risking dilution of the core framework. The overall assessment is positive but conditional on near‑term experimental validation (particularly Test 4) and resolution of the weight prediction problem.  

**Combined score (MIN of three reviewer sums) = 39/55**

---

## Reviewer A – Domain Expert (Fact‑check, Methodology, PI)

### Score Table (1–5, 1 = lowest, 5 = highest)

| # | Criterion | Score | Brief Justification |
|---|-----------|-------|---------------------|
| 1 | Significance of the question | 5 | Addresses a core problem: integration of multiple aging processes. |
| 2 | Novelty of the framework | 4 | First formalised multi‑counter architecture with explicit falsifiability; builds on existing theories. |
| 3 | Clarity of hypothesis | 4 | Clear axioms and mathematical definition; tissue‑integrated load is well‑defined. |
| 4 | Methodological rigor | 4 | Pre‑registration, sample size justification, multiple falsification tests; some parameters (nᵢ*, τᵢ) lack independent calibration. |
| 5 | Feasibility | 3 | Test 4 is tractable; weight prediction (Problem 1) remains a major risk; cost estimates for Tests 1,2,5 are high. |
| 6 | Falsifiability | 5 | Excellent: explicit threshold (R²<0.05 at α=0.001, N≥2000) and pre‑registration. |
| 7 | Data support | 3 | Mostly theoretical; only CDATA has preliminary experimental support (Sobol ablation). |
| 8 | Impact (if true) | 5 | Could transform biomarker integration, clinical trial design, and geroscience. |
| 9 | Reproducibility | 4 | Pre‑registration, open‑source crate planned; some assays (GT335, qFISH) are standardised. |
| 10 | Limitations acknowledged | 5 | Clearly lists unresolved problems and false‑positive risk in coupling matrix. |
| 11 | Overall recommendation | 4 | Strong concept but requires near‑term validation to be taken seriously. |
| **Score Sum** | | **46/55** | |

### Detailed Comments

**Strengths**
- The core mathematical formalism (Axioms M1–M4) is logically consistent and avoids dimensional inconsistencies that plague many aging theories.
- The commitment to falsifiability is exemplary. The operational threshold (R² < 0.05) is stringent but justified by power analysis.
- The decision to require a‑priori weight prediction (Axiom M3) is a crucial guard against overfitting.
- The document honestly identifies the ABL‑2 paradox as partially resolved and lists five open problems – a sign of intellectual integrity.

**Weaknesses**
- **Problem 1 (a‑priori weight prediction) is the single greatest weakness.** Without independent calibration, the entire framework remains a “decorated null hypothesis”. The author states it “remains unresolved” but does not provide a timeline or concrete strategy beyond a vague “seek seed grant”.
- **Coupling matrix Γ is mostly unfilled.** It is acceptable to set γᵢ = 0 by default, but the document claims “All Γ entries must be measured, not fitted.” This is unrealistic in the short term; a Bayesian approach with weakly informative priors would be more practical (and is mentioned in the EIC section, but not in the main text).
- **The extension addenda (stem‑cell‑centric, damage shadow) are interesting but poorly integrated.** The piRNA candidate (counter #6) is proposed based on worm data and a single human study (Kraus 2026). The rule that “no new counters without mammalian non‑germline validation” is stated but not enforced – this risks mission creep.
- **The “R² = -0.093” validation result is interpreted as a model limitation, but it actually indicates that the simple linear model is worse than a constant baseline.** This is a red flag for the additive constant‑terms. The author acknowledges it, but does not explain why such a negative value should not be considered a failure of the framework.

**Recommendations**
1. Prioritise the weight‑prediction calibration (use CDATA data to set w₁, then compare predicted vs. observed tissue dominance in a pilot cohort of N=50 mice).
2. Pre‑register a Bayesian coupling estimation protocol before any Γ measurement; allow weakly informative priors to avoid empty matrices.
3. Require that any extension (e.g., piRNA) is submitted as a separate paper with its own falsification test, not appended to MCOA v1.0.
4. Provide a clear explanation for the negative R² – is it due to wrong scaling, missing counters, or a genuine violation of the linearity assumption?

---

## Reviewer B – Fluff/Impact Auditor

### Score Table (1–5)

| # | Criterion | Score | Brief Justification |
|---|-----------|-------|---------------------|
| 1 | Significance of the question | 5 | Central question in aging; high impact if solved. |
| 2 | Novelty of the framework | 4 | Unique integration; but is it more than a rebranding of known hallmarks? |
| 3 | Clarity of hypothesis | 5 | Very clear; axioms and formal definitions are accessible. |
| 4 | Methodological rigor | 3 | Ambitious but many gaps (see below). |
| 5 | Feasibility | 2 | Budget estimates (Test 1: $1.5M, Test 2: $800k, Test 5: $2.8M) are huge; no funding secured; timeline is optimistic. |
| 6 | Falsifiability | 5 | Gold‑standard; pre‑registration is a strong asset. |
| 7 | Data support | 2 | Almost no experimental data; the main manuscript is a Perspective, not a research article. |
| 8 | Impact (if true) | 5 | Could become the standard framework for geroscience. |
| 9 | Reproducibility | 4 | Pre‑registration helps; but assays are not described in sufficient operational detail. |
| 10 | Limitations acknowledged | 5 | Excellent; no hidden overhype. |
| 11 | Overall recommendation | 3 | High risk, high reward. Not yet ready for implementation without further validation. |
| **Score Sum** | | **43/55** | |

### Detailed Comments

**Strengths**
- The author avoids exaggeration. The document states “MCOA is **not** a replacement for CDATA”, and “MCOA does not privilege any counter a priori” – this is refreshingly honest.
- The integration with EIC Pathfinder (WP1 as a software standard) is a pragmatic route to obtain funding without over‑promising biological results.
- The recognition that “DNAmAge is not a valid surrogate for systemic function” (Damage Shadow extension) is important and counters hype in the epigenetic clock field.

**Weaknesses**
- **Over‑engineering risk.** The formalism is elegant but adds complexity that may not be necessary. For example, the coupling matrix Γ is likely very sparse, and trying to measure each entry in vivo is a multi‑million dollar effort. Why not start with a simpler additive model with a few strong interactions?
- **The “fluff” component is actually low** – the document is refreshingly free of buzzwords. However, the impact language (“could transform clinical trial design”) is unsupported by any user‑needs analysis or stakeholder input.
- **The timeline is unrealistic.** The Perspective was submitted to *Nature Aging* on 2026‑04‑25; editorial decision is pending. Yet the document already lists “success criteria – submission by 2026‑04‑25” as achieved. This is fine, but the next steps (Test 4, weight prediction) have no concrete start date.
- **Budget audibility.** The cost estimates are round numbers and seem pulled from thin air. $1.5M for 3 years for Test 1 (6 tissues, 4 counters, 4 timepoints, N=85/timepoint) is plausible, but no breakdown of personnel, reagents, animal costs is given. For a grant proposal this would be insufficient.

**Recommendations**
1. Moderate the scope: focus first on Counters 1–3 (centriole, telomere, mitochondria) and defer epigentic drift and proteostasis until the core is validated.
2. Provide a detailed financial breakdown for any funding application, with contingency for the most likely risk (weight prediction failure).
3. Engage with a potential user community (e.g., longevity biotechs) to test whether the MCOA output metrics are actually actionable.

---

## Reviewer C – Red Team (Counter‑arguments, Bias, Alternative Explanations)

### Score Table (1–5)

| # | Criterion | Score | Brief Justification |
|---|-----------|-------|---------------------|
| 1 | Significance of the question | 4 | Important but not novel; many have said aging is multi‑cause. |
| 2 | Novelty of the framework | 3 | Formalisation is new, but the underlying ideas (Hallmarks of Aging, multiple damages) are well‑known. |
| 3 | Clarity of hypothesis | 3 | Axioms are clear, but the term “damage” is not defined operationally; what counts as damage per counter? |
| 4 | Methodological rigor | 2 | The “operational threshold” R²<0.05 is arbitrary; why not R²<0.01? The power analysis is done for R²=0.3, but the test is for R²<0.05 – the required N would be much larger. |
| 5 | Feasibility | 2 | The assumption that wᵢ can be predicted from tissue parameters is unsupported; the author admits this is unresolved. |
| 6 | Falsifiability | 4 | Pre‑registration is good, but see power analysis issue below. |
| 7 | Data support | 1 | Zero direct tests of MCOA; all evidence is indirect (CDATA data, literature). |
| 8 | Impact (if true) | 4 | Would be a unifying framework, but high risk of being ignored because it is too complicated. |
| 9 | Reproducibility | 3 | Pre‑registration helps; but the assays (e.g., GT335) have variability and lack standardised protocols. |
| 10 | Limitations acknowledged | 5 | Honest, but the author does not seem to consider the possibility that MCOA is fundamentally wrong (e.g., aging is a single process, not multiple counters). |
| 11 | Overall recommendation | 2 | Premature for a formal peer‑reviewed framework; requires at least one experimental validation. |
| **Score Sum** | | **33/55** | |

### Detailed Comments

**Counter‑Arguments & Bias**
1. **The falsifiability threshold is weak.** The claim “R² < 0.05 for every counter i” is an AND condition. But given five counters, the probability that at least one will be above 0.05 by chance even if all are null is high (using multiple comparisons). The author uses α=0.001, but the power is calculated for a single test. A proper correction (Bonferroni: α = 0.0002) would require N >> 2000. The pre‑registered analysis should specify a hierarchical or omnibus test.

2. **The axiom “no single counter is sufficient” (M1) is a metaphysical statement.** It cannot be falsified because there is always an unknown counter that could be the singular cause. The author tries to escape by requiring “≥2”, but if a single counter explains 90% of variance, MCOA is still technically true but trivial. A better test is whether the best single counter outperforms a weighted sum.

3. **A‑priori weight prediction is a circular dependency.** The author says wᵢ must be predicted from independent data. But those independent data (division rate, metabolic intensity, etc.) themselves contain aging‑relevant changes. For example, TERT expression changes with age; using it as an independent predictor contaminates the weight. There is no guarantee that these “cell‑biological parameters” are time‑invariant across lifespan.

4. **The ABL‑2 paradox resolution is hand‑waving.** The document says “Sobol ablation already shows CP dominance; supplement with causal test (Test 2A)”. But Test 2A is not described in sufficient detail to judge whether it can distinguish between centriole and telomere causality. Without this, the entire Counter 1 claim rests on correlational evidence.

5. **The extension addenda contain contradictions.** The Damage Shadow meta‑analysis finds r=0.09 between ΔDNAmAge and Δfunction. This would imply that Counter 4 (epigenetic drift) has essentially zero correlation with function, yet the MCOA model uses epigenetic drift as a canonical counter. The author acknowledges “DNAmAge not a valid surrogate”, but does not replace Counter 4 with a functional epigenetic metric. This is a fundamental inconsistency.

6. **Biases:**
   - Confirmation bias: the author’s own CDATA work is placed as Counter 1, and the “order of counters” is justified by centriole’s centrality – but no independent ranking method (e.g., based on effect size in existing data) is given.
   - Funding bias: the EIC Pathfinder budget is split with GLA as lead; there is no independent oversight of experimental results.

**Alternative Explanation**
The multi‑counter framework could be perfectly fitted to any dataset by adjusting the weights and coupling integrals, even if aging is a single process with multiple biomarkers. The only way to rule this out is to test the a‑priori predictions. Since those predictions are not yet made, MCOA remains an unfalsifiable framework despite its rhetoric.

**Recommendations**
1. Replace the current falsification test with an omnibus F‑test that compares the multi‑counter model to the best single‑counter model, using nested models and cross‑validation.
2. Conduct a simulation study where artificial data generated from a known single‑counter model is fed into the MCOA pipeline; if the pipeline incorrectly selects multiple counters, the framework’s diagnostic power is proven weak.
3. Require that the weight prediction is done using at least two independent tissues (e.g., liver vs. brain) and the weights are validated on a third tissue before acceptance.
4. Either drop Counter 4 (epigenetic drift) or replace it with a functional readout (e.g., histone mark turnover, not methylation age).

---

## Final Combined Score

| Reviewer | Score Sum | % of 55 | Notes |
|----------|-----------|---------|-------|
| A (Domain Expert) | 46/55 | 84% | Generally positive, but highlights major risk in weight prediction. |
| B (Impact Auditor) | 43/55 | 78% | Cautious due to funding and feasibility gaps. |
| C (Red Team) | 33/55 | 60% | Strong methodological concerns, particularly falsifiability and data support. |
| **Combined = MIN** | **33/55** | **60%** | The weakest link dominates. |

**Overall Verdict:** **Conditional Pass.** The framework is conceptually strong and admirably self‑critical, but it requires at least one experimental validation (pre‑ferably Test 4) and resolution of the a‑priori weight prediction problem before it can be considered a validated theory. The red‑team concerns about the falsification test power and the lack of operational damage definitions must be addressed in a revised version. The project is encouraged to proceed, but with the recommendation to narrow scope and provide a clear timeline for the critical experiments.