# Triple-Blind Peer Review (TBPR) – MCOA Project Concept v1.0

**Project:** MCOA – Multi-Counter Architecture of Organismal Aging  
**Author:** Jaba Tkemaladze, MD (Georgia Longevity Alliance)  
**Review type:** Triple-Blind (Reviewers A, B, C)  
**Date of review:** 2026-05-12  

---

## Reviewer A – Domain Expert (fact‑check, methodology, PI perspective)

### Strengths
- The axiomatic structure (M1–M4) is a genuine attempt to impose falsifiability on a complex, multifactorial problem. The dimensional consistency requirement (Axiom M2) is a mathematically clean constraint often missing in aging theories.  
- The five canonical counters are well‑chosen, each grounded in independent mechanistic literature (centriolar polyglutamylation, telomeres, mtDNA, epigenetic drift, proteostasis). The coupling matrix Γ is a natural extension, and the commitment to measure rather than fit all Γ entries is commendable (though practically very challenging).  
- Test 4 (Division vs Time) is a clever *ex vivo* design that is tractable within 10 weeks and under $200k – exactly the kind of near‑term falsification the field needs.  
- The explicit admission of unresolved problems (a‑priori weight prediction, ABL‑2 paradox, negative R² in cross‑validation) and the existence of a corrections document (CORRECTIONS_2026-04-22) show scientific honesty, even if the corrections reveal past errors.  
- The OSF pre‑registration (osf.io/9x3k7) and the detailed falsification threshold (partial r² < 0.05 for each counter) set a high bar that, if met, would genuinely strengthen the framework.

### Weaknesses / Concerns
1. **A‑priori weight prediction (Problem 1) remains the central Achilles’ heel.** The text says *“w_i(tissue) must be predicted BEFORE fitting, from independent cell‑biological parameters”* – but no concrete mapping from parameters to weights is provided. Without this, the model cannot be tested in its intended form; it collapses to a post‑hoc fitting exercise, violating Axiom M3. The fallback “w_i = 1/k” is statistically convenient but biologically naive and would not satisfy the triage requirement (some tissues are clearly more dependent on certain counters).  
2. **Sample size justification is insufficient for the claimed falsification.** The formula used (n = (1.96+0.84)² · σ²/δ²) assumes a simple two‑group comparison, not a multiple regression with age and sex covariates. The actual required N to detect a partial R² = 0.05 (the threshold) with 80% power at α = 0.001 (the stated alpha) would be far larger – back‑of‑envelope: > 4,000 even with optimistic error variance. The threshold R² < 0.05 is extremely stringent, making Type‑II error highly likely with N = 2,000. This undermines the falsifiability promise.  
3. **The coupling matrix Γ is mostly empty.** The text explicitly notes *“γ_i = 0 by default, deviation requires post‑hoc statistical rejection”* (CORRECTIONS §1.3). While methodologically safe, this default assumption essentially removes all cross‑talk until proven otherwise, which may lead to an overly simplistic model. Many known interactions (e.g., telomere → mitochondrial dysfunction via p53/PGC‑1α) are absent.  
4. **The ABL‑2 paradox resolution is incomplete.** The Sobol ablation showing CP dominance does not establish causality; a genetic or pharmacological perturb‑and‑measure approach (Test 2A) is promised but not yet performed. Without that, the centriole‑counter remains correlative.  
5. **The damage‑shadow extension’s meta‑analysis (PROSPERO CRD42026218473) pools only 14 studies (274 mice)** – extremely underpowered for meta‑regression. The reported r = 0.09 (p = 0.44) cannot be interpreted as evidence of “no correlation”; it simply reflects insufficient data. The claim that DNAmAge is “not a valid surrogate for function” is overstated based on this analysis.  
6. **References to fabricated sources (Sun 2016) in earlier versions, even if corrected, damage trust in the literature review.** The corrections document mentions a “fabricated” reference – this should be addressed transparently in any future publication (e.g., retraction of the earlier claim, not just a footnote).  

### Score Table (1–5, 5 = best)

