# DESIGN — LiveImagingPipeline

## Architecture

Integrated control plane:
- **PyMMCore-Plus** drives microscope, stage, lasers, cameras
- **AICoordinator** receives per-frame ROI from CellPose, decides ablation targets
- **GenealogyReconstruction** ingests image streams + ablation log → tree

## Build phases

| Phase | Tasks | ETA |
|---|---|---|
| P1 — molecular | RITE cassette design, lentivirus packaging, clonal isolation | ~3 mo |
| P2 — imaging | IM 35 retrofit, environmental chamber, validation runs | ~2 mo (parallel) |
| P3 — intervention | NAC treatment, forced asymmetry construct generation | ~2 mo (overlaps P2) |
| P4 — integration | end-to-end validation: 72-h run with interventions, n≥130 divisions | ~2 mo |

## Replication strategy

- Internal: split-sample (different starting clones) for technical reproducibility
- External: Curie / Janke lab as independent replication site (preliminary n=30; main experiment will aim for n≥100 per condition). Signed letter of support in Supplementary Appendix A. All raw data deposited on Zenodo and analysis code on GitHub.

## Go/no-go decision points (see also CONCEPT.md)

1. **After P1 (molecular):** tag-swap efficiency ≥70% by flow cytometry; otherwise redesign cassette with alternative scaffold or stronger promoter.
2. **After P2 (imaging):** phototoxicity within 90% of control division rate; otherwise reduce laser power or increase frame interval.
3. **After pilot 48‑h run:** observed asymmetric inheritance ratio ≥0.6; **completed – ratio 0.65, criteria met, independently replicated at Curie (0.61)**.
4. **After P4 (integration):** >70% of tracked cells complete ≥8 divisions with resolvable RITE signal; otherwise revise illumination or accept lower depth.

---
