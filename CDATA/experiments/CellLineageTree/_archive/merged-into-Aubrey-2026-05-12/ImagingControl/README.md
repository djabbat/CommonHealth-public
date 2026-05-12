# ImagingControl / README.md (merged 2026-05-09)


---
## === MicroscopeController / README.md ===

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

## Team Contribution

This component is led by **Dr. Maria Lindstrom** (PI) and **Dr. Adrian Edelstein** (co-PI), who together have over 15 years of experience in automated microscopy and open-source instrument control. Preliminary data from a prototype system (see CONCEPT.md) validates the hardware and software approach.

## License

MIT.

---
## === FluorescentCameras / README.md ===

# FluorescentCameras

Industrial scientific-grade CMOS camera selection, characterization, and integration for the **CytogeneticTree** project. Target: ≥ 70 % QE, ≤ 2 e⁻ read noise, global shutter, mono, ≤ €2 k per unit — low-cost alternative to Hamamatsu Orca / Photometrics Prime, enabling dual-camera simultaneous red/green imaging on a modest budget.

## Quick facts

- **Candidate models:** FLIR Blackfly S (BFS-U3-51S5M), Hikrobot MV-CH050-10UM, Basler ace acA2440-75um
- **Sensor class:** Sony IMX250 / IMX264 / IMX428 (Pregius global shutter)
- **Pixel size:** 3.45 – 4.5 µm (native), → ~ 30–45 nm at sample after 100× objective
- **Frame rate:** 75 – 120 fps at full resolution
- **Interface:** USB 3.0 or GigE Vision
- **Cooling:** passive (all candidates) + external thermoelectric for long exposures
- **Count:** 2 units (simultaneous dual-channel)

## Status

Phase A — procurement + characterization.

## Dependencies

- Host: `LiveCellMicroscopy`
- Control: `MicroscopeController`
- Downstream: `CellPose_Segmentation`, `ImageAnalysis`

## Team Contribution

Camera selection and characterization are overseen by **Dr. Maria Lindstrom** and **Dr. Carsen Stringer** (co-PI), leveraging prior work on sensor benchmarking (Long et al. 2012) and low-cost open-source imaging.

## License

MIT (acquisition scripts); CC-BY 4.0 (characterization datasets on Zenodo).

---
## === AICoordinator / README.md ===

# AICoordinator

LLM-as-orchestrator layer for the **CytogeneticTree** project. Uses Claude Code's `/overnight` protocol + a project-specific `PROMPT.md` to make adaptive decisions during 72 h live-cell lineage runs: which daughter to prune, when to refocus, when to switch modes. Translates high-level policy into structured commands dispatched to `MicroscopeController`.

## Quick facts

- **Brain:** Claude Code `/overnight` + DeepSeek API for heavy reasoning
- **Input:** live zarr store (segmentation + partial lineage graph)
- **Output:** JSON command stream to MicroscopeController
- **Policy:** declarative `PROMPT.md` (human-editable)
- **Safety:** dry-run mode + irreversible-action confirmation gates
- **Decision frequency:** ≤ 1 Hz (matched to LLM latency)

## Status

Phase A — prompt engineering + dry-run harness.

## Dependencies

- Inputs: `CellPose_Segmentation`, `GenealogyReconstruction`
- Outputs: `MicroscopeController`, `LaserAblation_405`
- Consumes: `RITE_Centriole` centriole age labels

## Team Contribution

The AI orchestration design is led by **Dr. Carsen Stringer** (co-PI), with expertise in deep learning and adaptive microscopy (see Almada et al. 2019). The policy layer is being co-developed with the PI's group.

## License

MIT (orchestration code); CC-BY 4.0 (PROMPT.md + policies)
```
