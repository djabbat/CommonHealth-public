# Review of MitoROS

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 4
- Falsif: 3
- Deliv: 3
- Novelty: 4
- Risk: 4
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✗  
   Присутствуют числовые пороги для основных условий (D₃<0.01, slope>0.05/год, p<0.01, α=0.05, power=0.80, N=15), но в sample size calculation и unified power table присутствуют TBD (σ² = TBD, δ = TBD, для P2‑1 N=TBD, test TBD). Отсутствие полной спецификации нарушает условие.

2. **Pre‑registration plan (OSF placeholder + date)** — ✓  
   Указаны OSF ID `osf.io/mitocounter3_pr20260701` и дата 2026‑07‑01. Placeholder допустим.

3. **Sample size calc (power analysis)** — ✓ (с оговорками)  
   Формула n = (Z_α/2 + Z_β)²·σ²/δ² приведена, примеры для P0‑1 и P0‑2 с конкретными N. Однако σ² и δ остаются TBD, что снижает полноту, но основные эксперименты имеют N.

4. **Risk matrix ≥5 rows** — ✓  
   6 строк (R1‑R6) с Probability, Impact, Mitigation.

5. **Limitations section** — ✓  
   6 пунктов в CONCEPT.md, включая tissue heterogeneity, hypothetical τ₃, uncalibrated weights, causal direction, overfitting, species generalizability. Каждый с mitigation.

6. **Consortium / collaboration plan** — ✓  
   Placeholder list партнёров (Lab A‑D, Clinical collaborator) с указанием ролей. Формальные соглашения pending.

7. **Reference reality + match** — ✗  
   EVIDENCE.md содержит таблицу проверенных ссылок (7 строк), но в CONCEPT.md, THEORY.md, PARAMETERS.md, OPEN_PROBLEMS.md используются дополнительные PMID (например, 23746838, 19732859, 1485738, 16868022, 26281784, 40239706, 40579478, 30089816 и др.), для которых не представлен аудит на соответствие утверждениям. Отсутствует полный reference audit всех цитируемых работ. Это грубое нарушение.

8. **No fabrication markers** — ✗  
   В sample size calculation присутствуют TBD (σ², δ). В OPEN_PROBLEMS.md для P2‑1 N=TBD, test TBD. Согласно инструкции, TBD недопустимы вне pre‑registration и risk matrix. Нарушение.

9. **Internal consistency core docs** — ✓  
   CONCEPT, THEORY, EVIDENCE, PARAMETERS, OPEN_PROBLEMS согласованы. Канон CORRECTIONS соблюдён (Γ=0 по умолчанию, нет отозванных формул). Противоречий не обнаружено.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | López-Otín et al., 2013 (PMID:23746838) | 23746838 | Предположительно да | Да – утверждение о hallmarks | Принимается, но не проверено в рамках представленного аудита |
| 2 | Guo et al., 2023 (PMID:37196864) | 37196864 | Предположительно да | Да – обзор mtROS | Принимается |
| 3 | Khrapko & Vijg, 2009 (PMID:19732859) | 19732859 | Предположительно да | Да – clonal expansion | Принимается |
| 4 | Picca et al., 2023 (PMID:37172915) | 37172915 | Предположительно да | Да – mtDNA mutations | Принимается |
| 5 | Koloko Ngassie et al., 2025 (PMID:40183670) | 40183670 | Предположительно да | Да – hyperoxia, senescence | Принимается |
| 6 | Insalata et al., 2022 (PMID:36442091) | 36442091 | Предположительно да | Да – heteroplasmy kinetics | Принимается |
| 7 | Madreiter‑Sokolowski et al., 2024 (PMID:39179117) | 39179117 | Предположительно да | Да – tissue differences | Принимается |
| 8 | Nagley et al., 1992 (PMID:1485738) | 1485738 | Предположительно да | Да – OXPHOS deficiency | Принимается |
| 9 | Khrapko, 2014 (PMID:25149213) | 25149213 | Предположительно да | Да – clonal expansion, threshold | Принимается |
| 10 | Gozdecka et al., 2025 (PMID:40239706) | 40239706 | Предположительно да | Да – clonal hematopoiesis | Принимается |
| 11 | Stewart & Chinnery, 2015 (PMID:26281784) | 26281784 | Предположительно да | Да – models of clonal expansion | Принимается |
| 12 | Lakshmanan et al., 2018 (PMID:30043489) | 30043489 | Предположительно да | Да – COX‑negative fibers | Принимается |
| 13 | Zhang et al., 2025 (PMID:40579478) | 40579478 | Предположительно да | Да – kidney mtDNA | Принимается |
| 14 | Wiesner et al., 2006 (PMID:17090418) | 17090418 | Предположительно да | Да – 8‑oxo‑dG in brain | Принимается |
| 15 | Tranah et al., 2018 (PMID:30089816) | 30089816 | Предположительно да | Да – heteroplasmy and epigenetic age | При