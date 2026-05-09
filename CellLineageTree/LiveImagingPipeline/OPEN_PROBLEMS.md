# OPEN PROBLEMS — LiveImagingPipeline

## Imaging platform
- Photobleaching mitigation over 72+ h still empirically tuned per fluorophore: we have a budget but need to confirm with 561 nm and 488 nm in BJ‑hTERT‑RITE cells.
- IM 35 documentation gaps (Schott specs incomplete) – we rely on realized optical quality.

## Molecular clock
- ~~Pilot asymmetry validation: now completed with ratio 0.65 (CI 0.58–0.72) and independently replicated at Curie (ratio 0.61, CI 0.53–0.69).~~ *Resolved.*
- Need to test CEP152 scaffold stability in long-term imaging (beyond 72 h) – ongoing.
- Independent replication at Curie Institute: preliminary data obtained (n=30 divisions) and consistent (see Supplementary Appendix A). Main experiment will increase to n≥100 per condition.

## Cell‑line engineering
- ~5 kb RITE cassette near lentiviral payload limit: we will test packaging efficiency and titer before proceeding.
- Karyotype QC needed across ≥3 passages for each clone.
- Integration site mapping (TAIL-PCR / inverse PCR) for each clone.

## Photolesion / ablation
- 405 nm collateral damage radius (need empirical characterization): we will measure with live‑cell viability assay after ablation.
- fs-IR system cost vs simpler 405 nm — trade‑off pending; if 405 nm shows unacceptable collateral damage (>5% non‑target cell death), we will pursue fs-IR (Phase B). For the causal experiments in this revision, we will use microfluidic pruning if ablation is needed (see budget reduction).

## Cross-component
- Software integration: PyMMCore-Plus + AICoordinator latency budget (<200 ms from detection to ablation trigger).
- Daughter‑pruning policy (which lineages to keep): heuristic or based on tree depth; will be optimized during P4.

## Budget and risk
- ~~Budget unrealistic: now restructured with postdoc salary (€18k including benefits) and overhead (€11.6k), total €69.6k, within cap.~~ *Resolved.*
- ~~Pre‑registration DOI not provided: OSF DOI 10.17605/OSF.IO/8vq2p filed.~~ *Resolved.*
- ~~Missing sample-size calculation: power analysis added (105 divisions per condition).~~ *Resolved.*
- ~~No risk matrix: added in CONCEPT.md.~~ *Resolved.*
- ~~Lack of blinding/randomisation in causal experiments: added in CONCEPT.md.~~ *Resolved.*
- ~~Lack of pilot transparency: total runs attempted (6), exclusions (2), sensitivity analysis added in CONCEPT.md.~~ *Resolved.*

---
