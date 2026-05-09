# LongevityCommon · THEORY (umbrella view)

**Status:** Canonical · regenerated 2026-04-28 from CONCEPT v5.6 + article §3-§4
**Authority:** Cross-cutting math summary; полные derivations — в `<subproject>/THEORY.md`

---

## §1. Notation (cross-subproject)

| Symbol | Meaning | Defined in |
|--------|---------|------------|
| `L_tissue(n,t)` | Tissue-level aging burden, MCOA aggregator | `MCOA/THEORY.md` |
| `w_i(tissue)` | Tissue-specific weight for counter `i` | `MCOA/THEORY.md` |
| `f_i(D_i(n,t))` | Counter-specific function on damage state `D_i` | `MCOA/THEORY.md` |
| `D(t)` | Centriolar damage (CDATA hypothetical counter) | `CDATA/THEORY.md` |
| `I(Z)` | Impedance / KL-divergence between actual and modeled state | `Ze/THEORY.md` |
| `τ_Ze` | Proper-time budget | `Ze/THEORY.md` |
| `α, β, λ` | Coupling constants (Ze) | `Ze/THEORY.md` |
| `v` | Ze velocity (binary symbol stream) | `BioSense/THEORY.md` (two conventions: Python ∈ [0,1], Article ∈ [-1,+1]) |
| `v*` | Theoretical fixed point ≈ 0.45631 (Python convention; ≈ -0.087 Article) | `BioSense/THEORY.md` §3 |
| `χ_Ze` | Composite biomarker ∈ [0,1] | `BioSense/THEORY.md` §3.4 |
| `(ε, δ, k, σ, q, T)` | DP + k-anon + DP-SGD parameters | `FCLC/THEORY.md` + `BioSense/THEORY.md` §6 |

---

## §2. Cross-level connections

```
MCOA L_tissue = Σᵢ wᵢ · fᵢ(Dᵢ)
 ↑
 один из Dᵢ — CDATA (hypothetical)

CDATA A(t) = a + b·D(t) + c·D(t)² (status: inconclusive)
 χ_Ze = g₀ − g₁·A(t) (linear bridge, 5 free params, underpowered)

Ze I(Z) = S(Z_real ‖ Z_model) (KL divergence)
 dτ_Ze/dt = −α·I(Z) (POSTULATED ansatz)
 E_Ze(a,b) = −a·b + δ·[(a·b)²−1/3] (CHSH deformation)

BioSense F = E − T·S − λ·I_pred (variational principle)
 v* = 0.45631 (k_λ = 1) (theoretical fixed point)
 χ_Ze = 1 − |v − v*| / max(v*, 1−v*) (per-modality)
 composite = Σ w_modality · χ_Ze (4 modalities EEG/HRV/resp/sleep)

FCLC ε_total ≈ 0.43 at (σ=1.5, q=0.013, T=5) (RDP composition)
 Krum aggregator (Byzantine ≤ 25%)
 threat model: semi-honest server, NOT active
```

## §3. Authoritative derivations

Полные derivations — в подпроектах:

| Lemma | Where | Status |
|-------|-------|--------|
| Burgholzer information-entropy equality `I = ⟨ΔS⟩_gen` | `Ze/THEORY.md §2` | Theorem (Burgholzer 2015 arXiv:1502.00214) |
| Quadratic CHSH deformation derivation | `Ze/THEORY.md §3` | Lemma B + C; `1.7478` constant verified by F3 test |
| Correlation decay `C(τ) = C₀·exp(−β·I·τ)` | `Ze/THEORY.md §4` | Lemma D (Gassner 2021) |
| QFI bound `F_Q ≥ 8C₀(βIτ)²(1−βIτ)` | `Ze/THEORY.md §5` | Lemma E (Abboud 2026) |
| Variational principle `F = E − T·S − λ·I_pred` | `BioSense/THEORY.md §3.1` | Friston 2010 framework |
| `λ` from thermodynamics | `BioSense/THEORY.md §3.2` | Lemma C (Wallace 2015 inferential argument) |
| `v* = 0.45631` fixed point | `BioSense/THEORY.md §3.3` | Theorem 1 (numerical extremum at `k_λ=1`) |
| CDATA bridge `A(D)`, `χ(A)` | `BioSense/THEORY.md §4` | Lemma D (linear, 5 params, underpowered) |
| RDP composition for DP-SGD | `FCLC/THEORY.md` | Mironov 2017 + Wang/Balle/Kasiviswanathan 2019 |

## §4. Что **не** теорема, а ansatz / hypothesis

(после v5 honest relabel)

| Claim | Old framing | New framing |
|-------|-------------|-------------|
| `dτ_Ze/dt = −α·I(Z)` | "derived from Burgholzer/Pearson" | **POSTULATED ansatz** by analogy с physical clocks; биология не валидирована |
| Bridge between `D(t)` (CDATA) and `χ_Ze` | "mechanistically anchored" | **5 free params** на N=196 underpowered; moved to Supplementary |
| `χ_Ze` predicts mortality | confirmatory | exploratory hypothesis-generating only; pre-registered N≥500 NOT yet run |
| Multimodal weights `(0.30, 0.30, 0.20, 0.20)` | "theoretically motivated" | **post-hoc** pilot fit; not theory-fixed |
| CDATA "Counter #1 in MCOA" | confident | status **inconclusive**; Sobol nested CV deferred to Cell-DT v4.0 |

## §5. Falsifiability (operational)

- **MCOA M4** (article §3.1): falsified if на pre-registered cohort `N ≥ 2000`, `α = 0.001`, partial r² для all-cause mortality (controlling age, sex) `< 0.05` для каждого counter. Power analysis: N=1875 для R²=0.3 at 80% power.
- **CDATA**: falsified if полная Sobol decomposition (S1+S2+ST) с nested CV на real GTEx-like data показывает что α-component не contributes значимо. Текущий Sobol на synthetic data: ABL-2 paradox (R²_no_α=0.833 vs full=0.778), но difference NOT significant (p=0.12 после correction).
- **Ze fixed point v***: falsified if swept-v* search на All-of-Us N≥500 показывает `v*_optimal` за пределами `[0.32, 0.58]` (sensitivity range для `k_λ ∈ [0.5, 2.0]`). **Test status:** done на N=500, `v*_optimal = 0.451 (95% CI 0.443-0.459)` — consistent с theory.
- **FCLC**: falsified as GDPR-compliant infrastructure if active server attack succeeds. Текущий статус: semi-honest secure only; известный блокер; v14 planned Q1 2027.

## §6. Связь с реализацией

- Все 6 канонических Ze quantities → `Ze/biosense-simulator/` (Rust)
- Все 5 канонических BioSense computations → `BioSense/biosense-simulator/` (Rust)
- FCLC RDP composition + Krum aggregator → `FCLC/fclc-core/src/dp/` + `aggregation/` (Rust, server-resident)
- MCOA aggregator → `MCOA/CellDT_v4/` (planned, не реализован полностью)
- CDATA bridge fitting → out of canonical simulator surface; Python prototype в `_archive/`
