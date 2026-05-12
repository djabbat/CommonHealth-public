## Triple-Blind Peer Review — CDATA v5.3

### Reviewer 1 (Cell Biologist / Aging)

**Summary:** This manuscript presents an interesting but ultimately unproven hypothesis for a centriolar aging clock. While the authors are transparent about the failure of their quantitative model (ABL-2 paradox), the deductive argument that “a third mechanism must exist” is logically weak – it assumes completeness of the known pathways and ignores other well-established candidates (e.g., DNA damage accumulation, mitochondrial dysfunction, loss of proteostasis). The experimental program is ambitious but relies on a single pivotal experiment (P0) that is both expensive and technically challenging. The writing is clear, but the central claim remains unconvincing without direct evidence.

**Detailed Comments:**

- **Significance:** The question of whether centriolar damage contributes to aging is important, but the current work does not provide sufficient evidence to make it a priority.
- **Hypothesis clarity:** The hypothesis is stated explicitly, but the “deductively constrained argument” overstates its strength. The phrasing “most parsimonious candidate” is an improvement over earlier versions, but it still implies logical necessity.
- **Logical reasoning:** The central axiom (Hayflick limit in hypoxia with telomerase) is correct, but the conclusion that this *requires* a non-renewable centriolar clock is a non sequitur. Many other cellular components satisfy the ¬R condition.
- **Experimental design:** The proposed experiments are logical, but the reliance on GT335-STED in HSCs as the primary test is a high-risk, high-cost approach. Alternative, less expensive initial tests (e.g., simple immunofluorescence for polyGlu in mouse HSCs) would have been more practical.
- **Computational model:** The model is overparameterized (32 parameters for ~35 data points) and fails cross-validation. The ABL-2 paradox is honestly reported, but it fundamentally undermines the claim that centriolar damage is the main driver. The planned non-linear coupling is speculative.
- **Data interpretation:** The interpretation of the ABL-2 paradox as a “discovery” rather than a limitation is a stretch. The model’s failure does not rule out the hypothesis, but it also does not advance it.
- **Limitations:** The authors acknowledge most major limitations, but the tone remains overly optimistic. The paradox should be presented as a major hurdle, not a strength.
- **Originality:** The idea of centriolar PTM as a division counter is novel, which is a clear strength.
- **Feasibility:** The experimental program, especially P0, is expensive and may be difficult to execute within a standard grant budget. The timeline is too optimistic.
- **Writing:** Well-structured, though the Russian language section is unnecessary for an English-language journal.
- **Reproducibility:** The decision to release code only upon acceptance is suboptimal; pre-repository or at least release upon submission is the current norm.

**Score Table:**

| Criterion | Score (1-5) |
|-----------|-------------|
| 1. Significance of the question | 3 |
| 2. Clarity of hypothesis | 4 |
| 3. Logical reasoning | 2 |
| 4. Experimental design | 3 |
| 5. Computational model quality | 1 |
| 6. Data analysis and interpretation | 2 |
| 7. Handling of limitations and paradox | 4 |
| 8. Originality | 4 |
| 9. Feasibility | 2 |
| 10. Writing and organization | 4 |
| 11. Reproducibility and open science | 3 |
| **Total** | **32/55** |

---

### Reviewer 2 (Computational Biologist / Modeler)

**Summary:** The manuscript attempts to formalize the centriolar clock hypothesis using a quantitative model, but the model validation is severely flawed. The leave-one-out cross-validation yields a negative R², and the Sobol analysis shows that the epigenetic term dominates. The ABL-2 paradox is correctly identified, but the authors then propose ad hoc modifications (non-linear coupling) that are not tested. The paper is better framed as a hypothesis note, not a full research paper. The experimental predictions are well-reasoned, but the connection to the model is weak.

**Detailed Comments:**

- **Significance:** The question is worthwhile, but the current contribution is primarily methodological critique, not evidence.
- **Hypothesis clarity:** The hypothesis is stated, but the logical foundations are overclaimed. The “deductively constrained” label does not make the argument deductive.
- **Logical reasoning:** The ¬R argument is plausible but not exclusive. The authors should discuss why other candidates (e.g., asymmetric histone distribution, RNA clock) are less likely.
- **Experimental design:** The experiments are sound but designed to test the hypothesis, not the model. The model’s failure makes the design less motivated.
- **Computational model quality:** **Major concern.** 32 parameters fitted to ~35 data points is extreme overfitting. The Sobol analysis shows the centriolar parameter is secondary. The model is not validated; it is a toy. The failure of LOO-CV (R²= -0.093) is not just a paradox – it indicates the model is useless for prediction.
- **Data interpretation:** The authors treat the ABL-2 paradox as a discovery, but it is simply a consequence of poor model specification. The proposed non-linear coupling is untested and could introduce further overfitting.
- **Limitations:** The authors admit overfitting but do not quantify its impact (e.g., via AIC or BIC). The lack of independent validation data is a critical gap.
- **Originality:** The meta-approach (model failure as hypothesis generator) is somewhat original, but the biological idea is not new.
- **Feasibility:** The experimental program is feasible in principle, but the computational aspect is unconvincing.
- **Writing:** Clear, but the technical details are insufficient for reproducibility (e.g., exact solver, tolerance, parameter boundaries).
- **Reproducibility:** Code not public; this is a deal-breaker for a model-heavy paper.

