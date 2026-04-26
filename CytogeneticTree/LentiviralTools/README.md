# LentiviralTools

Molecular biology workflow for the **CytogeneticTree** project: plasmid design, third-generation lentivirus packaging in HEK293T, transduction of BJ-hTERT (and other) target lines, single-cell clonal selection, karyotyping, and QC. Supplies validated clonal lines to the rest of the pipeline — primarily for the RITE_Centriole system.

## Quick facts

- **Plasmid source:** Twist Bioscience synthesis + Addgene backbones
- **Packaging:** 3-gen (pMD2.G, psPAX2, pLenti-payload) in HEK293T
- **Titer target:** ≥ 10⁷ TU/mL
- **Selection:** Puromycin 2 µg/mL
- **Clonal isolation:** single-cell FACS → 96-well
- **QC:** karyotype (G-banding), functional tamoxifen response, marker expression

## Status

Phase A — procurement + facility prep.

## Dependencies

- Upstream consumer: `RITE_Centriole`
- Downstream users: `LiveCellMicroscopy`, `DifferentiationAnnotation`
- BSL-2 facility required

## License

CC-BY 4.0 (protocols); plasmids deposited on Addgene (community non-profit).
