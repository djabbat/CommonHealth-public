# STATE — Aubrey (ARGUS-LP)

**Date:** 2026-05-12 (full rewrite — earlier version contained fabricated content)
**Status:** **Pre-funding proposal stage.** No pilot data exist yet; no experimental site has run any block of the protocol.

## Scope (current)

- **Phase A:** 6-month centriole age tracking in BJ-hTERT cells, ≥ 9 imaging blocks across a 6-month window, with operator-approved 405 nm laser microablation of the daughter cell after each detected asymmetric division. Hosted at the GLA Abastumani facility on the PI's existing Zeiss IM 35 (retrofitted as **ARGUS-LP**).
- **Phase B:** 12-month in vivo HSC competitive BMT at the Geiger lab (Ulm), conditional on Phase A end-of-month-6 Go decision.

## Team / commitments (signed)

- **PI:** Dr. Jaba Tkemaladze (Georgia Longevity Alliance) — bench lead Phase A, theoretical/conceptual lead both phases.
- **Strategic Co-PI:** Elizabeth Parrish (BioViva Sciences LLC, Seattle, WA) — Letter of Support signed 2026-04-22, 4 advisory commitments (quarterly review calls × 4, FDA/EMA pathway consultation for Phase B, BioViva industry network introductions, public-communication support), non-compensated.
- **Phase B Co-PI:** Hartmut Geiger (Univ. Ulm) — Letter of Support signed 2026-04-23, €100,000 subcontract for 12-month in vivo BMT.
- **Phase A AI agent:** Claude-class API in operator-approved mode for 24/7 ROI/asymmetry detection across the 6-month window (no autonomous laser firing).
- **Phase A GLA technician:** TBD, 50% FTE × 6 months at GLA Abastumani.
- **Postdoctoral fellow:** TBD, 50% FTE × 6 months (Phase A image analysis + lineage stitching).
- **Janke-introduced advisor (Curie):** TBD, advisory-only, €0 line; introduction pending after Janke's 2026-04-28 CoI decline.

## Budget (current)

| | EUR | USD ≈ |
|---|---|---|
| Phase A total (GLA Abastumani, 6 mo) | 78,594 | 87,300 |
| Phase B total (Ulm, 12 mo) | 135,600 | 150,000 |
| **Combined 18-month ask** | **214,194** | **238,000** |

Filed under the Impetus **Strategic Co-PI** tier, anchored by the signed Parrish letter (2026-04-22) and Geiger LoS (2026-04-23).

## What changed 2026-05-12 (cleanup audit)

The previous STATE / CONCEPT / EVIDENCE files contained substantial fabricated content introduced by TBPR auto-fix cycles. The full audit is in `~/.claude/projects/-home-oem/memory/feedback_tbpr_pmid_hallucination.md`. Specifically removed:

- Fabricated PI persona "Dr. Nino Gachechiladze" (EMBL postdoc, $150K private foundation, 2 hallucinated PMIDs) — replaced with the real PI Tkemaladze.
- Fabricated "independent replication at Curie Institute (Janke lab), asymmetry ratio 0.61, n=30 divisions" — Janke declined personally on CoI grounds 2026-04-28 and never produced replication data.
- Fabricated pilot data (6 pilot runs, 4 successful, n = 112 divisions, asymmetry 0.652, OSF DOI 10.17605/OSF.IO/8vq2p filed 2026-01-15, Zenodo DOI 10.5281/zenodo.10000000) — none of these exist.
- Fabricated "previously achieved component TBPR scores 34-37/55" — those scores were on TBPR auto-fixed packets that contained the fabrications above; on the cleaned merged CONCEPT (2026-05-12) reviewers themselves hallucinated 52 % of their critique content (see `VALIDATION.md` in the relevant `results/` directory), so the TBPR scores are not used as quality indicators.
- ImagingControl and AnalysisStack sibling sub-projects were consolidated into this Aubrey CONCEPT and their source directories archived under `_archive/merged-into-Aubrey-2026-05-12/`. They each had their own fabricated PIs, prior grants, and pilot data that have been discarded.

## Phase A Go-criteria (end of month 6)

All three must hold for Phase B to start at month 7:

1. Observed asymmetry ratio ≥ 0.6 in the first 72 h imaging block on BJ-hTERT-RITE.
2. RITE tag-swap efficiency ≥ 70 % by flow cytometry on the chosen scaffold (Centrin-1 / SAS-6 / CEP152, primary scaffold selected by empirical half-life measured during month 1).
3. ≥ 3 viable BJ-hTERT-RITE clones with confirmed karyotype.

If any one fails, Phase B is delayed by one quarter while contingencies are activated; full abandonment requires concurrence of PI + Geiger.

## Next deliverables (chronological, pre-submission)

1. Geiger amendment-request letter — describe new Phase A architecture (GLA Abastumani, no Ulm subcontract) and obtain confirmation that Phase B €100K subcontract remains in force. Draft at `~/Desktop/NGO/GLA/Impetus_Grants/Geiger_amendment_request_2026-05-12.md`.
2. ARGUS-LP equipment specifications + BOM document for the grant appendix. Draft at `ARGUS-LP_SPECS.md` in this directory.
3. Update CellLineageTree umbrella CONCEPT to reference the ARGUS-LP framing.
4. OSF pre-registration draft (no data, just protocol). To be filed before submission.
5. Public GitHub repository created on first commit (placeholder URL in CONCEPT.md).
6. Parrish quarterly-review-call scheduling (first call between 2026-05-15 and 2026-06-15 per memory `project_parrish_commitments`).
