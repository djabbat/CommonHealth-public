# Triple-Blind Peer Review (TBPR) – MCOA Project Concept v1.0

**Reviewers:**  
- **Reviewer A** – Domain expert (aging biology, methodology, PI)  
- **Reviewer B** – Fluff/impact auditor  
- **Reviewer C** – Red team (counter-arguments, bias)  

---

## Reviewer A – Domain Expert

### Score table

| # | Criterion | Score (1–5) | Justification |
|---|-----------|-------------|---------------|
| 1 | Originality | 3 | Integrates existing damage counters (telomere, mtDNA, epigenetics, proteostasis) into a weighted framework; centriole polyglutamylation is novel but not yet empirically supported. |
| 2 | Significance | 4 | If validated, could unify disparate aging theories and guide multi-target interventions. High relevance. |
| 3 | Methodology/Formalism | 3 | Dimensional consistency requirement (Axiom M2) is appropriate; but coupling matrix Γ entries are mostly unknown and the a‑priori tissue weighting (Problem 1) remains unsolved with no clear path. |
| 4 | Data/Evidence | 2 | No primary data presented; relies on literature (some corrected in CORRECTIONS). Pre‑registration (osf.io/9x3k7) exists but protocol details are sparse. |
| 5 | Clarity & Structure | 4 | Well‑organized, explicit axioms, formal definitions, and relationship to subprojects. Extensions addenda (11) help contextualize but also add complexity. |
| 6 | Feasibility | 3 | Test 4 (division vs time) is tractable. Test 1 (longitudinal mouse) is high‑cost ($1.5M) and still missing a key weight‑prediction method. |
| 7 | Falsifiability | 4 | Axiom M4 defines an operational falsification threshold (partial r² < 0.05 each counter). However, N = 2000 is borderline for r² = 0.05; power analysis given for R² = 0.3, not for the threshold itself. |
| 8 | Reproducibility | 3 | Code and schemas planned (Rust crate, JSON schemas), but not yet available. Pre‑registration partly mitigates, but no independent replication planned. |
| 9 | Transparency | 3 | Self‑corrections are commendable, but some earlier statements (e.g., fabricated Sun 2016 citation) raise concerns about original vetting. |
| 10 | Impact | 4 | Could become a standard for comparing aging intervention effects across tissues. Potential to drive consortium‑scale research. |
| 11 | Ethics/Safety | 3 | No obvious ethical violations, but mortality endpoint in humans requires rigorous ethical oversight; no human study plan outlined. |

**Score Sum: 36/55**

### Overall evaluation (Reviewer A)

