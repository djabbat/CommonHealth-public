## Triple-Blind Peer Review (TBPR)

**Project:** MCOA — Multi-Counter Architecture of Organismal Aging  
**Version:** 1.0 (with 2026-05-10 addenda)  
**Reviewers:** A (domain expert), B (fluff/impact auditor), C (red team)

---

### Reviewer A: Domain Expert (Fact-check, Methodology, PI)

| Criteria | Score (1–5) |
|----------|-------------|
| 1. Originality | 5 |
| 2. Significance | 5 |
| 3. Soundness of hypothesis/theory | 4 |
| 4. Experimental design / feasibility | 3 |
| 5. Data/evidence (including corrections) | 4 |
| 6. Interpretation / logical coherence | 4 |
| 7. Clarity of presentation | 4 |
| 8. Completeness (coverage of key issues) | 3 |
| 9. Falsifiability / testability | 5 |
| 10. Impact (potential to advance the field) | 4 |
| 11. Overall recommendation | 4 |
| **Score Sum** | **45/55** |

**Comments:**  
The MCOA framework is ambitious and conceptually elegant. The formalisation of aging as a weighted sum of parallel damage counters is a genuine advance, and the explicit incorporation of falsifiability (Axiom M4) is commendable. The corrections file (CORRECTIONS_2026-04-22) shows responsiveness to earlier critiques, e.g., removing the fabricated reference and tightening the statistical threshold.

However, I see two major concerns. First, the **a priori weight prediction** (Problem 1) remains unresolved. While the framework *requires* weights fixed before fitting, the provided cell-biological parameters (division rate, metabolic intensity, TERT expression, etc.) are not yet linked to a validated model. The project’s own risk matrix gives this a probability 4 / impact 5—that is a showstopper if not addressed. Second, the **ABL-2 paradox** is only partially resolved; the Sobol ablation argument is suggestive but not causal. The coupling matrix Γ entries are said to be measured, not fitted, yet no pilot data are shown.

Test 4 (Division vs Time) is well-designed, but the sample size N ≥ 2000 is aspirational; the OSF pre-registration acknowledges it may be smaller. The negative R² (-0.093) from cross-validation is correctly noted as a model limitation, which is honest, but it raises doubts about model stability.

Overall, the theoretical core is strong and falsifiable, but empirical validation is thin. I would recommend major revisions focused on (i) demonstrating at least one tissue’s a priori weight prediction using publicly available single-cell data, and (ii) publishing a pre-registered pilot of Test 4 before claiming the framework is “in implementation.”

---

### Reviewer B: Fluff/Impact Auditor

| Criteria | Score (1–5) |
|----------|-------------|
| 1. Originality | 4 |
| 2. Significance | 4 |
| 3. Soundness of hypothesis/theory | 3 |
| 4. Experimental design / feasibility | 2 |
| 5. Data/evidence (including corrections) | 3 |
| 6. Interpretation / logical coherence | 3 |
| 7. Clarity of presentation | 4 |
| 8. Completeness (coverage of key issues) | 3 |
| 9. Falsifiability / testability | 5 |
| 10. Impact (potential to advance the field) | 3 |
| 11. Overall recommendation | 3 |
| **Score Sum** | **37/55** |

**Comments:**  
The MCOA project has clear rhetorical strengths: it positions itself as the “mother-project” of the LongevityCommon stack, claims five canonical counters, and promises an a priori weighting scheme. It has a Nature Aging manuscript, visualisations, and even an OSF pre-registration. All of this sounds impressive.

But when I strip away the packaging, the intellectual depth is thinner than it appears. The “five canonical counters” are not new; telomere attrition, mitochondrial damage, epigenetic drift, proteostasis collapse, and even centriolar polyglutamylation (CDATA) are well-known. The novelty lies in the weighted-sum formalism—but the weights are not provided for any tissue yet, and the author states that post-hoc fitting is explicitly prohibited. That means the current version cannot make a single quantitative prediction until the weight problem is solved. The project is therefore **not yet actionable**.

The Risk Matrix is honest but alarming: “A priori weights not predictable from cell-biological data” is flagged as probability 4 (high) and impact 5 (critical). If this risk materialises, the entire MCOA framework collapses. Furthermore, the “Damage Shadow” extension (2026-05-10) undermines a key counter (epigenetic drift) by arguing that DNAmAge does not correlate with function—contradicting the very premise of using epigenetic clocks as an MCOA counter. That inconsistency is not addressed.

The budget estimates for experimental tests (Test 1: $1.5M; Test 4: <$200k) suggest the author is aware of scale, but no consortium letters of intent are presented, only “proposed partners.” The EIC Pathfinder plan is vague; WP1 is to produce a “software library + white paper” but with only one postdoc and half a PhD.

