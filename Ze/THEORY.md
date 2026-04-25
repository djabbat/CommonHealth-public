# THEORY — Ze

**Канон:** `Ze Theory.pdf` + `Ze Теория.pdf`. Этот файл — навигатор по формальному ядру книги.

## 1. Source of truth

Полная теория — в книгах (24 главы + 4 приложения). НЕ дублировать здесь — только указывать главы.

## 2. Канонические величины

| Символ | Определение | Глава |
|---|---|---|
| `Z` | множество различимых состояний | 1 |
| `γ(τ)` | трасса вселенной | 1.4 |
| `I(Z) = S(Z_real ‖ Z_model)` | энтропический импеданс (KL-дивергенция) | 2 |
| `t = ∫ I(τ') dτ'` | физическое время | 3 |
| `d(A,B) = min_γ ∫ I dt` | метрика реальности (не риманова) | 4.1 |
| `K = −I` | знание = негэнтропия | 5.5 |
| `C = −dI/dt` | сознание = скорость уменьшения ошибки | 5.7, 12.1 |
| `Φ_Ze = ∮ I dt` | интегральная мера сознания | 12.2 |
| `E_Ze(a,b) = −a·b + δ·[(a·b)² − 1/3]` | Ze-деформация Bell | 7.2 |

## 3. Иерархия порождения (гл. 5)

```
Universe → Dark Energy → Energy → Knowledge → Time → Being
  γ(t)      dim(Z)      ‖dZ/dt‖   −I         ∫I dτ   local optimizer of I
```

## 4. Ключевые предсказания

| # | Тест | Глава |
|---|---|---|
| T1 | `S(H) = 2.828·(1−2αH)`, α≈0.03 | 8.4, 19 |
| T2 | `γ_Ze = −0.031 ± 0.007` (Planck 2018 P(k)) | 10.4 |
| T3 | H₀ = 69.2 ± 1.8 (5σ→2.1σ tension) | 10.5 |
| T4 | S₈ = 0.798 ± 0.010 (3σ→1.4σ) | 10.5 |
| T5-T7 | EEG `Φ_Ze` correlates | 21 |
| T8 | shift-опт CHSH `δ·1.7478 ≈ 0.085` | 7.4, 19 |

Подробно — `OPEN_PROBLEMS.md §1` Validation gaps.

## 5. 5 generations principle (гл. 5.1-5.7)

- **Universe:** trajectory γ(τ) в Z
- **Dark Energy:** dim(Z) ∝ t_D — рост размерности state space
- **Energy:** E ~ ‖dZ/dt‖ — rate of state change
- **Knowledge:** K = −I = −S(Z_real ‖ Z_model) — negative entropy
- **Time:** t = ∫ I(τ') dτ' — integrated prediction error
- **Being:** local optimizer of I (learn or cheat)

Feedback loops (гл. 5.8): Being→time, Knowledge→energy, Being→dark energy.

## 6. Quantum mechanics через Ze (гл. 6-8)

- **QM as projection** (гл. 6): `P(1→2) = e^(−S(Z_1‖Z_2)) / Σ e^(...)` (Gibbs rule)
- **Schrödinger** = limit для small I (гл. 6.2)
- **|ψ|²** = density of entropic metric, not probability of finding particle (гл. 6.3)
- **Collapse** = local update of observer's model (гл. 6.4)
- **Bell deformation** (гл. 7): no-signaling preserved, shift-опт углы дают `δ·1.7478`
- **Quantum damping** (гл. 8): `Ẑ = e^(−βÎ⊗Î)`, prediction `S(H) = 2.828·(1−2αH)`

## 7. Cosmology (гл. 9-11)

- Action: `S = ∫d⁴x √(−g)[R/(16πG) + ½∇_μI∇^μI + V(I) + L_mat]`
- Cosmological eq: `Ï + 3HÏ + m_Ze²I = 3(ä/a)/Λ_Ze`
- Bounce instead of singularity при I→I_max (гл. 10.1)
- Quantization of impedance, NOT metric (гл. 11)

## 8. Consciousness (гл. 12-15)

- `C = −d/dt S(Z_real ‖ Z_model)` — rate of error reduction
- `Φ_Ze ≈ 0.1` критическая фаза для сознания (EEG calibration, Tononi 2015)
- Critical slowing down + long-range correlations + symmetry breaking при I→I_crit

## 9. Что не в этом файле

- Полные доказательства теорем — гл. 1-24 + Appendix A
- Численные параметры симулятора — `PARAMETERS.md`
- Архитектура кода — `DESIGN.md`
- Литература — `EVIDENCE.md`
- Лимитации — `OPEN_PROBLEMS.md`

---

**Правило:** при изменении в книге (новая редакция) — обновить здесь номера глав и формулы. Источник истины ВСЕГДА — последняя версия PDF.
