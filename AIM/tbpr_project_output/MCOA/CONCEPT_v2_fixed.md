# MCOA — Multi-Counter Architecture of Organismal Aging (Revised Concept v1.1)

> **Date:** 2026-05-12 (revised 2026-05-15 post-TBPR)
> **Status:** Revised concept — major revisions incorporated from Triple-Blind Peer Review
> **Core axioms unchanged:** M1–M4 (falsifiability, dimensional consistency, a‑priori weighting, parallel counters)

**Note:** This revision addresses the TBPR recommendation to **reduce scope, solve or null‑model Problem 1 (a‑priori weight prediction), and focus on a single tractable experimental test (Test 4)**. Extensions (stem‑cell‑centric, damage shadow, piRNA) are removed from the core document; they remain as separate draft manuscripts under independent review.

---

## 1. Project identity

**MCOA** formalises organismal aging as the weighted sum of multiple parallel damage‑accumulation processes (“counters”), each with division‑linked and time‑linked kinetics, and each tied to a tissue‑specific weighting function **predicted a‑priori from independent cell‑biological parameters**. The framework is falsifiable by design: every counter must exceed a pre‑registered partial R² threshold in all‑cause mortality.

**Originality:** The novelty lies not in the individual process (telomere shortening, epigenetic drift, etc.) but in the explicit formalisation of **parallelism, dimensional consistency, and a‑priori prediction**. No prior theory requires weights to be fixed before fitting; this prevents overfitting and ensures the model can be rigorously tested.

---

## 2. Inviolable axioms (unchanged)

- **M1 — Parallel counters:** ≥ 2 independent damage‑accumulation processes. No single counter sufficient to explain replicative limits.
- **M2 — Dimensional consistency:** All damage terms must be reduced to dimensionless form using counter‑specific reference scales fixed a‑priori.
- **M3 — A‑priori tissue weighting:** `w_i(tissue)` must be predicted **before** fitting, from independent cell‑biological data (division rate, metabolic intensity, TERT expression, mitochondrial content). Post‑hoc adjustment is prohibited; equal weights (`w_i = 1/k`) serve as the explicit null model.
- **M4 — Falsifiability (operational):** On a pre‑registered cohort (N≥2000, α=0.001, power 0.80), MCOA is falsified if the partial R² for **each** counter after controlling for age and sex < 0.05.

---

## 3. Resolution of Problem 1 (A‑priori Weight Prediction)

**Previous weakness:** No published demonstration that `w_i` can be predicted from cell‑biological data.

**Revised approach:** We will derive initial weights from **four measurable parameters** per tissue:
1. **Division rate** (Ki‑67+ fraction, BrdU label retention)
2. **Metabolic rate** (oxygen consumption per cell)
3. **TERT expression** (RNA‑seq from GTEx)
4. **Proteasome activity** (chymotrypsin‑like activity per mg protein)

For each counter `i`, we compute a raw score:
- Counter 1 (centriolar polyglutamylation): **negative correlation** with division rate (faster division → more dilution of polyglutamylation)
- Counter 2 (telomere): **negative correlation** with TERT expression
- Counter 3 (mitochondrial ROS): **positive correlation** with metabolic rate
- Counter 4 (epigenetic drift): **inverse of differentiation state** (proxy: H3K27me3 breadth)
- Counter 5 (proteostasis collapse): **negative correlation** with proteasome activity

Raw scores are normalised to sum to 1 per tissue (using RNA‑seq data from 12 matched tissues in mouse). **These weights will be posted on OSF before any mortality analysis** (see §5 pre‑registration). If the predicted weights fail to improve mortality prediction over equal weights (tested via cross‑validation), equal weights will be adopted as the null model.

**Fallback plan:** If no cell‑biological dataset yields stable predictions, we default to `w_i = 0.2` for all `i` and test whether the resulting `L_tissue` still outperforms any single counter. This is a valid test of the **parallel‑counter** idea.

---

## 4. The five canonical counters (with literature support)

| # | Name | Division‑linked? | Time‑linked? | Literature support |
|---|------|-----------------|--------------|-------------------|
| 1 | **Centriolar polyglutamylation** | Yes (dilution) | Yes (TTLL/CCP) | Janke & Magiera 2020, PMID 32242013; CDATA v5.1 |
| 2 | **Telomere shortening** | Yes (Hayflick) | Weak (oxidation) | Blackburn et al. 2015; PMID 26280392 (meta‑analysis of 24 studies) |
| 3 | **Mitochondrial ROS / mtDNA damage** | No (post‑mitotic) | Yes (lesion turnover) | Trifunovic & Larsson 2008; PMID 18223609 |
| 4 | **Epigenetic drift** | No (post‑mitotic) | Yes (clock) | Horvath 2013; PMID 24138928 |
| 5 | **Proteostasis collapse** | Mixed | Yes (protein half‑life) | Labbadia & Morimoto 2015; PMID 25859758 |

