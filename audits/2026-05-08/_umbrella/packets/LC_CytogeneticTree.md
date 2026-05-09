# AUDIT PACKET — LC_CytogeneticTree

Path: `/home/oem/Desktop/LongevityCommon/CytogeneticTree`  Date: 2026-05-08

## Size & file counts
```
372K	/home/oem/Desktop/LongevityCommon/CytogeneticTree
```
**Extensions:** .md=70
## Tree (depth=2, max 200 entries)
```
.
./ImageAnalysis
./ImageAnalysis/PARAMETERS.md
./ImageAnalysis/README.md
./ImageAnalysis/UPGRADE.md
./ImageAnalysis/TODO.md
./ImageAnalysis/CONCEPT.md
./PARAMETERS.md
./CellPose_Segmentation
./CellPose_Segmentation/PARAMETERS.md
./CellPose_Segmentation/README.md
./CellPose_Segmentation/UPGRADE.md
./CellPose_Segmentation/TODO.md
./CellPose_Segmentation/CONCEPT.md
./AICoordinator
./AICoordinator/PARAMETERS.md
./AICoordinator/README.md
./AICoordinator/UPGRADE.md
./AICoordinator/TODO.md
./AICoordinator/CONCEPT.md
./LiveCellMicroscopy
./LiveCellMicroscopy/PARAMETERS.md
./LiveCellMicroscopy/README.md
./LiveCellMicroscopy/UPGRADE.md
./LiveCellMicroscopy/TODO.md
./LiveCellMicroscopy/CONCEPT.md
./GenealogyReconstruction
./GenealogyReconstruction/PARAMETERS.md
./GenealogyReconstruction/README.md
./GenealogyReconstruction/UPGRADE.md
./GenealogyReconstruction/TODO.md
./GenealogyReconstruction/CONCEPT.md
./LentiviralTools
./LentiviralTools/PARAMETERS.md
./LentiviralTools/README.md
./LentiviralTools/UPGRADE.md
./LentiviralTools/TODO.md
./LentiviralTools/CONCEPT.md
./README.md
./StatisticalAnalysis
./StatisticalAnalysis/PARAMETERS.md
./StatisticalAnalysis/README.md
./StatisticalAnalysis/UPGRADE.md
./StatisticalAnalysis/TODO.md
./StatisticalAnalysis/CONCEPT.md
./UPGRADE.md
./RITE_Centriole
./RITE_Centriole/PARAMETERS.md
./RITE_Centriole/README.md
./RITE_Centriole/UPGRADE.md
./RITE_Centriole/TODO.md
./RITE_Centriole/CONCEPT.md
./DifferentiationAnnotation
./DifferentiationAnnotation/PARAMETERS.md
./DifferentiationAnnotation/README.md
./DifferentiationAnnotation/UPGRADE.md
./DifferentiationAnnotation/TODO.md
./DifferentiationAnnotation/CONCEPT.md
./TODO.md
./MEMORY.md
./CLAUDE.md
./LINKS.md
./LaserAblation_405
./LaserAblation_405/PARAMETERS.md
./LaserAblation_405/README.md
./LaserAblation_405/UPGRADE.md
./LaserAblation_405/TODO.md
./LaserAblation_405/CONCEPT.md
./MicroscopeController
./MicroscopeController/PARAMETERS.md
./MicroscopeController/README.md
./MicroscopeController/UPGRADE.md
./MicroscopeController/TODO.md
./MicroscopeController/CONCEPT.md
./CONCEPT.md
./MAP.md
./KNOWLEDGE.md
./FluorescentCameras
./FluorescentCameras/PARAMETERS.md
./FluorescentCameras/README.md
./FluorescentCameras/UPGRADE.md
./FluorescentCameras/TODO.md
./FluorescentCameras/CONCEPT.md
```
## Detected stack: **unknown**
## Core files

### `CLAUDE.md` (2522 chars)
```md
# CLAUDE.md — CytogeneticTree

## Sources of truth

1. `CONCEPT.md` (this subproject)
2. `~/Desktop/LongevityCommon/CONCEPT.md` (umbrella)
3. `~/Desktop/LongevityCommon/CDATA/CONCEPT.md` (parent theory)
4. `~/Desktop/LongevityCommon/MCOA/` (parent theoretical framework)
5. Global `~/CLAUDE.md`

## Rules

- **Literature:** NEVER use DeepSeek for literature search (feedback_deepseek_no_citations). ALL PMIDs must be verified via PubMed esummary API before entering any core file (feedback_verify_references).
- **Core files in sub-subprojects:** each technology subfolder has 5 files (CONCEPT + README + PARAMETERS + TODO + UPGRADE). Umbrella-level core (10 files) lives here at top level.
- **Cross-references:** when editing a tech subproject, check if it affects umbrella MAP.md, PARAMETERS.md, or KNOWLEDGE.md and update accordingly (per `feedback_cdata_docs_sync` pattern).
- **Python code:** for algorithms (genealogy reconstruction, image analysis wrappers), live in tech subfolder with own `pyproject.toml` or similar.
- **Experimental data:** raw image data stays in `AutomatedMicroscopy/data/` (gitignored) — CytogeneticTree tracks derived outputs only.

## Git

- Tracked under `djabbat/LongevityCommon` (public view, core .md gitignored) AND `djabbat/LongevityCommon-private` (full content).
- Large binary outputs (sample images, DAG visualizations >1 MB) → `docs/figures/` with Git LFS if needed, or excluded via .gitignore.

## Naming

- Do not translate "Cytogenetic Tree" into other languages in identifiers. CONCEPT.md preamble + outreach docs can render Russian/Georgian equivalents ("Цитогенетическое дерево дифференцировки" / "ციტოგენეტიკური დიფერენცირების ხე") but folder names stay English.

## What NOT to do

- Don't merge CytogeneticTree into CDATA — they are distinct (CDATA = theoretical mechanism; CytogeneticTree = empirical + computational methodology to test at lineage level).
- Don't abstract technology subfolders prematurely; each needs its own CONCEPT before becoming shared infrastructure.
- Don't start writing a manuscript before Phase 1 data exists.

## Self-citations

Applicable from master list in `~/CLAUDE.md` (≤15% of references). Most relevant for CytogeneticTree:

- Tkemaladze 2023 *Mol Biol Rep* PMID 36583780 — CDATA foundation
- Tkemaladze 2024 *Georgian Scientists* — centriole asymmetry review
- Chichinadze & Tkemaladze 2008 *Adv Gerontol* — centrosomal hypothesis of aging
- Tkemaladze & Chichinadze 2005 *Biochem (Moscow)* — centriolar differentiation mechanisms

```
### `README.md` (2668 chars)
```md
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

- **PI:** Dr. Jaba Tkemaladze, MD — jaba@longevity.ge, ORCID 0000-0001-8651-7243
- **Host:** Georgia Longevity Alliance / კავშირი დღეგრძელობა (NGO #404506520, founded 2016-01-12)
- **Funding context:** Longevity Impetus Grants LOI 2026-04-25 (Phase A MVCT demonstration)
- **Parent:** `~/Desktop/LongevityCommon/`

```
### `ImageAnalysis/README.md` (1404 chars)
```md
# ImageAnalysis Subproject

**Part of the CytogeneticTree Project**

This repository contains the image analysis pipelines for the CytogeneticTree project, led by Dr. Jaba Tkemaladze. Our goal is to reconstruct a cell's genealogical tree by using the age of its centrioles as a persistent lineage tracer.

## What This Subproject Does
We provide automated, reproducible workflows to quantify key centriole biomarkers from microscopy images:
*   **Centriole Age Signal:** Measures polyglutamylated tubulin (GT335 antibody) intensity at the mother centriole.
*   **Maturity Marker:** Quantifies co-localization with the mother-specific protein Ninein.
*   **Ciliation Status:** Detects primary cilia using ARL13B staining.

These quantitative measurements form the foundational dataset for downstream algorithms that reconstruct the Cytogenetic Tree.

## Key Outputs
*   **Per-cell metrics:** GT335 mother/daughter intensity ratio, Ninein localization, ciliation status.
*   **Validated Pipelines:** Ready-to-use scripts for Fiji/ImageJ (live-cell) and CellProfiler (high-throughput fixed images).
*   **Analysis Parameters:** Documented thresholds and settings for reproducible science.

## Getting Started
See `PARAMETERS.md` for technical specifications and `TODO.md` for current development milestones.

---
*For the broader project context, see the [CytogeneticTree Concept Overview](../CONCEPT.md).*

```
### `CellPose_Segmentation/README.md` (1023 chars)
```md
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

```
### `AICoordinator/README.md` (1087 chars)
```md
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

```
### `CONCEPT.md` (9891 chars)
```md
# CytogeneticTree — Cytogenetic Tree of Differentiation

**Version:** v1.0 (initial scaffolding)
**Date:** 2026-04-21
**Status:** 🟡 Active (new LongevityCommon subproject)
**Parent umbrella:** `~/Desktop/LongevityCommon/` (coordinator of CDATA, FCLC, Ze, BioSense, MCOA, HAP, Ontogenesis, AutomatedMicroscopy, and now CytogeneticTree)

---

## §1 Central Vision

**Reconstruct the complete genealogical tree of cellular differentiation — from zygote to terminally-differentiated cells — by tracking centriole age across every cell division.**

The centriole is the one cellular organelle that is **structurally heritable, asymmetric, and long-lived**:

- **Heritable:** each daughter cell inherits centrioles from the mother; never synthesized *de novo* in somatic cells (except rare *de novo* biogenesis in early embryos)
- **Asymmetric:** mother and daughter centriole differ morphologically (sub-distal appendages, distal appendages). In many stem-cell divisions the OLDER mother centriole goes to the self-renewing daughter (Yamashita 2007, Wang 2009, Royall 2023)
- **Long-lived:** centriole proteins are not continuously exchanged; polyglutamylation and other post-translational marks accumulate monotonically across divisions

These three properties make the centriole the **ideal physical marker of cell age in a lineage**. If we can mark centriole age (e.g., via RITE pulse-chase fluorescent tagging) and follow every asymmetric division in a population, we can reconstruct the **complete genealogical DAG** of cellular differentiation.

Such a tree would reveal:

1. **Which lineage branches accumulate centriole damage** (aging trajectories)
2. **Which asymmetric divisions conserve youth** vs squander it (regenerative vs senescent fates)
3. **Where differentiation commitment happens** in terms of centriole inheritance
4. **The full DAG from zygote to terminally-differentiated somatic cell**
5. **Empirical validation of CDATA** — do "old-centriole lineages" proliferate less, as predicted?

---

## §2 Scientific Framing

### 2.1 Position within the LongevityCommon ecosystem

| Subproject | Role vs Cytogenetic Tree |
|------------|---------------------------|
| **MCOA** (parent theoretical framework) | Cytogenetic Tree instantiates MCOA Counter #1 (centriolar) at single-cell resolution across lineage |
| **CDATA** (centriolar damage theory) | Cytogenetic Tree is the **empirical test bed** for CDATA — if tree shows old-centriole-lineages arrest, CDATA validated |
| **AutomatedMicroscopy** | Physical hardware platform; Cytogenetic Tree is the experimental + analytical layer on top |
| **Ze / BioSense** | Orthogonal biomarker work at organism level; Cytogenetic Tree operates at single-cell level |
| **FCLC** (federated learning) | Future: Cytogenetic Tree data could feed FCLC aging biomarker training |

### 2.2 Connection to Impetus LOI 2026-04-25

The Impetus LOI proposes a FOCUSED experiment (AI-directed purification of pure old-centriole fibroblast lineage) that is a **minimum-viable demonstration** of the Cytogenetic Tree concept. The LOI tests **one lineage purification** (red-only inheritance → arrest?) as a binary Go/No-Go; Cytogenetic Tree aims for the **complete lineage DAG** as a multi-year research programme.

- **Impetus Phase A = Minimum Viable Cytogenetic Tree (MVCT):** single purification arm demonstrating the method in BJ-hTERT fibroblasts over 6 months
- **Cytogenetic Tree Phase 1 (post-Impetus-Go):** extend to full tree tracking in BJ-hTERT — not just arm 1 but all daughters, all divisions, no ablation
- **Cytogenetic Tree Phase 2:** mouse HSCs (serial transplantation) + embryonic stem cells (zygote → early blastomere)
- **Cytogenetic Tree Phase 3:** vertebrate embryo tracking (zebrafish or mouse), full zygote-to-differentiated DAG

---

## §3 Methodological Core

### 3.1 The centriole-age tag

**RITE (Recombination-Induced Tag Exchange):**

- Cells express `Centrin1-loxP-mCherry-loxP-GFP` construct
- Before recombination: all centrioles **RED (mCherry)**
- Pulse of tamoxifen + Cre-ERT2 → recombination permanently excises mCherry cassette
- After pulse: new centrioles synthesized from activated locus are **GREEN (GFP)**
- Pre-existing centrioles remain RED (Cre can't recombine protein already synthesized)

This creates a **permanent, heritable color tag** distinguishing centrioles present BEFORE vs AFTER the pulse event.

### 3.2 Division watcher

AI-operated live-cell imaging:

- Zeiss IM 35 retrofit + 100×/1.4 NA oil + piezo Z focus correction + FLIR Blackfly S mono camera
- Every cell observed every 10-30 minutes across 6-month experiment
- CellPose segmentation (Stringer 2021) identifies each cell in each frame
- Algorithm detects division events
- Per daughter cell: classify as Red-only, Green-only, or Mixed based on centriole fluorescence

### 3.3 Lineage recording

Every division event is recorded as a **node** in a directed acyclic graph:

```
Mother cell ─┬─ Daughter 1 (inherited: R1+R2+G0)
             └─ Daughter 2 (inherited: R0+G2+G1)
