# Review of CytogeneticTree

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- **Premise:** 4 — The core idea (centriole-age as lineage marker) is original, biologically grounded, and timely. Strong literature anchoring.
- **Method:** 3 — Detailed but riddled with inconsistent thresholds and unresolved technical risks. Lacks a single coherent analytical framework.
- **Evidence:** 3 — Literature support is solid (verified PMIDs) but no pilot data, no construct validation, no algorithm prototype. Entirely conceptual.
- **Falsifiability:** 2 — Thresholds exist but are internally contradictory (N=24 vs. N=43 vs. N=10 per arm; α=0.001 vs. 0.05; power 0.95 vs. 0.80). Not operationalisable as written.
- **Deliverability:** 3 — Timeline is optimistic (6 months for RITE construct design+validation+main experiment). Budget appears reasonable but missing key cost items (FACS, sequencing, consumables for long-term culture).
- **Novelty:** 4 — High novelty in the intersection of centriole-age tagging + lineage reconstruction + RITE application. Genuine firsts in the combination.
- **Risk:** 2 — Unvalidated construct, no FACS method for centriole colour, phototoxicity, algorithm nonexistent, consortium largely placeholder. Very high technical risk.

## Checklist (✓/✗ each + explanation)

| # | Condition | Status | Explanation |
|---|-----------|--------|-------------|
| 1 | **Operationalised falsifiability (numeric thresholds)** | ✗ | The proposal contains multiple contradictory threshold tables: one with N=43/arm (α=0.001, power=0.95, d=1.2), another with N=24/arm (α=0.001, power=0.90, d=0.8), and an earlier one with N=10/arm (α=0.05, power=0.80). Although later sections claim earlier values are superseded, the document still includes the conflicting tables without clear hierarchy. No unified, binding threshold table is presented. |
| 2 | **Pre-registration plan (OSF placeholder + date)** | ✓ | OSF ID osf.io/TBD provided; planned registration date 2026-07-01; content outline (endpoints, exclusion criteria, stopping rules) present. |
| 3 | **Sample size calculation (power analysis)** | ✗ | Multiple formulas and results: e.g., N=43/arm (δ=1.2σ, α=0.001, power=0.95), N=24/arm (δ=0.8, α=0.001, power=0.95), N=10/arm (δ=0.5, α=0.05, power=0.80). Inconsistency makes the calculation non-binding. No single definitive calculation for the primary endpoint. |
| 4 | **Risk matrix ≥5 rows** | ✓ | Contains risk matrices with 5–7 rows, each with probability × impact × mitigation. |
| 5 | **Limitations section explicit** | ✓ | Multiple limitations sections (technical risks, sample size assumptions, generalisability). |
| 6 | **Consortium / collaboration plan (even placeholder list)** | ✓ | Table of potential partners with roles and status (most TBD). Letters of support pending. Acceptable as placeholder. |
| 7 | **All references PubMed/Crossref-verified or explicitly marked as pre-print** | ✗ | KNOWLEDGE.md claims all PMIDs verified, but one reference (Lee & Luo 1999 *Neuron*) is flagged as "REFERENCE VERIFICATION PENDING". It is not explicitly labelled as a pre-print and lacks a valid DOI. This violates the condition. |
| 8 | **No fabrication markers ([REF_NEEDED] / [PMID_REMOVED])** | ✗ | Contains "[Reference removed pending verification — see Limitations section for details]" and "[REFERENCE VERIFICATION PENDING]" markers. These are fabrication markers indicating unresolved references. |

## Top 5 text-level fixes (REVISE_MAJOR required)

1. **CONCEPT.md:Falsifiability** — Replace all contradictory threshold tables with a single, binding table. Example: use α=0.001 (Bonferroni-corrected for 3 comparisons → per-test α=0.00033), power=0.95, effect size Cohen's d=1.2, N=43/arm (attrition-adjusted). Delete all earlier N=24, N=10, N=15 tables. Update the text to state: "This table is the sole binding falsifiability threshold for the proposal; any previous values are null and void."

2. **CONCEPT.md:Sample size calculation** — Remove all but one calculation. Align with the unified falsifiability table. Example: `n = (Z_α/2 + Z_β)² · 2σ² / δ²` with α=0.001, power=0.95, d=1.2 → N=43 per arm. Delete the separate calculations for H2/H3; state they use the same N per arm. Provide G*Power output screenshot if possible.

3. **CONCEPT.md:Limitations** — Add a dedicated limitation about the unresolved fabrication markers and pending reference verification. State explicitly that the Lee & Luo 1999 reference will be either verified (PMID check) or replaced with a verified pre-print/DOI before submission. Remove the "[Reference removed pending verification]" placeholder.

4. **KNOWLEDGE.md:Lee & Luo reference** — Either (a) verify PMID 10197526 and confirm it matches the claim, or (b) replace with a verified reference that supports the same statement, or (c) delete the reference and remove the associated claim. Do not leave it as "REFERENCE VERIFICATION PENDING" in the final version.

5. **CONCEPT.md:Pre-registration** — Update the pre-registration content to explicitly reference the unified falsifiability table and single sample size calculation. Add a statement: "No deviations from these numeric thresholds will be permitted without a documented OSF amendment." Remove any contradictory statements about "conservative placeholder" or "will be updated once pilot data available."

## PACKET
*(No additional content – review only.)*