# THEORY — AnalysisStack

This document provides the theoretical underpinnings of the AnalysisStack pipeline, covering each core component.

## 1. Segmentation (CellPose)

CellPose (Stringer et al. 2021, PMID: 33318659) relies on a U‑Net architecture that learns a topographical flow representation. For each pixel, the network predicts a vector pointing toward the centre of the cell instance. Instance masks are recovered by following these flows to their sinks. The theoretical advantage is that this flow-based approach naturally handles touching cells and does not require explicit watershed seeding. CellPose 2.0 (Pachitariu & Stringer 2022, PMID: 36344832) adds human‑in‑the‑loop fine‑tuning, leveraging user corrections to improve segmentation in challenging domains. CellPose 3.0 extends the architecture with image restoration capabilities, enabling denoising of low‑SNR live‑cell data.

## 2. Feature extraction (centriole punctum analysis)

Centriole detection is formulated as a diffraction‑limited spot localization problem. The theoretical limit for resolving two point sources is given by the Rayleigh criterion ~0.61λ/NA. For typical centriole imaging (λ=560 nm, NA=1.4), the resolution limit is ~240 nm, comparable to the centriole diameter (200–500 nm). We therefore treat each centriole as a Gaussian spot whose centre can be located with sub‑pixel precision using methods such as radial symmetry (Parthasarathy 2012, PMID: 22550944) or convolutional neural network regression (spotiflow; Dohmen et al. 2024). The red/green channel assignment for mother/daughter centrioles follows the RITE‑centriole system (Jones et al. 2022, PMID: 35758709), where the older centriole is labelled with a different fluorophore than the younger one.

## 3. Mixed‑effects modelling for hierarchical data

Lineage tracking produces nested data: cells within lineages within experimental replicates. To account for this hierarchy, we employ mixed‑effects models (Pinheiro & Bates 2000, ISBN 978‑0‑387‑98957‑0). The linear predictor is:
y_ij = β₀ + β₁x_ij + u_i + ε_ij
where y_ij is the outcome (e.g., centriole retention time) for cell j in lineage i, β are fixed effects, u_i ~ N(0,σ²_lin) are random intercepts for lineages, and ε_ij is residual error. This framework appropriately partitions variance between and within lineages.

## 4. Phylogenetic reconstruction theory adapted to mitotic lineages

Standard phylogenetic methods assume bifurcating trees with known branch lengths (in units of sequence divergence). Here, branch lengths correspond to time (frames) between divisions, and the tree topology is determined by which daughter inherits the older centriole. We use a maximum‑parsimony approach: at each asymmetric division, the daughter with the older centriole is assigned to the branch that continues the “old” lineage; the daughter with the younger centriole starts a new branch. This heuristic is biologically motivated by the observation that older centrioles tend to segregate with stem‑like fates (Yamashita et al. 2007, PMID: 17928863). The tree is reconstructed as a directed acyclic graph (DAG) in NetworkX, with nodes representing cells and edges representing parent–daughter relationships.

## 5. Single‑cell omics for terminal classification

Terminal cell‑type annotation uses a supervised classifier trained on scRNA‑seq reference data (e.g., Tabula Sapiens, PMID: 35549404). We employ a multinomial logistic regression with elastic‑net regularisation (Zou & Hastie 2005, PMID: 15826765) to map imputed expression from our cultured cells to reference cell‑type signatures. Classification confidence is assessed via posterior probabilities and a minimum threshold (ρ > 0.5) to avoid ambiguous assignments.
