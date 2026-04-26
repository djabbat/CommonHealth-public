# RITE_Centriole

Recombination-Induced Tag Exchange (RITE) adapted for centriole age tracking. A Cre-ER^T2 / loxP cassette drives an irreversible mCherry → GFP swap on a centriolar scaffold protein upon tamoxifen pulse. Centrioles assembled before the pulse stay red; those assembled after become green. This creates a direct optical clock for centriole age, enabling reconstruction of the full cytogenetic lineage tree from zygote to differentiated cell — the core aim of the parent **CytogeneticTree** project.

## Quick facts

- **Target proteins (candidates):** Centrin-1, SAS-6, CEP152
- **Induction:** 4-hydroxytamoxifen (4-OHT), 1–4 h pulse
- **Readout:** dual-channel live-cell fluorescence (561 nm + 488 nm)
- **Host cells (Phase A):** HEK293T for validation; BJ-hTERT for lineage experiments
- **Delivery:** lentivirus + clonal selection (see sister subproject `LentiviralTools`)

## Status

Phase A (Design + Validation) — in planning. See `TODO.md`.

## Dependencies

- Upstream: `LentiviralTools`
- Downstream: `LiveCellMicroscopy`, `CellPose_Segmentation`, `ImageAnalysis`, `GenealogyReconstruction`

## License

CC-BY 4.0 (manuscripts) · MIT (code, constructs deposited to Addgene).
