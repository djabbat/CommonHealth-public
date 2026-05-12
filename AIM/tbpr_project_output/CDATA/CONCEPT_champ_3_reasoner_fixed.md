# CDATA — Concept v5.4 — Counter #1 (Centriolar) in MCOA

**Status:** Hypothesis framework, pre-experimental. The quantitative model (Cell‑DT v3.0) is **not validated**; its failure (ABL‑2 paradox) is the central finding. This document is restructured as a **hypothesis paper** that uses that failure to generate four testable competing hypotheses.

**Changes from v5.3 (per TBPR 2026‑05‑16):**
1. Tone and logical language further softened — “deductively constrained hypothesis” replaced with “plausibility argument.”
2. Other known ¬R candidates (DNA damage, mitochondrial dysfunction, proteostasis loss) explicitly acknowledged and a rationale for focusing on centrioles provided.
3. Computational model limitations quantified: overfitting noted with effective degrees of freedom; LOO‑CV R² = −0.093 highlighted as model uselessness for prediction, not a paradox.
4. Experimental program includes a lower‑cost pilot (conventional immunofluorescence) before the full STED study.
5. Code release plan moved to “upon submission” (Zenodo DOI provided). Pre‑registration of P0 on OSF.
6. Russian language section retained as a translation footnote for the author’s convenience; English version governs.
7. All “discovery” or “strength” language regarding the ABL‑2 paradox replaced with “indication of model misspecification.”
8. Limitations section expanded with AIC/BIC comparison and independent validation gap.
9. Reproducibility details added: solver, tolerances, parameter boundaries.
10. Four hypotheses described with explicit experimental outcomes that are all publishable.

---

## Executive Summary

CDATA proposes that polyglutamylation (polyGlu) of the mother centriole in stem cells acts as a division‑counting clock, independent of telomeres and ROS. This hypothesis is derived from a **plausibility argument**: because stem cells senesce in hypoxia with active telomerase, a third, non‑renewable (¬R) mechanism must exist. However, this argument does **not** logically force the centriole — several other ¬R candidates exist. We focus on the centriole because it uniquely combines non‑renewability, division‑proportionality, and a known structural clock (polyGlu) that damages ciliary signalling.

We built a quantitative ODE model (32 parameters, Cell‑DT v3.0 Rust simulator) to formalize the hypothesis. **The model fails its primary validation test.** Leave‑one‑out cross‑validation yields a mean `R² = −0.093`, indicating the model’s predictions are worse than a constant baseline — the model is useless for prediction. Sobol sensitivity analysis shows the centriolar damage parameter (`α = 0.0082`) contributes only 22.4% of model variance, while the epigenetic rate parameter (`β_epi = 0.403`) dominates. Ablating the centriolar component **improves** predictive accuracy (R² from 0.778 to 0.833). This is **not** a “paradox” but a clear indication that the model is misspecified: the linear epigenetic term fits the approximately linear target trajectory better than the saturating centriolar damage function.

This failure does **not** disprove the centriolar hypothesis — it reveals that the **current model** cannot distinguish the two mechanisms. We propose four testable biological alternatives (H1–H4) and an experimental program to resolve them. All four outcomes advance understanding.

**Corrected statement:** CDATA is a plausibility argument for a centriolar ¬R clock. The computational model is insufficient; the experimental program is designed to test the core predictions directly.

---

## Central Argument (Revised for Tone)

**Axiom 1 — Hayflick limit in hypoxia with telomerase**  
Stem cells in hypoxic (1–3% O₂) cultures with active telomerase still reach a Hayflick limit (~200 PD) (Peters‑Hall et al., *FASEB J* 2020; PMID 31914653).

**Axiom 2 — Defective ciliary signalling from the old mother centriole**  
The mother centriole, inherited by the stem‑cell daughter, becomes the basal body of the primary cilium. PolyGlu accumulation on the mother centriole impairs Hedgehog/Wnt signalling, reducing niche‑dependent self‑renewal.

**Axiom 3 — Reduced stem‑cell division rate with an old centriole**  
Impaired ciliary signalling lengthens interdivision time and increases symmetric differentiating divisions, depleting the pool.

**¬R Argument (plausibility, not proof):**

```
PREMISE 1: If telomere + ROS theories were complete, hTERT + hypoxia → infinite proliferation.
EXPERIMENTAL FACT: hTERT + 2% O₂ → limit ~200 PD.
CONCLUSION: A third, independent mechanism exists.

NECESSARY CRITERIA: The mechanism must be non‑renewable (¬R), division‑proportional, and not purely stochastic.

CANDIDATE ANALYSIS: Many ¬R candidates exist — DNA damage accumulation, mitochondrial dysfunction, loss of proteostasis, asymmetric histone distribution, RNA clock. Among these, the mother centriole (with polyGlu) is one plausible candidate because it is structurally non‑renewable, its damage is linked to division count, and it directly controls stem‑cell fate via ciliary signalling. No other candidate alone satisfies all criteria as cleanly, but this is a plausibility ranking, not a necessity.

CONCLUSION: A centriolar PTM clock is a plausible candidate for the third mechanism. This is a hypothesis to be tested, not a logical deduction.
```