In short, MCOA is a nice skeleton but lacks muscle and blood. It may become impactful if the weight prediction problem is solved and the first pilot data are convincing. Right now, it is more aspirational than operational. I recommend conditional acceptance only after a clear demonstration of a priori weights for at least one tissue and a successful small-scale Test 4 simulation.

---

### Reviewer C: Red Team (Counter-arguments, Bias)

| Criteria | Score (1–5) |
|----------|-------------|
| 1. Originality | 3 |
| 2. Significance | 3 |
| 3. Soundness of hypothesis/theory | 2 |
| 4. Experimental design / feasibility | 2 |
| 5. Data/evidence (including corrections) | 2 |
| 6. Interpretation / logical coherence | 2 |
| 7. Clarity of presentation | 3 |
| 8. Completeness (coverage of key issues) | 2 |
| 9. Falsifiability / testability | 4 |
| 10. Impact (potential to advance the field) | 2 |
| 11. Overall recommendation | 2 |
| **Score Sum** | **27/55** |

**Comments:**  
I am going to be deliberately critical because that is my role, but I believe the project has fatal flaws that the other reviewers may have downplayed.

**1. The “five canonical counters” are not independent.** Centriolar polyglutamylation (Counter #1) is likely downstream of epigenetic control and proteostasis. The coupling matrix tries to capture this, but the project explicitly states that all Γ entries must be measured, not fitted. Good luck measuring the causal link between centriole polyglutamylation and mitochondrial damage in a living organism. The risk of infinite regress is high.

**2. The sample size requirement (N ≥ 2000) is unrealistic for a single-lab tractable test (Test 4).** The author claims Test 4 is “<$200k / 10 weeks” but that number assumes a small organoid experiment, not a mortality study on N=2000. The pre-registration notes that the actual sample size may be less, yet the falsification threshold (Axiom M4) demands N ≥ 2000 at α = 0.001 to detect a partial r² of 0.05. If the actual cohort is smaller, the test loses statistical power, and the framework cannot be falsified—meaning it becomes unfalsifiable in practice. This is a classic bait-and-switch: a falsifiability claim that is operationally unattainable.

**3. The corrections file (CORRECTIONS_2026-04-22) reveals a history of sloppiness.** A fabricated reference (Sun 2016) was used; the earlier R² threshold was retroactively superseded; the “provisional threshold R² < 0.5” was called a mistake. While I applaud the correction, it suggests the author is prone to overstatement. The negative R² (-0.093) from cross-validation is not a neutral “model limitation”; it means the model is worse than the mean. That is devastating for a framework that claims to predict mortality.

**4. The “Stem-Cell-Centric” and “Damage Shadow” extensions seem to contradict the core MCOA.** The Damage Shadow meta-analysis shows that ΔDNAmAge correlates with Δfunction with r=0.09 (p=0.44). But Counter #4 (epigenetic drift) is explicitly part of the weighted sum. If DNA methylation age is not a valid functional surrogate, then how can epigenetic drift be a meaningful damage counter? The author acknowledges this as an “open problem” but gives no pathway to resolution. The hypothesis that “GrimAge EAA” is a better readout is not supported by the meta-analysis (which used epigenetic clocks broadly). This is a major inconsistency.

**5. The “Master-Counter Hypothesis” (GrimAge EAA) is presented as an integrative readout, but it violates Axiom M4.** GrimAge is a composite biomarker trained on mortality data; it inherently captures multiple damage processes. Using it as a “master counter” is no different from fitting a black box model—exactly what MCOA purports to avoid. The a priori weight prediction requirement becomes impossible if the master counter is itself a trained model.

**6. The budget and consortium are aspirational.** No letters of intent, no confirmed funding. The EIC Pathfinder proposal is not yet submitted. The project is all concept, no execution.

I recommend rejection in its current form. The theoretical elegance is overshadowed by unresolved empirical contradictions, unattainable falsifiability conditions, and a track record of corrections. The author should go back to the bench, collect pilot data for one counter (e.g., centriole polyglutamylation in a simple mouse tissue), establish a priori weights from independent cell-biological parameters, and *then* propose the framework. Until that happens, MCOA is a castle in the sky.

---

### Combined Score

| Reviewer | Score Sum | 
|----------|-----------|
| A (domain expert) | 45 |
| B (impact auditor) | 37 |
| C (red team) | 27 |
| **Combined Score = MIN(45, 37, 27)** | **27/55** |

**Final recommendation:** Major Revisions / Conditional Reject. The red team’s concerns undermine the confidence in the project’s current viability. To proceed, the author must provide:
- A successful small-scale empirical demonstration of at least one a priori weight prediction (e.g., centriole-based counter in murine HSCs).
- Resolution of the epigenetic drift functional surrogate inconsistency.
- A realistic experimental plan for Test 4 with a clear sample size justification that matches the falsification threshold (or an explicit downgrade of the threshold to a feasible N).
- Concrete consortium letters or commitment letters from at least two collaborating labs.

Without these, the framework remains mathematically elegant but empirically unsupported.