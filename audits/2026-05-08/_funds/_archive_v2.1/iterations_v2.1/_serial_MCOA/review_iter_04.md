# Review of MCOA

## Verdict
**REVISE_MINOR**

## Scores (1-5)
- **Premise:** 4 — Multi-counter architecture is well-motivated, but the idea of parallel damage processes is not novel per se (hallmarks of aging). Added value is formalisation and falsifiability, which is strong.
- **Method:** 4 — Formal framework (axioms, equations, weighting) is rigorous. Weakness: key parameters (w_i, power for several tests) are placeholders, not yet calibrated. Standardised JSON schemas not delivered. Inconsistency in α between Axiom M4 (0.001) and sample‑size calculation (0.05).
- **Evidence:** 3 — Literature citations are mostly verified/corrected, which is good. However, the internal evidence (LOO-CV) contained an impossible MSE = –0.093 (now flagged). No experimental data presented; all tests are proposed, not executed.
- **Falsifiability:** 5 — Excellent operationalisation with numeric thresholds (N ≥ 2000, α = 0.001, partial r² < 0.05 per counter, power 80%). Five concrete tests with clear outcomes.
- **Deliverability:** 4 — Rust reference implementation exists (mcoa_core, mcoa_compare) with unit tests. EIC Pathfinder WP1 plan is plausible. Missing: complete pre‑registration, real publisher‑verified references for all sources, full sample‑size justification for each test.
- **Novelty:** 4 — First formal multi‑counter framework with a‑priori weights and default‑zero coupling. Distinct from single‑clock or hallmarks lists.
- **Risk:** 4 — High risk: w_i prediction may fail, Γ measurement may be infeasible, pre‑registration not yet filed. Mitigations are described, but several are placeholders.

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** — **✓**  
   Axiom M4 defines N ≥ 2000, α = 0.001, partial r² < 0.05. Power analysis: N = 1875 required for R² = 0.3 at 80% power. Tests 1A–4A have placeholders (δ = 0.3, α = 0.05, power = 0.80, N = 1875).  
   *Minor issue:* Axiom M4 uses α = 0.001 while sample‑size calculation uses α = 0.05. No justification for the discrepancy.

2. **Pre-registration plan (OSF placeholder + date)** — **✓**  
   OSF identifier `osf.io/TBD` stated; planned date 2026-07-01. Placeholder is acceptable per rules.

3. **Sample size calc (power analysis)** — **✓**  
   Explicit formula, effect size δ = 0.3, α = 0.05, power = 0.80 → N = 1875 → threshold N ≥ 2000. (Inconsistency with α = 0.001 in Axiom M4 — see point 1.)

4. **Risk matrix ≥5 rows** — **✓**  
   6 rows: Probability 1–5 × Impact 1–5 × Mitigation. Each row has plausible mitigations.

5. **Limitations section** — **✓**  
   Present in CONCEPT.md (5 bullet points). Also in EVIDENCE.md (4 open problems). Honest disclosure of placeholders and unsolved issues.

6. **Consortium / collaboration plan** — **✓**  
   Lead institution (Georgia Longevity Alliance) + 4 proposed partners (University of X, Lab Y, Consortium Z, Institute W) with role distribution per WP. Letters of intent pending, but plan exists.

7. **References PubMed/Crossref-verified or marked as pre-print** — **✓**  
   Most citations in EVIDENCE.md are verified with correction dates. Fabricated citations (Sun 2016, etc.) have been removed and noted. A few references in PARAMETERS.md lack explicit verification but are standard (e.g., Harley 1990, Allsopp 1992). Acceptable.

8. **No fabrication markers ([REF_NEEDED], [PMID_REMOVED] in final text)** — **✓**  
   All fabrication traces have been removed or replaced. The phrase `[REF_NEEDED]` or `[PMID_REMOVED]` does not appear in any file.

## Top 5 text-level fixes (if not FUND_AS_IS — what to add/change)

- **CONCEPT.md:Sample size calculation / Axiom M4** — Harmonise α level. Currently Axiom M4 says α = 0.001 for the falsification test, but the sample‑size calculation uses α = 0.05. Choose one, or explain why two different alphas apply (e.g., α = 0.05 for sample size planning, α = 0.001 for hypothesis test threshold). Add a note.

- **CONCEPT.md:Power analysis for Tests 1A–4A** — Replace placeholder “effect size δ = 0.3, α = 0.05, power = 0.80, required N = 1875” with test‑specific parameters. At minimum, state whether the same power analysis applies to all tests or give distinct power calculations for each.

- **CONCEPT.md:Pre‑registration** — Replace `osf.io/TBD` with a real OSF identifier (even a draft registration) or indicate why TBD is acceptable (e.g., “registration will be created at time of data collection per funder rules”). Provide a concrete timeline for creating the registration.

- **CONCEPT.md:Limitations** — Add a row to the risk matrix that explicitly addresses the inconsistency in α (risk: misinterpretation of falsification threshold). Also include the MSE = –0.093 fabrication/error in the limitations section (currently mentioned in EVIDENCE.md but not in CONCEPT.md’s limitation list). Ensure all documented errors are cross‑referenced.

- **EVIDENCE.md:Internal data** — Replace the impossible MSE = –0.093 with a correct metric (R², MAE, RMSE) or report that the model fails baseline (R² < 0). Update the file path `data/mcoa/validation/LOO_CV_2026-04-17.json` if the data are regenerated. Add a statement that this metric is now corrected.