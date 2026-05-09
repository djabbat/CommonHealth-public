# Review of CytogeneticTree

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 3
- Falsif: 1
- Deliv: 2
- Novelty: 5
- Risk: 3

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** — ✗  
   Multiple inconsistent thresholds appear across the proposal (N=24, N=43, α=0.001 with power=0.95, α=0.05 with power=0.80, effect sizes d=1.2 vs h=0.8 vs HR=2.0). Statements such as “N=24 is a conservative placeholder that will be updated” undermine binding falsifiability. No single unified table is presented.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   OSF ID “osf.io/TBD” and planned date “2026-07-01” are provided. However, the content described references multiple conflicting threshold tables, which weakens the plan.

3. **Sample size calc (power analysis)** — ✗  
   Multiple calculations with contradictory inputs (α=0.05, power=0.80 → N=6; α=0.001, power=0.95 → N=43; α=0.001, power=0.90 → N=24; plus attrition adjustments that differ). No single, clear, binding calculation is enforced.

4. **Risk matrix ≥5 rows** — ✓  
   A risk matrix with 7 rows (Probability × Impact × Mitigation) is present in CONCEPT.md.

5. **Limitations section** — ✓  
   A dedicated Limitations section is present, listing technical and conceptual risks. However, it does not mention the internal contradictions in falsifiability or sample size.

6. **Consortium / collaboration plan** — ✓  
   A placeholder table (roles, names, status) is provided. All but one role are “TBD” or “Placeholder”. This meets the minimum requirement for a plan, but is very weak.

7. **References PubMed/Crossref-verified or explicitly marked as pre-print** — ✗  
   The Lee & Luo 1999 reference is explicitly flagged as unverified, and its DOI is described as “fabricated or pointing to wrong article”. No pre‑print DOI is given as alternative. The proposal contains a “[Reference removed pending verification]” marker, which is not an acceptable verification status.

8. **No fabrication markers ([REF_NEEDED] / [PMID_REMOVED])** — ✗  
   The text includes “[Reference removed pending verification — see Limitations section for details]” and comments about a fabricated DOI. These are clear fabrication markers that must be eliminated.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

1. **CONCEPT.md: Falsifiability table + Sample size calculation** — Unify all thresholds into a single **binding** table. Remove all contradictory placeholder statements (e.g., “N=24 is a conservative placeholder that will be updated”). Specify one α, one power, one effect size, one N per arm. Ensure the pre‑registration plan references this unified set only.

2. **CONCEPT.md: Pre‑registration plan** — State explicitly that the falsifiability thresholds are **fixed** and any change requires an OSF amendment **before** data analysis. Remove the phrase “will be updated once pilot data are available”.

3. **KNOWLEDGE.md: Reference verification** — The Lee & Luo (1999) reference must be **verified** (check PMID 10197526 and DOI) or **replaced** with a verified preprint DOI. If unverifiable, remove the claim and the reference. Remove all “REFERENCE VERIFICATION PENDING” flags.

4. **KNOWLEDGE.md: Fabrication markers** — Delete the comments “[Reference removed pending verification…]” and the sentence about the fabricated DOI. Replace with either a clean reference or a statement that the claim is unsupported and removed.

5. **CONCEPT.md: Consortium plan** — Replace all “TBD” placeholder partners with at least **named potential collaborators** or a concrete recruitment plan (e.g., “Dr. Jane Smith, University of Cambridge – letter of intent pending”). Provide at least one confirmed partner beyond Ilya Zheleznov, or explain why the project can proceed without them.

## PACKET
*(Proposal files as provided by the user: CONCEPT.md, PARAMETERS.md, KNOWLEDGE.md, README.md — see above.)*