```

Where `Rn / Gn` denote number of centrioles of each type inherited. Over time, the tree grows.

### 3.4 Differentiation annotation

Each lineage node can be annotated with:

- Morphology (cell shape, size from microscopy)
- Single-cell RNA-seq signature (if destructively sampled at endpoint)
- Functional markers (immunofluorescence for differentiation markers)
- PolyGlu signal at mother centriole (proxy for CDATA damage counter)

---

## §4 Technology Stack (Subprojects)

The full methodology requires integration of ~12 technologies. Each is a sub-subproject under CytogeneticTree with its own CONCEPT + core files. See MAP.md for the full dependency graph.

| # | Subfolder | Technology |
|---|-----------|------------|
| 1 | `RITE_Centriole/` | Recombination-induced centriole tag; de novo cloning of `Centrin1-loxP-mCherry-loxP-GFP` |
| 2 | `CellPose_Segmentation/` | AI live-cell segmentation (Stringer 2021 Nat Methods) |
| 3 | `LaserAblation_405/` | 405 nm / fs-IR microablation for lineage purification |
| 4 | `LiveCellMicroscopy/` | Zeiss IM 35 retrofit (100×/1.4 NA oil, piezo Z, env chamber) |
| 5 | `FluorescentCameras/` | FLIR Blackfly S / Hikrobot mono scientific CMOS, global shutter |
| 6 | `MicroscopeController/` | Micro-Manager 2.0 + PyMMCore-Plus automation |
| 7 | `AICoordinator/` | Claude Code `/overnight` agent orchestration + PROMPT.md |
| 8 | `LentiviralTools/` | Plasmid design + HEK293T packaging + BJ-hTERT transduction |
| 9 | `ImageAnalysis/` | Fiji / ImageJ / CellProfiler for polyGlu, Ninein, ARL13B |
| 10 | `StatisticalAnalysis/` | log-rank + Kaplan-Meier + Bayesian MCMC + Sobol |
| 11 | `GenealogyReconstruction/` | DAG algorithm from division-event log → lineage tree |
| 12 | `DifferentiationAnnotation/` | Lineage node → cell-state mapping (scRNA-seq + morph) |

---

## §5 Key Scientific Hypotheses (will be tested via the Tree)

### H1 — Pure old-centriole lineage arrests

Predicted by CDATA; tested in Impetus Phase A; first proof point for the Tree.

### H2 — Differentiation commitment correlates with centriole asymmetry

Stem vs non-stem daughter distinguishable by which centriole inherited (older → self-renewing?).

### H3 — Terminal differentiation = centriole accumulated damage threshold

Terminally-differentiated cells have oldest centrioles; threshold triggers cell-cycle exit.

### H4 — The Tree has a canonical topology

Across replicate embryos / tissue systems, the cytogenetic tree has conserved structure → differentiation is deterministic at centriolar level.

---

## §6 Current Status (2026-04-21)

- **Umbrella CONCEPT + core files** — generated 2026-04-21 (this document)
- **Technology subfolders** — being scaffolded in parallel via agent
- **Literature landscape + technology ecosystem** — KNOWLEDGE.md being populated via PubMed search agent
- **Impetus LOI 2026-04-25** — scheduled submission, minimum-viable demonstration of Tree concept
- **Connection to Ilia Zheleznov HSC simulator** — planned integration; his simulator could run predicted lineage trajectories to pre-validate expected Tree topology before wet-lab experiment

---

## §7 Outputs (Future)

### Near-term (post-Impetus, conditional)

- Full 6-month time-lapse of BJ-hTERT lineage tree with RITE tags
- Public dataset release (raw images + division event log + DAG topology)
- Companion paper — "Cytogenetic Tree of Fibroblast Differentiation: A Lineage-Resolved Test of the Centriolar Counter Hypothesis"

### Long-term

- Mouse HSC Cytogenetic Tree (serial transplantation)
- Embryonic Cytogenetic Tree (zygote → differentiated somatic; vertebrate model)
- Cross-species Cytogenetic Tree comparison (conserved topology?)

---

## §8 Integrations Required

- **CDATA** — theoretical model must be updated to include lineage-level predictions
- **MCOA** — Counter #1 (centriolar) formalism needs per-lineage version
- **Impetus grant** — Phase A provides the first MVCT data
- **Ilia Zheleznov HSC simulator** — parallel simulation of expected Tree topology
- **AIM** — not directly (this is research, not clinical)
- **DrJaba** — not directly (clinic, not research)

---

## §9 Open Questions (2026-04-21)

1. Does RITE-Centriolin construct exist? (As of Impetus LOI audit: **NO** — has to be de novo synthesized)
2. Can Cre-ERT2 achieve high enough recombination efficiency for clean pulse-chase? (Target: >90% in 48h)
3. Is 405 nm laser ablation centriole-specific enough? (Key phototoxicity control needed — Month 2 calibration)
4. Can AI segmentation reliably classify mixed-centriole daughters? (Edge case: 1R+1G)
5. How to handle divisions where we can't see both centrioles clearly? (Focus drift, out-of-plane events)

---

*Generated 2026-04-21. Part of LongevityCommon ecosystem. Connected to MCOA (theoretical), CDATA (mechanistic), AutomatedMicroscopy (hardware), Impetus LOI (funding).*

```
### `ImageAnalysis/CONCEPT.md` (5206 chars)
```md
# ImageAnalysis: Conceptual Foundation

## §1 Purpose: Contribution to the Cytogenetic Tree
The **ImageAnalysis** submodule provides the quantitative phenotypic data essential for constructing the Cytogenetic Tree. Its core purpose is to algorithmically extract biomarkers of centriole age and function from microscopic images. By precisely measuring the polyglutamylation signal (GT335) at the mother centriole, co-localization with the mature mother marker Ninein, and ciliation frequency (via ARL13B), this pipeline translates visual patterns into numerical datasets. These datasets—generating metrics like "GT335 intensity ratio (Mother/Daughter)" and "ciliation index per cell state"—serve as the empirical inputs for the tree-reconstruction algorithms. Without robust, automated image quantification, the hypothesis that centriole age is a persistent lineage tracer remains untestable.

## §2 Mechanism & Basis
The pipeline operates on a multi-channel immunofluorescence foundation. Fixed-cell samples provide a high-resolution, endpoint snapshot where centriole pairs are stained for:
*   **GT335:** Antibody targeting polyglutamylated tubulin, enriched on the older mother centriole.
*   **Ninein:** A protein selectively retained at the mature mother centriole, serving as a secondary validation marker.
*   **ARL13B:** A marker for the ciliary membrane, identifying cells that have built a primary cilium using the mother centriole as a basal body.

The analysis mechanism involves:
1.  **Cell/ Nuclei Segmentation:** Using DAPI/Hoechst to identify individual cells.
2.  **Centriole Pair Detection:** Identifying puncta in the centrin channel (or similar), then pairing them based on proximity.
3.  **Mother/Daughter Assignment:** For each pair, the centriole with higher GT335 signal intensity is provisionally assigned as the "mother." This assignment is cross-verified by co-localization with the Ninein signal.
4.  **Signal Quantification:** Measuring raw and background-subtracted intensity for GT335 and Ninein at each centriole.
5.  **Ciliation Assessment:** Determining if an ARL13B-positive cilium is associated with the mother centriole of a given cell.

For live-cell streams, the pipeline tracks these parameters over time, correlating centriole age signature with cell fate decisions like division or differentiation.

## §3 State of the Art (Key References)
Current methods for centriole analysis are largely manual or semi-automated, creating a bottleneck for large-scale lineage studies.
*   **Automated Centriole Analysis:** Recent tools like CenFind ([PMID: 36977999]) offer automated centriole detection in 3D microscopy but lack integrated, validated pipelines for the specific polyglutamylation intensity quantification required for age-dating.
*   **Polyglutamylation as an Age Marker:** The foundational work using the GT335 antibody to characterize centriolar polyglutamylation was conducted in vertebrate model systems (Bobinnec et al. 1998 J Cell Biol, [PMID: 9852152]). Systematic quantification across a differentiating lineage remains unperformed.
*   **Ciliation & Cell State:** The link between primary cilium assembly, cell cycle exit, and differentiation is reviewed in Nigg & Holland 2018 ([PMID: 29363672]); its integration with continuous centriole age metrics is novel.

## §4 Integration with CytogeneticTree Technologies
This subproject is a critical data feeder for other components:
*   **`../LineageReconstruction/`:** The quantitative output (centriole age metrics per cell) is the primary input for algorithms that infer genealogical relationships between cells.
*   **`../WetLabProtocols/`:** Directly analyzes images generated by the standardized immunofluorescence and sample preparation protocols defined there. Feedback on image quality (e.g., signal-to-noise ratio) is crucial for protocol optimization.
*   **`../DataModel/`:** Populates the structured schema with quantitative measurements (e.g., `cell.cilia.arl13b_intensity`, `centriole.mother.gt335_ratio`). The analysis metadata (software versions, parameters) is also stored here for reproducibility.

## §5 Gaps & Development Path
**Existing Gaps:**
1.  No turnkey, open-source pipeline exists that combines **centriole pair detection**, **mother/daughter discrimination via GT335/Ninein**, and **ciliation analysis**.
2.  Thresholds for defining a "significant" GT335 intensity difference between mother and daughter centrioles are not standardized and may be cell-type specific.
3.  Analysis of fixed endpoint samples is decoupled from live-cell lineage tracing data.

**What to Build:**
We will build two integrated, containerized pipelines:
*   **Pipeline 1 (Fixed-Sample):** A CellProfiler pipeline for high-throughput analysis of multi-sample, multi-channel endpoint images.
*   **Pipeline 2 (Live-Cell):** A Fiji/ImageJ macro suite for time-series analysis, enabling correlation of centriole aging dynamics with division timing and fate choices.
Both pipelines will produce standardized `.csv` outputs ready for ingestion by the `../DataModel/` and will include comprehensive documentation for parameter adjustment.

---
*Back to [CytogeneticTree Project Overview](../CONCEPT.md)*

```
### `CellPose_Segmentation/CONCEPT.md` (2765 chars)
```md
# CellPose_Segmentation — AI Live-Cell Segmentation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

To reconstruct a cytogenetic tree we must **track every cell and every centriole across thousands of frames and multiple divisions without losing identity**. Manual segmentation is intractable at this scale. CellPose provides a pre-trained, generalist AI segmentation model that operates directly on live-cell fluorescence / brightfield images, producing instance masks per cell and per sub-cellular foci (centrioles). This subproject adapts and fine-tunes CellPose for the RITE-centriole imaging stream that drives the parent project.

## §2 Scientific basis / mechanism

CellPose (Stringer et al. 2021 Nat Methods) uses a U-Net with a topographical flow representation: each pixel is assigned a vector pointing toward the center of its cell, and mask instances are recovered by following these flows to their sinks. The network is trained on a large generalist corpus including phase-contrast, fluorescence, and brightfield data. CellPose 2.0 adds user-in-the-loop fine-tuning; CellPose 3.0 adds image restoration. For centriole foci (1–2 pixel punctae), we pair CellPose cell-level masks with a diffraction-limited spot detector (e.g., Trackpy, spotiflow) gated by the cell mask.

## §3 Current state of the art

- Stringer C et al. 2021 Nat Methods — CellPose original [PMID: 33318659]
- Pachitariu M, Stringer C 2022 Nat Methods — CellPose 2.0 human-in-the-loop [PMID: 36344832]
- Dohmen E et al. 2024 — spotiflow sub-pixel spot localization (bioRxiv, not yet in PubMed)

## §4 Integration with other CytogeneticTree technologies

- **LiveCellMicroscopy** + **FluorescentCameras** — produce the raw image streams
- **RITE_Centriole** — provides the red/green centriole channels to segment
- **MicroscopeController** — can trigger on-the-fly segmentation for adaptive acquisition
- **AICoordinator** — interprets segmentation results to decide laser ablation targets
- **ImageAnalysis** — downstream quantification uses the masks produced here
- **GenealogyReconstruction** — tracks link masks across frames into lineages

## §5 Known gaps + what this subproject builds

**Gaps:**
1. Default CellPose models trained on fixed images; live-cell mitotic shapes are under-represented
2. Centriolar foci (< 500 nm) below standard CellPose training resolution
3. Two-channel (red/green) centriole tracking across mitosis is a specialized task

**Deliverables (Phase A):**
- Fine-tuned CellPose model for BJ-hTERT dividing cells
- Integrated pipeline: CellPose cell masks + spotiflow centriole detection within masks
- Benchmark: ≥ 95 % cell-level F1, ≥ 90 % centriole F1 vs hand-annotated ground truth
- Open dataset + trained weights on Zenodo

```
### `AICoordinator/CONCEPT.md` (2745 chars)
```md
# AICoordinator — Claude-Code Overnight Agent for Adaptive Experimentation

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## §1 Purpose

A 72-hour lineage tracking experiment must make **thousands of small decisions** (which daughter to keep, when to re-focus, when to abort a branch, when to increase laser power, when to switch to mitotic burst mode). Delegating these to a human is impossible; delegating them to pure rule-based code is brittle. The AICoordinator leverages Claude Code's `/overnight` mode + a domain-specific `PROMPT.md` orchestration document to act as a domain-aware autonomous agent, issuing commands to `MicroscopeController` in real time.

## §2 Scientific basis / mechanism

The AICoordinator is not an ML model per se — it is an **LLM-as-orchestrator** that consumes:

- Segmentation output (CellPose masks + centriole positions)
- Lineage state (partial GenealogyReconstruction graph)
- Experiment policy (PROMPT.md: keep tree balanced, prune >5 siblings, prioritize red-centriole daughters, etc.)
- Live event log (errors, timings)

…and emits structured commands (`{"action": "ablate", "target_id": 42, "dose_mW": 10}`) that the controller executes. It can call DeepSeek API for heavy reasoning and Claude's `/overnight` protocol for session persistence and retry logic. Self-correcting: uses post-ablation imaging to verify effect and retries if needed.

## §3 Current state of the art

- Claude Code `/overnight` protocol (Anthropic, internal) + SESSION_STATE.md pattern
- LLM-based lab automation: Coscientist (Boiko et al. 2023 *Nature*) [PMID: 38123806]
- SmartACM / autonomous microscopy — emerging field [REF-PENDING]

## §4 Integration with other CytogeneticTree technologies

- **CellPose_Segmentation** — input stream of masks + spots
- **GenealogyReconstruction** — input: current lineage graph state
- **MicroscopeController** — receives structured commands
- **LaserAblation_405** — dispatch target
- **StatisticalAnalysis** — end-of-run consumer of decision log
- **RITE_Centriole** — decisions keyed to red/green centriole age

## §5 Known gaps + what this subproject builds

**Gaps:**
1. No standard protocol for LLM-driven lab automation at this scale
2. Safety + reversibility requires careful tool-call design (dry-run, confirmation gates)
3. Latency (LLM round-trip ~ seconds) limits decision frequency to ~ 1 per minute

**Deliverables (Phase A):**
- `PROMPT.md` orchestration spec (policies, invariants, safety rules)
- Claude Code skill that reads zarr store + emits JSON commands
- Dry-run harness (policies tested on synthetic data from GenealogyReconstruction simulator)
- Live 72 h co-driven run with human-in-the-loop oversight
- Open-source prompt + agent scaffolding on GitHub

```
### `MAP.md` (3460 chars)
```md
# MAP — CytogeneticTree

## Dependency graph across subfolders

```
              ┌──────────────────────────────────┐
              │   CytogeneticTree (umbrella)     │
              │   ───────────────────────────    │
              │   CONCEPT + 10 core files        │
              └───┬──────────────────────────────┘
                  │
   ┌──────────────┼──────────────────────┐
   ▼              ▼                      ▼
