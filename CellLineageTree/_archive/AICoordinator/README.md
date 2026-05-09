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

## License

MIT (orchestration code); CC-BY 4.0 (PROMPT.md + policies).
