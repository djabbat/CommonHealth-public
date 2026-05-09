# Review of CytogeneticTree

## Verdict
**REVISE_MAJOR**

## Scores (1–5)
- **Premise**: 4 – high originality, centriole-as-physical-marker is elegant
- **Method**: 2 – multiple contradictory calculations; key steps (RITE→centriole, FACS by colour) not validated; pipeline integration unclear
- **Evidence**: 2 – good literature landscape but critical references unresolved; no pilot data for effect size
- **Falsifiability**: 1 – no single, consistent numeric threshold set; α, power, N vary across sections
- **Deliv**: 2 – timeline and deliverables based on unresolved placeholders; risk of underpowering high
- **Novelty**: 4 – genuine gap at intersection of RITE, centriole biology, lineage reconstruction
- **Risk**: 3 – risk matrix exists but mitigation strategies are generic; key technical risks underestimated
- **RefIntegrity**: 1 – non-verified references present; fabrication markers ([REF_VERIFY], removed references) left in final text; one cited DOI leads to wrong paper

## Checklist (✓/✗ + explanation)

1. **Operationalised falsifiability (numeric thresholds)** ✗  
   Multiple conflicting tables (N=43, N=24, N=15; α=0.001, α=0.05; power=0.95, 0.90). No single binding threshold set. Section claims “unified” but later contradicts itself.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   OSF ID = `osf.io/TBD`, planned date 2026-07-01. Acceptable placeholder for proposal stage.

3. **Sample size calculation (power analysis with formula)** ✗  
   Three different calculations (N=43 with formula, N=24 with formula, N=15 with formula). Final “binding” calculation (N=43) conflicts with the “unified falsifiability table” (N=24). No consistency.

4. **Risk matrix ≥5 rows** ✓  
   Contains 7 rows with probability, impact, mitigation. However, mitigations are vague (e.g., “test 3 alternative constructs” without budget or timeline).

5. **Limitations section** ✓  
   Yes, a dedicated list of 6+ limitations is present.

6. **Consortium / collaboration plan** ✓  
   Placeholder list of roles; some confirmed (Ilya Zheleznov), most TBD. Acceptable for proposal stage.

7. **Reference reality + match** ✗  
   - **Unresolved**: Lee & Luo 1999 *Neuron* marked “REFERENCE VERIFICATION PENDING” – no validated PMID.  
   - **Unresolved**: Parrinello 2003 cited in PARAMETERS.md – marked `REF_VERIFY` without DOI.  
   - **Fabrication cleanup**: One reference removed pending verification; its claim is retained without independent support.  
   - **DOI mismatch**: KNOWLEDGE.md notes PMID 31485075 was initially attributed to Loeffler but is actually quantum physics (corrected to 31485073). This indicates earlier uncritical reference usage.  
   Additionally, the component cites “Royall 2023” (verified), but the claim in CONCEPT.md about RITE in centrioles not having prior art is well-supported. However, unresolved references and fabrication cleanup markers violate condition 7.

8. **No fabrication markers** ✗  
   `[REF_VERIFY]`, `[Reference removed pending verification]`, `[REFERENCE VERIFICATION PENDING]` are present in final text. These are not acceptable.

9. **Internal consistency core documents** ✗  
   - CONCEPT.md contains **three different sample size calculations** and **two different falsifiability threshold tables**.  
   - PARAMETERS.md uses α=0.05, while CONCEPT.md uses α=0.001 for primary tests.  
   - Pre-registration plan description differs between sections (one says “N=43 per arm”, another “N=24 per arm”).  
   - “Single binding sample size calculation” is contradicted by the “unified falsifiability table” that uses N=24.

## Reference audit

All references from KNOWLEDGE.md and CONCEPT.md are listed below. Only the ones that fail verification or match are flagged.