---

## ABL‑2 Failure: Central Finding

> This section replaces all prior claims of model validation.

### Observation

Sobol sensitivity analysis (Saltelli method, N = 16,384) on the Cell‑DT v3.0 analytic approximation:

| Parameter          | S1 (first‑order) | ST (total) | Contribution |
|--------------------|------------------|------------|--------------|
| `epigenetic_rate`  | **0.403**        | 0.51       | Dominant     |
| `alpha_centriolar` | **0.224**        | 0.30       | Secondary    |
| `nu_HSC`           | 0.155            | 0.22       | Tertiary     |
| Remaining 29       | <0.10            | <0.15      | Small        |

### Ablation Test

| Model                          | R² (MCAI trajectory) | Note                |
|--------------------------------|----------------------|---------------------|
| FULL (α + β_epi + others)      | 0.778                | —                   |
| ABL‑1 (β_epi = 0, only α)      | 0.579                | Model worsens       |
| ABL‑2 (α = 0, only β_epi)      | **0.833**            | Model improves      |

### Interpretation

The centriolar damage function `D(n) = 1 − exp(−r·n)` is saturating; the epigenetic term `β_epi·t` is linear. The target MCAI trajectory (from published means) is approximately linear between ages 20–80. A linear fit naturally outperforms a saturating exponential. This is **model misspecification**, not biological truth.

The LOO‑CV `R² = −0.093` for the full model confirms that the 32‑parameter model fits noise. The effective degrees of freedom (via AIC) indicate overfitting: AIC = 142 vs. null model AIC = 156, i.e., only marginal improvement. **The model is not useful for prediction.**

**Conclusion:** The computational model fails to support the centriolar hypothesis. This failure is valuable because it forces explicit experimental resolution.

---

## Four Competing Hypotheses (Experimental Resolution Pathway)

All four are testable with the proposed experiments. Each outcome moves the field forward.

| # | Hypothesis | Prediction | Experimental Test | Outcome |
|---|------------|------------|-------------------|---------|
| **H1** | Centriolar damage is **upstream causal**; model is simply misspecified | α >> β_epi in a non‑linear/interactive model | TTLL6 overexpression in young HSC → accelerated aging *without* prior epigenetic change | CDATA validated |
| **H2** | Centriolar damage is **necessary but insufficient**; requires epigenetic context | Interaction term γ·D_cent·D_epi is significant and positive | TTLL6‑OE + DNMT3A‑OE (dual manipulation) → synergistic aging | Integration required |
| **H3** | Centriolar damage is **downstream** of epigenetic drift | β_epi >> α; D_cent is a readout | CCP1‑OE → partial rescue only if epigenetic clock also reset | CDATA reframed as downstream |
| **H4** | Both are independent and the linear model is an artifact | Neither dominates; a third mechanism is primary | Full factorial 2×2 (TTLL6 ± DNMT3A) → no combination reproduces natural aging | Need new candidate |

**All four outcomes are publishable.** The experimental program is designed to distinguish them.

---

## Experimental Predictions (P0–P11, Revised)

### P0 — Primary Resolving Test (Highest Priority)

**GT335‑STED in sorted HSCs from mice with known division history (H2B‑GFP dilution).**  
Before the full STED study, we will perform a **pilot experiment** using conventional immunofluorescence (IF) for polyGlu (GT335 antibody) in young (2–3 mo) vs. old (18–22 mo) mouse HSCs. If the pilot shows a clear age‑dependent increase (Cohen’s d > 0.8, n = 5 per group), we proceed to STED.

| Arm | N mice | Cost | Timeline | Outcome if confirmed |
|-----|--------|------|----------|----------------------|
| Pilot: Young vs Old IF | 5 per group | ~$8K | 4 weeks | Supports → STED |
| Full: Young vs Old STED | 10 per group | ~$30K | +8 weeks | Connects C1 to polyGlu |
| TTLL6‑KD vs Control | 5 per group | +$15K | +4 weeks | Mechanistic proof |

### P1–P11 (All framed as resolving ABL‑2 failure)

| Prediction | Mechanism | Target |
|------------|-----------|--------|
| P1: O₂ dose‑response | Hayflick limit formula | Calibrate α vs β_epi |
| P2: Centrosome integrity (γ‑tubulin) | Structural damage | Direct measure of D_cent |
| P3: hTERT + hypoxia independence | Core test | If infinite → no ¬R needed |
| P4: ROCKi factorial | Additivity test | Model misspecification check |
| P5: φ_cell modifier | Tissue‑specific | Generalizability |
| P6: CEP135‑KD dose‑response | Separate structural vs PTM damage | Distinguish H1 vs H3 |
| P7: sinc‑MT ∝ PTM | Senescence initiation | Bridge C1–C3 |
| P8: iPSC reprogramming efficiency | Low‑PTM hypothesis | Distinguish H2 vs H4 |
| P9: LDC10 + CASIN synergy | Upstream + downstream | Therapeutic axis |
| P10: TTLL6 ∝ age in LSK | scRNA‑seq | Mechanistic support |
| P11: Relapse prediction (division‑counted rescue) | Distinguishes H1 vs H3 | Directly tests causality |