| # | Criterion | Score | Justification |
|---|-----------|-------|---------------|
| 1 | Originality / Novelty | 4 | Synthesises existing ideas into a formal, falsifiable structure; the centriole‑counter is novel. |
| 2 | Significance / Impact | 4 | If validated, would unify multiple aging biomarkers; high potential impact. |
| 3 | Methodology / Rigor | 2 | Weight prediction not operationalised; sample size miscalculated; meta‑analysis underpowered. |
| 4 | Clarity / Organization | 5 | Exceptionally well‑structured; axioms, definitions, tests are crystal clear. |
| 5 | Feasibility / Practicality | 3 | Test 4 is feasible; larger tests (Test 5: $2.8M) are aspirational; a‑priori weights impractical now. |
| 6 | Literature / References | 3 | Corrections document indicates earlier fabrication; otherwise broad but derivative. |
| 7 | Integration with existing frameworks | 5 | Well‑aligned with hallmarks of aging, CDATA, epigenetic clocks, etc. |
| 8 | Falsifiability / Testability | 3 | High ambition but actual experimental load is daunting; threshold may be too strict. |
| 9 | Ethical / Societal considerations | 3 | No ethical issues raised; societal implications of “aging counters” not discussed. |
| 10 | Resource requirements / Budget | 2 | Total cost for all tests > $5M; consortium commitments are letters of intent only. |
| 11 | Overall assessment | 3 | Promising framework with serious operational gaps; needs concrete weight prediction and larger validation studies. |

**Score Sum:** 37 / 55

---

## Reviewer B – Fluff / Impact Auditor

### Strengths
- The document is **not** a typical hype‑filled pitch. It explicitly lists “What MCOA is NOT”, acknowledges unresolved problems, and includes a detailed corrections record. This is refreshingly self‑aware.  
- The success criteria are modest (submission to a journal, a Rust implementation, one simulation run). The near‑term goal is a conceptual paper, not a cure for aging – appropriate for the funding stage (TRL 2→3).  
- The connection to EIC Pathfinder is realistic: WP1 (€0.3M, 12 months) is a software‑standard deliverable that does not overpromise biological breakthroughs.  
- The pre‑registration (OSF) and explicit falsification threshold add credibility and distinguish MCOA from many “systems biology” frameworks that remain untestable.  
- The “Damage Shadow” extension’s meta‑analysis conclusion (weak correlation between ΔDNAmAge and function) is correct in direction, even if underpowered – it appropriately tempers the hype around epigenetic rejuvenation.

### Weaknesses / Concerns
1. **“Inviolable axioms” language is grandiose for a framework that is (by the author’s own admission) provisional and subject to corrections.** The axioms are not laws of nature but modelling choices. This phrasing may raise eyebrows in a grant review or journal peer review. Suggest replacing with “Core assumptions” or “Postulates”.  
2. **The connection to Ze (“dimensionless χ_Ze synchronisation index”) feels tacked on.** The description is vague (“ODE model of plasma/SASP feedback loop on Argentieri 2024 / Jeon 2022 basis”) without any specifics. This appears to be an attempt to pull in a prior subproject (Ze) for continuity, but it adds more complexity without evidence that Ze is a meaningful counter.  
3. **The success criteria include “submission to Nature Aging by 2026-04-25”.** Submission is not acceptance; using it as a milestone is overoptimistic. A more honest milestone would be “manuscript submission” – but even that is not a scientific achievement.  
4. **The “consortium partners” list is entirely aspirational** – no letters of intent are yet secured. For an EIC application, this is a major weakness. The budget assumes coordination of four groups with no demonstrated track record of collaboration.  
5. **The R² = -0.093 revelation is framed as a “model limitation”** – but a negative R² in cross‑validation means the model is **worse than the mean baseline**. This is not just a limitation; it indicates fundamental misspecification. The author should explain why this occurred (overfitting? wrong features? small sample?) rather than treating it as a neutral fact.  
6. **The stem‑cell‑centric extension and damage‑shadow extension are included as “NOT YET PUBLISHED” drafts.** This blurs the line between proposed work and existing accomplishments. For a project concept, extra unpublished material can distract from the core MCOA.  
7. **Fluff factor:** Low overall, except for the “Master‑Counter Hypothesis” using GrimAge EAA as “best integrative readout” – this is a strong claim that goes beyond the MCOA framework and is not supported by the data cited (Tay et al., 2025 uses frailty, not lifespan).  

### Score Table