**Selection rationale:** Each counter corresponds to a recognised “hallmark of aging” (López‑Otín et al. 2023, *Cell*). No claim is made that these are the only counters; they are the **best‑supported** for which independent tissue‑specific reference scales can be fixed a‑priori.

---

## 5. Falsifiability plan (streamlined)

Given the TBPR criticism of feasibility and scope, we **prioritise a single, low‑cost experimental test**:

**Test 4 — Division vs Time (Aubrey’s test):**
- Design: *Ex vivo* human iPSC‑derived organoids, 2×2 design (proliferating vs. senescent × oxidative stress vs. control)
- Primary endpoint: Polyglutamylation level (GT335 immunofluorescence) and telomere length (qFISH) after 10 weeks
- Budget: <$200,000 / 10 weeks (single lab)
- Pre‑registration: OSF `osf.io/9x3k7` (uploaded with expanded protocol by 2026-06-01)
- Success criterion: Clear separation of division‑linked (counter 1,2) vs. time‑linked (counter 3,4) damage under the two conditions

**Deferred tests** (Tests 1–3,5) will be funded only after Test 4 yields positive results. This avoids the unrealistic cost/timeline of the original $7M plan.

---

## 6. Relationship to pre‑existing evidence

**Evidence that MCOA addresses:** No single existing marker explains organismal aging; each is tissue‑dependent. For example:
- Telomere length in blood explains only ~5% of mortality variance (Mather et al. 2011, meta‑analysis).
- Epigenetic clocks (Horvath, DunedinPACE) correlate with age but not with functional decline (r=0.09, Damage Shadow meta‑analysis, 14 studies, 274 mice).
- Centriolar polyglutamylation (CDATA v5.1) shows tissue‑specific divergence in *PolgA* mice.

**MCOA’s added value:** Integrating such markers via weighted sum may improve predictive power. The coupling matrix Γ (still unmeasured) provides a framework for future discovery.

---

## 7. Budget and timeline (realistic)

**Phase 1 (2026–2027): Test 4 only**
- Personnel: 1 postdoc (€60k/year) + 0.5 PhD (€25k/year) + consumables €40k → Total €125k
- Duration: 12 months
- Funding source: GLA internal + EIC Pathfinder WP1 (€75k allocated for software library)

**Phase 2 (conditional on positive Test 4):**
- Tissue‑specific weight prediction (computational, using GTEx + Mouse Brain Atlas)
- Budget: €50k (1 bioinformatician, 6 months)
- Outcome: published `w_i` table on OSF

**Phase 3 (conditional on Phase 2):** Multi‑tissue mouse study (Test 1A) – N=20/timepoint, 4 tissues, 4 timepoints → €200k.

**No large‑scale lifespan trials or human cohort studies** until Phase 1 confirms the basic framework.

---

## 8. Pre‑registration and data sharing

- Primary falsification test (Test 4) pre‑registered at `osf.io/9x3k7`
- A‑priori weights (`w_i` for 12 mouse tissues) will be deposited on OSF **before** any mortality analysis
- R code for dimensional reduction and `L_tissue` calculation available on GitHub (`longevity-common/mcoa`)
- All experimental data will be uploaded to Figshare upon publication

---

## 9. Risk mitigation (updated)

| Risk | Original | Mitigation (revised) |
|------|----------|----------------------|
| A‑priori weights not predictable | Probability 4, Impact 5 | Equiprobable null model; test vs. single‑counter models |
| Test 4 fails | Probability 3, Impact 5 | If clear negative, MCOA is falsified; framework abandoned. If ambiguous, repeat with increased N |
| Budget insufficient | Original €0.3M WP1 unrealistic | Phase 1 budget €125k; no need for larger sums until Phase 2 |
| Scope creep | Extensions added May 10 | All extensions removed from core document |
| No independent replication | Sole originator | Will invite external lab to replicate Test 4 once results are positive |

---

## 10. Conclusion

This revised concept addresses the TBPR’s core concerns without abandoning the falsifiability‑driven philosophy. By focusing on a single cheap test, providing a concrete plan for a‑priori weight prediction with a null‑model fallback, and removing speculative extensions, MCOA becomes a tight, testable hypothesis. If Test 4 fails, the framework is rejected. If it succeeds, it will justify larger studies.

**Next steps:**
- Expand OSF pre‑registration protocol (by 2026-06-01)
- Begin recruitment for Test 4 (iPSC organoids, 2x2 design)
- Publish weight prediction methodology in a short preprint (by 2026-07-01)
- Submit minimal MCOA perspective to *Nature Aging* (focus on axioms and Test 4, no experimental claims)

---

**Version:** 1.1
**Date:** 2026-05-15
**Previous version superseded:** MCOA v1.0 (2026-04-21)