# AnalysisStack / README.md (merged 2026-05-09)


---
## === CellPose_Segmentation / README.md ===

# CellPose_Segmentation

AI‑based live‑cell segmentation pipeline for the **CytogeneticTree** project. Built on CellPose (Stringer et al. 2021) and extended with spotiflow‑style sub‑pixel spot detection for centriolar foci. Outputs per‑frame instance masks of cells + centriole (red/green) positions, feeding the lineage tracker downstream.

## Quick facts

- **Backbone:** CellPose 3.0 (generalist cyto3 model + fine‑tuned weights for BJ‑hTERT)
- **Input:** 2D fluorescence time‑lapse (up to 3 channels) at 100× oil
- **Output:** HDF5 with per‑frame cell masks + centriole centroid tables
- **Hardware target:** single RTX 4070 / 4080 or equivalent
- **Throughput target:** ≥ 2 fps inference on 1024 × 1024 frames

## Status

Phase A — design + benchmark. See `TODO.md`.

## Dependencies

- Upstream: `LiveCellMicroscopy`, `FluorescentCameras`, `RITE_Centriole`
- Downstream: `ImageAnalysis`, `GenealogyReconstruction`, `AICoordinator`

## License

MIT (code); CC‑BY 4.0 (fine‑tuned weights + training dataset on Zenodo).

---
## === ImageAnalysis / README.md ===

# ImageAnalysis Subproject

**Part of the CytogeneticTree Project**

This repository contains the image analysis pipelines for the CytogeneticTree project, led by Dr. Jaba Tkemaladze. Our goal is to reconstruct a cell's genealogical tree by using the age of its centrioles as a persistent lineage tracer.

## What This Subproject Does
We provide automated, reproducible workflows to quantify key centriole biomarkers from microscopy images:
* **Centriole Age Signal:** Measures polyglutamylated tubulin (GT335 antibody) intensity at the mother centriole.
* **Maturity Marker:** Quantifies co‑localization with the mother‑specific protein Ninein.
* **Ciliation Status:** Detects primary cilia using ARL13B staining.

These quantitative measurements form the foundational dataset for downstream algorithms that reconstruct the Cytogenetic Tree.

## Key Outputs
* **Per‑cell metrics:** GT335 mother/daughter intensity ratio, Ninein localization, ciliation status.
* **Validated Pipelines:** Ready‑to‑use scripts for Fiji/ImageJ (live‑cell) and CellProfiler (high‑throughput fixed images).
* **Analysis Parameters:** Documented thresholds and settings for reproducible science.

## Getting Started
See `PARAMETERS.md` for technical specifications and `TODO.md` for current development milestones.

---
*For the broader project context, see the [CytogeneticTree Concept Overview](../CONCEPT.md).*

---
## === StatisticalAnalysis / README.md ===

# StatisticalAnalysis

This directory contains the statistical core for the **CytogeneticTree** project led by Dr. Jaba Tkemaladze.

## Overview
We develop and apply statistical models to analyze the retention of the maternal centriole during cellular differentiation. The goal is to quantify how centriole "age" correlates with cell fate, providing quantitative parameters to help reconstruct lineage trees.

## Key Analyses
* **Survival Analysis:** Kaplan‑Meier curves and log‑rank tests to compare centriole retention probabilities across different lineage branches.
* **Bayesian Modeling:** Using MCMC (via Stan/PyMC) to fit parametric survival models and estimate posterior distributions for centriole loss rates.
* **Sensitivity & Identifiability:** Global variance‑based sensitivity analysis (Sobol method) to assess which model parameters are well‑constrained by the data and to avoid over‑interpretation.

## Usage
This package is designed for researchers analyzing centriole inheritance data. It inputs binarized centriole fate tables and hypothesized tree structures, outputting statistical comparisons, parameter estimates, and identifiability diagnostics.

For the overarching project concept, see the main [CytogeneticTree CONCEPT.md](../CONCEPT.md).

---
## === GenealogyReconstruction / README.md ===

# GenealogyReconstruction

**Sub‑Subproject of CytogeneticTree | Algorithmic Lineage Reconstruction**

This repository contains the core algorithms for the **GenealogyReconstruction** module, part of Dr. Jaba Tkemaladze's broader CytogeneticTree project. The goal is to computationally reconstruct the complete genealogical tree of cell differentiation, from zygote to terminal cells.

## How It Works
The algorithm takes two key inputs:
1. A log of cell division events.
2. Decisions on which daughter cell inherited the older "mother" centriole during each asymmetric division.

Using these, it builds a directed acyclic graph (DAG) in Python's NetworkX, representing the full lineage tree. This graph serves as the essential scaffold for mapping future cytogenetic and epigenetic data, enabling a unified view of a cell's ancestry and state.

## Key Features
* **Centriole‑Based Logic:** Uses the biologically established link between older centriole inheritance and cell fate to inform tree branching.
* **Robust to Noise:** Includes methods to handle common imaging artifacts like focus drift and ambiguous signals.
* **Interoperable:** Outputs a standard graph structure ready for integration with other CytogeneticTree analysis modules.

For a detailed conceptual overview, see the main [CytogeneticTree CONCEPT.md](../CONCEPT.md) and our subproject [CONCEPT.md](./CONCEPT.md).

== END

---
## === DifferentiationAnnotation / README.md ===

# DifferentiationAnnotation

**Sub‑Subproject of the CytogeneticTree Initiative**

A cell's lineage is its history; its state is its current identity. This module bridges the two.

**DifferentiationAnnotation** is the interpretive layer of the CytogeneticTree project. It takes the raw lineage connections between cells—reconstructed by tracking the age‑asymmetric inheritance of centrioles—and maps each cell onto a specific differentiation state (e.g., "pancreatic progenitor," "post‑mitotic neuron").

We achieve this by integrating three layers of evidence:
* **Molecular Profile:** Single‑cell RNA sequencing data reveals marker gene expression.
* **Morphological Signature:** High‑resolution imaging captures shape and structural features.
* **Protein Expression:** Key immunofluorescence stains provide definitive anchor points.

By fusing this data onto the lineage tree scaffold, we generate the **Annotated Differentiation Tree**. This is the fundamental map needed to pinpoint exactly when and where fate decisions occur during development, tissue regeneration, and disease.

For a comprehensive overview of the scientific vision, see the parent project [CytogeneticTree Concept](../CONCEPT.md).