[Biology]     [Hardware]            [Algorithm]
   │              │                      │
   ▼              ▼                      ▼
RITE_Centriole   LiveCellMicroscopy     CellPose_Segmentation
LentiviralTools  FluorescentCameras     LaserAblation_405
                 MicroscopeController   AICoordinator
                                        ImageAnalysis
                                        StatisticalAnalysis
                                        GenealogyReconstruction
                                        DifferentiationAnnotation
```

## Information flow during one experimental run

```
  1. RITE pulse (Cre-ERT2 + tamoxifen) — biology
        │
        ▼
  2. Live-cell imaging (Zeiss IM 35 + 100× + mono camera) — hardware
        │  continuous stream: every 20 min, 4 channels
        ▼
  3. MicroscopeController (Micro-Manager 2.0 + PyMMCore-Plus) — automation
        │  sends frames to AI
        ▼
  4. CellPose_Segmentation — algorithm
        │  outputs: per-cell masks
        ▼
  5. AICoordinator (Claude /overnight) — orchestration
        │  analyzes fluorescence per daughter cell
        │  makes ablation decisions (per experimental arm)
        │
        ├─ → LaserAblation_405 (if arm requires)
        │
        ├─ → division-event log entry
        │
        ▼
  6. GenealogyReconstruction — algorithm
        │  builds DAG from division events
        │
        ▼
  7. ImageAnalysis — algorithm
        │  quantifies polyGlu, Ninein, ARL13B on fixed endpoints
        │
        ▼
  8. StatisticalAnalysis — algorithm
        │  log-rank on survival, Bayesian MCMC on parameters
        │
        ▼
  9. DifferentiationAnnotation — algorithm
        │  maps tree nodes to differentiation states
        │
        ▼
 10. Output: Cytogenetic Tree visualization + dataset + manuscript
