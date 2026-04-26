# CytogeneticTree

**Cytogenetic Tree of Differentiation** — a LongevityCommon subproject to reconstruct the complete genealogical tree of cellular differentiation, from zygote to terminally-differentiated cells, by tracking centriole age across every asymmetric division.

## Why

The centriole is the one organelle that is (a) structurally heritable, (b) asymmetrically inherited at stem-cell divisions, and (c) long-lived with monotonically accumulating damage marks (polyglutamylation). These properties make centriole age an **ideal physical marker of cellular provenance** in a lineage.

## How

1. Tag centrioles with RITE pulse-chase fluorescent system (red before pulse, green after)
2. Observe every division with AI-operated live-cell microscopy
3. Classify each daughter by which centrioles it inherited (red-only / green-only / mixed)
4. Reconstruct the division-event DAG as a lineage tree
5. Annotate tree nodes with differentiation markers, polyGlu signal, functional assays

## Phases

- **Phase 0 (2026-04):** CONCEPT scaffolding, technology subproject stubs, literature landscape
- **Phase 1 (2026 Q3, contingent on Impetus Go):** Full 6-month BJ-hTERT lineage tree — MVCT
- **Phase 2 (2027):** Mouse HSC Cytogenetic Tree (serial transplantation)
- **Phase 3 (2028-29):** Vertebrate embryo (zygote → differentiated somatic)

## Repo structure

```
CytogeneticTree/
├── CONCEPT.md         (the vision + scientific framing)
├── README.md          (this file)
├── CLAUDE.md          (Claude-specific rules for this subproject)
├── TODO.md            (prioritized action list)
├── PARAMETERS.md      (key technical / budget / timing values)
├── MAP.md             (dependency graph + integrations)
├── MEMORY.md          (Claude session memory; dated entries)
├── LINKS.md           (external URLs, Addgene plasmids, etc.)
├── KNOWLEDGE.md       (verified PubMed citations + technology ecosystem)
├── UPGRADE.md         (future extensions beyond MVP)
│
└── [technology sub-subprojects]
    ├── RITE_Centriole/
    ├── CellPose_Segmentation/
    ├── LaserAblation_405/
    ├── LiveCellMicroscopy/
    ├── FluorescentCameras/
    ├── MicroscopeController/
    ├── AICoordinator/
    ├── LentiviralTools/
    ├── ImageAnalysis/
    ├── StatisticalAnalysis/
    ├── GenealogyReconstruction/
    └── DifferentiationAnnotation/
```

## Contact

- **PI:** Dr. Jaba Tkemaladze, MD — jaba@longevity.ge, ORCID 0000-0002-3826-7982
- **Host:** Georgia Longevity Alliance / კავშირი დღეგრძელობა (NGO #404506520, founded 2016-01-12)
- **Funding context:** Longevity Impetus Grants LOI 2026-04-25 (Phase A MVCT demonstration)
- **Parent:** `~/Desktop/LongevityCommon/`
