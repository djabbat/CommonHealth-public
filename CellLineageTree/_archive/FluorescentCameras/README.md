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

## License

MIT (acquisition scripts); CC-BY 4.0 (characterization datasets on Zenodo).
