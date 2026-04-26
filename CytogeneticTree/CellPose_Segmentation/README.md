# CellPose_Segmentation

AI-based live-cell segmentation pipeline for the **CytogeneticTree** project. Built on CellPose (Stringer et al. 2021) and extended with spotiflow-style sub-pixel spot detection for centriolar foci. Outputs per-frame instance masks of cells + centriole (red/green) positions, feeding the lineage tracker downstream.

## Quick facts

- **Backbone:** CellPose 3.0 (generalist cyto3 model + fine-tuned weights for BJ-hTERT)
- **Input:** 2D fluorescence time-lapse (up to 3 channels) at 100× oil
- **Output:** HDF5 with per-frame cell masks + centriole centroid tables
- **Hardware target:** single RTX 4070 / 4080 or equivalent
- **Throughput target:** ≥ 2 fps inference on 1024 × 1024 frames

## Status

Phase A — design + benchmark. See `TODO.md`.

## Dependencies

- Upstream: `LiveCellMicroscopy`, `FluorescentCameras`, `RITE_Centriole`
- Downstream: `ImageAnalysis`, `GenealogyReconstruction`, `AICoordinator`

## License

MIT (code); CC-BY 4.0 (fine-tuned weights + training dataset on Zenodo).
