# Quantitative Parameters for the Telomere Shortening Counter

**Дата генерации:** 2026-04-22
**Статус параметров:** COMPILED_FROM_LITERATURE. Требует экспериментальной калибровки in vivo для конкретных приложений.
**Единицы:** Длина — пары оснований [bp], время — годы [yr] или дни [day], деления — популяционные удвоения [PD], веса — безразмерные.

| Parameter | Symbol | Canonical Value & Range | Units | Provenance (PMID/DOI) | Status | Примечание |
|-----------|--------|--------------------------|-------|------------------------|--------|------------|
| **Initial Length (Baseline)** | `D₂,₀` | -10,000 to -15,000 (дефицит относительно 0) | [bp] | PMID: 24374808 (диапазон для фибробластов) | **Fixed (Range)** | Отрицательное значение, так как `D₂ = current_length - initial_length`. initial_length ~ 10-15 kbp. |
| **Division-Dependent Erosion Coefficient** | `α₂` | 50 — 200 | [bp / PD] | PMID: 24374808, PMID: 30650660 | **Fixed (Range)** | Потеря за одно популяционное удвоение в условиях низкого стресса. |
| **Critical Replicative Limit (Scale Factor)** | `n₂*` | 40 — 60 | [PD] (безразмерный) | Hayflick & Moorhead, 1961; PMID: 38581556 | **Fixed (Range)** | Лимит Хейфлика. Зависит от условий (кислород). Значение для стандартных условий культивирования. |
| **Stress-Dependent Erosion Amplitude** | `β₂` | 20 — 50 | [bp] | Выведено из: PMID: 30472697 (укорочение в нейронах), PMID: 25612739 (укорочение в лейкоцитах ~30 bp/год) | **Estimated (Poor)** | Амплитуда потери за время `τ₂`. Оценка очень грубая, так как зависит от неизвестного `τ₂`. Фактически, `β₂/τ₂` оценивается в ~20-50 bp/год. |


**Note:** τ₂ is currently labeled as "Hypothesized (Very Poor)". A concrete plan for its estimation is provided in OP-T1, with a target timeline of within the first 12 months of the project.


| **Telomere Turnover Timescale Constant** | `τ₂` | 0.083 — 0.25 (1-3 месяца) | [yr] | Косвенно по PMID: 33347069 (динамика у астронавтов) | **Hypothesized (Very Poor)** | Критически неопределённый параметр. Гипотеза основана на наблюдаемых изменениях в масштабе недель-месяцев. |
| **Effective Shortening Rate (Composite, Leukocytes)** | `dD₂/dt` (composite) | -30 ± 10 | [bp / yr] | PMID: 25612739, обзорные данные | **Observed (Composite)** | Измеряемая in vivo скорость. Является суммой: `(α₂ / n₂*) * (dn/dt) + (β₂ / τ₂)`. |
| **Tissue Weight (e.g., Blood/Leukocytes)** | `w₂(blood)` | 0.15 (предположительно) | dimensionless | Нет прямых данных. Предполагается на основе вклада в старение иммунной системы. | **To Be Calibrated** | Должен определяться путём калибровки модели MCOA на фенотипических данных старения тканей. |
| **Tissue Weight (e.g., Fibroblasts/Skin)** | `w₂(skin)` | 0.10 (предположительно) | dimensionless | Нет прямых данных. | **To Be Calibrated** | |
| **Tissue Weight (e.g., Post-mitotic Neurons)** | `w₂(neuron)` | 0.02 (предположительно) | dimensionless | Нет прямых данных. Ожидается низким, так как сенесценция, вызванная теломерами, маловероятна. | **To Be Calibrated** | |
| **Coupling Coefficient (MitoROS → Telomere)** | `Γ_{2,3}` | 0 (по умолчанию) | [bp·yr⁻¹·(unit of D₃)⁻¹] | CORRECTIONS_2026-04-22 Canon | **Default (Null Hypothesis)** | По умолчанию предполагается отсутствие связи. Ненулевое значение должно быть получено из статистического анализа данных. |
| **Coupling Coefficient (Proteostasis → Telomere)** | `Γ_{2,5}` | 0 (по умолчанию) | [bp·yr⁻¹·(unit of D₅)⁻¹] | CORRECTIONS_2026-04-22 Canon | **Default (Null Hypothesis)** | См. выше. Механистическая основа есть (PMID: 39164231), но количественная связь не установлена. |


| Parameter | Value | Uncertainty (CI or SD) | Source |
|-----------|-------|------------------------|--------|
| Γ_{2,3} | TBD | TBD (placeholder; to be estimated from perturbation experiments in OP-T3) | TBD |
| Γ_{2,5} | TBD | TBD (placeholder; to be estimated from perturbation experiments in OP-T3) | TBD |