The MCOA concept is ambitious and intellectually coherent, but it suffers from a critical gap: the a‑priori tissue weighting function (`w_i(tissue)`) is declared an “inviolable axiom” but is not currently achievable. Without it, the framework reduces to a post‑hoc fitting exercise – exactly what Axiom M3 prohibits. The author acknowledges this (Problem 1 P0‑blocker). Until a protocol for deriving weights from independent cell‑biological parameters (division rate, TERT, etc.) is provided, the theory remains untestable at the organismal level. The falsifiability threshold (r² < 0.05) is a strong standard, but the power analysis supplied is for R² = 0.3, not for 0.05; sample size requirements for the actual threshold would be far larger (e.g., N ≫ 10 000). The centriole counter (#1) is intriguing but requires independent replication of CDATA results before being canonically accepted. The corrections document is honest but reveals that some foundational literature was initially erroneous – this damages credibility. Overall, the project deserves further development but is not yet ready for large‑scale funding as a standalone theory.

---

## Reviewer B – Fluff/Impact Auditor

### Score table

| # | Criterion | Score (1–5) | Justification |
|---|-----------|-------------|---------------|
| 1 | Originality | 3 | A “weighted sum of parallel counters” is a natural extension of the hallmarks of aging (López‑Otín 2013) and the network theory (Kirkwood). Centriole novelty is real but limited. |
| 2 | Significance | 4 | Could shape funding allocation in geroscience if widely adopted. |
| 3 | Methodology | 2 | Formal apparatus is mathematically sound, but the coupling matrix and tissue weights are essentially placeholders. The claim “all Γ entries must be measured, not fitted” is aspirational without a concrete measurement plan for most entries. |
| 4 | Data/Evidence | 1 | No experimental data. Pre‑registration is just a registration, not data. The extension manuscripts are “NOT YET PUBLISHED” drafts. |
| 5 | Clarity | 4 | Well written, good use of tables and definitions. The 5‑counter list is clear. |
| 6 | Feasibility | 3 | $6M+ total cost for all tests is high; near‑term Test 4 ($200k) is plausible. |
| 7 | Falsifiability | 5 | Explicit threshold and pre‑registration – best practice. However, the threshold is applied to each counter individually; if only one counter has r² < 0.05, does the whole MCOA fail? The wording “for every counter i” is strict but may never be met because some counters (e.g., epigenetic drift) are well‑established predictors. |
| 8 | Reproducibility | 3 | Plans for open‑source code are good, but no timeline for release. |
| 9 | Transparency | 4 | Self‑corrections enhance transparency; however, the earlier inclusion of fabricated references is a serious black mark. |
| 10 | Impact | 4 | Could facilitate interdisciplinary communication (biologists, mathematicians, clinicians). |
| 11 | Ethics/Safety | 3 | No human data collection proposed yet; but use of mortality endpoints would need careful consent and data handling. |

**Score Sum: 36/55**

### Overall evaluation (Reviewer B)

The MCOA project has strong narrative appeal and a clear structure, but it currently over‑promises and under‑delivers. The impact on the field could be substantial if the framework is validated, but the absence of even preliminary empirical support (beyond the existing counters) makes it more a “vision paper” than a testable research programme. The budget of multiple millions for full validation is not justified without a proof‑of‑concept for at least one counter’s weight prediction. The inclusion of a corrections appendix is praiseworthy but also signals that the initial review process was insufficient. I recommend that the authors first complete the Division vs Time test (Test 4) and publish a single‑counter weight prediction before seeking large‑scale funding. As it stands, the concept is intelligently packaged but lacks the substance to convince a sceptical community.

---

## Reviewer C – Red Team (Counter‑Arguments & Bias)

### Score table

| # | Criterion | Score (1–5) | Justification |
|---|-----------|-------------|---------------|
| 1 | Originality | 2 | The multi‑counter idea has been proposed before (e.g., “multiple causes of aging” de Grey, “pillars of aging” Kennedy 2014). The centriole counter is the only genuinely new element, but its role as a “master counter” (as hinted in extension) is not substantiated. |
| 2 | Significance | 3 | Even if true, translation would require measurement of 5+ counters in every tissue – impractical for clinical use. More parsimonious theories (e.g., epigenetic clock as umbrella marker) already explain more variance. |
| 3 | Methodology | 2 | The coupling matrix Γ is essentially unknown; the claim “must be measured, not fitted” is a virtue but means the model is currently vacuous. The dimensional reduction (D_i₀, α_i, β_i) adds 5N parameters; with 5 counters, that’s >20 parameters per tissue – prone to overfitting despite a‑priori constraints. |
| 4 | Data/Evidence | 1 | No new data. The only supporting evidence is literature that is itself contested (telomere length as predictor, for example, only weakly correlates with mortality after age 70). The corrections show that some cited “evidence” was fabricated, which severely undermines trust. |
| 5 | Clarity | 4 | Clear presentation, but clarity does not compensate for lack of empirical backbone. |
| 6 | Feasibility | 2 | A priori weight prediction from cell‑biological parameters is a wicked problem: division rate, metabolic intensity, etc. vary within tissue across time and with health status. The concept assumes static weights – unrealistic. |
| 7 | Falsifiability | 3 | The threshold r² < 0.05 for all counters is too strict: epigenetic clocks alone often yield r² > 0.05 for mortality after controlling for age and sex (see Marioni 2015, r² ≈ 0.07). So MCOA might be falsified immediately, but the authors likely chose a threshold guaranteed to be passed – unless they fail to measure the epigenetic counter correctly. This is a “heads‑I‑win, tails‑you‑lose” framing. |
| 8 | Reproducibility | 2 | No code, no data, no detailed protocol lengths or timelines for most tests. Pre‑registration is a shell (osf.io/9x3k7) – lacks specifics. |
| 9 | Transparency | 3 | Self‑correction is good; but why were fabricated references not caught before manuscript submission? This suggests a lax culture. |
| 10 | Impact | 2 | The “grand unified theory” style is common and often fails to gain traction. The paper is likely to be seen as a speculative framework, not a robust model. |
| 11 | Ethics/Safety | 3 | No major concerns, but the use of “mortality endpoint in humans” without a specific study plan is ethically irresponsible if it creates expectations. |

**Score Sum: 27/55**

### Overall evaluation (Reviewer C)

The MCOA project has conceptual appeal but is riddled with methodological weaknesses and overreach. The core problem is that the model has too many degrees of freedom (5 counters × tissue weights × coupling constants) and too few empirical constraints. The “a‑priori weight prediction” axiom is laudable but practically unattainable with current knowledge, making the theory unfalsifiable in the near term. The claim that centriole polyglutamylation is counter #1 and a “unifying structural counting device” is a strong but unsupported assertion; there is no evidence that it integrates signals from other counters. The falsifiability test (Axiom M4) is flawed: it asks for each counter to have partial r² < 0.05, but the well‑powered epigenetic clock studies (e.g., Horvath clock) already exceed that threshold, so the author would either have to exclude that counter or redefine it – both moves would be ad hoc. Furthermore, the sample size justification (N = 2000) is underpowered for detecting r² = 0.05; the actual required N would be in the tens of thousands. The corrections document, while transparent, reveals that the author has previously published fabricated references – this is a major red flag. I cannot recommend funding or endorsement of this project without independent validation of at least one counter and a resolution of the weight‑prediction blocker.

---

## Final Combined Score

- **Reviewer A** – 36/55  
- **Reviewer B** – 36/55  
- **Reviewer C** – 27/55  

**Combined Score = MIN(36, 36, 27) = 27/55**

### Recommendation

**Major Revisions Required** – The project concept is promising in its formal structure but currently lacks empirical support and contains a critical unresolved blocker (a‑priori weight prediction). The falsifiability criterion is well‑intentioned but potentially self‑defeating. The credibility damage from the retractions must be addressed through independent replication. I recommend resubmission after:

1. Performing and reporting Test 4 (Division vs Time) with results.
2. Providing a concrete, independent method for deriving at least one tissue‑specific weight.
3. Updating the power analysis for the actual r² = 0.05 threshold.
4. Expanding the pre‑registration with full protocol details.
5. Demonstrating integrity through open data and code release for the centriole counter.