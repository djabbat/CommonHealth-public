# LaserAblation_405

Targeted single-cell (and single-organelle) laser ablation module for the **CytogeneticTree** project. A 405 nm diode provides reliable nuclear killing; an optional fs-IR path (800 nm) enables sub-micron organelle-scale cuts. Control integrates with the microscope's PyMMCore-Plus stack and the AI coordinator to prune unwanted daughter lineages and keep the experimental tree manageable.

## Quick facts

- **Primary laser:** 405 nm, ≥ 100 mW diode, pulsed or CW
- **Secondary (optional):** fs-IR 800 nm, < 200 fs pulses, MHz rep rate
- **Steering:** 2-axis galvo + shared 100× / 1.4 NA oil objective
- **Targeting:** ROI masks from CellPose + policy from AICoordinator
- **Latency goal:** < 200 ms from decision → ablation complete

## Status

Phase A — hardware sourcing + integration.

## Dependencies

- Upstream: `CellPose_Segmentation`, `AICoordinator`
- Integrated with: `LiveCellMicroscopy`, `MicroscopeController`
- Logged by: `GenealogyReconstruction`

## License

MIT (control code); CC-BY 4.0 (protocols).
