# Review of AIM

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsif: 3
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

### 1. Operationalised falsifiability (numeric thresholds)
**✓** — ПАМ-13 Δ ≥ 5.4; α = 0.05 (two-sided); power = 0.80; N ≥ 55 per group; stopping rule; interim analysis.  
**Однако:** В CONCEPT.md есть блок «α = 0.05, Bonferroni-corrected for ≤2 comparisons → p < 0.025 per comparison», что не согласуется с THEORY.md («α = 0.05 for primary; Bonferroni only for secondary endpoints»). Создаёт путаницу, но числовые пороги в целом присутствуют. Условие формально выполнено.

### 2. Pre-registration plan (OSF placeholder + date)
**✓** — OSF ID `osf.io/XXXXX` (placeholder); planned registration 2026-09-01; design, outcome, analysis plan описаны. Выполнено.

### 3. Sample size calc (power analysis)
**✓** — Формула: n = (Z_α/2 + Z_β)²·2σ²/δ²; подстановка: (1.96+0.84)²·2·10²/5.4² ≈ 55/group; total N=132 (20% dropout). Sensitivity analysis для σ=8,10,12. Выполнено.

### 4. Risk matrix ≥5 rows
**✓** — В CONCEPT.md три разных матрицы, каждая содержит