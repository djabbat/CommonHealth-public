# Review of CytogeneticTree

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4 (оригинальная и важная идея)
- Method: 3 (RITE-центриольная метка — главный технический риск; не проверен)
- Evidence: 3 (литературная поддержка сильная, но прямая экспериментальная валидация отсутствует)
- Falsif: 2 (пороги есть, но множественные противоречивые версии до унификации)
- Deliv: 2 (6-месячное прижизненное наблюдение, невалидированный конструкт, не реализованный алгоритм)
- Novelty: 5 (беспрецедентное сочетание RITE + центриольного возраста + полного дерева)
- Risk: 2 (высокий технический риск)

## Checklist (✓/✗ each + explanation)
1. **✓** Operationalised falsifiability (numeric thresholds) — есть, хотя с противоречиями (N=43/24/30). Унифицированная таблица с α=0.001, power=0.95, d=1.2, N=43 принята как окончательная, но множественные ранние версии снижают доверие.
2. **✓** Pre-registration plan (OSF placeholder + date) — OSF ID osf.io/TBD, дата 2026-07-01. Плейсхолдер допустим.
3. **✓** Sample size calc (power analysis) — есть несколько расчётов (N=24, N=43). Последний с α=0.001, power=0.95, d=1.2 → N=43. Расчёт приведён.
4. **✓** Risk matrix ≥5 rows — 7 строк, каждая с probability, impact, mitigation.
5. **✓** Limitations section — явный раздел, 6 пунктов.
6. **✓** Consortium / collaboration plan — таблица ролей, часть подтверждена (Ilya Zheleznov), часть TBD. Допустимо.
7. **✗** References PubMed/Crossref-verified — KNOWLEDGE.md утверждает, что все PMID верифицированы, но одна ссылка (Lee & Luo 1999 Neuron) помечена как «REFERENCE VERIFICATION PENDING» и не помечена как preprint. Другая ссылка удалена (PMID_REMOVED). Таким образом, не все ссылки верифицированы или явно обозначены как препринты.
8. **✗** No fabrication markers — в тексте присутствует явная аннотация «This reference has been removed pending verification» (маркер [PMID_REMOVED]). Это запрещённый маркер фабрикации. Также есть «FABRICATION CLEANUP applied».

## Top 5 text-level fixes
- `CONCEPT.md:Limitations` — удалить или заменить все неверифицированные ссылки; ссылки Lee & Luo 1999 и удалённая ссылка должны быть либо верифицированы, либо заменены на верифицированные препринты, либо удалены. 
- `CONCEPT.md` — устранить противоречия в численных порогах: унифицировать N, α, power, effect size во всех разделах (Falsifiability, Sample size, Pre-registration). В текущей версии есть N=24, N=43, α=0.001 и α=0.05, power=0.95 и 0.80.
- `CONCEPT.md:Risk matrix` — добавить риск, связанный с неверифицированными партнёрами: вероятность дезертирства placeholder-партнёров высока; необходимы backup.
- `KNOWLEDGE.md` — явно пометить Lee & Luo 1999 как препринт, если он не прошёл верификацию; иначе удалить. Удалить все маркеры «removed reference».
- `PARAMETERS.md` — согласовать статистические параметры с окончательными значениями из CONCEPT.md (α=0.001, power=0.95, N=43). Сейчас в PARAMETERS.md α=0.05, β=0.20 — несоответствие.

## PACKET
# CytogeneticTree


=== CytogeneticTree/CONCEPT.md ===
# CytogeneticTree — Cytogenetic Tree of Differentiation

**Version:** v1.0 (initial scaffolding)
**Date:** 2026-04-21
**Status:** 🟡 Active (new LongevityCommon subproject)
**Parent umbrella:** `~/Desktop/LongevityCommon/` (coordinator of CDATA, FCLC, Ze, BioSense, MCOA, HAP, Ontogenesis, AutomatedMicroscopy, and now CytogeneticTree)

---

## §1 Central Vision

**Reconstruct the complete genealogical tree of cellular differentiation — from zygote to terminally-differentiated cells — by tracking centriole age across every cell division.**

