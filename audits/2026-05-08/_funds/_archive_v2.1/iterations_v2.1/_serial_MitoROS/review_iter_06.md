# Review of MitoROS

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 4
- Falsif: 4
- Deliv: 3
- Novelty: 3
- Risk: 4
- RefIntegrity: 3

## Checklist (✓/✗ + объяснение по каждому из 9 условий)
1. **✓ Operationalised falsifiability (numeric thresholds)** — Числовые пороги представлены (p<0.001, slope>0.05/год, R²>0.9, α=0.05, β=0.20, f²=0.35). Есть небольшая путаница между α=0.05 и p<0.001, но в целом условие выполнено.

2. **✓ Pre-registration plan (OSF placeholder + date)** — Указан OSF ID `osf.io/mitocounter3_pr20260701` и дата 2026-07-01, включены гипотезы, анализ, критерии исключения. Есть второй placeholder `osf.io/TBD` в другом месте документа, но это допустимо, если будет заменён.

3. **✗ Sample size calc (power analysis)** — Unified Power Analysis Table содержит строки с `TBD` (σ², δ для P0-1, P0-2, P2-1). Условие требует конкретные числовые значения с формулой и подстановкой. `TBD` не допускается. Нарушение.

4. **✓ Risk matrix ≥5 rows** — 6 строк (R1–R6) с Probability, Impact, Mitigation, Owner, Monitoring. Выполнено.

5. **✓ Limitations section** — Отдельный раздел с 6 ограничениями и планами митигации. Выполнено.

6. **✓ Consortium / collaboration plan** — Placeholder список партнёров (Lab A–D, Clinical collaborator) с описанием ролей, хотя все `[TBD]`. Допустимо.

7. **✗ Reference reality + match** — Две ссылки вызывают сомнение в реальности:
   - PMID: 40239706 (Gozdecka et al., 2025) — может быть не проиндексирован или не опубликован. Требует проверки.
   - PMID: 40579478 (Zhang et al., 2025) — аналогично.
   Остальные ссылки реальны и соответствуют тексту. Условие не выполнено до подтверждения валидности этих PMID.

8. **✗ No fabrication markers** — В Unified Power Analysis Table (CONCEPT.md) и в OPEN