**Score Table:**

| Criterion | Score (1-5) |
|-----------|-------------|
| 1. Significance of the question | 2 |
| 2. Clarity of hypothesis | 3 |
| 3. Logical reasoning | 3 |
| 4. Experimental design | 3 |
| 5. Computational model quality | 1 |
| 6. Data analysis and interpretation | 2 |
| 7. Handling of limitations and paradox | 3 |
| 8. Originality | 4 |
| 9. Feasibility | 2 |
| 10. Writing and organization | 3 |
| 11. Reproducibility and open science | 2 |
| **Total** | **28/55** |

---

### Reviewer 3 (Stem Cell Biologist / Aging Researcher)

**Summary:** This is an interesting hypothesis paper that honestly presents a failed computational model, then uses the failure to generate four testable hypotheses. The deductive argument for a third aging clock is plausible, though not airtight. The experimental program, especially the P0 test using GT335-STED in HSCs, is well designed and could provide definitive evidence. However, the cost and technical complexity are high, and the link to the model is weak. Overall, the work is promising but needs empirical support before it can be considered a significant contribution.

**Detailed Comments:**

- **Significance:** Aging clocks are a hot topic; a centriolar counter would be a major advance. The paper’s honesty about its failure is refreshing.
- **Hypothesis clarity:** Clearly stated. The “deductively constrained” language is toned down appropriately.
- **Logical reasoning:** The three axioms are well supported, but the conclusion that the centriole is the *only* candidate is overstated. The authors should acknowledge that the argument is a plausibility argument, not a proof.
- **Experimental design:** P0 is the right experiment. The use of H2B-GFP to track division history is elegant. However, the timeline of 8 weeks is optimistic; sorting HSCs and performing STED nanoscopy is time-consuming.
- **Computational model quality:** The model is elegantly simple, but it clearly fails. The authors use this failure to refine hypotheses, which is acceptable for a hypothesis paper. However, they should not claim the model is “formalizing the hypothesis” if it doesn’t work.
- **Data interpretation:** The ABL-2 paradox is correctly interpreted as a challenge to model specification, not a disproof of the biology. The four competing hypotheses are a strong framework.
- **Limitations:** Adequately acknowledged, though the post-mitotic aging gap is not well addressed.
- **Originality:** The idea of polyGlu as a division counter is original and testable.
- **Feasibility:** P0 is expensive but justified. The authors should consider a pilot experiment (e.g., using young vs. old mouse HSCs with conventional immunofluorescence) before launching the full STED study.
- **Writing:** Excellent, clear, and well-organized. The Russian language section is unusual but adds flavor.
- **Reproducibility:** The plan to release code upon submission is acceptable if the paper is accepted, but pre-registration of P0 is a good step.

**Score Table:**

| Criterion | Score (1-5) |
|-----------|-------------|
| 1. Significance of the question | 4 |
| 2. Clarity of hypothesis | 4 |
| 3. Logical reasoning | 3 |
| 4. Experimental design | 4 |
| 5. Computational model quality | 3 |
| 6. Data analysis and interpretation | 3 |
| 7. Handling of limitations and paradox | 3 |
| 8. Originality | 5 |
| 9. Feasibility | 3 |
| 10. Writing and organization | 4 |
| 11. Reproducibility and open science | 3 |
| **Total** | **39/55** |

---

### Combined Score

**Combined = MIN(32, 28, 39) = 28/55**

**Recommendation:** Major revision required. The hypothesis is interesting but the computational model is inadequate, and the deductive argument is not rigorous enough to support the strong claims. The experimental program is promising, but it needs to be better justified as a standalone set of tests, independent of the flawed model. The authors should either remove the model entirely (focusing on the deductive framework and experiments) or significantly improve the model validation (e.g., using independent datasets, reducing parameters, and testing alternative specifications before claiming a paradox). The paper cannot be accepted in its current form.