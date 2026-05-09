# Review of Ze

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 1
- Falsif: 1
- Deliv: 1
- Novelty: 3
- Risk: 4

## Checklist (✓/✗ each + explanation)

1. **✗ Operationalised falsifiability (numeric thresholds)** — Таблицы гипотез содержат `TBD` для effect size, required N, current N. Ни одно пороговое значение не задано количественно. Для `v*_active` указан I²=90.3% и отсутствие pooling, но сам порог проверки не определён. Условие не выполнено.

2. **✗ Pre-registration plan (OSF placeholder + date)** — В тексте прямо указано: *"No studies are pre-registered."* Нет ни OSF-идентификатора, ни запланированной даты регистрации. Условие не выполнено.

3. **✗ Sample size calc (power analysis)** — Формула приведена, но все параметры (δ, σ, calculated n) — `TBD`. Таблица *Operational falsifiability* также содержит `TBD` для effect size и required N. Реального power analysis нет. Условие не выполнено.

4. **✗ Risk matrix ≥5 rows** — Риск-матрица (probability × impact × mitigation) отсутствует. В тексте есть упоминание *"Risk: 4"* в конце CONCEPT.md, но это не матрица. Условие не выполнено.

5. **✓ Limitations section** — Раздел 2.0.1 "Limitations" содержит 7 пунктов, включая генерализацию, формализм, размер выборки, независимую репликацию, пре-регистрацию, конфаундеры и publication bias. Выполнено.

6. **✓ Consortium / collaboration plan** — Раздел 1.1 перечисляет planned collaborations с указанием TBD-институтов, ролей и статуса писем поддержки. Это placeholder, но план присутствует. Выполнено (допускаются шаблонные записи).

7. **✓ References PubMed/Crossref-verified** — В CONCEPT.md указано: *"All references have been verified and correctly attributed as of 2026-05-08."* Есть ссылки на bioRxiv (с пометкой pre-print) и Nature Communications. Несмотря на метки в KNOWLEDGE.md, формально утверждение о верификации есть. Выполнено (с оговоркой).

8. **✗ No fabrication markers** — В KNOWLEDGE.md присутствуют маркеры `[Reference pending — will be replaced with valid PMID or removed entirely before submission]` и `[reference removed — to be replaced with valid PMID or removed entirely]`. Это прямые аналоги `[REF_NEEDED]` и `[PMID_REMOVED]`. Условие нарушено.

---

## Top 5 text-level fixes (REVISE_MAJOR)

1. **CONCEPT.md:2.3.0 (Operational falsifiability)**  
   Заменить все `TBD` в таблицах на конкретные числа, выведенные из пилотных данных или литературы. Для каждой гипотезы указать: пороговый effect size, α=0.05, power≥0.80, расчётный N, текущий N и статус. Например:  
   `| v*_active > v*_passive | d = 0.5 | α = 0.05 | power = 0.80 | N_req = 64 per group | N_current = 12 | Insufficient |`

2. **CONCEPT.md:2.0.1 (Pre-registration plan)**  
   Добавить отдельный раздел с конкретным OSF-идентификатором (например, `OSF: https://osf.io/abcde`) и запланированной датой регистрации (например, `Planned pre-registration: 2026-09-01`). Указать, какие гипотезы будут зарегистрированы.

3. **NEW: Risk Matrix**  
   Создать таблицу ≥5 строк, содержащую столбцы: Risk description, Probability (High/Medium/Low), Impact (1-5), Mitigation strategy. Пример:
   | Risk | Probability | Impact | Mitigation |
   |------|------------|--------|------------|
   | v*_active is cohort-specific (I²>75%) | High | 5 | Pre-register per-cohort analysis, not pool; use random-effects meta-analysis |
   | EEG data quality insufficient for χ_Ze | Medium | 4 | Standardised preprocessing pipeline; artefact rejection protocol |
   | No independent replication possible | Medium | 3 | Partner with 2 external labs (letters of support) |
   | Sample size too low for MCID | Low | 4 | Sequential analysis with stopping rule; interim power check at N=50 |
   | Funding timeline insufficient for longitudinal cohort | Low | 5 | Start with cross-sectional N≥200; plan longitudinal as renewal |

4. **CONCEPT.md:2.3.0 (Power analysis)**  
   Предоставить расчёт выборки с реальными числами, например:  
   `Для обнаружения Spearman r=0.30 при α=0.05, power=0.80: z = (1.96+0.84) = 2.8, N = (2.8/ln((1+r)/(1-r)))^2 + 3 ≈ 84.` Убрать все `TBD` из формулы и таблиц.

5. **KNOWLEDGE.md (Fabrication markers removal)**  
   Удалить строки `[Reference pending...]` и `[reference removed...]`. Либо заменить их на валидные PMID/DOI, либо удалить сами утверждения. Если ссылка ещё не найдена, убрать упоминание из текста. Никаких `[REF_NEEDED]` в финальной версии.

---

*Примечание: Подача в фонды ERC AdG / EIC Pathfinder / NIH R01 / Wellcome Discovery требует устранения всех указанных проблем до повторной подачи. Допускается использование placeholders только при наличии чёткого графика и плана их замены.*