# Review of MitoROS

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsifiability: 3
- Deliv: 4
- Novelty: 4
- Risk: 4
- RefIntegrity: 3 (Reference Integrity Score)

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   Присутствуют числовые пороги (D₃<0.01, p<0.001; slope>0.05/year, p<0.01; quadratic term p<0.05; power α=0.05 β=0.20, f²=0.35). Указаны N для отдельных условий (N=30, N=45, N=25). Выполнено.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Placeholder ID `osf.io/TBD` и `osf.io/mitocounter3_pr20260701`, дата 2026-07-01. Допустимо.

3. **Sample size calc (power analysis)** — ✗  
   Формула приведена, но параметры σ² и δ — TBD (оценка по пилотным данным). В Unified Power Analysis Table для P2-1 N=TBD, test TBD. Требуется конкретная подстановка для каждого эксперимента. Нарушение.

4. **Risk matrix ≥5 rows** — ✓  
   6 строк (R1–R6) с Probability, Impact, Mitigation, Owner, Monitoring. Выполнено.

5. **Limitations section** — ✓  
   6 пунктов с конкретными мерами смягчения. Выполнено.

6. **Consortium / collaboration plan** — ✓  
   Placeholder список (Lab A–D, Clinical collaborator) с ролями. Выполнено.

7. **Reference reality + match** — ✗ (с флагами)  
   Все ссылки имеют PMID/DOI, но несколько вызывают сомнение:  
   - PMID 40239706 (Gozdecka 2025) – статья будущего, статус неизвестен → [REF_VERIFY]  
   - PMID 40579478 (Zhang 2025) – препринт, принят к публикации, но реальность ID не подтверждена → [REF_VERIFY]  
   - PMID 40183670 (Koloko Ngassie 2025) – аналогично → [REF_VERIFY]  
   - PMID 1485738 (Nagley 1992) – не удалось однозначно подтвердить существование → [REF_VERIFY]  
   Соответствие тексту в целом правдоподобно, но без верификации нельзя считать выполненным. Флагов >2 → не REVISE_MINOR.

8. **No fabrication markers** — ✗  
   В Sample size calculation (CONCEPT.md) присутствуют TBD для σ² и δ. В Unified Power Analysis Table (OPEN_PROBLEMS.md) для P2-1 указано N=TBD, test=TBD. Эти места должны содержать конкретные числа. Placeholder допустим только в pre-registration и risk matrix. Нарушение.

9. **Internal consistency core docs** — ✗  
   Противоречие: в разделе Falsifiability указаны конкретные пороги и N, а в Sample size calculation те же параметры остаются TBD. Параметры τ₃ в PARAMETERS.md имеют статус «гипотетичен» и не согласованы с числовыми порогами фальсификации. В THEORY.md и CONCEPT.md дублируются power analysis с расхождениями. Нарушение.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | López-Otín et al. 2013 – hallmarks of aging | PMID 23746838 | Вероятно, реальна | Да | OK |
| 2 | Guo et al. 2023 – ROS and mtDNA damage | PMID 37196864 | Вероятно, реальна | Да | OK |
| 3 | Khrapko & Vijg 2009 – clonal expansion | PMID 19732859 | Вероятно, реальна | Да | OK |
| 4 | Picca et al. 2023 – mtDNA mutations | PMID 37172915 | Вероятно, реальна | Да | OK |
| 5 | Lakshmanan et al. 2018 – mtDNA deletions muscle | PMID 30043489 | Вероятно, реальна | Да | OK |
| 6 | Zhang et al. 2025 – tRNA mutations kidney | PMID 40579478 | Сомнительно (2025, препринт) | Вероятно | [REF_VERIFY] |
| 7 | Wiesner et al. 2006 – 8-oxo-dG brain | PMID 17090418 | Вероятно, реальна | Да | OK |
| 8 | Tranah et al. 2018 – heteroplasmy epigenetic age | PMID 30089816 | Вероятно, реальна | Да | OK |
| 9 | Khrapko et al. 2014 – tissue specificity | PMID 25149213 | Вероятно, реальна | Да | OK |
| 10 | Madreiter-Sokolowski et al. 2024 – BER tissues | PMID 39179117 | Вероятно, реальна | Да | OK |
| 11 | Gozdecka et al. 2025 – clonal hematopoiesis | PMID 40239706 | Сомнительно (2025) | Предположительно | [REF_VERIFY] |
| 12 | Koloko Ngassie et al. 2025 – hyperoxia senescence | PMID 40183670 | Сомнительно (2025) | Вероятно | [REF_VERIFY] |
| 13 | Insalata et al. 2022 – mtDNA deletion kinetics | PMID 36442091 | Вероятно, реальна | Да | OK |
| 14 | Nagley et al. 1992 – OXPHOS deficiency | PMID 1485738 | Сомнительно (старый ID) | Вероятно | [REF_VERIFY] |
| 