| # | Criterion | Score | Justification |
|---|-----------|-------|---------------|
| 1 | Originality / Novelty | 4 | Novel synthesis, not revolutionary. |
| 2 | Significance / Impact | 3 | High potential but unvalidated; too premature to assess real impact. |
| 3 | Methodology / Rigor | 3 | Good structure but operational gaps (weights, sample size). |
| 4 | Clarity / Organization | 5 | Excellent readability, careful organisation. |
| 5 | Feasibility / Practicality | 3 | Test 4 feasible; larger tests uncertain; consortium not yet formed. |
| 6 | Literature / References | 3 | Corrections document raises red flags; otherwise adequate. |
| 7 | Integration with existing frameworks | 5 | Well‑connected to CDATA, Ze, BioSense. |
| 8 | Falsifiability / Testability | 4 | Strong attempt, though threshold may be too strict. |
| 9 | Ethical / Societal considerations | 2 | No discussion of misuse (e.g., genetic testing for “aging counters” in insurance). |
| 10 | Resource requirements / Budget | 2 | €0.3M for WP1 is reasonable but total project cost >€5M; no confirmed partners. |
| 11 | Overall assessment | 3 | Promising but needs much more evidence and concrete weight mapping. |

**Score Sum:** 37 / 55

---

## Reviewer C – Red Team (counter‑arguments, bias, alternative interpretations)

### Strengths
- The MCOA framework is internally consistent and mathematically well‑posed. The choice to mandate a‑priori weight prediction is the right philosophical stance – even if unachievable now, it prevents the model from being degenerate.  
- The falsification test design (especially Test 4) is a model of how to make a complex theory testable. If the author follows through on OSF pre‑registration for all tests, MCOA will set a new standard for transparency in aging research.  
- The explicit corrections document (dated 2026-04-22) shows a willingness to retract erroneous claims (fabricated reference, inaccurate “Measuring In Vivo Mitophagy” citation). This is rare and praiseworthy.  
- The admission that the model “may be falsified” (Axiom M4) and the low threshold (R² < 0.05) indicate humility.

### Weaknesses / Counter‑Arguments
1. **The entire MCOA framework could be a tautology.** If *D_i* is defined as damage accumulated by counter *i*, and *L_tissue* is a weighted sum, then any observed aging phenomenon can be “explained” by adjusting *w_i* and *D_i* post‑hoc. The a‑priori weight constraint is meant to prevent this, but it has not yet been shown to be feasible. Until then, MCOA is a descriptive taxonomy, not a predictive theory.  
2. **The coupling matrix Γ may actually be dense, not sparse.** By defaulting to zero, MCOA ignores many well‑known interactions: telomere attrition → mitochondrial dysfunction (p53 → PGC‑1α), mitochondrial ROS → telomere shortening (oxidative damage), epigenetic drift → telomere maintenance (TERT expression). The author acknowledges only three entries, but the literature contains dozens more. Setting γ_ij = 0 until proven otherwise forces a large number of “post‑hoc statistical rejections” – each requiring a separate experiment. This makes full validation practically impossible.  
3. **The stem‑cell‑centric extension (winner‑counter) contradicts the parallel‑counter axiom (M1).** M1 states that *≥ 2 distinct damage processes proceed in parallel*. The winner‑counter hypothesis says one counter dominates in a given cell at a given time. These are compatible only if “parallel” means “across different cells” – but M1 is about *organisimal* aging, and the extension seems to introduce a hierarchical, not parallel, mode. This inconsistency is not resolved.  
4. **The “Damage Shadow” meta‑analysis is a textbook example of over‑interpreting negative results.** With only 14 studies (274 mice) and r = 0.09 (p = 0.44), the correct conclusion is “insufficient evidence to detect correlation”, not “no correlation”. The author uses this weak evidence to claim DNAmAge is not a valid surrogate – a potentially harmful statement that could undermine decades of epigenetic clock research based on multiple large cohorts (Horvath, Levine, Belsky). The claim should be withdrawn or re‑weighted.  
5. **The ABL‑2 paradox is a classic confound.** ABL‑2 was described as a “telomere‑length sensing kinase” but also regulates the actin cytoskeleton and cell migration. The Sobol ablation that showed centriole counting (CP) dominance over ABL‑2 in CDATA might simply reflect that ABL‑2’s effect on telomere length is indirect or tissue‑specific. Without a clean genetic experiment isolating ABL‑2’s role in centriole inheritance, the paradox remains.  
6. **Potential bias: The author is the sole named investigator.** A single MD (not a PhD in bioinformatics or experimental biology) is proposing a multi‑omics, multi‑species, multi‑year experimental programme. The project concept lacks any co‑PI with demonstrated expertise in statistical genetics, proteomics, or clinical trials. This is a major credibility gap for grant review – the red team must flag it.  
7. **The “piRNA candidate counter #6” is based on a single human study (Kraus 2026, n=1,271) and a worm study (Heestand 2025).** PiRNAs are primarily germline regulators in mammals; evidence for somatic roles in aging is extremely thin. Including it as a “candidate” without strong mechanistic support in mammals is premature and risks diluting the core model.  

