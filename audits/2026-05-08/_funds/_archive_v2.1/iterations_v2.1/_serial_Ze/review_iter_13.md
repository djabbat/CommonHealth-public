# Review of Ze

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 2
- Deliv: 2
- Novelty: 4
- Risk: 2

## Checklist (✓/✗ each + explanation)

1. **✗ Operationalised falsifiability (numeric thresholds)** — Все эффекты и пороги — `TBD` или `placeholder`. Нет ни одного законченного числового критерия с N≥ для конкретной гипотезы. «Operational falsifiability for each hypothesis» пустая (все effect size = TBD). Требование не выполнено.

2. **✗ Pre-registration plan (OSF placeholder + date)** — В разделе Limitations указано «No studies are pre-registered». Нет OSF-идентификатора (хотя бы placeholder), нет запланированной даты. Требование не выполнено.

3. **✗ Sample size calc (power analysis)** — Формулы приведены, но ни одно число не заполнено (σ, δ, N — всё TBD). Таблица power analysis — пустые ячейки. Нет конкретного расчёта. Требование не выполнено.

4. **✗ Risk matrix ≥5 rows** — Никакой матрицы рисков нет. Есть раздел Limitations (7 строк), но это не risk matrix (probability × impact × mitigation). Требование не выполнено.

5. **✓ Limitations section** — Явный раздел `2.0.1. Limitations` в CONCEPT.md присутствует, перечислены 7 пунктов. Выполнено.

6. **✓ Consortium / collaboration plan** — Раздел `1.1. Consortium / partners` содержит placeholder-список (TBD institution, letters of support pending). Требование допускает placeholder — выполнено (минимально).

7. **✗ References PubMed/Crossref-verified** — Ссылки на BrainYears (bioRxiv preprint — OK), Nature Communications (Crossref-verified — OK), Science for Me (не PubMed/Crossref, не помечен как preprint), WHOOP (новостная статья, не рецензированная). Не все ссылки верифицированы или явно помечены. Требование не выполнено.

8. **✗ No fabrication markers** — В KNOWLEDGE.md присутствуют маркеры `[no reference available — to be determined]` и `[reference removed — to be replaced with valid PMID or removed entirely]`. Это fabrication markers (указывают на удалённые сфабрикованные ссылки). Требование отсутствия маркеров не выполнено.

**Итог:** 2/8 выполнено. 6 грубых нарушений → обязательный REVISE_MAJOR.

## Top 5 text-level fixes (добавить/изменить)

1. **CONCEPT.md → раздел «Operational falsifiability for each hypothesis»**  
   Заменить все `TBD` на конкретные числа: effect size (Cohen's d или r), SD из пилотных данных или литературы, итоговый N. Пример: «v*_active > v*_passive: d=0.5 (из Cuban EEG N=88, SD≈0.12), α=0.05, power=0.80 → N=64 per group». Убрать примечания «placeholder».

2. **CONCEPT.md → раздел «Pre-registration plan» (добавить)**  
   Вписать: «Pre-registration will be submitted to OSF (placeholder ID: osf.io/xxxxx) by [конкретная дата, напр. 2026-09-01]. Pre-registration will include all hypotheses, analysis plan, and exclusion criteria.»

3. **CONCEPT.md → раздел «Sample size calculation»**  
   Для каждой гипотезы указать конкретный расчёт:  
   `n = (Z_α/2 + Z_β)² · σ² / δ²` с подставленными числами, source оценки σ (пилотные данные или PMID). Не «TBD».

4. **CONCEPT.md → новый раздел «Risk Matrix»**  
   Таблица из ≥5 строк с колонками: Risk description, Probability (1-5), Impact (1-5), Mitigation, Residual risk. Пример:
   - Валидация v*_active на другой когорте. Prob=3, Impact=4, Mitigation: запланировать N≥200 в WP1, Residual=2.
   - Невалидность MCID. Prob=5, Impact=5, Mitigation: только после N≥50 SEM-based. Residual=3.
   - Fabrication markers в литературе. Prob=2, Impact=5, Mitigation: полный аудит всех ссылок. Residual=1.
   - Отсутствие независимой репликации. Prob=4, Impact=4, Mitigation: collaboration с TBD партнёром. Residual=2.
   - Высокая гетерогенность I²=90%. Prob=5, Impact=4, Mitigation: per-dataset bootstrap, не пулить до I²<50%. Residual=2.

5. **KNOWLEDGE.md — удалить все fabrication markers**  
   Убрать строки `[no reference available — to be determined]` и `[reference removed — to be replaced...]`. Каждая такая ссылка должна быть либо заменена на реальную PubMed-ссылку, либо полностью удалена из текста. Если данных нет — не писать. Текущее состояние неприемлемо.

**Дополнительно (не обязательный, но настоятельный):**  
- CONCEPT.md: добавить дату pre-registration (не просто «запланировано»).
- Все ссылки (WHOOP, Science for Me) — либо найти PubMed-версию, либо явно пометить как «non-peer-reviewed source / news article».

## PACKET
Ze (проект предоставлен в виде файлов CONCEPT.md, PARAMETERS.md, KNOWLEDGE.md, README.md)