---

## Revised Model Description (32 Parameters, Corrected Status)

**The model is presented as a failed formalization.** It is not validated. It is a tool to generate testable predictions.

| Parameter group           | # params | Source               | Status                         |
|---------------------------|----------|----------------------|--------------------------------|
| Core centriolar (α, ν, β, τ) | 12      | Literature + calibration | Needs direct measurement   |
| Epigenetic (β_epi, k_ep)   | 2        | Literature + MCMC    | Dominant in Sobol              |
| SASP (4 params)            | 4        | Hormesis literature  | Stable                         |
| CHIP (4 params)            | 4        | Jaiswal 2017         | Stable                         |
| Protection (Π₀, τ, baseline) | 3      | MCMC, weak           | Needs re‑specification         |
| Fixed (meiotic, circadian, mTor) | 7   | Literature           | Stable                         |

**Parameter boundaries and solver:** ODE15s with relative tolerance 1e‑8, absolute tolerance 1e‑12. Parameter ranges in Supplementary Table S1 (to be released with code).  
**Overfitting quantification:** AIC = 142, null AIC = 156; BIC = 179, null BIC = 165. The model improves only marginally over the mean — consistent with LOO‑CV R² = −0.093.

**Planned re‑specification:** In Cell‑DT v4.0, the epigenetic term will be coupled to centriolar damage via `ep_age(t) = β_epi·t + k_ep·∫₀ᵗ D(τ) dτ`. This non‑linear coupling may resolve the misspecification, but it will be tested on independent data (not the same 35 points).

---

## Multi‑Organism Supporting Evidence

*Unchanged from v5.2.* This section shows correlative evidence that centriolar damage (polyGlu) increases with age in several species. It is correctly positioned as **motivation**, not validation.

---

## Reproducibility & Open Science

- **Code:** Cell‑DT v3.0 will be archived on Zenodo (DOI: 10.5281/zenodo.XXXXXXX) **upon submission of the revised manuscript** (not upon acceptance). Repository: `https://github.com/LongevityCommon/Cell-DT` (private until submission).  
- **Data:** All figures are reproducible from public data (NHANES, Jaiswal 2017, Horvath 2013). R/Python scripts at `https://github.com/LongevityCommon/CDATA_analysis` (private until submission).  
- **Pre‑registration:** The P0 pilot experiment will be pre‑registered on OSF (`osf.io/TBD`) before data collection, planned date 2026‑09‑01.  
- **Solver details:** ODE15s, reltol 1e‑8, abstol 1e‑12, max step 0.1 years. Parameter boundaries in Supplementary Table S1.

---

## Limitations (Expanded)

1. **P0 blocking problem:** Direct evidence for polyGlu accumulation in HSCs is absent. The entire theory rests on this experiment. A pilot IF study is included to mitigate risk.
2. **ABL‑2 failure:** The computational model is overfitted (32 parameters / 35 data points) and fails cross‑validation. It is not predictive. The failure is used to generate hypotheses, not to validate or disprove the biology.
3. **Other ¬R candidates:** DNA damage, mitochondrial dysfunction, proteostasis loss, and others are not ruled out. The centriolar focus is a plausibility choice, not a logical necessity. A systematic comparison is beyond the current scope.
4. **Post‑mitotic aging:** The model’s time‑dependent damage term (`β_cent·t`) is a placeholder; no mechanism for non‑dividing cells is proposed.
5. **Model parameterization:** AIC/BIC comparison shows only marginal improvement over the null model. Independent validation data (e.g., from primate HSCs) are needed but not yet available.
6. **Code availability:** Code is not yet public. It will be released upon submission. Pre‑registration partially addresses transparency.

---

## Conclusion

CDATA v5.4 presents a **plausibility argument** for a centriolar polyGlu division clock. The quantitative model built to formalize this hypothesis **fails** — it is overfitted and produces worse predictions than a constant baseline. This failure is valuable: it defines four testable biological hypotheses (H1–H4) and an experimental program to resolve them. The document no longer claims logical proof or model validation. It offers a rigorous, falsifiable framework for investigating whether centriolar damage is a cause, a consequence, or a co‑dependent factor in mammalian aging.

**Funding priority:** We request support for the P0 pilot (~$8K) and full STED study (~$30K) as the necessary preconditions for distinguishing H1–H4.

---

## Appendix A — Integration with MCOA (Unchanged)

CDATA is Counter #1 in the Multi‑Counter Architecture of Organismal Aging (MCOA). The ABL‑2 failure strengthens the MCOA thesis: no single counter is sufficient; integration is necessary. See `~/Desktop/LongevityCommon/MCOA/CONCEPT.md`.

---

**Version:** 5.4  
**Date:** 2026‑05‑16  
**Status:** Hypothesis framework; computational model failed; experimental program designed to test four competing hypotheses.