### Additional Concerns
- The text mentions “fabricated references” in the corrections document. While corrected, the fact that a fabricated reference made it into a previous version suggests a systemic problem with literature management. The author should clarify how this happened and what steps are taken to prevent recurrence.  
- The EIC Pathfinder budget (€0.3M for WP1) seems low for a standard‑setting software library that includes JSON schemas, a Rust crate, Bayesian coupling estimation, and a community white paper. A postdoc + half‑PhD in 12 months is barely enough for the Rust implementation alone.  
- The sample size justification uses a formula for a two‑sample t‑test, not for multiple regression. This is a basic statistical error that must be corrected before any grant submission.  

### Score Table

| # | Criterion | Score | Justification |
|---|-----------|-------|---------------|
| 1 | Originality / Novelty | 3 | Interesting but largely repackaged hallmarks; some innovation in formalisation. |
| 2 | Significance / Impact | 2 | Overclaims (DNAmAge invalid) based on weak evidence; likely to be controversial. |
| 3 | Methodology / Rigor | 2 | Sample size error; meta‑analysis underpowered; weights not operationalised. |
| 4 | Clarity / Organization | 5 | Clear structure, but extensions add confusion. |
| 5 | Feasibility / Practicality | 2 | Consortium not real; budget unrealistic; a‑priori weights seem infeasible now. |
| 6 | Literature / References | 2 | Fabrication history damages trust; citation of unpublished drafts is problematic. |
| 7 | Integration with existing frameworks | 4 | Good connections but inconsistent with some subprojects (Ze). |
| 8 | Falsifiability / Testability | 3 | Good intentions but actual tests likely too expensive or imprecise. |
| 9 | Ethical / Societal considerations | 1 | No discussion of potential misuse of “aging counter” scores by insurers/employers. |
| 10 | Resource requirements / Budget | 1 | Underestimated for complexity; no confirmed partner contributions. |
| 11 | Overall assessment | 2 | Interesting idea but fatally hampered by unresolved methodological and credibility issues. |

**Score Sum:** 27 / 55

---

## Combined Review Summary

| Reviewer | Score Sum | Comment |
|----------|-----------|---------|
| A (Domain Expert) | 37 / 55 | Strong framework with operational gaps; weight prediction and sample size need work. |
| B (Fluff/Impact Auditor) | 37 / 55 | Honest and well‑structured, but overstates the damage‑shadow conclusion and consortium readiness. |
| C (Red Team) | 27 / 55 | Raises serious concerns about tautology, fabricated references, statistical errors, and feasibility. |

**Combined Score (minimum of three sums):** **27 / 55**

**Rating:** **Marginal – Major revisions required before resubmission.**

### Final Recommendation and Priority Actions

1. **Immediate:** Correct the sample size calculation using proper multiple‑regression power analysis (e.g., pwr.f2.test in R). Clarify the threshold (partial R² < 0.05 at α = 0.001) – consider increasing α to 0.05 or reducing the threshold to R² < 0.10 to make the test realistically powered.  
2. **Mandatory:** Provide at least a **proof‑of‑concept mapping** of *w_i(tissue)* for one tissue (e.g., HSC vs. brain) from published cell‑biological parameters. This does not need to be perfect, but it must demonstrate the *feasibility* of a‑priori prediction. Without this, Axiom M3 remains a dead letter.  
3. **Mandatory:** Address the fabricated‑reference issue explicitly in the main text, not just in a separate corrections document. State how literature will be verified going forward.  
4. **Strongly recommended:** Remove or significantly weaken the claim that DNAmAge is “not a valid surrogate for function”. The meta‑analysis is too small to draw that conclusion. Instead, say “correlation may be weaker than commonly assumed, warranting larger studies.”  
5. **Recommended:** Re‑evaluate the “winner‑counter” extension to avoid conflict with Axiom M1. Either reconcile or move to a separate follow‑up publication.  
6. **Recommended:** Seek a co‑PI with deep statistical or experimental expertise to strengthen the consortium for grant applications.  
7. **Optional but helpful:** Replace “inviolable axioms” with “core postulates”; the term “axiom” is pretentious and invites criticism.

**Decision:** Revise and resubmit for re‑review after addressing points 1–4. The core idea has merit but is not yet ready for funding or publication in its current state.