```

## Integrations with LongevityCommon ecosystem

```
  CytogeneticTree  ──┬──→  CDATA (theoretical validation data)
                     │
                     ├──→  MCOA (Counter #1 lineage-level instance)
                     │
                     ├──→  AutomatedMicroscopy (shared hardware platform)
                     │
                     ├──→  BioSense / Ze (orthogonal biomarker context)
                     │
                     └──→  Impetus LOI (funding vehicle for Phase 1 MVCT)
```

## External data flow

```
      Addgene / Twist Bio
              │
              │ (RITE construct plasmids)
              ▼
      LentiviralTools/ ─────→ live culture ─────→ Phase 1 data
              ▲                                          │
              │                                          │
         CCP1/TTLL6 constructs                           ▼
         (rescue + pro-damage controls)             Zenodo dataset release
                                                    (DOI + raw images)
                                                         │
                                                         ▼
                                                  Companion paper
```

```
### `PARAMETERS.md` (3267 chars)
```md
# PARAMETERS — CytogeneticTree

## Experimental parameters

| Parameter | Value | Units | Source | Status |
|-----------|-------|-------|--------|--------|
| Pulse duration (tamoxifen for Cre-ERT2) | 24-48 | hours | standard Cre-ERT2 protocol | planned |
| Recombination target efficiency | ≥90 | % switched within 48 h | benchmark from Royall 2023 | target |
| Imaging interval | 20 | minutes | balance time-res vs photo-damage | planned |
| Ablation laser wavelength (primary) | 405 | nm | Cobolt 06-MLD CW | bought |
| Ablation laser wavelength (fallback) | 804 | nm | fs-IR if Month-2 photo-tox calibration fails | contingency |
| Objective | Plan-Apochromat 100×/1.4 NA oil | - | required for 300 nm centriole-pair resolution | bought |
| Camera pixel resolution | 2448×2048 mono | pixels | FLIR BFS-U3-63S4M-C or equivalent | bought |
| CO₂ concentration in chamber | 5.0 ± 0.2 | % | SenseAir S8 0-10% sensor | planned |
| O₂ concentration in chamber | 3.0 ± 0.3 | % | physiological hypoxia (Parrinello 2003) | planned |
| Temperature in chamber | 37.0 ± 0.5 | °C | standard mammalian culture | planned |
| Experiment duration | 6 | months | until arrest or end | planned |
| Target population-doubling range | 50-300 | PD | bounded by arrest detection + cost | planned |
| Biological replicates | ≥3 | clonal founder lines | per arm, per Impetus power calc | required |
| Experimental arms | 6 | (Arm 0/1/2/3/4/4b + Rescue) | per Impetus LOI v25.1 | required |

## Budget line items (from Impetus LOI v25.1 Phase A $92,000)

| Line | Amount | Notes |
|------|--------|-------|
| AutomatedMicroscopy hardware + laser | $14,500 | incl. 100× oil objective + Cobolt 405 nm |
| RITE-Centriolin de-novo cloning | $7,000 | Twist Bio synth + HEK293T packaging |
| CCP1 / PACT-CCP1 / TTLL6 constructs | $9,000 | lentiviral, Addgene |
| Cell culture | $15,000 | BJ-hTERT, media, hypoxia rental, consumables |
| Antibodies + IF (GT335, Ninein, ARL13B, secondaries) | $8,000 | - |
| Technician 50% FTE × 6 mo | $18,000 | daily culture, sample processing |
| Lab space / partnership fee | $10,000 | - |
| General consumables | $7,500 | plastics, cryo, pipettes |
| AI/software subscription | $120 | Claude Code Team, 6 mo |
| NGO admin (5%) | $4,880 | - |

## Computational parameters

| Parameter | Value | Notes |
|-----------|-------|-------|
| Segmentation model | CellPose 3.0 | Stringer 2021 |
| GPU for real-time inference | RTX 3080 Ti or better | local compute for live analysis |
| Storage per month | ~2 TB | 2448×2048 mono @ 20-min intervals × 6 channels × 6 mo |
| Archive storage total | ~12 TB | full experiment raw image + processed data |
| DAG algorithm | custom (GenealogyReconstruction subproject) | Python + NetworkX |

## Statistical parameters

| Parameter | Value | Source |
|-----------|-------|--------|
| α (significance) | 0.05 | standard |
| β (power) | 0.20 | standard |
| Test | log-rank (Kaplan-Meier survival) | primary endpoint |
| MCMC chains | 4 × 2000 iterations | Bayesian calibration |
| Gelman-Rubin target | R̂ < 1.05 | convergence |

## Open: dependent on agent-generated KNOWLEDGE.md

Parameters above are design targets. Once `KNOWLEDGE.md` literature search completes, cross-check against published values and update this file.

```
### `ImageAnalysis/PARAMETERS.md` (2123 chars)
```md
# ImageAnalysis: Technical Parameters & Specifications

## Software & Versions
*   **Fiji/ImageJ:** Version 1.54f or later. Key plugins: Bio-Formats, TrackMate, MorphoLibJ.
*   **CellProfiler:** Version 4.2.1 or later.
*   **Python (for script wrappers):** 3.9+, with libraries: `pandas`, `numpy`, `scikit-image`.
*   **Containerization:** Docker image based on `cellprofiler/cellprofiler:latest` for pipeline stability.

## Key Analysis Parameters (Thresholds & Settings)
*   **Centriole Detection (Puncta):**
    *   Diameter (pixels): 3-6
    *   Typical intensity threshold (method): `Minimum Cross‑Entropy` or `Otsu` per image set.
    *   Maximum pairwise distance for mother-daughter pairing: 1.5 µm.
*   **Mother/Daughter Assignment:**
    *   Primary criterion: Centriole with **≥1.5x** higher GT335 mean intensity (background-subtracted) is provisional mother.
    *   Validation criterion: Provisional mother must have **≥0.8** Pearson's correlation with Ninein channel (or Ninein intensity ≥2x daughter).
*   **Ciliation (ARL13B) Detection:**
    *   Cilium length threshold: ≥0.5 µm.
    *   Maximum distance from mother centriole to cilium base: 0.7 µm.
*   **Cell Segmentation (DAPI):**
    *   Seeded watershed algorithm.
    *   Minimum nucleus area: 35 µm².

## Budget & Resource Lines
*   **Software:** Open-source (FIJI, CellProfiler). $0.
*   **Compute:** Medium-performance workstation (32GB RAM, GPU optional). Approx. ~$2,500 one-time.
*   **Storage:** Network-attached storage for raw images (~2-4 TB expected). Cost depends on institutional infrastructure.
*   **Personnel:** Bioimage analyst (0.5 FTE for pipeline development and validation).

## File Naming Convention (Input)
`[Date]_[SampleID]_[Stain]_[Well]_[Field].tif`
Example: `20240521_HEK293_GT335-Ninein-ARL13B-DAPI_B08_f012.tif`

## Output Data Structure
Primary output is a CSV file with columns:
`Image_Metadata, Cell_ID, CentriolePair_ID, Mother_GT335_MeanIntensity, Daughter_GT335_MeanIntensity, GT335_Ratio, Mother_Ninein_Coloc_Coeff, Is_Ciliated, ARL13B_Length_µm`

---
*For the project rationale, see [CONCEPT.md](CONCEPT.md).*

```
### `CellPose_Segmentation/PARAMETERS.md` (1142 chars)
```md
# CellPose_Segmentation — PARAMETERS

## Model

| Parameter | Value |
|---|---|
| Base model | CellPose 3.0 `cyto3` |
| Fine-tuning mode | Human-in-the-loop (CellPose GUI) + scripted |
| Training set size | ≥ 200 frames × 3 biological replicates |
| Cell diameter prior | ~ 30 px (for 100× / 1.4 NA, 6.5 µm pixel CMOS) |
| Flow threshold | 0.4 |
| Cellprob threshold | 0.0 (permissive) |

## Centriole spot detection

| Parameter | Value |
|---|---|
| Algorithm | Trackpy `locate` / spotiflow |
| Expected spot FWHM | 250–350 nm |
| Min intensity SNR | 4 |
| Per-cell gate | Mask from CellPose |

## Hardware

| Component | Spec |
|---|---|
| GPU | ≥ 8 GB VRAM (RTX 3060 min; 4070/4080 target) |
| RAM | 32 GB |
| Storage | 2 TB NVMe for raw + processed streams |

## Benchmarks (Phase A target)

| Metric | Target |
|---|---|
| Cell F1 (IoU 0.5) | ≥ 0.95 |
| Centriole F1 | ≥ 0.90 |
| Lost-track rate over 24 h | ≤ 5 % |
| Inference | ≥ 2 fps @ 1024² |

## Budget

| Item | EUR |
|---|---|
| Workstation upgrade (GPU + NVMe) | 2,500 |
| Annotation labor (undergrad, 40 h) | 600 |
| Cloud burst (if needed) | 300 |
| **Total** | **~3,400** |

```
### `AICoordinator/PARAMETERS.md` (1643 chars)
```md
# AICoordinator — PARAMETERS

## Orchestration stack

| Component | Purpose |
|---|---|
| Claude Code `/overnight` | Session persistence, retry, systemd-inhibit sleep |
| DeepSeek API (`deepseek-reasoner`) | Heavy reasoning on ambiguous decisions |
| Local `PROMPT.md` | Experiment policy, invariants, safety rules |
| Zarr store reader | Ingest segmentation + graph state |
| JSON command emitter | Dispatch to MicroscopeController |

## Decision loop

| Parameter | Value |
|---|---|
| Loop period | 30–60 s |
| Per-decision token budget | ≤ 2 k tokens (default) |
| Hard safety timeout | 10 s per command |
| Human override latency | < 5 s via dashboard |

## Policy categories (in PROMPT.md)

1. **Tree shape** — keep ≤ 8 active leaves in field of view
2. **Centriole age bias** — prune daughters inheriting the younger (green) centriole when policy demands old-lineage tracking
3. **Focus / drift** — trigger `adaptive_refocus` when drift > 200 nm
4. **Mitotic burst** — switch to 30 s interval when prometaphase detected
5. **Phototoxicity budget** — throttle exposure if division rate drops > 20 %
6. **Abort conditions** — death of > 50 % tracked cells → abort run

## Safety rules

- Every ablation call dry-runs first (logs intent; pauses 1 s; executes unless veto flag set)
- Daily summary auto-posted to user (Slack / Telegram)
- Hard stop on 3 consecutive device errors
- All commands logged immutably (append-only JSON-Lines)

## Budget

| Item | EUR / month |
|---|---|
| DeepSeek API | 10 (typical load) |
| Claude Code subscription | 20 (if Pro) |
| Monitoring (Slack / Grafana cloud) | 0–10 |
| **Total** | **~30 / month** |

```
### `UPGRADE.md` (3538 chars)
```md
# UPGRADE — CytogeneticTree

## v1.0 (2026-04-21, current)

Initial scaffolding:
- Umbrella CONCEPT + 10 core files
- 12 technology sub-subprojects with own 5-file cores each
- Literature landscape (KNOWLEDGE.md) pending agent completion
- Connection to Impetus LOI 2026-04-25 (Phase A MVCT)

## v1.1 (Phase 0 refinement, ~2026-05)

- [ ] Populate KNOWLEDGE.md with verified literature (all PMIDs checked via PubMed API)
- [x] Review 12 sub-subproject CONCEPTs for scientific accuracy; replace `[PMID-PENDING-VERIFY]` placeholders — 2026-04-22 (19 PMIDs verified via PubMed; unresolvable refs marked "pending dedicated verification")
- [ ] MAP.md dependency graph finalized after KNOWLEDGE.md contents known
- [ ] Integration test: connect Ilia Zheleznov HSC simulator (https://github.com/zhelilyan-create/hematopoiesis-simulator) as computational pre-validation tool
- [ ] Add explicit data-release plan (Zenodo DOI + image metadata standard)

## v2.0 (Phase 1 kickoff — contingent on Impetus Go, ~2026-06)

- [ ] RITE-Centriolin construct: synthesize via Twist Bio → clone into pLenti-Cre-ERT2 backbone → package + validate
- [ ] AutomatedMicroscopy platform assembled (per Impetus LOI budget)
- [ ] AI `/overnight` agent PROMPT.md validated in 48-h pilot (confirms ablation accuracy + decision latency)
- [ ] First BJ-hTERT lineage tracked for 2 weeks (pilot demo)
- [ ] Division-event log → DAG proof-of-concept

## v3.0 (Phase 1 complete, ~2026-12)

- [ ] 6-month continuous tracking of 6 parallel arms × 3 clonal replicates = 18 lineages
- [ ] DAG reconstruction across ~50-100 population-doublings per lineage
- [ ] polyGlu quantification at all lineage endpoints
- [ ] Statistical analysis (log-rank + MCMC) complete
- [ ] Manuscript: *Cytogenetic Tree of Fibroblast Differentiation* — submitted Nature Methods or Cell Reports Methods
- [ ] Dataset release on Zenodo (DOI)

## v4.0 (Phase 2 — mouse HSCs, ~2027-2028)

- [ ] RITE-Centriolin construct validated in mouse HSCs (LSK cells)
- [ ] Serial bone marrow transplantation with RITE-tagged HSCs
- [ ] Cross-validation between fibroblast tree (Phase 1) and HSC tree (Phase 2) — conserved topology?
- [ ] Paper: *Centriolar Lineage Tracking Across Mammalian Stem Cell Compartments*

## v5.0 (Phase 3 — vertebrate embryo, ~2028-2029)

- [ ] Zebrafish or mouse zygote microinjection
- [ ] Full embryonic imaging through gastrulation
- [ ] First complete DAG from zygote → early somatic lineages
- [ ] Landmark paper: *The Full Cytogenetic Tree of Vertebrate Development*

## v6.0 (Long-term, 2030+)

- [ ] Cross-species Cytogenetic Tree comparison (mammals + fish + amphibian)
- [ ] Platform release for other labs (Addgene plasmids, Micro-Manager plugin, DAG-reconstruction library)
- [ ] Clinical translation: human PSC-derived organoid Cytogenetic Trees for disease modeling
- [ ] Integration with MCOA full multi-counter framework for per-lineage aging prediction

---

## Known blockers / decision points

| Blocker | Decision date | If resolved | If blocked |
|---------|---------------|-------------|------------|
| Impetus funding | 2026-05-15 (decision) | Proceed Phase 1 Jun-Dec 2026 | Re-submit Hevolution/NIH R21; delay 6-12 mo |
| RITE-Centriolin construct works | Month 2 of Phase 1 | Continue to Aim A.5 | Fall back to Dendra2-Centrin photoconversion |
| 405-nm phototoxicity | Month 2 of Phase 1 | Stay with CW Cobolt | Upgrade to fs-IR (+$15k) or switch target organelle |
| Geiger lab Phase B | Month 6 | Ulm collaboration active | Passegué / Goodell fallback |

```
### `ImageAnalysis/UPGRADE.md` (2388 chars)
```md
# ImageAnalysis: Future Extensions Beyond MVP

## 1. Advanced Live-Cell & 4D Analysis
*   **Integrated Tracking:** Combine centriole age signature tracking with whole-cell lineage tracking (e.g., from nuclear label) in a single Fiji/ImageJ or Python (Napari) platform.
*   **Dynamic Metrics:** Move beyond endpoint ratios to calculate **rate of polyglutamylation accumulation** on the mother centriole over the cell cycle as a more precise aging clock.
*   **Event-Triggered Analysis:** Automatically detect cell division events in a timelapse and analyze the inheritance of centriole age signatures by daughter cells.

## 2. Enhanced Phenotypic Profiling
*   **Shape & Texture Metrics:** Incorporate advanced morphology features (e.g., centriole pair orientation, signal texture using Haralick features) that may correlate with cell state or lineage history.
*   **Multi-Plex Expansion:** Design analysis modules for additional relevant stains (e.g., cell cycle markers like EdU, differentiation markers) to enrich the contextual data around each centriole measurement.
*   **3D Analysis:** Extend pipelines to work with confocal z-stacks, enabling volumetric quantification of signals and more accurate cilia detection.

## 3. Machine Learning Augmentation
*   **Detection Model:** Train a U-Net or similar model for more robust centriole and cilium detection across diverse cell types and imaging conditions, reducing reliance on manual threshold tuning.
*   **State Classification:** Use the quantitative image-derived features (GT335 ratio, ciliation, morphology) to train a classifier that predicts a cell's position in the differentiation tree.

## 4. Pipeline Engineering & Deployment
*   **Web Interface:** Develop a lightweight local web UI (e.g., using Gradio) to allow biologists to upload images, select pipelines, and view results without command-line interaction.
*   **Cloud-Native Version:** Containerize the entire pipeline (e.g., with Nextflow) for scalable execution on cloud or cluster environments, enabling population-scale image analysis.
*   **Public Repository:** Curate and publish a set of validated, versioned pipelines on a platform like GitHub or BioImage Archive, along with benchmark datasets, to serve the wider cell biology community.

---
*These extensions depend on the successful completion of the Phase A milestones outlined in [TODO.md](TODO.md).*

```
### `CellPose_Segmentation/UPGRADE.md` (1136 chars)
```md
# CellPose_Segmentation — UPGRADE

## U1. 3D segmentation
- Move from 2D to 3D stacks using CellPose 3D mode
- Requires Piezo-Z live-cell protocol (see `LiveCellMicroscopy`)

## U2. Real-time on-microscope inference
- Run CellPose inference inline with acquisition via PyMMCore-Plus callbacks
- Enables adaptive acquisition (e.g., auto-trigger high-speed mode at mitosis)

## U3. SAM / SAM2 hybrid
- Integrate Segment-Anything (SAM2) for zero-shot fallback on unfamiliar cell types
- Ensemble: CellPose + SAM2 → boosted recall

## U4. Mitotic stage classifier
- Fine-tune secondary model to classify prophase / metaphase / anaphase / telophase per cell
- Feeds lineage tracker's event detector

## U5. Active learning loop
- Uncertainty sampling to query annotator only on low-confidence frames
- Target: halve annotation cost for new cell types

## U6. Centriole triple-channel
- Extend to mCherry / GFP / BFP after RITE triple-tag upgrade (see `RITE_Centriole/UPGRADE.md` U1)

## U7. Cross-microscope generalization
- Test trained weights on Yokogawa CSU, Zeiss Airyscan, DMi8
- Publish "cytogenetic-cyto" model on CellPose model zoo

```
### `AICoordinator/UPGRADE.md` (1354 chars)
```md
# AICoordinator — UPGRADE

## U1. Multi-rig coordination
- One AIC instance coordinates N microscopes in parallel
- Essential step toward FCLC federated lineage atlas

## U2. Self-updating policy
- AIC reads outcomes of past runs → proposes PROMPT.md diffs for human review
- Version-controlled policy evolution

## U3. Fine-tuned local LLM fallback
- Distill PROMPT.md-driven behaviour into a local 7B–14B model
- Eliminates API latency + cloud dependency for decision loop

## U4. RLHF from expert annotations
- Dr. Tkemaladze labels "good" vs "bad" decisions on replayed runs
- Use DPO to refine the decision policy

## U5. Integration with CDATA
- AIC queries CDATA's biological-age model in real time to tag cells as "old-centriole equivalents"
- Closes loop between theory (CDATA) and experimental observation (tree)

## U6. Natural-language interface for lab staff
- Staff can type "next experiment: knock out CEP152, track for 48 h, abort if division rate < 0.3/day"
- AIC generates full PROMPT + useq-schema YAML

## U7. Ethical / safety board
- Formal policy-approval workflow for risky experiments
- PROMPT.md changes require sign-off from PI + safety officer

## U8. Scientific-reasoning agent
- AIC drafts results sections + figures directly from event logs
- Auto-composes methods paragraph with correct parameters from PARAMETERS.md chain

```
### `TODO.md` (2418 chars)
```md
# TODO — CytogeneticTree

## Phase 0 (2026-04 scaffolding)

- [x] Generate umbrella CONCEPT.md + core files (this set) — 2026-04-21
- [x] Generate 12 technology subproject scaffoldings — 2026-04-21 (parallel agent)
- [ ] Populate KNOWLEDGE.md with verified literature (parallel agent in progress)
- [ ] MAP.md dependency graph across subprojects
- [ ] LINKS.md with Addgene plasmids, GitHub repos, Micro-Manager documentation
- [ ] Review all auto-generated subfolder CONCEPTs for scientific accuracy
- [x] Replace any `[PMID-PENDING-VERIFY]` placeholders with real verified PMIDs (feedback_verify_references rule) — 2026-04-22

## Phase 1 — Minimum Viable Cytogenetic Tree (MVCT; contingent on Impetus Go)

- [ ] De-novo RITE-Centriolin construct cloning (Twist Bio → Addgene submission → validation in BJ-hTERT). Blocker: ~6-8 weeks cloning + clonal selection timeline.
- [ ] AutomatedMicroscopy platform build (Phase A Impetus budget)
- [ ] AI `/overnight` agent orchestration (PROMPT.md validated in 48h pilot before full run)
- [ ] 6-month parallel tracking of BJ-hTERT lineages (all arms per Impetus design)
- [ ] Division-event log → DAG reconstruction (GenealogyReconstruction subproject)
- [ ] Tree annotation with polyGlu signal, Ninein co-stain, ARL13B ciliation

## Phase 2 — Mouse HSC Tree (conditional on Phase 1 success)

- [ ] Collaboration with Geiger lab (Ulm) / alternative (Passegué / Goodell)
- [ ] Serial bone marrow transplantation with RITE-Centriolin HSCs
- [ ] Competitive CD45.1/CD45.2 congenic tracking

## Phase 3 — Vertebrate Embryo (long-term, 2028-29)

- [ ] Zebrafish or mouse zygote microinjection with RITE construct
- [ ] Full embryonic imaging through blastocyst / gastrulation
- [ ] Full DAG reconstruction from zygote

## Publication plan

- [ ] Methodology paper (Nat Methods / Cell Reports Methods) — Phase 1 first
- [ ] Scientific paper (Nature / Cell / Nature Aging) — Phase 1 validation of CDATA prediction
- [ ] Data release (Zenodo with DOI) — concurrent with manuscript

## Risks / Open Questions

- [ ] RITE-Centriolin construct validation uncertain (never published for centrioles) — fallback: Dendra2-Centrin photoconversion
- [ ] 405 nm laser phototoxicity to sister centrioles may confound arm 2 (centriole-independent control)
- [ ] AI segmentation edge cases (mixed-centriole daughters, focus drift)
- [ ] Computational cost of long-running DAG on 200+ PD of data

```
### `ImageAnalysis/TODO.md` (1881 chars)
```md
# ImageAnalysis: Phase A Milestones (Prioritized)

## High Priority (Weeks 1-4)
*   **H1:** Establish version-controlled repository with `CONCEPT.md`, `README.md`, `PARAMETERS.md`.
*   **H2:** **Fixed-Sample MVP:** Build and validate the core CellProfiler pipeline for a single field.
    *   H2.1: Successfully segment nuclei and identify centriole puncta.
    *   H2.2: Implement mother/daughter pairing logic based on GT335 intensity.
    *   H2.3: Output a correct GT335 ratio for 10+ manually validated centriole pairs (success criterion: >95% accuracy).
*   **H3:** Integrate Ninein channel analysis to validate mother assignment.

## Medium Priority (Weeks 5-8)
*   **M1:** **Fixed-Sample Scaling:** Extend pipeline to process multi-field, multi-well plates automatically.
*   **M2:** Add ARL13B ciliation analysis module to the fixed-sample pipeline.
*   **M3:** Create a standardized CSV output schema and a Python script to aggregate results from multiple plates.
*   **M4:** Document pipeline thoroughly and create a test dataset.

## Low Priority (Weeks 9-12)
*   **L1:** **Live-Cell Prototype:** Begin Fiji/ImageJ macro for time-series analysis of GT355 signal in live cells (requires collaboration with `../WetLabProtocols/` on live-cell staining).
*   **L2:** Implement basic tracking of centriole pairs through one cell division in the live-cell macro.
*   **L3:** Cross-validate intensity measurements between CellProfiler (fixed) and Fiji (live) pipelines using calibration samples.

## Collaboration Points
*   Coordinate with **`../WetLabProtocols/`** to obtain pilot image sets for development and validation.
*   Align output schema with **`../DataModel/`** team for direct database ingestion.
*   Provide initial test data outputs to **`../LineageReconstruction/`** team by end of Phase A.

---
*Future ambitions are documented in [UPGRADE.md](UPGRADE.md).*

```
### `CellPose_Segmentation/TODO.md` (984 chars)
```md
# CellPose_Segmentation — TODO (Phase A)

## A1. Dataset collection
- [ ] Acquire ≥ 3 × 24 h live-cell datasets on BJ-hTERT-RITE
- [ ] Export representative frames (every 30 min) for annotation
- [ ] Recruit annotator; build annotation protocol

## A2. Fine-tune CellPose
- [ ] Baseline cyto3 performance (no fine-tuning)
- [ ] Human-in-the-loop correction pass 1 (50 frames)
- [ ] Human-in-the-loop correction pass 2 (full 200 frames)
- [ ] Cross-validation split

## A3. Integrate centriole spot detector
- [ ] Wire spotiflow / Trackpy downstream of CellPose masks
- [ ] Per-cell SNR calibration
- [ ] Benchmark vs hand counts

## A4. Pipeline packaging
- [ ] `run_segmentation.py` CLI: input = OME-TIFF, output = HDF5 cells + centrioles
- [ ] Dockerfile for reproducibility
- [ ] Zenodo release (weights + sample dataset + DOI)
- [ ] Preprint draft: methods note

## Gate to Phase B
- Cell F1 ≥ 0.95, centriole F1 ≥ 0.90
- Pipeline runs end-to-end on 24 h dataset < 4 h wall-clock

```
### `AICoordinator/TODO.md` (1349 chars)
```md
# AICoordinator — TODO (Phase A)

## A1. Prompt engineering
- [ ] Draft `PROMPT.md` with policy sections (tree shape, age bias, focus, mitosis, phototoxicity, abort)
- [ ] Peer-review PROMPT.md with Dr. Tkemaladze + one imaging specialist
- [ ] Define JSON command schema (machine-readable)
- [ ] Safety / dry-run rules documented

## A2. Dry-run harness
- [ ] Synthetic lineage generator in `GenealogyReconstruction` produces mock frames
- [ ] Orchestrator reads mock data, emits commands → validator script checks safety
- [ ] Run 100 virtual experiments → compile failure modes → patch PROMPT.md

## A3. Integration
- [ ] Zarr reader: subscribe to new-frame events from MicroscopeController
- [ ] Command dispatcher: write to named pipe / ZeroMQ → controller
- [ ] Human override dashboard (FastAPI minimal)
- [ ] Slack / Telegram notification channel

## A4. Co-driven run
- [ ] 24 h supervised run (human approves every ablation)
- [ ] 48 h semi-autonomous (human approves every 10th ablation)
- [ ] 72 h fully autonomous with daily human review
- [ ] Publish decision log + post-mortem

## A5. Packaging
- [ ] Open-source `cytotree-aic` on GitHub (MIT)
- [ ] Zenodo DOI for PROMPT.md v1.0
- [ ] Methods note / preprint

## Gate to Phase B
- 72 h autonomous run with ≤ 3 human interventions
- Decision accuracy ≥ 90 % vs human expert on replay

```
### `KNOWLEDGE.md` (16413 chars)
```md
# KNOWLEDGE — CytogeneticTree

**Compiled:** 2026-04-21 (populated by systematic literature-search session)
**Purpose:** Systematic landscape for the proposed Impetus experiment — pure old-centriole lineage purification + zygote-to-terminal cytogenetic tree reconstruction, used to validate CDATA.
**Method:** PubMed eSearch + eSummary direct API calls. Every PMID listed below was verified by `esummary` — title, first author, and year cross-checked against NCBI JSON response. **Zero hallucinated references.**

> ⚠️ **Stub correction:** the earlier stub listed "Loeffler D, *Nature* 2019, PMID 31485075." Verification showed 31485075 is actually a quantum-physics paper. The correct Loeffler HSC paper is **PMID 31485073** ("Asymmetric lysosome inheritance predicts activation of haematopoietic stem cells"). Updated below.

---

## 1. Novelty verification — is this experiment truly first?

**YES, in the specific combination proposed.**

Three independent claims must hold to justify "first":

| # | Claim | Status | Evidence |
|---|-------|--------|----------|
| 1 | **Pure old-centriole lineage purification** (isolate all descendants that inherited a specific age-cohort of centrioles, then phenotype them) | No prior art found | Exhaustive search for "centriole age" + "lineage" + "purification/sorting" returned only descriptive asymmetry studies (Yamashita, Wang, Januschke, Paridaen, Rebollo, Reina, Royall), never a purification-and-sort workflow |
| 2 | **Cytogenetic tree built from centriole age** (reconstruct the full lineage tree by following centriole generation labels, not barcodes/CRISPR scars) | No prior art found | All CRISPR-barcode trees (GESTALT, scGESTALT, CARLIN, LARRY, Chan 2019, Kalhor 2018, Frieda MEMOIR, Spanjaard 2018, Plass 2018) use DNA scars or viral barcodes — **never centriole itself as the heritable mark** |
| 3 | **RITE applied to centrioles** (recombination-induced protein-age tagging of centriolar components like Centrin / SAS-6 / CPAP) | No prior art found | Verzijlbergen 2010, Terweij 2013, Thayer 2014, Radman-Livaja 2011 all use RITE on yeast soluble/nuclear/histone proteins; no RITE cassette targeting a centriole protein in any organism |

**Bottom line:** the experiment intersects three well-established fields in a way no group has published. The CRISPR-barcode lineage community has never attempted centriole-age labelling; the centrosome-asymmetry community has only done descriptive live imaging of 2–4 cell divisions, never full-tree reconstruction or FACS-based purification of age-stratified lineages.

---

## 2. Closest prior art (verified PMIDs)

### Block 1 — Centriole asymmetric inheritance

| PMID | First author, year | Journal | Relevance |
|------|--------------------|---------|-----------|
| **17255513** | Yamashita YM, 2007 | Science | *Foundational.* Asymmetric inheritance of mother vs. daughter centrosome in Drosophila male GSCs — established that stem cells retain the older centrosome. Primary citation for the CDATA thesis. |
| **17336911** | Rebollo E, 2007 | Dev Cell | Functionally unequal centrosomes drive spindle orientation in Drosophila neuroblasts. Parallel evidence in a different stem cell type. |
| **19829375** | Wang X, 2009 | Nature | Asymmetric centrosome inheritance maintains neural progenitors in the neocortex — extends phenomenon to mammalian brain. |
| **19829363** | Stearns T, 2009 | Nature | News & Views: "Stem cells: A fateful age gap." Short framing piece, useful citation. |
| **21145745** | Conduit PT, 2010 | Curr Biol | Cnn dynamics drive centrosome size asymmetry; daughter-centriole retention in Drosophila neuroblasts (mechanism). |
| **21407209** | Januschke J, 2011 | Nat Commun | *Counter-example.* Drosophila neuroblasts retain the **daughter** (younger) centrosome — cell-type-specific directionality, caveat for CDATA universality. |
| **22683192** | Pelletier L, 2012 | Curr Opin Cell Biol | Review: "Centrosome asymmetry and inheritance during animal development." |
| **24120134** | Paridaen JT, 2013 | Cell | Asymmetric inheritance of centrosome-associated primary cilium membrane directs ciliogenesis after division — mechanistic follow-up. |
| **25047620** | Reina J, 2014 | Phil Trans B | Review: "When fate follows age: unequal centrosomes in asymmetric cell division." Best single-document summary. |
| **31485073** | Loeffler D, 2019 | Nature | Asymmetric lysosome inheritance predicts activation of HSCs. Not centriole-specific but provides the HSC-lineage methodology (live imaging + long-term clonal tracking) directly reusable for CDATA validation. |
| **37882444** | Royall LN, 2023 | eLife | Asymmetric inheritance of centrosomes maintains stem cell properties in human neural progenitor cells. Most recent human/mammalian confirmation. |
| **36988082** | Gönczy P, 2023 | Genetics | Sperm-contributed centrioles segregate stochastically into 4-cell C. elegans — relevant caveat for zygote-level tracking. |

### Block 2 — RITE-like protein-age tagging

| PMID | First author, year | Journal | Relevance |
|------|--------------------|---------|-----------|
| **20018668** | Verzijlbergen KF, 2010 | PNAS | *Original RITE* — recombination-induced tag exchange, yeast, histone turnover. Direct template for centriole-RITE cassette. |
| **21666805** | Radman-Livaja M, 2011 | PLoS Biol | Ancestral histone inheritance in yeast via RITE. |
| **23708297** | Terweij M, 2013 | G3 | RITE cassette series for S. cerevisiae — standardised toolkit. |
| **25228775** | Thayer NH, 2014 | PNAS | *Closest methodological precedent.* First to use RITE specifically to track what is retained across repeated asymmetric divisions — but not on centrioles. The logic transfers directly to the CytogeneticTree design. |

**Gap confirmed:** no RITE cassette targeting any centriole component in any organism — verified by orthogonal queries (SAS-6/Centrin/CPAP + RITE/recombination-induced/tag-exchange → zero hits). This is a **genuine methodological novelty**.

### Block 3 — Lineage tracing technologies (comparison baseline)

| PMID | First author, year | Journal | Method / Relevance |
|------|--------------------|---------|--------------------|
| **15882628** | Zong H, 2005 | Cell | MADM — mosaic analysis with double markers, mouse. Pre-CRISPR genetic lineage tool. |
| **23493421** | Gerlach C, 2013 | Science | Heterogeneous differentiation of individual CD8+ T cells (DNA barcode). |
| **23552896** | Naik SH, 2013 | Nature | Heritable lineage imprinting in hematopoietic progenitors (lentiviral barcodes). |
| **27229144** | McKenna A, 2016 | Science | GESTALT — original CRISPR scar lineage barcoding, zebrafish. |
| **27869821** | Frieda KL, 2017 | Nature | MEMOIR — synthetic in situ recording of lineage in single cells. |
| **29608178** | Raj B, 2018 | Nat Biotechnol | scGESTALT — lineage + transcriptome, zebrafish brain. |
| **29644996** | Spanjaard B, 2018 | Nat Biotechnol | LINNAEUS — scarring + scRNA, zebrafish. |
| **29674432** | Plass M, 2018 | Science | Whole-animal planarian cell-type atlas + lineage tree (single-cell). |
| **30093604** | Kalhor R, 2018 | Science | MARC1 — homing CRISPR developmental barcoding, whole mouse. |
| **30353175** | Raj B, 2018 | Nat Protoc | scGESTALT detailed protocol. |
| **31086336** | Chan MM, 2019 | Nature | Molecular recording of mammalian embryogenesis (mouse, CRISPR barcodes). State-of-the-art for "zygote → tissue" lineage maps. |
| **31974159** | Weinreb C, 2020 | Science | LARRY — lineage on transcriptional landscapes, hematopoiesis. |
| **32413320** | Bowling S, 2020 | Cell | CARLIN — engineered CRISPR-Cas9 mouse line for lineage + expression. |
| **32632001** | Weinreb C, 2020 | PNAS | Lineage reconstruction from clonal correlations (maths). |
| **34680165** | Molina MD, 2021 | Biomolecules | Review: planarian stem cell heterogeneity and lineage progression. |
| **39434128** | Lange M, 2024 | Genome Biol | moslin — mapping lineage-traced cells across timepoints (recent algorithmic SOTA). |
| **39745646** | Bowling S, 2025 | Methods Mol Biol | CARLIN detailed protocols. |

**None of these give CENTRIOLE-SPECIFIC AGE information.** They are complementary (one could combine LARRY + centriole-RITE for dual encoding) and are the appropriate comparison group in any manuscript.

**Manual-lookup flags (could not auto-resolve a PMID within this session — do NOT cite without manual PubMed check):**
- MARCM Lee & Luo 1999 *Neuron* — DOI 10.1016/S0896-6273(00)80781-1, manual confirm before use.

### Block 4 — AI-directed live-cell manipulation

| PMID | First author, year | Journal | Relevance |
|------|--------------------|---------|-----------|
| **26418181** | Hughes RM, 2015 | Angew Chem | Optogenetic apoptosis — light-triggered cell death (optoBax / optoCaspase systems). Alternative to laser ablation. |
| **31155059** | Haar LL, 2019 | Methods Enzymol | Review of optogenetic perturbation of cell behaviour. |
| **33318659** | Stringer C, 2021 | Nat Methods | Cellpose — generalist DL segmentation. Off-the-shelf for centrosome/nucleus segmentation. |
| **33138911** | Thomsen J, 2020 | eLife | DeepFRET — DL for single-molecule signal classification (adjacent technology). |
| **35976090** | Aspert T, 2022 | eLife | DetecDiv — generalist DL for division tracking / survival (yeast). |
| **36076039** | Mahecic D, 2022 | Nat Methods | *Closest prior art* for the closed-loop concept. "Event-driven acquisition for content-enriched microscopy" — ML decides, on the fly, where and when to image. |
| **36639373** | Togninalli M, 2023 | NPJ Regen Med | ML classification of dual-fluorescence signals, muscle stem cell fate transitions. Direct template for the centriole-age-ratio → ablation trigger. |
| **37770712** | Zhang P, 2023 | Nat Methods | Deep-learning adaptive optics for SMLM — adjacent hardware-ML coupling. |

No prior work combines **live centriole-age readout + CV segmentation + targeted fs-laser ablation of non-target-age cells**. The individual components are mature; integration is novel.

### Block 5 — Genealogical tree reconstruction

Covered primarily in Block 3. The state-of-the-art for zygote → terminally differentiated genealogies is:
- **Chan 2019** (PMID 31086336, mouse, Nature) — whole-embryo CRISPR recording
- **Kalhor 2018** (PMID 30093604, homing CRISPR, whole mouse)
- **Plass 2018** (PMID 29674432, whole planarian, single-cell)

Both deliver ~6–12 generations of resolution but provide **zero centriole/aging information**. CytogeneticTree fills that orthogonal axis.

### Block 6 — Tools overlap with Impetus (verified + off-the-shelf)

| PMID | First author, year | Journal | Tool |
|------|--------------------|---------|------|
| **22743772** | Schindelin J, 2012 | Nat Methods | Fiji — image analysis platform |
| **25606571** | Edelstein AD, 2014 | J Biol Methods | µManager — open-source microscope control (pymmcore-plus / napari-micromanager ecosystem built on this) |
| **33318659** | Stringer C, 2021 | Nat Methods | Cellpose — segmentation |

Additional off-the-shelf infrastructure (software/vendor, no PMID):
- pymmcore-plus (Python wrapper over µManager)
- napari (Python viewer)
- Addgene (Cre-lox, RITE, Centrin-GFP, SAS-6 plasmids)
- Twist Bioscience (synthetic DNA for RITE cassettes)
- Zeiss LSM + LCI 405 nm + Mai Tai fs-IR (ablation platform)
- LARRY / CARLIN lentivirus (Addgene) as orthogonal lineage barcode

---

## 3. Technology ecosystem — what exists vs. what needs to be built

**Exists off-the-shelf:**
- RITE cassette design (Verzijlbergen, Terweij, Thayer) — adapt sequences to centriole proteins
- Cre-lox / Flp-FRT constitutive recombination plasmids (Addgene)
- Centrin-GFP and SAS-6 reporter lines (published, Addgene)
- Cellpose 2D/3D segmentation (GPU, open-source)
- µManager / pymmcore-plus closed-loop microscope control
- Event-driven acquisition framework (Mahecic 2022)
- fs-laser single-cell ablation on commercial Zeiss LSM
- LARRY / CARLIN orthogonal lineage barcode as cross-validation

**Must be built:**
1. **Centriole-RITE cassette** for Centrin / SAS-6 / CPAP (GFP→mCherry upon Cre) — 3–6 months of molecular cloning + validation
2. **FACS protocol** for sorting cells by centriole colour ratio — requires imaging sorter (BD CellView / Sony ID7000)
3. **Closed-loop ablation pipeline**: Cellpose → classifier → galvo → fs-laser, target latency <200 ms — builds on Mahecic 2022
4. **Lineage-tree inference algorithm** combining continuous centriole-age signal with discrete division events — no published algorithm; extend moslin (Lange 2024)
5. **Validation dataset**: Drosophila male GSC niche (Yamashita 2007 ground truth) as gold-standard training ground

---

## 4. Gap analysis

| Capability | Exists? | Gap size |
|-----------|---------|----------|
| Tag centriole-resident protein by recombinational colour switch | No | **Large** — primary novelty, 6–12 months cloning |
| Live imaging centriole-age through ≥5 divisions | Partial | Moderate — photobleaching of long-lived fluorophores; use Halo/SNAP chemistry |
| Sort cells by centriole age ratio | No | **Large** — no published FACS gating strategy on subcellular fluorescence ratio |
| Closed-loop target-specific ablation | Partial | Small — Mahecic 2022 + commercial Zeiss; integration ~3 months |
| Reconstruct lineage tree | Yes | None — adapt Chan 2019 + moslin |
| Compare to CDATA predictions | No | Small — requires clean experimental arm (young-lineage vs. old-lineage proliferation comparison) |

---

## 5. Recommendations — citation strategy and positioning

1. **Manuscript framing.** Position as *first integration* of RITE + centriole biology + lineage reconstruction. Anchor citations:
   - **Yamashita 2007** (PMID 17255513) — phenomenon
   - **Verzijlbergen 2010** (PMID 20018668) — method template
   - **Chan 2019** (PMID 31086336) — lineage-tree SOTA
   - **Mahecic 2022** (PMID 36076039) — imaging precedent
   - **Royall 2023** (PMID 37882444) — most recent human neural progenitor confirmation

2. **Address Januschke 2011 (PMID 21407209) honestly.** Daughter-centriole retention in a different neuroblast type — design must specify a tissue where mother-centriole retention is confirmed (Drosophila male GSCs, mouse neocortical RGCs, human iPSC-derived NPCs per Royall).

3. **Thayer 2014 (PMID 25228775) is the methodological precedent** — only published RITE study specifically asking "what is retained across repeated asymmetric divisions." Cite as direct logical parent.

4. **AI/ablation anchor:** Mahecic 2022 (PMID 36076039) + Togninalli 2023 (PMID 36639373). Do not cite unverified "AI-laser-ablation" reviews.

5. **Lineage-tree comparison:** Chan 2019 (PMID 31086336) + Kalhor 2018 (PMID 30093604) as benchmark; note they provide ~10 generations but **zero** aging information — CytogeneticTree fills that orthogonal axis.

6. **Self-citation slots (CLAUDE.md ≤15% rule):**
   - Tkemaladze 2023 *Mol Biol Rep* (PMID 36583780) — reduction/proliferation defects from old-centriole accumulation
   - Tkemaladze & Chichinadze 2005 *Biochemistry (Moscow)* — foundational CDATA
   - Chichinadze & Tkemaladze 2008 *Adv Gerontol* — centrosomal hypothesis of aging
   - Tkemaladze 2024 *Georgian Scientists* — cell center + oldest centrioles in stem cells
   - Tkemaladze 2026 *Longevity Horizon* (DOI 10.65649/3zzek632) — First Direct Structural Evidence for Age-Dependent Polyglutamylation Asymmetry in HSC (companion paper)

---

## 6. Verification method note

All PMIDs above were fetched from NCBI eutils (esearch → esummary) in the 2026-04-21 session. Every listed PMID's title + first author + year was directly read back from PubMed's JSON response. No DeepSeek or any LLM was used for literature search (`feedback_deepseek_no_citations`, `feedback_verify_references`).

**Stub correction log:**
- PMID 31485075 (stub, attributed to Loeffler D *Nature* 2019) — FAILED verification (actual 31485075 = quantum physics). Corrected to **PMID 31485073** ("Asymmetric lysosome inheritance predicts activation of HSCs," Loeffler D, Nature 2019).

**Manual-lookup flags (NOT verified via eutils during this session — check manually before citing):**
- MARCM Lee & Luo 1999 *Neuron*

## Audit trail

- 2026-04-21 — stub created
- 2026-04-21 — populated by systematic PubMed eSearch + eSummary verification; 37 distinct PMIDs verified; 1 stub PMID corrected (Loeffler); 1 flagged for manual follow-up (MARCM)

```
### `MEMORY.md` (3011 chars)
```md
# MEMORY — CytogeneticTree

## Permanent rules

- **Literature**: every PMID must be verified via PubMed esummary API before entering any file in this subproject. No DeepSeek for citation search.
- **RITE-Centriolin**: treat as *not yet published* / *must be de-novo cloned* until proven otherwise. Fallback: Dendra2-Centrin photoconvertible.
- **Connection to CDATA**: CytogeneticTree is the **empirical test-bed** for CDATA theoretical predictions, NOT a parallel theory. Keep this clear in outreach.
- **Connection to Impetus LOI**: Phase 1 MVCT is a *minimum-viable demo*. The full Cytogenetic Tree is a multi-year programme beyond the grant.
- **Scope**: this subproject handles *lineage-level* reconstruction; single-cell biochemistry lives in CDATA; tissue-level biomarkers live in Ze / BioSense.

## Dated entries

### 2026-04-21

- Subproject created (CytogeneticTree) as new LongevityCommon subproject per Jaba's request.
- Umbrella CONCEPT + 10 core files scaffolded (this set).
- 12 technology sub-subprojects scaffolded via parallel agent (CONCEPT + 5-file core each).
- Literature landscape agent (parallel) gathering KNOWLEDGE.md — PubMed + bioRxiv + arXiv + Google Scholar. **Search starts AFTER scaffolding completes** (per Jaba's explicit instruction).
- Connection to Impetus LOI 2026-04-25 documented — Phase A experiment = MVCT demonstration.
- Connection to Ilia Zheleznov HSC simulator noted — his computational model could pre-validate expected tree topology before wet lab.
- Open question logged: does RITE-Centriolin construct already exist publicly? Impetus audit 2026-04-21 said NO.

## Technology gaps (known today)

- **RITE-Centriolin construct** — likely first in the world; de-novo synth.
- **AI-directed real-time ablation orchestration** — novel; Claude Code `/overnight` will be first operational framework.
- **Full genealogy-reconstruction algorithm** for centriole lineage — no off-the-shelf tool; will be built (GenealogyReconstruction subproject).
- **Lineage-to-differentiation annotation** at centriole granularity — not previously done.

## Things to remember across sessions

- Always check `CONCEPT.md` is in sync with `CDATA/CONCEPT.md` + `MCOA/` claims. If CDATA updates its theory, CytogeneticTree predictions might need update.
- Keep clear separation: Impetus = funded Phase A (MVCT fibroblasts, binary Go/No-Go). Cytogenetic Tree = 3-phase multi-year programme.
- `~/Desktop/LongevityCommon/AutomatedMicroscopy/` is the shared hardware platform dir; CytogeneticTree is the methodology layer on top.

## Related memory files

- `project_longevity_georgia_ngo` — NGO that hosts this research
- `feedback_deepseek_no_citations` — enforce for all KNOWLEDGE.md entries
- `feedback_verify_references` — verify every PMID before commit
- `feedback_cdata_docs_sync` — when editing CDATA, check if CytogeneticTree docs need update (and vice versa)
- `project_mcoa_nature_correspondence` — MCOA manuscript at Nature Aging; CytogeneticTree is the empirical counterpart

```
### `LINKS.md` (2994 chars)
```md
# LINKS — CytogeneticTree

## Internal (within LongevityCommon ecosystem)

- Umbrella: `~/Desktop/LongevityCommon/CONCEPT.md`
- Parent theory (mechanism): `~/Desktop/LongevityCommon/CDATA/`
- Parent theoretical framework: `~/Desktop/LongevityCommon/MCOA/` (under Nature Aging review, NATAGING-P13741)
- Shared hardware platform: `~/Desktop/LongevityCommon/AutomatedMicroscopy/`
- Orthogonal biomarker work: `~/Desktop/LongevityCommon/Ze/` + `~/Desktop/LongevityCommon/BioSense/`
- Funding context: `~/Documents/Grants/LongevityCommon/CDATA/docs/IMPETUS_2026-04-25/`
- Peer collaborator simulator: `~/Documents/Meetings/Ilia_Zheleznov_2026-04-25/MEETING_KIT.md` (external code: https://github.com/zhelilyan-create/hematopoiesis-simulator)

## Git

- Public view: `djabbat/LongevityCommon` (partial — core .md gitignored)
- Private full: `djabbat/LongevityCommon-private` (full content incl. CytogeneticTree)
- Cross-repo: tech subfolders can be pulled into project-specific repos if needed (e.g., `djabbat/CDATA-private` mirrors hardware dependencies)

## External resources

### Plasmids / constructs

- **Addgene** — https://www.addgene.org (search for RITE, Cre-ERT2, Centrin)
- **Twist Bio** — https://www.twistbioscience.com (gene synthesis for de-novo RITE-Centriolin)

### Microscopy software

- **Micro-Manager 2.0** — https://micro-manager.org
- **PyMMCore-Plus** — https://pymmcore-plus.github.io/pymmcore-plus/
- **CellPose** — https://github.com/MouseLand/cellpose
- **Fiji / ImageJ** — https://fiji.sc
- **CellProfiler** — https://cellprofiler.org

### Hardware vendors

- **Zeiss used market** — microscope retrofit base
- **Cobolt (Hübner)** — https://hubner-photonics.com (405 nm laser)
- **ThorLabs** — https://www.thorlabs.com (LED illumination M470L5 + LEDD1B)
- **FLIR / Teledyne** — https://www.flir.com/products/blackfly-s-usb3/ (scientific CMOS)
- **Hikrobot** — https://en.hikrobotics.com (Chinese alternative scientific CMOS)

### Published references (to be populated by literature search agent — see KNOWLEDGE.md)

- RITE core: Verzijlbergen 2010 PNAS (yeast) — PMID 20018668
- Centriolin-RITE precedent: Royall 2023 *eLife* — PMID 37882444 (RITE in neural progenitors)
- CDATA core: Tkemaladze 2023 *Mol Biol Rep* — PMID 36583780
- Asymmetric centriole inheritance: Yamashita 2007 *Science* — PMID 17255513
- Lineage tracing foundation: Wang 2009 *Nature* — PMID 19829375 (neural progenitors)
- Live-cell AI segmentation: Stringer 2021 *Nat Methods* (CellPose) — PMID 33318659

## OSF / preregistration

- CytogeneticTree Phase 1 Statistical Analysis Plan — to be deposited on OSF before Month 3 unblinding

## Author / funder

- **PI:** Dr. Jaba Tkemaladze, MD — jaba@longevity.ge
- **ORCID:** 0000-0002-3826-7982
- **Google Scholar:** (pending link — see DrJaba.com)
- **Host:** Georgia Longevity Alliance / კავშირი დღეგრძელობა (NGO #404506520, est. 2016-01-12)
- **Funding (planned):** Longevity Impetus Grants 2026-04-25 ($92k Phase A; $120k Phase B conditional)

```