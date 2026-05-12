# EVIDENCE — Aubrey (ARGUS-LP)

**Revised 2026-05-12** — full rewrite. Earlier version contained 14+ fabricated PMID citations, fabricated independent-replication claim from the Janke lab, fabricated $150K private-foundation funding statement, and fabricated pilot-data numbers (Hsp104-GFP yeast SPB content conflated with mammalian BJ-hTERT). All such content has been removed. This file now describes only **evidence the project relies on**, with every PMID verified against PubMed esummary on 2026-05-12.

## Reference verification protocol

Every PMID in this file has been individually checked via the NCBI esummary API on 2026-05-12. If a citation becomes stale or a reviewer requests an additional citation, it is to be re-checked against PubMed before insertion.

## 1. Imaging platform (ARGUS-LP physical layer)

The ARGUS-LP retrofit builds on the PI's existing Zeiss IM 35 inverted microscope at the GLA Abastumani host facility. Resolution at ~300 nm centriolar foci with 100×/1.4 NA oil immersion is standard in the centrosome live-imaging field. Specific supporting citations on resolution benchmarks, environmental-chamber stability over multi-day runs, and dual-channel sequential acquisition will be added to the Methods section after PubMed verification in the funded-phase Methods document. Two general-purpose live-imaging platform references that are PubMed-verified and relevant to the automation layer (§3) are retained here:

- **Pitrone PG et al. 2013, *Nat Methods*** — OpenSPIM: an open-access light-sheet microscopy platform. PMID 23749304. *(PubMed-verified 2026-05-12. Architectural precedent for an open-hardware automated long-term imaging platform.)*
- **Almada P et al. 2019, *Nat Commun*** — Automating multimodal microscopy with NanoJ-Fluidics. PMID 30874553. *(PubMed-verified 2026-05-12. Precedent for automated reagent and channel switching during long-term live-cell acquisition.)*

The 24/7 AI-agent loop (§3) is what makes long-term operation feasible at single-PI scale; it is a novel function of the ARGUS-LP platform and is not derived from any prior publication.

## 2. Molecular clock — RITE system

The RITE (Recombination-Induced Tag Exchange) method tracks protein age by Cre/loxP-controlled fluorophore swap. We adapt it to mammalian centriolar scaffolds (Centrin-1, SAS-6, CEP152). The original method:

- **Verzijlbergen KF et al. 2010, *Proc Natl Acad Sci U S A*** — Recombination-induced tag exchange to track old and new proteins. PMID 20018668. *(PubMed-verified 2026-05-12.)*

The original RITE paper is silent on centriolar scaffold turnover rates in mammalian cells. Empirical measurement of Centrin-1 / SAS-6 / CEP152 half-life in BJ-hTERT under the ARGUS-LP imaging protocol is the first deliverable of the funded phase; without that measurement we cannot commit a primary scaffold a priori. Three parallel scaffolds will be labelled and the slowest-turnover one chosen for the primary age readout.

## 3. Asymmetric centriole inheritance in stem cells — supporting evidence

The motivating observation is that asymmetric centriole inheritance contributes to stem-cell maintenance in some metazoan systems, and that loss of this asymmetry may track with stem-cell exhaustion in aging. PubMed-verified anchors:

- **Yamashita YM et al. 2007, *Science*** — Asymmetric inheritance of mother versus daughter centrosome in stem cell division. PMID 17255513. *(PubMed-verified 2026-05-12. The original Drosophila germline finding.)*
- **Nigg EA. 2018, *Nat Rev Mol Cell Biol*** — Once and only once: mechanisms of centriole duplication and their deregulation in disease. PMID 29363672. *(PubMed-verified 2026-05-12. This review covers the centriole-duplication machinery whose error rate ARGUS-LP aims to quantify; it does **not** establish stem-cell exhaustion claims and is not cited here as such.)*

The PI's own conceptual parent paper:

