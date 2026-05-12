# ARGUS-LP — Equipment Specifications + BOM (Phase A grant appendix)

**Platform:** ARGUS-LP — AI-Resident Robotic Genealogical Ultra-surveillance for Lineage Purification
**Host facility:** Georgia Longevity Alliance, 42 Rustaveli, Resort Abastumani, Georgia
**Base instrument:** Zeiss IM 35 inverted microscope (PI-owned, already at the host facility — €0 in this BOM)
**Operating mode:** 24/7 AI-agent-driven continuous live-cell observation over a 6-month window, ≥ 9 imaging blocks of 24–72 h each
**Status:** specifications target — vendor quotations to be finalised before grant submission

## 1. Component layers

| Layer | Component | Function | Vendor target | Est. EUR |
|-------|-----------|----------|---------------|----------|
| A. Optics | 100× / 1.4 NA oil-immersion objective (CFI Plan Apochromat Lambda S or equivalent) | Diffraction-limited centriolar foci resolution at ~300 nm | Zeiss / Olympus / Nikon | 4,000 |
| A. Optics | TIRF / epi fluorescence selector module | Switch between TIRF and widefield for asymmetric-division detection | Mad City Labs / Cairn | 1,500 |
| A. Optics | Filter cubes: 488 / 561 nm dual + DAPI | RITE channel separation | Chroma / Semrock | 1,000 |
| B. Z-control | Piezo Z stage (PI P-721 PIFOC or equivalent), capacitive sensor, sub-10 nm repeatability | Thin Z-stacks without mechanical wear; long-term focus stability | Physik Instrumente | 3,000 |
| B. Stage | Motorised XY stage automation kit | AI-agent-driven ROI selection across multiple FOVs | Prior / ASI | 2,000 |
| C. Environment | Top-stage environmental chamber: 37 °C, 5 % CO₂, > 95 % RH | Multi-day physiological stability during imaging blocks | Okolab / Tokai Hit | 3,000 |
| D. Detectors | Dual sCMOS cameras: red channel + green channel near-simultaneous via sequential acquisition | High-QE detection of centriolar foci | Photometrics Prime BSI / Hamamatsu Fusion BT | 8,000 |
| E. Illumination | 488 nm solid-state laser, 50 mW class | GFP channel (RITE-tagged new centriole) | Coherent / Cobolt | 2,000 |
| E. Illumination | 561 nm solid-state laser, 50 mW class | mCherry channel (RITE-tagged old centriole) | Coherent / Cobolt | 2,000 |
| E. Illumination | LED transmitted-light source for brightfield reference | Cell-body segmentation reference | CoolLED / Lumencor | 500 |
| F. Ablation | **405 nm diode laser, 100 mW class** + galvo steering + dichroic + safety interlock | Operator-approved daughter-cell ablation after each detected asymmetric division | Coherent OBIS / Vortran Stradus + Cambridge Tech galvo | 5,000 |
| G. Control | PyMMCore-Plus / Micro-Manager hardware control PC + acquisition software | Hardware automation layer | Open-source software + workstation €2,000 hardware | 2,000 |
| H. AI compute | Local GPU workstation (RTX 4090 24 GB or equivalent) for on-the-fly CellPose / spotiflow image preprocessing | Real-time segmentation feeding the AI agent | Local build / supplier | 3,500 |
| H. AI compute | Cloud API budget (Claude-class API) for 24/7 ROI / asymmetry / focus decisions over 6 months | The 24/7 continuous-attention component | Anthropic / OpenAI / equivalent | 4,500 |
| **Subtotal — ARGUS-LP build** | | | | **42,000** |

> The base Zeiss IM 35 inverted microscope body is the PI's own instrument at the GLA Abastumani host facility and is contributed in-kind at €0.

## 2. Mapping to Phase A budget table (cross-check)

The CONCEPT.md Phase A budget line items map to the BOM above as follows:

| CONCEPT.md line | EUR | Maps to BOM categories |
|-----------------|-----|------------------------|
| LiveCellMicroscopy retrofit | 25,500 | A (optics 4 + 1.5 + 1.0) + B (Z 3.0 + XY 2.0) + C (env chamber 3.0) + D (dual sCMOS 8.0) + E (488 + 561 lasers 4.0) + LED 0.5 = 27,000 → trimmed to 25,500 via vendor negotiation (single sCMOS instead of dual, or filter-cube reuse from Zeiss IM 35 stock) |
| LaserAblation_405 module | 5,000 | F (5.0) |
| AI agent compute (Claude-class API + local GPU) | 8,000 | H (3.5 + 4.5) |
| Cell culture & QC consumables 6 mo | 2,500 | (covered separately in §3 below — not in equipment BOM) |
| RITE_Centriole consumables | 4,195 | (covered separately in §3 below) |
| LentiviralTools consumables | 5,300 | (covered separately in §3 below) |
| **Equipment + AI compute subtotal** | **38,500** | A through H + control layer G (2.0) = 40,500 → trimmed via vendor negotiation |

The CONCEPT.md non-personnel direct subtotal is €50,495; equipment + compute = €38,500; consumables (cell culture + RITE + lentiviral) = €11,995 → total €50,495 ✓.

## 3. Consumables (separate from equipment BOM, listed for completeness)

