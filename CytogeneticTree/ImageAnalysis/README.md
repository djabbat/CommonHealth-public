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
