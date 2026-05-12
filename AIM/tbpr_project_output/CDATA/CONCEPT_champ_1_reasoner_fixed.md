=== CDATA — CONCEPT v5.3 (REVISED per TBPR 2026-05-16) ===

---

# CDATA — Concept v5.3 — Counter #1 (Centriolar) in MCOA

**Status:** Pre-experimental theory with strong deductive foundation. The model's core computational prediction (centriolar damage as dominant driver of organismal aging biomarkers) was **falsified** by internal cross-validation (ABL-2 paradox). This is presented as the primary finding, not a limitation. The document is restructured as a **hypothesis framework** for experimental resolution of the paradox.

> **This version incorporates all fixable recommendations from the Triple-Blind Peer Review (2026-05-16).**  
> The three key changes:  
> 1. The document is reframed as a **hypothesis paper**, not a validated model.  
> 2. The "logical proof" language is replaced with "deductively constrained hypothesis."  
> 3. The ABL-2 paradox is elevated to the central scientific finding, not a buried limitation.

---

## Executive Summary (Revised)

CDATA proposes that polyglutamylation (polyGlu) of the mother centriole in stem cells acts as a division-counting clock, independent of telomeres and ROS. This hypothesis is derived from a deductively constrained argument: since stem cells senesce in hypoxia with active telomerase, a third, non-renewable (`¬R`) mechanism must exist.

We built a quantitative ODE model (32 parameters, Cell-DT v3.0 Rust simulator) to formalize this hypothesis. **The model fails its primary validation test.** Leave-one-out cross-validation (LOO-CV) yields a mean `R² = -0.093`, indicating the model's predictions are worse than a constant baseline. Sobol sensitivity analysis reveals that the centriolar damage parameter (`α = 0.0082`) contributes only 22.4% of model variance, while the epigenetic rate parameter (`β_epi = 0.403`) dominates. Strikingly, removing the centriolar component from the model **improves** predictive accuracy (ABL-2 paradox: `R²` increases from 0.778 to 0.833).

**This does not disprove the centriolar hypothesis.** Instead, it reveals a fundamental biological question: Is centriolar damage an upstream cause of aging, a downstream consequence of epigenetic drift, or are they co-dependent? The theory, model, and negative validation together form a testable framework. We propose four competing hypotheses (see §B.4.4), each resolvable by the experimental program described herein.

**The corrected statement of the theory:**  
CDATA is a deductively constrained hypothesis for a centriolar `¬R` clock. The current computational model is insufficient to distinguish this clock from an epigenetic clock given existing data. The experimental program is designed to resolve this ambiguity.

---

## Центральный аргумент (Corrected for Language)

**Аксиома 1 — Hayflick в гипоксии с теломеразой**  
Стволовые клетки в гипоксической среде (1–3% O₂) при активной теломеразе всё равно достигают предела Хейфлика (Peters-Hall et al., *FASEB J* 2020; PMID 31914653).

**Аксиома 2 — Дефектный сигналинг реснички из старой материнской центриоли**  
Материнская центриоль, наследуемая стволовой дочерью, является базальным телом первичной реснички. Накопление polyGlu на стареющей центриоли ухудшает сигналинг (Hedgehog/Wnt), нарушая восприятие нишевых сигналов самообновления.

**Аксиома 3 — Снижение темпа деления стволовых клеток со старой центриолью**  
Нарушение цилиарного сигналинга удлиняет межделительный интервал и увеличивает долю симметричных дифференцировочных делений, что приводит к истощению пула.

**¬R Аргумент (deductively constrained hypothesis, not proof):**

```
ПОСЫЛКА 1: Если теломерная + ROS теории полны, то hTERT + гипоксия → бесконечная пролиферация.
ЭКСПЕРИМЕНТАЛЬНЫЙ ФАКТ: hTERT + 2% O₂ → предел ~200 PD.
ВЫВОД: Существует третий, независимый механизм.

НЕОБХОДИМЫЙ КРИТЕРИЙ: Этот механизм должен быть не-обновляемым (¬R), деление-пропорциональным, и не-стохастическим.

АНАЛИЗ: Единственная структура, удовлетворяющая всем условиям среди известных ¬R-кандидатов, — материнская центриоль (с полиглутамилированием).

ЗАКЛЮЧЕНИЕ: Центриолярное PTM-накопление — наиболее парсимоничный кандидат на роль третьего механизма. Это сильно обоснованная гипотеза, а не формальное доказательство.
```

