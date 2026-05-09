# MCOA 6×6 Comparison — Calibrated Counters

**Date:** 2026-04-21
**Horizon:** 36,500 days (100 years)
**Division rate:** 0.01 / day
**Output:** `matrix_rms_delta.csv` (91 rows = 6 tissues × 15 pairs + self)

## Calibration source

Each counter's `α_i`, `β_i`, `n_i*`, `τ_i` fit by least-squares against
literature-anchored points from meta-analyses (each anchor cites a
verified PubMed PMID; details in `<Subproject>/scripts/calibrate.py`
and `<Subproject>/PARAMETERS_calibrated.json`).

Calibrated parameters:

| Counter | # | α | β | γ | n* | τ (days) | d_crit | PMIDs |
|---------|---|------:|------:|--:|------:|---------:|-------:|--------|
| CDATA (Centriolar) | 1 | — | — | — | — | — | — | pending CLI wrapper |
| Telomere | 2 | 0.55 | 0.00 | 0.0 | 50 | 32,850 | 0.55 | 2342578, 12855956 |
| MitoROS | 3 | 0.00 | 0.50 | 0.0 | 100 | 29,200 | 0.50 | 26942670, 23965628 |
| EpigeneticDrift | 4 | 0.00 | 1.00 | 0.0 | 100 | 36,500 | 0.75 | 24138928, 35029144 |
| Proteostasis | 5 | 0.28 | 0.28 | 0.0 | 80 | 29,200 | 0.60 | 29127110, 30733602, 34563704 |

## Average Δ (RMS over 100-yr trajectory) per counter pair

| Counter i | Counter j | mean Δ | std | min | max |
|-----------|-----------|------:|----:|----:|----:|
| Telomere | MitoROS | 1.729 | 1.106 | 0.425 | 3.189 |
| Telomere | EpigeneticDrift | 1.700 | 0.871 | 0.750 | 3.015 |
| Telomere | Proteostasis | 1.186 | 0.805 | 0.223 | 2.211 |
| MitoROS | Proteostasis | 0.543 | 0.302 | 0.202 | 0.977 |
| EpigeneticDrift | Proteostasis | 0.514 | 0.162 | 0.361 | 0.804 |
| MitoROS | EpigeneticDrift | 0.245 | 0.064 | 0.173 | 0.325 |

## Physical interpretation

1. **Telomere vs all others (Δ 1.2-1.7):** Telomere is purely
 division-dominant (α=0.55, β=0), others are time-dominant or mixed.
 Across a 100-yr trajectory, divisions dominate Telomere output
 while negligibly affecting MitoROS/EpigeneticDrift. Signature of
 *counter-type separation* — exactly what MCOA Axiom M1 predicts:
 counters carry independent information.

2. **MitoROS vs EpigeneticDrift (Δ 0.25, smallest):** Both pure
 time-dominant (β-only), different β magnitude (0.5 vs 1.0) and τ.
 Trajectory shapes identical up to scaling — Δ reflects magnitude
 only, not shape. This pair is **information-redundant** for
 age-prediction and should be combined into a single weighted
 counter in clinical applications.

3. **Proteostasis: intermediate.** Mixed α=β=0.28 blends Proteostasis
 between families. Close to time-dominant pair (Δ ≈ 0.5), still
 far from Telomere (Δ ≈ 1.2). Expected MCOA behaviour for a counter
 coupled to both division and time.

4. **Tissue variance:** Telomere max Δ (3.0-3.2 in Neuron/
 Cardiomyocyte) reflects its post-mitotic α-scaling (α×0.05 there)
 — the counter is *effectively inert* in post-mitotic tissues.
 Contrast with high-turnover Intestinal Crypt (Telomere min Δ).
 Expected tissue-specific behaviour (MCOA Axiom M3).

## Validation against MCOA axioms

- **M1 parallel counters** ✅ Non-zero Δ for every cross-pair.
- **M2 dimensional consistency** ✅ D_i bounded in [0, d_critical].
- **M3 a-priori tissue weighting** ✅ Post-mitotic vs proliferative
 contrast recovered.
- **M4 falsifiability** partial — each counter testable against cited
 PMIDs; real data integration in Phase 8 pending.

## Known limitations

1. **CDATA pending CLI wrapper** — cell-dt-sim binary to add.
2. **No Γ coupling** — all γ=0 here; Γ_ij measurement via ~~MCOA Test 2~~ [отозвано — see CORRECTIONS §1.3]
 requires empirical data.
3. **Literature-anchor calibration, not data-fit.** Parameters shift
 when GEO integrated.
4. **6-tissue panel coarse** — expand to ESCs, iPSC-derived,
 endothelial, osteoblasts per Nature Aging paper.

## Next steps

- [ ] Add CDATA CLI `cell-dt-sim` matching --tissue/--days/--rate
- [ ] Integrate one real GEO per counter (EpigeneticDrift first —
 GSE40279 Horvath training set)
- [ ] Measure Γ_1,2 (centriolar × telomere) from published data
- [ ] Phoenix LiveView dashboard importing matrix_rms_delta.csv
- [ ] CI check: every Rust crate PR triggers compare_all.py run
