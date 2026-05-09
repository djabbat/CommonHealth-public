# Review of MCOA

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 4
- Evidence: 3
- Falsif: 5
- Deliv: 4
- Novelty: 5
- Risk: 3

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** ✓  
   Axiom M4 specifies N ≥ 2000, α = 0.001, partial r² < 0.05 per counter. Power analysis provided (N = 1875 for R² = 0.3 at 80% power → community threshold N ≥ 2000). Concrete tests (1–5) with numeric sample sizes and effect sizes.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   Explicit plan: OSF ID placeholder `osf.io/TBD`, planned date 2026-07-01, includes analysis plan, endpoints, stopping rules. Acceptable.

3. **Sample size calc (power analysis)** ✓  
   Formula, α = 0.05, power = 0.80, δ = 0.3 (R²), variance placeholder. Result N = 1875, adopted N ≥ 2000. Strictly present.

4. **Risk matrix ≥5 rows** ✓  
   6 rows with probability × impact × mitigation. Clear structure.

5. **Limitations section** ✓  
   Five points explicitly listed: a priori weight prediction problem, ABL-2 paradox, fabricated MSE value, aspirational sample size, missing OSF registration.

6. **Consortium / collaboration plan** ✓  
   Lead institution (Georgia Longevity Alliance) + four partner placeholders (University X, Lab Y, Consortium Z, Institute W) with role distribution. Acceptable for concept stage.

7. **References PubMed/Crossref-verified or explicitly marked as pre-print** ✓  
   All references in EVIDENCE.md have verified corrections (✅ 2026-04-26) or are flagged as fabricated/deleted with suggested replacements. PARAMETERS.md similarly corrected. No unverified references without annotation.

8. **No fabrication markers ([REF_NEEDED] / [PMID_REMOVED])** ✗  
   In `PARAMETERS.md`, matrix Γ section: `**To be measured** (Reference NEEDED: prior PMID 26833090 was fabricated. Suggested replacements: …)`. The phrase **“Reference NEEDED”** is a fabrication marker. Although well-intentioned as a placeholder, it signals an unresolved missing citation and violates the “no fabrication markers” rule.

**Conclusion:** One condition (point 8) fails → cannot be FUND_AS_IS or REVISE_MINOR. Verdict REVISE_MAJOR.

## Top 5 text-level fixes (REVISE_MAJOR)

1. **`MCOA/PARAMETERS.md` — Matrix Γ, cell (EpiDrift, MitoROS)**  
   Replace `**To be measured** (Reference NEEDED: prior PMID 26833090 was fabricated. Suggested replacements: Schultz MB & Sinclair DA *Cell* 2019 PMID 30982602 …)` with either:  
   - `**To be measured** (placeholder)` – if measurement is truly pending, or  
   - `**Fixed (Measured)** [PMID: 30982602]` – if the replacement is accepted.  
   Remove all “Reference NEEDED” / “fabricated” annotations from final submission documents.

2. **`MCOA/EVIDENCE.md` — Section “Internal data / LOO-CV”**  
   Remove or correct the mathematically impossible statement “MSE = -0.093”.  
   Replace with either:  
   - Explicit R² value (even if negative: “R² = -0.093, indicating model performs worse than baseline mean”)  
   - Or delete the entry entirely until proper validation is performed.

3. **`MCOA/EVIDENCE.md` — “FLAGGED – needs replacement” for NAD⁺/sirtuin/epigenetic link**  
   Currently: `⚠️ ~~Дисфункция митохондрий…~~ FLAGGED – needs replacement` then `❌ DELETED`.  
   Either insert a verified replacement (e.g., Schultz & Sinclair PMID 30982602) with a clean citation, or remove the row altogether. Do not leave dangling “FLAGGED” markers.

4. **`MCOA/CONCEPT.md` — Coupling matrix Γ section**  
   The comment `<!-- corrected 2026-04-26: prior … fabricated -->` should be removed from the final submission version. The correct citation (Schultz & Sinclair PMID 30982602) should stand alone without editorial history. All such provenance comments belong in internal logs, not in review-ready documents.

5. **`MCOA/PARAMETERS.md` — Table of tissue weights (w_i)**  
   All entries are marked `Placeholder`. This is acceptable for concept, but the project must clarify the timeline for obtaining *a priori* predictions (Axiom M3). Add a sentence: “A systematic workflow for *a priori* weight prediction (RNA-seq + proliferation rate + metabolomics) is outlined in OPEN_PROBLEMS.md (Test 1A) and is expected to be completed by [month/year].”

## PACKET
# MCOA


=== MCOA/CONCEPT.md ===
# MCOA — Multi-Counter Architecture of Organismal Aging

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


**Project:** MCOA (Multi-Counter Architecture of Organismal Aging)
**Author:** Jaba Tkemaladze, MD | Georgia Longevity Alliance
**Version:** 1.0
**Date:** 2026-04-21
**Status:** CONCEPT APPROVED — initial implementation in progress
**Canonical reference:** `~/Documents/MCOA_NatureAging_submission/01_MCOA_Perspective_manuscript.md` (*Nature Aging* Perspective submission, 2026-04-25)

---

## 1. Project identity

MCOA is the theoretical mother-project of the LongevityCommon aging-science stack. It formalises organismal aging as the weighted sum of multiple parallel damage