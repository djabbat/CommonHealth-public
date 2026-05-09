# Ze Vectors Theory — PARAMETERS

Key numerical constants and formulas of the Ze framework.

---

## Core Constants

> **Convention note (2026-05-07):** root `~/Desktop/LongevityCommon/PARAMETERS.md § 1`
> defines **Article form** as canonical for cross-subproject API and
> manuscripts (`v*_active = −0.08738`). Values below are stated in
> **Python (internal) form** `[0, 1]` because Ze theorem proofs and
> theorem-related code use the Python normalization. Conversion:
> `Article = 2 · Python − 1`.

| Symbol | Value (Python, internal) | Article equivalent | Derivation | Meaning |
|--------|--------------------------|-------------------:|------------|---------|
| **v*_passive** | `1 − ln 2 ≈ 0.3069` | `−0.3862` | Entropy maximization of binary stream | Theorem-derived passive optimum |
| **Z*** | `1/(1+e⁻¹) ≈ 0.7311` | n/a (different scale) | Logistic fixed point | Optimal Ze index |
| **v*_health** | `≈ 0.35–0.45` | `[−0.30, −0.10]` | Clinical HRV data | Healthy heart Ze velocity range |
| **v*_active (approx)** | `≈ 0.456` | `≈ −0.087` | Cuban pilot, see `CONCEPT § 8` | Empirical health threshold (active agent) |

Note: `v*_passive = 1 − ln 2 ≈ 0.3069` (theorem) is **distinct** from
`v*_active ≈ 0.456` (empirical Cuban pilot, Python form). The latter
appears in BioSense χ_Ze biomarker; root PARAMETERS § 1 names it as
`v*_active = −0.08738` in Article form.

---

## Parameter Formulas

```
Ze velocity: v = (N_T − N_S) / (N_T + N_S) ∈ [−1, +1]
Ze index: Z = N_T / N ∈ [0, 1]
Ze complexity: τ = H(stream) / log₂(N) ∈ [0, 1]
Ze variability: χ = (max − min) / mean
Ze impedance: ζ = τ / v
```

---

## Stream Encoding

Given measurements x₁, x₂, ..., xₙ:
```
zᵢ = T if xᵢ > xᵢ₋₁
zᵢ = S if xᵢ ≤ xᵢ₋₁
```

---

## Spacetime Emergence

```
ds² = −c² dτ_Ze² + dx²
```
where τ_Ze is the Ze time parameter from the counting process.

Time dilation: `Δt' = Δt / √(1 − v²/c²)`
(derived from Ze counter slowing in moving frame — without Lorentz postulate)

---

## Ze Impedance at Phase Transitions

| System state | ζ = τ/v | Interpretation |
|-------------|---------|----------------|
| Ordered / stable | high ζ | Resists perturbation |
| Chaotic / plastic | low ζ | Easily perturbed |
| Superconductor (T → Tc) | ζ → ∞ | Prediction: falsifiable |

---

## Ze Market Signals

| Signal | Condition | Action |
|--------|-----------|--------|
| Equilibrium | v → v* | Hold |
| Trend | v < v* | Buy signal |
| Bubble | v > v* | Sell signal |
| Crisis precursor | τ drops sharply (S-burst) | Alert |

---

## Ze DNA Predictions

| Region | Predicted v |
|--------|------------|
| Exons (coding) | ≈ 0.30 |
| Introns (non-coding) | ≈ 0.45 |
| Telomeres | ≈ 0.25 |

---

## Ze HRV Clinical Thresholds

| Clinical state | v range | Interpretation |
|----------------|---------|----------------|
| Healthy | 0.35–0.45 | Balanced ANS |
| Stress | v → 0 or v → 1 | T or S dominance |
| Arrhythmia | unstable v | Irregular pacemaker |
| Bradyarrhythmia | HR < 60 + low v | Vagal dominance |
| Tachyarrhythmia | HR > 100 + high v | Sympathetic storm |
| Seizure precursor | v → 0 (T-burst) | 2–10 min warning |

---

## Ze Randomness Test (PRNG Quality)

Ideal RNG: v ≈ 0.5 (stable), τ growing linearly.
Threshold: |v − 0.5| > 0.05 within 10⁴ samples → suspect PRNG.

---

*Last updated: 2026-03-28*
