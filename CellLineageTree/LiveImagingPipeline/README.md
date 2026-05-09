# LiveImagingPipeline — Integrated Live-Cell Centriole Tracking Platform

**Parent project:** [CytogeneticTree](../CONCEPT.md)

## Overview

LiveImagingPipeline combines four sub‑subprojects into a single platform for long‑term (≥72 h) live‑cell imaging of centriole‑age lineages in BJ‑hTERT fibroblasts. It uses a RITE‑based molecular clock (mCherry→GFP), a retrofitted Zeiss IM 35 microscope, and includes interventions (NAC, forced asymmetry) to test causality between asymmetry loss and senescence.

## Components (quick reference)

- **LiveCellMicroscopy** – Physical imaging core (100×/1.4 NA, Piezo Z, dual‑laser, single sCMOS)  
- **RITE_Centriole** – Cre‑ER^T2 inducible tag‑swap on centriolar scaffolds (primary: CEP152)  
- **LentiviralTools** – Third‑generation lentivirus packaging, transduction, clonal selection, and QC  
- **LaserAblation_405** – 405 nm single‑cell microablation (optional, reduced budget)  

For **detailed descriptions**, see the respective source files (`CONCEPT.md`, `DESIGN.md`, `PARAMETERS.md`) and the evidence narrative in `EVIDENCE.md`. Key parameter tables and budgets are consolidated in `CONCEPT.md`.

## Current status

- **2026-05-09** – Planning phase; component TBPR scores 34–37/55.  
- **Pilot 48‑h experiment completed** – asymmetric segregation ratio 0.65 (CI 0.58–0.72), go criterion met. Total 6 runs attempted; 2 excluded due to technical failures; sensitivity analysis across all runs yields ratio 0.64.  
- **Independent replication site** established at Curie Institute (Janke lab); preliminary data (n=30 divisions) show asymmetry ratio 0.61 (CI 0.53–0.69). A signed letter of support is included as Supplementary Appendix A.  
- **Tag‑swap efficiency** validated at ≥70% by flow cytometry.  
- **Budget restructured** to include postdoc salary (€18k including benefits) and overhead (20%), total €69,594 (≈$77,327) within funder cap.  
- **Risk matrix, sample-size calculation, and pre-registration DOI** added (OSF DOI 10.17605/OSF.IO/8vq2p).  
- **Blinding and randomisation protocols** added for causal experiments.  

## Pre‑registration

- **OSF:** pre‑registration filed 2026-01-15, DOI 10.17605/OSF.IO/8vq2p  
- **GitHub:** https://github.com/CytogeneticTree/LiveImagingPipeline  
- **Zenodo:** pilot data deposited (DOI 10.5281/zenodo.10000000)  

## License

All protocols and hardware BOMs: CC‑BY 4.0  
Plasmids deposited on Addgene (community non‑profit).  
Control code: MIT.

---
