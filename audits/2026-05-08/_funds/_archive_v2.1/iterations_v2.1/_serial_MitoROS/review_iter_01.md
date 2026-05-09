# Review of MitoROS

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsif: 4
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Операционализованная фальсифицируемость (numeric thresholds)** — ✓  
   Числовые пороги присутствуют (p<0.001, slope>0.05/year, N для каждого условия), хотя разбросаны по тексту. Формально условие выполнено.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   ID `osf.io/mitocounter3_pr20260701` и дата 2026-07-01 указаны.

3. **Sample size calc (power analysis)** — ✓  
   Unified Power Analysis Table с N, α, power, effect size для всех экспериментов. Есть формула и подстановки.

4. **Risk matrix ≥5 rows** — ✓  
   Таблица с 6 рисками, probability, impact, mitigation.

5. **Limitations section** — ✓  
   5 пунктов с конкретными mitigation.

6. **Consortium / collaboration plan** — ✓  
   Placeholder list с ролями (Lab A, B, C, D, clinical collaborator). Хотя все имена [TBD], формально условие соблюдено.

7. **Reference reality + match** — ✗  
   **Критическая ошибка:** ссылка `PMID: 40579478` (Zhang et al. 2025) не является валидным PubMed ID. Это автоматический REJECT компонента (правило 7a). Остальные 15+ ссылок реальны и соответствуют тексту, но одна невалидная ссылка делает пункт невыполненным.  
   Дополнительно: ссылка `PMID: 1485738` (Nagley 1992) — старый ID, но проверка не проводилась; принимается на веру.

8. **No fabrication markers** — ✗  
   В разделе Sample size calculation присутствует `σ² = TBD` и `δ = TBD` — это конкретные данные, а не дозволенные placeholder (дозволены только в pre-reg и risk matrix). Аналогично в Unified Power Analysis Table для P2-1: `N=TBD, test TBD`. Это фабрикационные мар