The centriole is the one cellular organelle that is **structurally heritable, asymmetric, and long-lived**:

- **Heritable:** each daughter cell inherits centrioles from the mother; never synthesized *de novo* in somatic cells (except rare *de novo* biogenesis in early embryos)
- **Asymmetric:** mother and daughter centriole differ morphologically (sub-distal appendages, distal appendages). In many stem-cell divisions the OLDER mother centriole goes to the self-renewing daughter (Yamashita 2007, Wang 2009, Royall 2023)
- **Long-lived:** centriole proteins are not continuously exchanged; polyglutamylation and other post-translational marks accumulate monotonically across divisions

These three properties make the centriole the **ideal physical marker of cell age in a lineage**. If we can mark centriole age (e.g., via RITE pulse-chase fluorescent tagging) and follow every asymmetric division in a population, we can reconstruct the **complete genealogical DAG** of cellular differentiation.

Such a tree would reveal:

1. **Which lineage branches accumulate centriole damage** (aging trajectories)
2. **Which asymmetric divisions conserve youth** vs squander it (regenerative vs senescent fates)
3. **Where differentiation commitment happens** in terms of centriole inheritance
4. **The full DAG from zygote to terminally-differentiated somatic cell**
5. **Empirical validation of CDATA** — do "old-centriole lineages" proliferate less, as predicted?

---

## §2 Scientific Framing


## Pre-registration plan

**OSF ID:** osf.io/TBD
**Planned registration date:** 2026-07-01

**Pre-registration content:**
- Primary endpoint: proliferation rate (log2 fold change) in old-centriole vs young-centriole lineages
- Secondary endpoints: proportion of asymmetric divisions, senescence marker intensity
- Binding falsifiability thresholds (see §Falsifiability)
- Statistical analysis plan: two-sample t-test, two-tailed, α=0.001, power=0.95
- Exclusion criteria: cell death, failed RITE labelling, technical artefacts
- Planned sample size: N=43 per arm (total N=86 lineages)

**Amendments:** Any changes to the above will be documented as a versioned amendment on OSF prior to data analysis.

**Linking note:** The pre-registration will contain the **unified falsifiability table** (N=43 per arm, α=0.001, power=0.95) and the **single sample size calculation** from this document. No deviations from these thresholds are allowed without a documented amendment on OSF.


## Sample size calculation

**Single, binding calculation for primary endpoint (proliferation rate, log2 fold change, old-centriole vs young-centriole lineages).**

**Test:** Two-sample t-test, two-tailed.

**Formula:** n = (Z_α/2 + Z_β)² · 2σ² / δ²

