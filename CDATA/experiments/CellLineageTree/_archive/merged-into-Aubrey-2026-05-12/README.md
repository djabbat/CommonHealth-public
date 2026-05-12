# Merged into LiveImagingPipeline (now Aubrey) — 2026-05-12

These two former sibling subprojects under `CellLineageTree/` were consolidated
into `../../LiveImagingPipeline/` on 2026-05-12 (Tkemaladze + Claude session).
On the same day the consolidated project was renamed **Aubrey** — the canonical
path is now `../../Aubrey/`. The historical archive folder name retains the
original `LiveImagingPipeline` slug for audit-trail continuity.

- `ImagingControl/` — automation/control plane (MicroscopeController, FluorescentCameras, AICoordinator)
- `AnalysisStack/`  — downstream analysis pipeline (CellPose, ImageAnalysis, StatisticalAnalysis, GenealogyReconstruction, DifferentiationAnnotation)

## Reason for archival

All three documents (LiveImagingPipeline + these two) described different
layers of **the same single multi-month centriole age tracking experiment** in
BJ-hTERT cells. Keeping them as separate "projects" duplicated team/budget/PMID
information and generated three contradictory budget totals ($85K, $92.5K,
€68,994). TBPR also scored them as if they were independent grants, which they
are not.

## What was kept

Only the architectural component tables (which components do which role) were
extracted from these documents and integrated into the merged
`LiveImagingPipeline/CONCEPT.md` Pipeline-overview table.

## What was discarded

- Fabricated PI "Dr. Maria Lindstrom (UC Irvine)" in ImagingControl
- Fabricated Co-PIs "Dr. Adrian Edelstein" and "Dr. Carsen Stringer"
- Fabricated PI affiliation "University of Cambridge" for Tkemaladze in AnalysisStack
- Fabricated prior grants ("Marie Curie €150K DeepCellTrack", "Wellcome £50K CentrioleAge")
- Fabricated pilot data (CellPose F1=0.92 on BJ-hTERT, 80% ablation success, 96% labelling agreement, $S. cerevisiae$ Hsp104-GFP runs)
- Wrong species (yeast SPB content was conflated with mammalian centriole work)
- Wrong journal attribution for Tkemaladze 2023 (claimed *Cell Reports* / *Mech Ageing Dev* — actual: *Mol Biol Rep*, PMID 36583780)
- Three contradictory budgets — superseded by the single budget in the merged CONCEPT.md
- ~25 PMID citations that did not survive PubMed esummary verification

See `~/.claude/projects/-home-oem/memory/feedback_tbpr_pmid_hallucination.md`
for the broader pattern.
