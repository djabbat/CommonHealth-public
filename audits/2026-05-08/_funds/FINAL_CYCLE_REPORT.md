# FINAL CYCLE REPORT — LongevityCommon Peer-Review Loop (2026-05-08)

**Loop strategy:** serial — каждый компонент через inner loop {review → text-level fixes → re-review} до ACCEPT (FUND_AS_IS / REVISE_MINOR) или плато (6 итераций без улучшения score).

**Reviewer:** DeepSeek-reasoner | **Fix-generator:** DeepSeek-chat

## Distribution

- **FUND_AS_IS**: 7 ✅
- **REVISE_MAJOR**: 3 ⚠️
- **REVISE_MINOR**: 3 ✅

**Total ACCEPT: 10/13**

## Per-component (by latest review mtime)

| Component | Final verdict | Last iter | Status |
|---|---|---|---|
| AIM | REVISE_MINOR | 1 | ✅ ACCEPT |
| AutomatedMicroscopy | FUND_AS_IS | 2 | ✅ ACCEPT |
| BioSense | FUND_AS_IS | 1 | ✅ ACCEPT |
| CDATA | REVISE_MINOR | 4 | ✅ ACCEPT |
| CytogeneticTree | REVISE_MAJOR | 14 | ⚠️ PLATEAU |
| EpigeneticDrift | FUND_AS_IS | 1 | ✅ ACCEPT |
| HAP | REVISE_MINOR | 5 | ✅ ACCEPT |
| MCOA | FUND_AS_IS | 3 | ✅ ACCEPT |
| MitoROS | REVISE_MAJOR | 10 | ⚠️ PLATEAU |
| Proteostasis | FUND_AS_IS | 1 | ✅ ACCEPT |
| Telomere | FUND_AS_IS | 1 | ✅ ACCEPT |
| UMBRELLA | FUND_AS_IS | 2 | ✅ ACCEPT |
| Ze | REVISE_MAJOR | 14 | ⚠️ PLATEAU |

## Что добавлено в core docs

Каждая итерация добавляла text-level разделы (additive only):
- `## Falsifiability` с numeric thresholds (N≥, p<, effect-size, power)
- `## Pre-registration plan` (placeholder OSF id + planned date)
- `## Sample size calculation` с power analysis
- `## Risk matrix` (probability × impact × mitigation)
- `## Limitations`
- `## Consortium / partners` (placeholder)

## Reality check

FUND_AS_IS / REVISE_MINOR от DeepSeek-reasoner ≠ реальный fund accept. Реальное ERC AdG / NIH R01 / Wellcome accept требует:
- Опубликованных preliminary data в peer-reviewed journals (нельзя выдумать)
- Signed LoIs от EU partners (для EIC Pathfinder ≥3 EU MS — нельзя выдумать)
- Pre-registered protocol на OSF/ClinicalTrials.gov с реальным ID
- PI track record

Loop добавил text-level structure (с placeholder'ами), которая удовлетворяет formal checklist DeepSeek-reasoner. Реальная подача в фонд требует замены placeholder'ов на actual data.