**Parameters:**
- α = 0.001 (two-tailed), Z_α/2 = 3.29
- Power = 0.95, β = 0.05, Z_β = 1.645
- Effect size δ = 1.2σ (Cohen's d = 1.2)
- σ² = pooled variance (TBD from pilot)

**Computation:**
- n = (3.29 + 1.645)² · 2σ² / (1.2σ)² = (4.935)² · 2 / 1.44 = 24.36 · 1.389 = 33.8
- Rounded up: n = 34 per arm
- Attrition (20%): n = 34 / 0.8 = 42.5 → **43 per arm**

**Software:** G*Power v3.1.9.7 (or pwr package in R).

**Consistency note:** This calculation (N=43 per arm) supersedes all earlier draft values (N=6, N=10, N=15, N=24, N=48). The falsifiability table is updated to match this single calculation. calculation here is the definitive one for the primary endpoint. The falsifiability table's N=24 is a conservative placeholder that will be updated to match this calculation once pilot data are available. All other hypotheses (H2, H3) will use the same N per arm as the primary endpoint.



- **OSF ID:** osf.io/TBD
- **Planned registration date:** 2026-07-01
- **Content:** The full analysis plan, including the unified falsifiability thresholds (above), sample size calculation, primary and secondary endpoints, and the go/no-go decision criteria (see Limitations section). No deviations from the pre-registered plan will be made without a transparent amendment.




## Falsifiability

**Single, binding numeric thresholds (consistent with sample size calculation).**

| Hypothesis | Null (H₀) | Test | α | Power | Effect size | N per arm | Decision rule |
|------------|-----------|------|----|-------|-------------|-----------|---------------|
| H1: Old-centriole lineages proliferate slower | No difference in proliferation rate | Two-sample t-test, two-tailed | 0.001 | 0.95 | d = 1.2 | 43 | Reject H₀ if p < 0.001 |
| H2: Old-centriole lineages senesce earlier | No difference in senescence onset | Log-rank test | 0.001 | 0.90 | HR = 2.0 | 43 | Reject H₀ if p < 0.001 |
| H3: Centriole age correlates with differentiation depth | No correlation | Spearman rank test | 0.001 | 0.90 | ρ = 0.5 | 43 | Reject H₀ if p < 0.001 |

**Note:** All N per arm = 43, matching the single binding sample size calculation. No other thresholds are valid.

All hypotheses are operationalised with numeric thresholds. This section is the single, binding threshold block for the entire proposal. All earlier draft calculations (including any N=6, N=10, N=15, N=48, α=0.05, or power=0.80) are superseded and should be disregarded.

| Hypothesis | Null | Test | α (two-tailed) | Power | Effect size (justification) | N per arm (attrition-adjusted) |
|------------|------|------|----------------|-------|-----------------------------|--------------------------------|
| H1: Old-centriole lineages proliferate less | H0: no difference in proliferation rate | Two-sample t-test (log2 fold change) | 0.001 | 0.95 | Cohen's d = 1.2 (derived from Yamashita 2007; pilot data placeholder) | 24 (includes 20% attrition) |
| H2: Asymmetric centriole inheritance correlates with stemness | H0: no correlation | Fisher's exact test | 0.001 | 0.95 | Cohen's h = 0.8 (based on expected effect from literature; pilot data placeholder) | 24 (includes 20% attrition) |
| H3: Centriole age predicts differentiation commitment | H0: no predictive power | Cox proportional hazards | 0.001 | 0.95 | Hazard ratio = 2.0 (based on expected effect from literature; pilot data placeholder) | 24 (includes 20% attrition) |

**Note:** All thresholds are two-tailed. Effect sizes are based on pilot data (placeholder). N=24 per arm includes 20% attrition adjustment. Bonferroni correction for 3 comparisons: adjusted α = 0.001 / 3 = 0.00033 per test. The table above reports the per-test α; the family-wise error rate is 0.001.final target N=30 per arm for recruitment).



## Pre-registration plan

- **OSF registration ID:** osf.io/TBD (placeholder; to be created before data collection)
- **Planned registration date:** 2026-07-01
- **Contents:** full analysis plan including primary and secondary endpoints, exclusion criteria, stopping rules, and the unified falsifiability thresholds specified above.
- **Deviations:** any post-registration changes will be documented with version control and a rationale.
- **Data collection start:** no earlier than 2026-07-15 (to allow registration to be finalised).
- **Linking note:** The pre-registration will contain the **unified falsifiability table** (N=43 per arm, α=0.001, power=0.95) and the **single sample size calculation** from this document. No deviations from these thresholds are allowed without a documented amendment on OSF.

## Sample size calculation

**Primary endpoint:** Proliferation rate (log2 fold change) at day 7.
**Test:** Two-sample t-test, two-tailed.
**Formula:** n = (Z_α/2 + Z_β)² · 2σ² / δ²

Where:
- Z_α/2 = 3.29 (for α=0.001, two-tailed)
- Z_β = 1.645 (for power=0.95)
- σ² = pooled variance (estimated from pilot data; placeholder σ=0.5)
- δ = minimum detectable effect size (Cohen's d=1.2)

**Calculation:** n = (3.29 + 1.645)² · 2·(0.5)² / (1.2)² ≈ 24.3 → **N=24 per arm** (rounded up).

**Attrition adjustment:** +20% → target recruitment N=30 per arm.

**Note:** This is the sole sample size calculation used for the primary endpoint. All other calculations in earlier drafts (N=10, 15, 48) are