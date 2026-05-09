# MicroscopeController

Automation layer for the **CytogeneticTree** microscope. Built on Micro-Manager 2.0 + PyMMCore-Plus, extended with project-specific high-level primitives (`acquire_tile_grid`, `ablate_mask`, `adaptive_refocus`) and YAML-driven experiment descriptions (useq-schema).

## Quick facts

- **Runtime:** Python 3.11, PyMMCore-Plus, useq-schema, napari (optional UI)
- **Config:** µManager hardware .cfg + project YAML
- **Primitives:** stage XY/Z, lasers, dual-camera sync, galvo ablation, environmental logging
- **Adaptive callbacks:** CellPose inference, refocus, ablation dispatch
- **Storage:** OME-NGFF zarr store with event log side-car

## Status

Phase A — scaffolding + integration tests.

## Dependencies

- Hardware: `LiveCellMicroscopy`, `FluorescentCameras`, `LaserAblation_405`
- Software downstream: `CellPose_Segmentation`, `AICoordinator`, `GenealogyReconstruction`

## License

MIT.
