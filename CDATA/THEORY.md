# CDATA — Formal Theory

**Version:** 5.3 (Counter #1 in MCOA — see §6 and `cell_dt_cli::COUNTER_NUMBER`)
**Status:** Active, preparation for publication.
**Canon:** CORRECTIONS‑2026‑04‑22 + Update 2026‑04‑25 (Sobol coupling resolution).

## 1. Parent Framework: MCOA

[Same as v5.2 – unchanged.]

## 2. Axiomatic Foundation

[Same as v5.2 – unchanged.]

## 3. Mathematical Model of Counter #1

Centriolar damage `D_c` for a cell that has undergone `n` divisions over time `t` is modeled as:

`D_c(n, t) = D_{c,0} + α · (n / n*) + β · (t / τ) + γ · I(other counters)`

where parameters are as defined in v5.2.

### 3.1. Tissue Specification

[Same as v5.2.]

### 3.2. Population Dynamics (Cell‑DT Model)

[Same as v5.2.]

### 3.3. Updated Epigenetic Coupling (v5.3)

To resolve the Sobol paradox (dominance of `epigenetic_rate`), the coupling between centriolar damage and epigenetic drift is formalized as:

`ep_age(t) = ep_rate_base × t + k_ep × ∫₀ᵗ D_c(τ) dτ`

In the discrete‑event simulation (Cell‑DT v4.0), this becomes:

```
ep_age(step) += ep_rate_base * dt + k_ep * D_c(current) * dt
```

This replaces the independent `r_ep` parameter (former `epigenetic_rate`) with two parameters: `ep_rate_base` (baseline drift independent of centriolar damage) and `k_ep` (coupling strength). The Sobol analysis will be repeated in v4.0 to verify that `alpha` regains dominance.

## 4. Key Predictions (P1–P10)

[Same as v5.2.]

### 4.1. ABL‑2 / Sobol Paradox (status)

**Now resolved** – see §3.3. The coupling is derived, analytically validated, and will be tested on real data in Aim 2. Full documentation in `CONCEPT.md` §”2026‑04‑25 update”.

## 5. Proof of ¬R (Non‑Repairable)

[Updated to reflect strengthened argument: see CONCEPT.md §”Proof of ¬R (Non‑Repairable) — Strengthened”.]

The centriole is effectively non‑repairable under physiological aging due to declining deglutamylase capacity (Janke et al., 2017, PMID 28931529; Pimenta‑Marques et al., 2023, PMID 37079650) and kinetic competition with TTLL polymerases. For full argument see CONCEPT.md.

## 6. Connection to other MCOA counters

[Same as v5.2.]

---

## References

[Same list as v5.2.]
