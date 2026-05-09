# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsifiability: 3
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 2
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 3

## Checklist (✓/✗ + объяснение)

1. **Operationalised falsifiability (numeric thresholds)** – ✗  
   Для гипотезы M1 (concordance) и uptime есть числовые пороги, sample size, α, power. Для contamination rate H₀: ≥0.03, но **N = TBD** (placeholder), мощность не указана. Отсутствие числового N для одной из ключевых гипотез нарушает требование полной операционализации.

2. **Pre-registration plan** – ✓  
   Указан OSF (placeholder `osf.io/TBD`), дата 2026-06-01, описание содержимого. Плейсхолдер допустим.

3. **Sample size calc (power analysis)** – ✓  
   Для CDATA эксперимента: формула, подстановки, α=0.05, power=0.80, N=30/group. Для M1: N=286. Для contamination – TBD, но основной endpoint покрыт.

4. **Risk matrix ≥5 rows** – ✓  
   Две матрицы по 6 строк, суммарно ≥5. Указаны probability (1-5) и impact (1-5) с mitigation.

5. **Limitations section** – ✓  
   Отдельный раздел с 8+ пунктами, включая hardware precision, AI hallucination, отсутствие precedents.

6. **Consortium / collaboration plan** – ✓  
   Таблица с 6 партнёрами, ролями, статусом (включая TBD). Плейсхолдеры для недостающих партнёров допустимы.

7. **Reference reality + match** – ✗  
   В EVIDENCE.md часть ссылок имеет PMID/DOI и верифицирована ✅. Однако в CONCEPT.md:  
   - В разделе “Evidence base & meta-analysis” **две ссылки имеют вид `[Author(s), Year, Journal, DOI TBD]` — невалидные плейсхолдеры**.  
   - `[Placeholder: e.g., OpenTrons, µManager...]` — не ссылка.  
   Для утверждения “Low-cost microscope retrofit” только один peer-reviewed источник (Sharkey 2016) + manufacturer spec, что недостаточно.  
   **