- **Tkemaladze J. 2023, *Mol Biol Rep*** — "Reduction, proliferation, and differentiation defects of stem cells over time: a consequence of selective accumulation of old centrioles in the stem cells?" PMID 36583780. *(PubMed-verified 2026-05-12. Theoretical foundation of CDATA Counter #1.)*

Additional self-citations are available from the PI's ~10 PubMed-indexed papers (see `~/.claude/projects/-home-oem/memory/publications.md`); they will be added after PMID re-verification in the funded-phase manuscript:

- Chichinadze K & Tkemaladze J. 2008, *Adv Gerontol* — centrosomal hypothesis of aging.
- Tkemaladze J & Chichinadze K. 2005, *Biochem (Moscow)* — centriolar differentiation mechanisms.
- Tkemaladze J. 2024, *Georgian Scientists* — centriole asymmetry review.

Self-citation budget across this evidence document: ≤ 15 % of total references (per `~/.claude/projects/-home-oem/memory/feedback_article_workflow.md`).

## 4. Laser ablation of daughter cells in Phase A (active component)

The Phase A protocol includes operator-approved 405 nm laser microablation of the daughter cell after each detected asymmetric division (per signed Parrish LoS 2026-04-22 and Geiger LoS 2026-04-23). This active lineage-purification component is supported by two PubMed-verified centriole-relevant ablation references:

- **Khodjakov A et al. 2001, *J Cell Biol*** — Centrosomes enhance the fidelity of cytokinesis in vertebrate cells. PMID 11285289. *(PubMed-verified 2026-05-12. Established the use of laser microsurgery on centrosomes in vertebrate live cells.)*
- **Colombelli J et al. 2005, *Traffic*** — In vivo selective cytoskeleton dynamics quantification. PMID 16262721. *(PubMed-verified 2026-05-12. Sub-cellular UV laser dose-response framework.)*

Additional dose-response and phototoxicity-threshold references will be added in the Methods after PubMed verification.

## 5. Cell-line engineering — lentiviral delivery (PubMed-verified)

- **Naldini L et al. 1996, *Science*** — In vivo gene delivery and stable transduction of nondividing cells. PMID 8602510. *(PubMed-verified 2026-05-12. The HIV-based lentivirus paradigm used for high-titre transduction of BJ-hTERT.)*
- **Dull T et al. 1998, *J Virol*** — A third-generation lentivirus vector with a conditional packaging system. PMID 9765382. *(PubMed-verified 2026-05-12. The 3-gen packaging system on which ARGUS-LP's RITE cassette delivery is built.)*

Clonal selection via FACS and karyotyping (G-banding) of BJ-hTERT-RITE clones follows community standards. Plasmids and protocols will be deposited on Addgene and Protocols.io on funding start to enable independent replication.

## 6. Phase B in vivo HSC support (forward-look only — not Phase A evidence)

Phase B leadership at the Geiger lab (Ulm) is supported by Geiger's signed LoS dated 2026-04-23 (€100,000 subcontract). The scientific framing in that LoS notes that Phase B extends Florian et al. 2018 *Cell Stem Cell* (Cdc42-driven polarity loss) and that the centriolar polyGlu hypothesis has not been tested previously in the transplantation paradigm. A separate Phase B EVIDENCE document will be assembled on Phase A Go.

## 7. What this document does NOT claim

- **No pilot data exist** for ARGUS-LP, BJ-hTERT-RITE asymmetry ratios, or daughter-cell ablation success rates as of 2026-05-12. Earlier versions of this file fabricated specific numbers (asymmetry 0.65 over 112 divisions, "independent replication" at Curie/Janke lab, $150K private foundation funding); all such claims have been removed. The funded phase will produce these data and they will be reported with raw lineage tracks on Zenodo (CC-BY 4.0).
- **No fabricated PMIDs**: every PMID number above has been individually verified against PubMed on 2026-05-12. If any reviewer or co-author cites a PMID that does not survive the same check, that citation must be removed before submission.
- **No off-platform claims**: the ARGUS-LP retrofit is a single Zeiss IM 35 microscope at GLA Abastumani. There is no parallel facility, no Curie replication site, no Cambridge / EMBL / TSU bench operation. The only off-site work is Phase B in vivo BMT at the Geiger lab in Ulm, conditional on Phase A end-of-month-6 Go.

## 8. Reference list (PubMed-verified 2026-05-12)

1. Pitrone PG et al. 2013, *Nat Methods*, PMID 23749304 — OpenSPIM.
2. Almada P et al. 2019, *Nat Commun*, PMID 30874553 — NanoJ-Fluidics.
3. Verzijlbergen KF et al. 2010, *PNAS*, PMID 20018668 — RITE original.
4. Yamashita YM et al. 2007, *Science*, PMID 17255513 — *Drosophila* asymmetric centrosome.
5. Nigg EA. 2018, *Nat Rev Mol Cell Biol*, PMID 29363672 — centriole duplication review.
6. Tkemaladze J. 2023, *Mol Biol Rep*, PMID 36583780 — CDATA Counter #1 parent (self-citation).
7. Khodjakov A et al. 2001, *J Cell Biol*, PMID 11285289 — centrosome laser microsurgery in vertebrates.
8. Colombelli J et al. 2005, *Traffic*, PMID 16262721 — sub-cellular UV laser dose-response.
9. Naldini L et al. 1996, *Science*, PMID 8602510 — lentivirus paradigm.
10. Dull T et al. 1998, *J Virol*, PMID 9765382 — third-generation lentivirus packaging.

Self-citation ratio: 1 / 10 = 10 % (compliant with ≤ 15 % cap).