**Ключевое изменение:** Фраза "логически необходимый вывод" заменена на "наиболее парсимоничный кандидат" во всех разделах, кроме этого, где объясняется замена.

---

## ABL-2 Paradox: Central Finding

> **This section is the core of the revised document.** It supersedes all previous claims of model validation.

### Observation

Sobol sensitivity analysis (Saltelli method, N=16,384) on the Cell-DT v3.0 analytic approximation:

| Параметр | S1 (first-order) | ST (total) | Вклад |
|----------|------------------|------------|-------|
| `epigenetic_rate` (β_epi) | **0.403** | 0.51 | Доминирующий |
| `alpha_centriolar` (α_cent) | **0.224** | 0.30 | Вторичный |
| `nu_HSC` | 0.155 | 0.22 | Третичный |
| Остальные 29 | <0.10 | <0.15 | Малый |

### Ablation Test

| Модель | R² (MCAI trajectory) | Примечание |
|--------|----------------------|------------|
| FULL (α + β_epi + остальные) | 0.778 | — |
| ABL-1 (β_epi=0, только α) | 0.579 | Модель ухудшается |
| ABL-2 (α=0, только β_epi) | **0.833** | **Модель улучшается** |

**Интерпретация:** The model that centers on centriolar damage is not only non-dominant but actively degrades predictive accuracy relative to a purely epigenetic model. This is a formal falsification of the claim that "centriolar damage alone explains aging trajectories."

### Explanation

The centriolar damage function `D(n) = 1 − exp(−r·n)` is non-linear and saturating. The epigenetic clock is linear (`ep_age = β_epi · t`). The MCAI target trajectory, derived from published literature means, is approximately linear between ages 20 and 80. A linear function will always fit a linear target better than a saturating exponential. This is a **model specification artifact**, not biological truth.

**Crucially, this does not disprove the centriolar hypothesis.** It only disproves the *current model's* ability to distinguish the two mechanisms. The experimental program is designed to resolve this.

---

## Four Competing Hypotheses (Experimental Resolution Pathway)

Given the ABL-2 paradox, we explicitly state four possible biological realities that could explain the data. All are testable.

| # | Hypothesis | Prediction | Experimental Test | Outcome |
|---|------------|------------|-------------------|---------|
| **H1** | Centriolar damage is **upstream causal**, but model is misspecified | α >> β_epi in a non-linear or interactive model | TTLL6-OE in young HSC → accelerated aging *without* prior epigenetic change | CDATA validated |
| **H2** | Centriolar damage is **necessary but insufficient**; requires epigenetic context | Interaction term γ·D_cent·D_epi is significant and positive | TTLL6-OE + DNMT3A-OE (dual manipulation) → synergistic aging | Integration required |
| **H3** | Centriolar damage is **downstream** of epigenetic drift | β_epi >> α; D_cent is a readout | CCP1-OE → partial rescue only if epigenetic clock is also reset | CDATA reframed as downstream |
| **H4** | Both are independent and the linear model is an artifact | Neither dominates; a third mechanism is primary | Full factorial 2×2 (TTLL6 ± DNMT3A) → no combination reproduces natural aging | Need new candidate |

**The experimental program is designed to distinguish H1–H4.** All four outcomes are publishable and advance understanding.

---

## Experimental Predictions (P0–P11, Corrected)

**P0 — Primary Resolving Test (Highest Priority):**  
*GT335-STED in sorted HSCs from mice with known division history (H2B-GFP dilution).*

| Arm | N mice | Cost | Timeline | Outcome if P0 confirmed |
|-----|--------|------|----------|-------------------------|
| Young (2–3 mo) vs Old (18–22 mo) | 10 per group | ~$30K | 8 weeks | C1 established → proceed to H2 |
| TTLL6-KD vs Control | 5 per group | +$15K | +4 weeks | C1 mechanistic proof |

**P1–P11** — As in v5.2, but all now framed as *resolving the ABL-2 paradox* rather than validating the theory:

| Prediction | Mechanism | Resolution Target |
|------------|-----------|------------------|
| P1: O₂ dose-response | Hayflick limit formula | Calibration of α vs β_epi |
| P2: Centrosome integrity | Structural damage | Direct measure of D_cent |
| P3: hTERT + hypoxia independence | Core test | Falsify if infinite → no ¬R needed |
| P4: ROCKi factorial | Additivity | Model specification test |
| P5: φ_cell modifier | Tissue-specific | Generalizability |
| P6: CEP135-KD dose-response | Separation of structural vs PTM damage | Distinguish H1 vs H3 |
| P7: sinc-MT ∝ PTM | Senescence initiation | C1–C3 bridge |
| P8: iPSC reprogramming efficiency | Low-PTM hypothesis | Distinguish H2 vs H4 |
| P9: LDC10+CASIN synergy | Upstream + downstream | Therapeutic axis |
| P10: TTLL6 ∝ age in LSK | scRNA-seq | Mechanistic support |
| **P11: Relapse prediction** | Division-counted rescue | **Distinguishes H1 vs H3** |

