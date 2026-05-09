# Review of CytogeneticTree

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- **Premise:** 4 – Ambitious and scientifically interesting, but reliance on unproven centriole-age universality weakens the foundation.
- **Method:** 3 – RITE construct does not exist; closed-loop pipeline is aspirational; some components are off-the-shelf but integration risk is high.
- **Evidence:** 2 – Literature base is broad but contaminated with unverified/fabricated references; internal evidence (CDATA) is not independently validated.
- **Falsifiability:** 3 – Numeric thresholds present but incoherent across sections (N=24 vs N=43, α=0.001 vs α=0.05). After cleaning, could be strong.
- **Deliv:** 2 – Timeline is optimistic (6-month RITE cloning, 6-month main experiment); consortium is >80% placeholder; budget appears insufficient for stated scope.
- **Novelty:** 5 – Intersection of centriole-age labelling, full-lineage DAG reconstruction, and RITE-on-centrioles is genuinely first.
- **Risk:** 1 – Extremely high: unvalidated construct, unverified references, missing partner commitments, contradictory thresholds.

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** – **✓** (thresholds exist: N=43, α=0.001, power≥0.90, effect sizes specified)  
   *However, multiple contradictory versions exist within the same document (N=24, α=0.05); the final unified table is acceptable if all others are removed.*

2. **Pre-registration plan (OSF placeholder + date)** – **✓** (osf.io/TBD, 2026-07-01, content described)

3. **Sample size calc (power analysis)** – **✓** (formula, parameters, attrition adjustment, N=43)  
   *Again, previous versions (N=24, N=15, N=10, N=6) create confusion.*

4. **Risk matrix ≥5 rows** – **✓** (7 rows present with probability, impact, mitigation)

5. **Limitations section** – **✓** (multiple limitations listed, including construct risk, purity, phototoxicity, algorithm, generalisability)

6. **Consortium / collaboration plan** – **✓** (table with roles and status, even if most are TBD; recruitment plan described)

7. **References PubMed/Crossref-verified or explicitly marked as pre-print** – **✗**  
   - KNOWLEDGE.md contains **“REFERENCE VERIFICATION PENDING”** for Lee & Luo 1999 (not verified, not marked as pre-print)  
   - Contains **“[Reference removed pending verification]”** – i.e., a fabrication marker  
   - The audit note states fabrication cleanup was applied, but the pending reference remains unresolved.

8. **No fabrication markers ([REF_NEEDED] / [PMID_REMOVED])** – **✗**  
   - “[REFERENCE VERIFICATION PENDING]” and “[Reference removed pending verification]” are functionally equivalent to fabrication markers.  
   - The document itself acknowledges a fabrication cleanup (REF_AUDIT_2026-05-08), indicating prior fabrication was present.

**Checklist result: 2 failures (items 7 and 8).** Therefore FUND_AS_IS / REVISE_MINOR are ineligible.

## Top 5 text-level fixes (must be implemented before resubmission)

1. **KNOWLEDGE.md: Resolve all unresolved references**  
   - Remove or verify `Lee & Luo 1999` (likely PMID 10197526, but it must be verified against PubMed/Crossref). If verifiable, add verified PMID; if not, delete the claim or replace with a pre-print DOI.  
   - Remove all `[Reference removed pending verification]` placeholders – either restore with a verified source or delete the associated sentence.  
   - Add explicit `[PREPRINT: ...]` markers for any reference that is not yet peer-reviewed.

2. **CONCEPT.md: Unify falsifiability thresholds and sample size**  
   - Delete all earlier draft values (N=6, N=10, N=15, N=24, α=0.05, power=0.80).  
   - Keep only the **final binding table** (N=43, α=0.001, power≥0.90).  
   - Ensure that PARAMETERS.md and KNOWLEDGE.md are synchronised: currently PARAMETERS.md states α=0.05, β=0.20, which directly contradicts CONCEPT.md. Fix PARAMETERS.md to match.

3. **CONCEPT.md: Remove all fabrication-cleanup commentary**  
   - The note about `FABRICATION CLEANUP applied` and the audit trail are internal markers that should not appear in a submission-ready document.  
   - Also remove any `[REF_AUDIT_2026-05-08]` annotations. The document must be clean of such metadata.

4. **CONCEPT.md: Strengthen consortium plan**  
   - Replace placeholder entries (e.g., “TBD suggested: Dr. Jane Smith”) with actual names and letters of support, or at minimum provide a credible recruitment timeline with concrete targets.  
   - Currently only **Ilya Zheleznov** is confirmed; all other roles are TBD. For a high-risk proposal this is insufficient.

5. **CONCEPT.md: Add explicit go/no-go decision points**  
   - The risk matrix mentions a go/no-go for RITE expression (Month 6), but this is not formalised in the limitations or milestones.  
   - Add a pre-registered stopping rule: e.g., “If centriole-RITE labelling <60% by Month 6, the project is terminated or pivots to SNAP-tag alternative.”  
   - Correspondingly, update the risk matrix and timeline.

**Additional minor issues:**  
- Multiple sections are duplicated (e.g., three separate “Pre-registration plan” blocks). Consolidate.  
- Some references in KNOWLEDGE.md are not listed in any main text; ensure all cited references are present and vice versa.  
- The budget in PARAMETERS.md ($92,000) appears insufficient for 6-month full-time technician, cloning, imaging hardware, and consumables – provide a justification or revise.

**If these fixes are implemented, the proposal could be re-evaluated for REVISE_MINOR or FUND_AS_IS.**