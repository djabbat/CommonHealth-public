# Review of EpigeneticDrift

## Verdict
**REJECT**

## Scores (1–5)
- **Premise:** 4
- **Method:** 4
- **Evidence:** 3
- **Falsifiability:** 5
- **Deliv.:** 3 (technical architecture is promising, but pre‑registration and consortium still placeholder‑heavy)
- **Novelty:** 4
- **Risk:** 3 (matrix present, but does not account for fabricated references)
- **RefIntegrity:** 1 (one lethal fabrication)

## Checklist (✓/✗)

| # | Condition | Status | Explanation |
|---|-----------|--------|-------------|
| 1 | **Operationalised falsifiability (numeric thresholds)** | ✓ | All tests list numeric decision rules: β₄ 95% CI ≠ 0, α₄ d≥0.3, coupling f²=0.15, |r|>0.3 p<0.01, power ≥0.80. |
| 2 | **Pre‑registration plan (OSF placeholder + date)** | ✓ | https://osf.io/TBD, planned 2026‑09‑01. Placeholder in pre‑reg plan is acceptable (rule 8 allows it). |
| 3 | **Sample size calc (power analysis)** | ✓ | Two calculations provided: t‑test N=64/group (Δ=0.2, α=0.05, power=0.80) and regression N=92 (f²=0.15). Formula and software (G*Power) stated. |
| 4 | **Risk matrix ≥5 rows** | ✓ | 5 rows: probability × impact × mitigation. Real risks (ABL‑2 confound, β/α separation, universal D₄, coupling measurability, tissue diversity). |
| 5 | **Limitations section** | ✓ | 5 honest limitations (causality, reversibility, layer inconsistency, tissue specificity, measurement noise). |
| 6 | **Consortium / collaboration plan** | ✓ | Three groups listed with roles (PI theoretical, Horvath clocks, Brunet experiments). Placeholder names acceptable at this stage. |
| 7 | **Reference reality + match** | **✗** | **LETHAL FAILURE.** PMID 41289991 (Arif et al. 2025) does **not** resolve in PubMed/Crossref – it is a fabricated identifier. Per rule: *“Невалидный идентификатор = автоматический REJECT компонента.”* All other checked references are real (see audit below). |
| 8 | **No fabrication markers** | **✗** | The fabricated PMID 41289991 is a strong fabrication marker. |
| 9 | **Internal consistency core docs** | ✓ | THEORY, EVIDENCE, PARAMETERS, OPEN_PROBLEMS, DESIGN logically coherent; no contradictions between core files. Cancelled MCOA Test 2 is properly struck‑through. |

## Reference audit

| # | Short citation | DOI/PMID | Real? | Matches text? | Decision |
|---|----------------|----------|-------|---------------|----------|
| 1 | Horvath & Raj 2018, *Nat Rev Genet* | PMID 29643443 | ✓ | ✓ | OK |
| 2 | Horvath 2013, *Genome Biol* | PMID 24138928 | ✓ | ✓ | OK |
| 3 | Belsky et al. 2022, *eLife* | PMID 35029144 | ✓ | ✓ | OK |
| 4 | Duan et al. 2022, *Ageing Res Rev* | PMID 36206857 | ✓ | ✓ | OK |
| 5 | Lu et al. 2019, *Aging* | PMID 30669119 | ✓ | ✓ | OK |
| 6 | Morandini et al. 2024, *Nat Aging* | PMID 37924441 | ✓ | ✓ | OK |
| 7 | Adelman et al. 2019, *Cancer Discov* | PMID 31085557 | ✓ | ✓ | OK |
| 8 | Deng et al. 2021, *Cell Stem Cell* | PMID 33571444 | ✓ | ✓ | OK |
| 9 | Bogeska et al. 2022, *Cell Stem Cell* | PMID 35858618 | ✓ | ✓ | OK |
| 10 | Kasbekar et al. 2023, *Nature* | PMID 37865087 | ✓ | plausible (inflammatory HSC) | OK |
| 11 | Kao et al. 2024, *Cell Stem Cell* | PMID 38402617 | ✓ | ✓ | OK |
| 12 | Meng et al. 2025, *Cell Stem Cell* | PMID 39271425 | ✓ | ✓ | OK |
| 13 | Yokomizo et al. 2024, *Blood* | PMID 38640057 | ✓ | ✓ | OK |
| 14 | Horvath et al. 2018, *Aging* | PMID 30048243 | ✓ | ✓ | OK |
| 15 | Lu et al. 2022, *Nat Aging* | PMID 36516495 | ✓ | ✓ | OK |
| 16 | Zheng et al. 2024, *Aging Cell* | PMID 38482631 | ✓ | ✓ (generalisability) | OK |
| 17 | Bischoff-Ferrari et al. 2025, *Nat Aging* | PMID 39900648 | ✓ | ✓ (intervention effect on clocks) | OK |
| 18 | Roberts et al. 2021, *Clin Epigenetics* | PMID 34587750 | ✓ | weak match (used for proteostasis‑methylation link, only indirect) | [REF_VERIFY] – weak match, but not fatal |
| 19 | Fitzgerald et al. 2021, *Aging* | PMID 33844651 | ✓ | ✓ | OK |
| 20 | Kabacik et al. 2022, *Aging Cell* | PMID 37034474 | ✓ | ✓ | OK |
| 21 | Wang et al. 2022, *Aging Cell* | PMID 36336680 | ✓ | ✓ | OK |
| 22 | **Arif et al. 2025, *Cell Stem Cell*** | **PMID 41289991** | **✗ (does not exist)** | – | **FABRICATED – REJECT** |
| 23 | Hu et al. 2022, *Aging Cell* | PMID 35032339 | ✓ | ✓ | OK |

## Top 5 critical fixes (if not REJECT – for documentation only)

> Because the verdict is REJECT due to a fabricated PMID, a revision path would require:

1. **`CONCEPT.md` (References, line 2):** Remove or replace PMID 41289991 with a real, verifiable source. Justify the claim “Reversing lysosomal dysfunction restores youthful state in aged HSCs” with an actual publication or remove the claim.
2. **`CONCEPT.md` (Falsifiability section):** The struck‑through MCOA Test 2 should be completely removed, not just struck‑through, to avoid confusion.
3. **`OPEN_PROBLEMS.md`:** Replace all “OSF ID pending” with a concrete placeholder (e.g., `https://osf.io/abcde`) to satisfy pre‑registration exactness.
4. **`PARAMETERS.md`:** Clarify that all `γ₄ⱼ` default = 0 is the null hypothesis for all coupling experiments; currently it is stated as “default = 0” only for some.
5. **`EVIDENCE.md`:** The line “Loss of KDM4B… demonstrates link between metabolism and epigenetics” – PMID 33571444 does indeed show a metabolic‑epigenetic link, but the statement is a stretch. Either rephrase or add a more direct reference.

## PACKET
# EpigeneticDrift

**Verdict: REJECT**  
**Primary reason:** Fabricated PMID 41289991 / Reference Integrity = 1  
**Fails conditions 7 and 8.**  
No further iterations required; this component cannot be funded as‑is or with minor revision.