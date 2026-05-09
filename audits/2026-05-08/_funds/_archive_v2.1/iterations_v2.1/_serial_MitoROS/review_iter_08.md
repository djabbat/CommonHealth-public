# Review of MitoROS

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 4
- Falsif: 4
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 4

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

### 1. Operationalised falsifiability (numeric thresholds) – ✗
Числовые пороги присутствуют (D₃ < 0.01, slope > 0.05, α=0.05, power=0.80, R²=0.9, d=0.8 и т.д.) как в CONCEPT.md, так и в OPEN_PROBLEMS.md. Однако отсутствует единая таблица фальсификационных порогов для всех параметров модели (α₃, β₃, τ₃, γ₃, λ_het, λ_les). Для τ₃ порог не задан (только оценка ±0.1). Для γ₃ порог по умолчанию 0, но не указано, при каком отклонении от нуля гипотеза независимости считается опровергнутой. Требуется полная спецификация.

### 2. Pre-registration plan (OSF placeholder + date) – ✓
Присутствует: `osf.io/mitocounter3_pr20260701`, дата 2026-07-01. Описаны первичные и вторичные гипотезы, план анализа, критерии исключения. Выполнено.

### 3. Sample size calc (power analysis) – ✗
Присутствует формула, примеры для P0-1, P0-2, P1-1, P2-1, Unified Power Analysis Table. **Проблема:** в таблице для P2-1 указано `N=TBD, test TBD`. Это недопустимо – sample size должен быть конкретным или хотя бы оценённым. Кроме того, в разделе Sample size calculation (CONCEPT.md) σ² = TBD, δ = TBD. Не выполнено.

### 4. Risk matrix ≥5 rows – ✓
Риск-матрица содержит 6 строк (R1–R6) с оценками