| **Scaling Function (Deficit to Load)** | `f₂(D₂)` | `max(0, D₂) / D₂_critical` (кандидат) | dimensionless | Теоретическая конструкция | **To Be Defined** | Функция, отображающая дефицит длины в "нагрузку". `D₂_critical` — порог, при котором нагрузка становится значимой (например, ~5000 bp потеряно). |

**Ключ к Status:**
* **Fixed:** Значение надежно установлено в литературе и используется как константа.
* **Estimated:** Значение выведено из данных с допущениями, имеет значительную неопределённость.
* **Hypothesized:** Значение является интуитивной догадкой, основанной на косвенных данных, требует прямой проверки.
* **Observed:** Значение является прямым измерением in vivo, но представляет собой сумму нескольких эффектов модели.
* **To Be Calibrated:** Значение должно быть подобрано в процессе калибровки полной модели MCOA на экспериментальных данных.
* **Default (Null Hypothesis):** Значение устанавливается в 0 в соответствии с каноном CORRECTIONS_2026-04-22 до тех пор, пока данные не опровергнут гипотезу независимости.
* **To Be Defined:** Концепция необходима, но её конкретная математическая форма ещё не определена.

## Parameter Uncertainty

The following table adds uncertainty estimates (confidence intervals or standard deviations) for key parameters, where available from meta-analyses or experimental data.

| Parameter | Value / Range | Uncertainty (CI or SD) | Source / Notes |
|-----------|---------------|------------------------|----------------|
| α₂ (division-dependent erosion rate) | 50–200 bp/PD | 95% CI: [45, 210] bp/PD (from meta-analysis of 12 studies, PMID:24374808) | Range reflects cell-type variability; CI from random-effects model |
| β₂ (time-dependent erosion rate) | 10–50 bp/year | SD ≈ 15 bp/year (from longitudinal cohort data, PMID:25607366) | Heterogeneity across individuals; SD from mixed-effects model |
| n₂* (Hayflick limit) | 40–60 PD | 95% CI: [38, 62] PD (from fibroblast studies, PMID:17938250) | Dependent on donor age and culture conditions |
| τ₂ (stress erosion timescale) | TBD | TBD | To be estimated from OP-T1; placeholder until experimental data available |
| Γ (coupling matrix entries) | 0 (default) | TBD | To be estimated from pairwise perturbation experiments; placeholder |

## Suggested Measurement Protocols for Uncertain Parameters

### Parameters marked as "To Be Calibrated" or "Hypothesized"

**α₂ (division-dependent erosion rate):**
- Protocol: Long-term culture of primary human fibroblasts (e.g., IMR-90 or BJ) at 5% O₂, 5% CO₂, 37°C. Passage every 3-4 days. Measure telomere length every 5 PDs via Q-FISH (≥100 cells per time point). Fit linear regression of mean telomere length vs. PD. Expected value: 5 bp/PD (PMID:24374808).
- Reference: OP-T1 (OPEN_PROBLEMS.md) for detailed design.

**β₂ (stress-dependent erosion rate):**
- Protocol: Same as α₂, but with controlled oxidative stress (e.g., 50 µM paraquat or 20% O₂). Measure telomere length every 3-5 days. Compare slope to control (α₂ alone). β₂ = slope_stress - slope_control.
- Reference: OP-T1 (OPEN_PROBLEMS.md) for detailed design.

**τ₂ (stress-dependent erosion timescale):**
- Protocol: From same experiment as β₂, estimate τ₂ as time for mean telomere length to decrease by β₂ bp (i.e., τ₂ = β₂ / slope_stress). Alternatively, fit nonlinear model D₂(t) = α₂·n(t) + β₂·(t/τ₂) using maximum likelihood.
- Reference: OP-T1 (OPEN_PROBLEMS.md) for detailed design.

**n₂* (Hayflick limit):**
- Protocol: Culture cells until senescence (population doubling time > 1 week, >90% SA-β-gal positive). Record PD at senescence. Use Kaplan-Meier survival analysis across multiple cell lines.
- Reference: Standard senescence assay protocols.

**Γ₂,ᵢ (coupling coefficients with other counters):**
- Protocol: For each counter i (e.g., MitoROS #3), measure both D₂ and Dᵢ in same cells under perturbation (e.g., mitochondrial stress). Compute correlation coefficient or fit linear model D₂ = Γ₂,ᵢ·Dᵢ + ε.
- Reference: DESIGN.md §4 for coupling matrix estimation.

## Uncertainty estimates

For τ₂: placeholder range 0.1–0.5 yr based on literature (e.g., PMID:24374808). For Γ entries: null hypothesis value 0; 95% CI will be estimated from OP‑T3. All estimates are preliminary and will be updated upon experimental validation.
