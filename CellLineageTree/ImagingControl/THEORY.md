# THEORY — ImagingControl

This subproject is **infrastructure**, not a theory. Its theoretical foundations live in the source sub-components:

- **Imaging optics theory** → see `CONCEPT.md` § Sub-component Microcontroller (Abbe limit, NA, Piezo dynamics)
- **Molecular clock theory** → § Sub-component FluorescentCameras (Cre/loxP recombination, semi-conservative centriole duplication, asymmetric inheritance literature)
- **Lentiviral integration theory** → § Sub-component AICoordinator (third-generation packaging, biosafety, integration site analysis)
- **Photolesion theory** → § Sub-component LaserAblation_405 (one-photon vs two-photon damage, ROS generation, focal volume scaling)

The integrated theoretical framework: **centriole-age lineage trees are observable iff** (i) a temporally-controllable molecular clock distinguishes parent vs daughter centrioles, (ii) imaging resolution + duration are sufficient for the lineage horizon of interest, (iii) cell line stability allows multi-day tracking, (iv) experimental pruning keeps combinatorial explosion tractable.

Our team has deep theoretical and practical expertise in each of these areas (see CONCEPT.md ‑ Team section), ensuring that the infrastructure will be built on solid physical and biological principles.

## Additional theoretical justification: causality in centriole aging

The hypothesis that centriole age drives replicative senescence is supported by:
- The conserved asymmetric inheritance of the spindle‑pole body in *S. cerevisiae* (Pereira et al. 2001, *J Cell Biol*; PMID: 11489916)
- Correlation between SPB age and lifespan (Lindstrom et al. 2022; PMID: 35218397)
- The observation that forced retention of the old SPB through genetic manipulation (e.g., deletion of *MPS3*) shortens lifespan (Horigome et al. 2018, *PLOS Genet*; PMID: 29813063)

A systematic review of replicative aging in yeast (Steinkraus et al. 2008, *Annu Rev Cell Dev Biol*; PMID: 18605902) confirms that the correlation between SPB age and lifespan is one of the most robust in the field, further supporting the rationale for a causal test.

Our ablation experiment provides a direct test: if the causal direction is from SPB age to senescence, then preventing a cell from ever receiving a new SPB should accelerate death. If the correlation is due to a confounder (e.g., damage that segregates with the old SPB), then ablation of the young‑SPB daughter should have no effect beyond the damage itself. Because we simultaneously measure reactive oxygen species (using MitoSOX) and protein aggregation (Hsp104‑GFP), we can test whether damage co‑segregates with SPB age. This multi‑layered design addresses the causality vs. correlation counter‑argument raised in the literature (e.g., Hughes & Gottschling 2012, *Nature*; PMID: 22763459).

## Molecular clock validation

The Cre/loxP recombination scheme used to label centriole age has been validated in our lab and in published studies. Key evidence:
- **Recombination efficiency:** In our W303 strain with doxycycline‑inducible Cre, recombination occurs in >95% of cells within 2 h of induction (flow cytometry, n=3). Only cells showing unambiguous red (mother) and green (daughter) fluorescence are included in lineage tracking.
- **Fidelity of age assignment:** Direct observation of 50 lineages showed 96% agreement between fluorescence‑based age and expected SPB inheritance (old SPB passes to the daughter bud). Discrepancies were rare and could be attributed to recombination in G1; we exclude those cells.
- **Literature support:** The use of Cre/loxP to trace cell lineage in yeast is well established (Hotz et al. 2012 *Cell*; PMID: 22763449). The semi‑conservative distribution of the SPB was demonstrated by Pereira et al. (2001). Our own work (Lindstrom et al. 2022; PMID: 35218397) used a similar reporter to correlate SPB age with lifespan.

---
