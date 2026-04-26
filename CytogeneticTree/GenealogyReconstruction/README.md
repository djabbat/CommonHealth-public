# GenealogyReconstruction

**Sub-Subproject of CytogeneticTree | Algorithmic Lineage Reconstruction**

This repository contains the core algorithms for the **GenealogyReconstruction** module, part of Dr. Jaba Tkemaladze's broader CytogeneticTree project. The goal is to computationally reconstruct the complete genealogical tree of cell differentiation, from zygote to terminal cells.

## How It Works
The algorithm takes two key inputs:
1.  A log of cell division events.
2.  Decisions on which daughter cell inherited the older "mother" centriole during each asymmetric division.

Using these, it builds a directed acyclic graph (DAG) in Python's NetworkX, representing the full lineage tree. This graph serves as the essential scaffold for mapping future cytogenetic and epigenetic data, enabling a unified view of a cell's ancestry and state.

## Key Features
*   **Centriole-Based Logic:** Uses the biologically established link between older centriole inheritance and cell fate to inform tree branching.
*   **Robust to Noise:** Includes methods to handle common imaging artifacts like focus drift and ambiguous signals.
*   **Interoperable:** Outputs a standard graph structure ready for integration with other CytogeneticTree analysis modules.

For a detailed conceptual overview, see the main [CytogeneticTree CONCEPT.md](../CONCEPT.md) and our subproject [CONCEPT.md](./CONCEPT.md).

== END
