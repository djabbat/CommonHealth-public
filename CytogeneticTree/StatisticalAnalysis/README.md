# StatisticalAnalysis

This directory contains the statistical core for the **CytogeneticTree** project led by Dr. Jaba Tkemaladze.

## Overview
We develop and apply statistical models to analyze the retention of the maternal centriole during cellular differentiation. The goal is to quantify how centriole "age" correlates with cell fate, providing quantitative parameters to help reconstruct lineage trees.

## Key Analyses
*   **Survival Analysis:** Kaplan-Meier curves and log-rank tests to compare centriole retention probabilities across different lineage branches.
*   **Bayesian Modeling:** Using MCMC (via Stan/PyMC) to fit parametric survival models and estimate posterior distributions for centriole loss rates.
*   **Sensitivity & Identifiability:** Global variance-based sensitivity analysis (Sobol method) to assess which model parameters are well-constrained by the data and to avoid over-interpretation.

## Usage
This package is designed for researchers analyzing centriole inheritance data. It inputs binarized centriole fate tables and hypothesized tree structures, outputting statistical comparisons, parameter estimates, and identifiability diagnostics.

For the overarching project concept, see the main [CytogeneticTree CONCEPT.md](../CONCEPT.md).