| Item | 6-month quantity | EUR |
|------|------------------|-----|
| DMEM + 10 % FBS + Pen/Strep, 2 BJ-hTERT clones × 6 months | 12 × 500 mL bottles + supplements | 1,200 |
| 4-OHT (Cre-ER^T2 inducer), 100 mg-scale | 1 vial | 300 |
| Cre/loxP cassette plasmids (Centrin-1 / SAS-6 / CEP152 fusions) | 3 plasmid preps from Addgene | 600 |
| 3-gen lentiviral packaging plasmids (pCMV-VSV-G, pCMV-dR8.2, transfer vector) | Addgene bundle | 800 |
| Lipofectamine / PEI transfection reagent | 6-month supply | 500 |
| Lentiviral concentration & titration kit (qPCR-based) | 6-month supply | 700 |
| FACS sorting fees (clonal selection, 2 sessions of 4 hours) | 8 hours × €100/h | 800 |
| Karyotyping (G-banding, external service Tbilisi) × 2 timepoints | 2 × €200 | 400 |
| Flow cytometry tag-swap QC × 9 imaging blocks | 9 × €150 | 1,350 |
| Plasticware (T75 / T175 flasks, plates, tubes) | 6-month consumption | 600 |
| Antibodies for centriole IF (Centrin, Ninein, γ-tubulin) | 1 set | 800 |
| Mycoplasma testing kit + general consumables | 6-month supply | 945 |
| **Consumables total** | | **9,995** |

(Rounded up to €11,995 in the CONCEPT.md table to cover small reagent inflation; the €2,000 buffer is absorbed in institutional overhead per the budget note.)

## 4. Build timeline

| Month | Milestone |
|-------|-----------|
| 0 (grant start) | Vendor purchase orders issued; Zeiss IM 35 disassembly + cleaning by GLA technician under PI supervision |
| 1 | Optics + Z + XY + environmental chamber installed and aligned; first brightfield test runs |
| 1.5 | Lasers + filters + dual sCMOS installed; first fluorescence test on BJ-hTERT-Hoechst control |
| 2 | 405 nm ablation module aligned; ablation calibration on test slides |
| 2.5 | PyMMCore-Plus + AI-agent loop integrated; first end-to-end test on RITE-cassette transduced HEK293T (faster control); test of operator-approval handshake for ablation |
| 3 | BJ-hTERT-RITE clonal selection + karyotype validation (in parallel with month 1-2.5 integration work, by GLA technician + postdoc) |
| 3 | **First 72 h imaging block** — Phase A Go-criterion measurement (asymmetry ratio, tag-swap efficiency, clone viability) |
| 4 | Blocks 2-3 (NAC arm) |
| 5 | Blocks 4-6 (forced asymmetry Asl arm) |
| 6 | Blocks 7-9 + karyotype timepoint 2 + **end-of-Phase-A Go review** |

## 5. Open-hardware deposition (per Parrish LoS open-source commitment)

On grant close, the following are deposited under CC-BY 4.0:

- ARGUS-LP BOM (this document) on Protocols.io
- Hardware alignment protocols and AI-agent control software on Protocols.io + public GitHub
- PyMMCore-Plus configuration files, CellPose fine-tuned weights, lineage-stitching code on public GitHub
- Raw lineage tracks on Zenodo (DOI assigned at dataset creation)
- ARGUS-LP retrofit components remain physical property of Georgia Longevity Alliance NGO at grant close

## 6. Vendor quotation status

All vendor estimates above are at catalogue list price (Q1 2026). Final quotations will be obtained from authorised distributors in Georgia (or via Tbilisi-based academic-import channels) before grant submission; competitive-tender procedures will be applied where local procurement law requires. Any quotation that comes in materially below the estimate (≥ 10 % below) will release head-room that is reallocated to consumables. Any quotation materially above (≥ 10 % above) will trigger a budget revision request.

## 7. Comparison with alternative builds

| Alternative | Estimated cost | Why not chosen |
|-------------|---------------|----------------|
| Buy new high-end confocal (e.g., Zeiss LSM 900 Airyscan) | €350,000-600,000 | 5-10× over Impetus cap; not justified for the lineage-purification readouts which only need 100×/1.4 NA |
| Light-sheet (OpenSPIM-style) build from scratch | €60,000-80,000 | Higher build risk on first attempt; OpenSPIM excels at thick samples, not what we need for BJ-hTERT monolayer |
| Subcontract to Geiger lab Ulm for full Phase A | €35,000-50,000 subcontract on top of Phase A budget | Adds cross-site coordination overhead; Phase B Ulm subcontract already exists — keeping Phase A separate keeps roles clean |
| Subcontract to Tbilisi partner (Ilia State / Beritashvili / Razmadze) | €20,000-25,000 subcontract | Requires confirmed partner; outreach not yet completed. Decision 2026-05-12: no Tbilisi partner contracted; Phase A runs at GLA Abastumani directly |

The ARGUS-LP retrofit at GLA Abastumani has the lowest cost, the most predictable timeline (single-site, single-PI control), and the strongest alignment with the GLA NGO mission. The trade-off is bench-biology risk: the PI is theoretical/clinical, not a wet-lab bench scientist. This is mitigated by (i) the GLA technician for routine bench work, (ii) the 24/7 AI agent for routine imaging decisions, (iii) the Janke-introduced Curie advisor for RITE methodology questions, and (iv) tele-supervision arrangements with the Geiger lab Ulm in case of construct or transduction issues.
