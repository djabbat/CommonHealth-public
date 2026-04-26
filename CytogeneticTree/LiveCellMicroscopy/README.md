# LiveCellMicroscopy

Retrofitted Zeiss IM 35 inverted microscope — physical imaging platform for the **CytogeneticTree** project. 100× / 1.4 NA oil objective, Piezo Z, environmental chamber, dual-laser (488 + 561 nm) fluorescence path, dual-camera simultaneous acquisition. Designed to run ≥ 72 h live-cell lineage experiments on BJ-hTERT-RITE cells with sub-diffraction centriole resolution.

## Quick facts

- **Body:** Zeiss IM 35 (used, retrofitted)
- **Objective:** 100× / 1.4 NA oil (Plan-Apochromat)
- **Z-axis:** Piezo (PI, Physik Instrumente P-721 or equivalent)
- **Lasers:** 488 nm 100 mW + 561 nm 100 mW solid state
- **Cameras:** 2 × scientific CMOS (see `FluorescentCameras`)
- **Chamber:** 37 °C / 5 % CO₂ / > 95 % RH (Okolab or custom)
- **Run duration:** up to 72 h

## Status

Phase A — sourcing + rebuild.

## Dependencies

- Shares control with: `MicroscopeController`, `LaserAblation_405`
- Sensors: `FluorescentCameras`
- Consumed by: `CellPose_Segmentation`, `ImageAnalysis`

## License

CC-BY 4.0 (protocols, BOM).