| # | Citation (short) | DOI/PMID | Real? | Matches text? | Decision |
|---|------------------|----------|-------|---------------|----------|
| 1 | Yamashita YM 2007 *Science* | PMID 17255513 | Yes | Yes – asymmetric centriole inheritance in Drosophila GSCs | OK |
| 2 | Wang X. 2009 *Nature* | PMID 19829375 | Yes | Yes – mammalian neural progenitors | OK |
| 3 | Royall LN 2023 *eLife* | PMID 37882444 | Yes | Yes – human neural progenitor cells | OK |
| 4 | Verzijlbergen KF 2010 *PNAS* | PMID 20018668 | Yes | Yes – original RITE method | OK |
| 5 | Thayer NH 2014 *PNAS* | PMID 25228775 | Yes | Yes – RITE to track what is retained across divisions | OK |
| 6 | Mahecic D 2022 *Nat Methods* | PMID 36076039 | Yes | Yes – event-driven acquisition | OK |
| 7 | Chan MM 2019 *Nature* | PMID 31086336 | Yes | Yes – mouse embryogenesis CRISPR barcoding | OK |
| 8 | Kalhor R 2018 *Science* | PMID 30093604 | Yes | Yes – MARC1 homing CRISPR | OK |
| 9 | Januschke J 2011 *Nat Commun* | PMID 21407209 | Yes | Yes – counter-example for CDATA | OK |
| 10 | Loeffler D 2019 *Nature* | PMID 31485073 | Yes | Yes – HSC asymmetric lysosome inheritance | OK (corrected) |
| 11 | Lee & Luo 1999 *Neuron* | PMID 10197526 (suggested) | **NOT VERIFIED** | Claim: MARCM technique for lineage analysis. The text says “REFERENCE VERIFICATION PENDING”. No evidence that the DOI 10.1016/S0896-6273(00)80701-1 is legitimate. | **FAIL** |
| 12 | Parrinello 2003 | No DOI/PMID given | **NOT VERIFIED** | Claim: physiological hypoxia 3% O₂. Marked `REF_VERIFY`. | **FAIL** |
| 13 | Gönczy P 2023 *Genetics* | PMID 36988082 | Yes | Yes – C. elegans centriole segregation | OK |
| 14 | Tkemaladze (self-citations) | various | Yes (presumed) | Not independently checked | Needs verification but less critical |
| 15 | All other PMIDs in Block 2,3,4,6 of KNOWLEDGE.md | listed | Yes per esummary | Yes | OK |

Two references fail verification (Lee & Luo 1999, Parrinello 2003). Additionally, the component contains one removed reference with retained claim, which is a fabrication marker.

## Top 5 text-level fixes

1. **Resolve all unresolved references and remove fabrication markers**  
   - `CONCEPT.md` and `KNOWLEDGE.md`: either provide a verified DOI/PMID for Lee & Luo 1999 (or replace with a verified source) or remove the claim.  
   - `PARAMETERS.md`: either add a verified DOI for Parrinello 2003 or remove the citation.  
   - Remove all `[REF_VERIFY]`, `[Reference removed pending verification]`, `[REFERENCE VERIFICATION PENDING]` markers.

2. **Unify falsifiability thresholds and sample size calculations**  
   - Delete all but one set of thresholds (α, power, effect size, N per arm). Recommend using the N=43 calculation (α=0.001, power=0.95, d=1.2) and consistently apply it in the falsifiability table and pre-registration plan.  
   - Remove the earlier tables with N=24, N=15, α=0.05.  
   - Ensure the “single binding sample size calculation” is the only one in the document.

3. **Fix internal contradictions between CONCEPT.md and PARAMETERS.md**  
   - `PARAMETERS.md` lists α=0.05, β=0.20; but `CONCEPT.md` uses α=0.001, power=0.95. Harmonise to the primary falsifiability thresholds.  
   - Align the pre-registration plan content so that it references the same unified thresholds everywhere.

4. **Clarify the operational implementation of two critical steps**  
   - **FACS by centriole colour**: no published precedent; add a paragraph explaining the expected gating strategy, purity requirements, and how the imaging sorter will be used.  
   - **RITE-centriole cassette**: specify which centriole protein (Centrin, SAS-6, CPAP?), design validation steps, and timeline. Currently “must be built” with 3–6 months, but no concrete plan.

5. **Strengthen the “Limitations” section by acknowledging unresolved statistical conflicts**  
   - Explicitly state that the sample size calculation assumes a large effect size (d=1.2) and that if pilot data show smaller effects, the study will be underpowered. Add a discussion of a possible effect size recalculation plan.

## PACKET
- **Verdict**: REVISE_MAJOR  
- **Scores**: Premise 4, Method 2, Evidence 2, Falsif 1, Deliv 2, Novelty 4, Risk 3, RefIntegrity 1  
- **Failing conditions**: 1 (falsifiability), 3 (sample size inconsistency), 7 (reference unreliability), 8 (fabrication markers), 9 (internal consistency)  
- **Key issues**: unresolved references, contradictory numeric thresholds, fabrication markers, α/power mismatch between core documents  
- **Action required**: Major overhaul to provide a single consistent falsifiability framework and verifiable reference set. After revision, re-review needed.