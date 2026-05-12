## Triple-Blind Peer Review (TBPR)  
**Project:** MCOA — Multi-Counter Architecture of Organismal Aging  
**Reviewed version:** v1.0 (2026-04-21, with addenda 2026-05-10)  

---

### Reviewer A – Domain Expert (Fact‑check, Methodology, PI)

| # | Criterion | Score (1–5) |
|---|-----------|-------------|
| 1 | Scientific merit / originality | 4 |
| 2 | Rationale / justification | 4 |
| 3 | Methodology / approach | 3 |
| 4 | Feasibility | 3 |
| 5 | Data / evidence support | 2 |
| 6 | Clarity / presentation | 5 |
| 7 | Significance / impact | 4 |
| 8 | Reproducibility / falsifiability | 5 |
| 9 | Resources / budget | 3 |
| 10 | Risk mitigation | 2 |
| 11 | Innovation | 4 |

**Score Sum:** 39/55  

**Comments:**  
The MCOA framework is conceptually elegant and fills a clear gap in aging theory by formalising parallel damage counters with explicit falsifiability thresholds. The commitment to *a priori* tissue weighting (Axiom M3) and the canonical test set (§6) is commendable. However, the project suffers from several weaknesses:  
- **A‑priori weight prediction** (Problem 1) remains completely unresolved – no cell‑biological parameters have been demonstrated to predict *w<sub>i</sub>(tissue)*. This is a critical blocker that threatens the entire framework.  
- **Evidence support** is weak: the only experimental data referenced are from the CDATA subproject (v5.1), and the ABL‑2 paradox (§2.2 of the Perspective) is only partially resolved (causal direction unproven).  
- **Feasibility** of the required multi‑tissue longitudinal mouse studies (Test 1) is unrealistic given the proposed budget ($1.5M / 3 years) and timeline.  
- **Budget estimates** for Tests 2–5 appear optimistic; the €0.3M for WP1 in the EIC Pathfinder is insufficient for a full software library, community white paper, and postdoc+PhD.  
- **Pre‑registration** (OSF osf.io/9x3k7) is a strong feature, but the protocol details are not yet expanded; sample size (N≥2000) is aspirational.  

**Recommendation:** Major revision required before any implementation. The priority must be to solve Problem 1 (weight prediction) or, failing that, to explicitly adopt equal weights as the null model and test whether the framework outperforms it.

---

### Reviewer B – Fluff/Impact Auditor

| # | Criterion | Score (1–5) |
|---|-----------|-------------|
| 1 | Scientific merit / originality | 3 |
| 2 | Rationale / justification | 2 |
| 3 | Methodology / approach | 2 |
| 4 | Feasibility | 2 |
| 5 | Data / evidence support | 1 |
| 6 | Clarity / presentation | 4 |
| 7 | Significance / impact | 3 |
| 8 | Reproducibility / falsifiability | 4 |
| 9 | Resources / budget | 2 |
| 10 | Risk mitigation | 2 |
| 11 | Innovation | 3 |

**Score Sum:** 28/55  

**Comments:**  
The project is heavy on jargon and meta‑formalisation but light on concrete experimental output. The “fluff” score is high – the document uses many words to describe what is essentially a re‑packaging of existing ideas (Hayflick limit, ROS, epigenetic drift) into a single mathematical framework.  

- **Impact** is overstated: the claim that MCOA “is not a replacement for CDATA, Ze, or BioSense” yet simultaneously calls itself the “meta‑framework” contradicts its own ambition. The actual added value (e.g., coupling matrix Γ) still lacks any measurement.  
- **Data evidence** is virtually non‑existent: the only empirical support comes from a single mouse model (PolgA) and a meta‑analysis that shows a null correlation (r=0.09) for DNAmAge vs. function. These are hardly convincing.  
- **Budget** figures are unrealistic – $2.8M for a 5‑arm mouse lifespan trial (Test 5) is low by industry standards; similarly, the €0.3M WP1 budget will not sustain a postdoc+PhD for 12 months with software deliverables.  
- **Risk mitigation** is weak – the only fallback (equal weights) is presented as a null model but not tested; the “seed grant” fallback is vague.  
- **Overall** – the project reads like an NIH grant proposal written by a theorist with no track record of experimental validation. The current version is not fundable in its present form.

**Recommendation:** Reject. The framework should be trimmed to one or two testable hypotheses (e.g., Test 4) before any larger submission.

---

### Reviewer C – Red Team (Counter‑arguments, Bias)

| # | Criterion | Score (1–5) |
|---|-----------|-------------|
| 1 | Scientific merit / originality | 3 |
| 2 | Rationale / justification | 2 |
| 3 | Methodology / approach | 3 |
| 4 | Feasibility | 2 |
| 5 | Data / evidence support | 1 |
| 6 | Clarity / presentation | 4 |
| 7 | Significance / impact | 3 |
| 8 | Reproducibility / falsifiability | 5 |
| 9 | Resources / budget | 2 |
| 10 | Risk mitigation | 1 |
| 11 | Innovation | 3 |

**Score Sum:** 29/55  

**Comments:**  
The red‑team perspective highlights several biases and unresolved counter‑arguments that the authors do not adequately address.

- **Confirmation bias** – the choice of five canonical counters is arbitrary and based on the author’s prior work (CDATA). The inclusion of a “counter #0” (centriole) as the first and “unifying” device suggests a predetermined narrative. The coupling matrix Γ is largely speculative and includes a fabricated reference (“Sun 2016”; corrected in the document, but not fully retracted).  
- **Falsifiability overemphasis** – while strong falsifiability criteria (Axiom M4, N≥2000, R²>0.05) are laudable, the project has no data to support *any* counter’s partial R². The threshold itself is arbitrary (why 0.05? Why not 0.01 or 0.1?). The document later uses R² = -0.093 as a “model limitation” but does not explain how that is consistent with the falsifiability standard.  
- **Power calculation** – the sample size of 2000 is based on an expected R²=0.3, but the actual effect sizes for individual counters may be far smaller (e.g., telomere length in blood predicts only ~1% of mortality variance). The power analysis may be misleading.  
- **Conflicts of interest** – the author (Jaba Tkemaladze) is the sole originator; no independent validation is planned beyond “replication group”. The EIC Pathfinder consortium partners are proposed but no letters of intent are presented. The plan to submit to *Nature Aging* as a Perspective (not original research) is appropriate, but the text reads like a research proposal, not a perspective.  
- **Extension addenda (May 10)** add additional hypotheses (piRNA, Damage Shadow) without addressing the foundational problems. This suggests scope creep.  
- **Proteostasis collapse (counter #5)** – the “VEXAS syndrome” reference (Molteni 2025) is an interesting anecdote but does not validate the counter as rate‑limiting in normal aging.  

**Recommendation:** Major revision or resubmission as a pure theoretical perspective, with all experimental claims removed until supporting data exist. The pre‑registration and falsifiability plan should be retained but the scope drastically reduced.

---

### Combined Score

| Reviewer | Score Sum |
|----------|-----------|
| A (Domain Expert) | 39/55 |
| B (Fluff/Impact) | 28/55 |
| C (Red Team) | 29/55 |
| **Combined (MIN)** | **28/55** |

---

### Editorial Decision

**Verdict:** **Reject** (Score < 30).  
**Rationale:** The project suffers from insufficient empirical support, unresolved core assumptions (a‑priori weight prediction), overly optimistic budget/timeline, and scope creep. While the conceptual framework is clever, it is not ready for funding or implementation as presented. The authors should focus on producing the required experimental data (especially Test 4), solving Problem 1, and reducing the narrative to a tight, testable set of hypotheses.