---

## Revised Model Description (32 Parameters, Corrected Status)

**The model is no longer presented as validated.** It is presented as a formalization of the hypothesis that failed, and a tool for designing experiments.

| Parameter group | # params | Source | Status |
|-----------------|----------|--------|--------|
| Core centriolar (α, ν, β, τ) | 12 | Literature + calibration | **Needs direct measurement** |
| Epigenetic (β_epi, k_ep) | 2 | Literature + MCMC | **Dominant in Sobol** |
| SASP (4 params) | 4 | Hormesis literature | Stable |
| CHIP (4 params) | 4 | Jaiswal 2017 | Stable |
| Protection (Π₀, τ, baseline) | 3 | MCMC, weak | Needs re-specification |
| Fixed (meiotic, circadian, mTor) | 7 | Literature | Stable |

**Key change:** The parameter `epigenetic_rate` is now recognized as having been incorrectly specified as additive. A planned re-specification in Cell-DT v4.0 will integrate it mechanistically:  
`ep_age(t) = β_epi · t + k_ep · ∫₀ᵗ D(τ) dτ`. This non-linear coupling is expected to resolve the ABL-2 paradox by making the two parameters structurally interdependent.

---

## Multi-Organism Supporting Evidence (Table, as in v5.2)

Unchanged. This section provides convergent evidence that centriolar damage correlates with aging across species. It is now correctly positioned as *motivation for the hypothesis*, not *validation of the model*.

---

## Reproducibility & Open Science (Corrected)

- **Code:** Cell-DT v3.0 will be archived on Zenodo (DOI: TBD) **upon submission of the revised manuscript**, not upon acceptance. Link: `https://github.com/LongevityCommon/Cell-DT` (private until submission).
- **Data:** All figures in this document are reproducible from public data (NHANES, Jaiswal 2017, Horvath 2013). R/Python scripts at `https://github.com/LongevityCommon/CDATA_analysis` (private until submission).
- **Pre-registration:** The experimental protocol for Test P0 (GT335-STED in HSC) will be pre-registered on OSF (`osf.io/TBD`) before data collection, with planned date 2026-09-01.

---

## Limitations (Acknowledged)

1. **P0 blocking problem:** Direct evidence for C1 and C2 in HSC is absent. The entire theory rests on this experiment.
2. **ABL-2 paradox:** The computational model fails to validate the centriolar hypothesis against existing data.
3. **Post-mitotic aging:** The model's time-dependent damage term (`β_cent · t`) is a placeholder; no specific molecular mechanism is proposed for non-dividing cells.
4. **Model parameterization:** 32 parameters fit to ~35 data points; risk of overfitting confirmed by LOO-CV.
5. **Code availability:** Code is not yet public; will be released upon submission of revised manuscript.

---

## Conclusion

CDATA v5.3 presents a deductively constrained hypothesis for a centriolar `¬R` aging clock. The quantitative model built to formalize this hypothesis **fails its internal validation**, revealing the ABL-2 paradox: an epigenetic model fits the data better than the centriolar model. This failure is not a dead end but a discovery. It defines four testable biological hypotheses, each resolvable by a targeted experimental program.

The revised document no longer claims "logical proof" or "model validation." It offers a rigorous framework for resolving a fundamental question: Is centriolar damage a cause, a consequence, or a co-dependent factor in mammalian aging?

**Funding priority:** We request support for the P0 experiment (GT335-STED in HSC, ≈$30K) as the necessary precondition for all further work.

---

## Appendix A — Integration with MCOA (Unchanged)

CDATA is Counter #1 in the Multi-Counter Architecture of Organismal Aging (MCOA). This framework is described in `~/Desktop/LongevityCommon/MCOA/CONCEPT.md`. The ABL-2 paradox strengthens the MCOA thesis: no single counter is sufficient; integration is necessary.

---

**Version:** 5.3  
**Date:** 2026-05-16  
**Status:** Pre-experimental hypothesis framework; ABL-2 paradox resolved as primary finding; experimental program designed to distinguish four competing hypotheses.