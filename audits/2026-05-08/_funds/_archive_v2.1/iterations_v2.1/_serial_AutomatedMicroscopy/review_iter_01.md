# Review of AutomatedMicroscopy

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 3
- Deliv: 2
- Novelty: 3
- Risk: 2
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✗  
   Пороги заданы для M1 (concordance >0.80 с power analysis N=286), uptime (>0.90), contamination (<0.03), cost (<$4,500). Однако sample size для contamination — `N = TBD`, для uptime — фиксирован 180 дней, но H₀ противоречит target‑prediction (см. п.9). Частично выполнено, но не полностью.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Указан OSF ID `osf.io/automicroscopy_cdata` и planned date 2026-06-01. Placeholder допустим.

3. **Sample size calc (power analysis)** — ✗  
   Для CDATA experiment формула приведена (d=0.75, α=0.05, power=0.80 → n=30/group). Для M1 — N=286. Для contamination — `N = TBD`. Design effect противоречиво: в одном месте `DE=1.2`, в другом `DE = TBD`. Нет полного покрытия.

4. **Risk matrix ≥5 rows** — ✓  
   Две матрицы (6 и 7 строк) с Probability × Impact × Mitigation. Формально выполнено.

5. **Limitations section** — ✓  
   Раздел присутствует (8 пунктов в CONCEPT.md, дополнительно в EVIDENCE.md).

6. **Consortium / collaboration plan** — ✓  
   Таблица партнёров с ролями и статусами. Есть заглушки (TBD), но план есть.

7. **Reference reality + match** — ✗ **AUTOMATIC REJECT**  
   Из 15 цитируемых работ 5 не имеют DOI/PMID/arXiv (Zeiss IM 35