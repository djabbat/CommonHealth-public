# Review of Ze

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- **Premise:** 3 — интересная интерпретативная рамка, но смешение теорем, гипотез и рабочих гипотез без чёткого разделения.
- **Method:** 2 — повсеместные placeholder (TBD), отсутствие конкретных операциональных определений для ключевых параметров (effect size, SD, MCID). Статистический план не завершён.
- **Evidence:** 2 — пилотные данные N≤196 с высокой гетерогенностью (I²=90.3%), отсутствие независимой репликации, все гипотезы непроверены.
- **Falsif:** 2 — попытка есть, но numeric thresholds не заданы (Cohen's d = TBD, r = TBD). Без конкретных чисел фальсифицируемость не операционализирована.
- **Deliv:** 3 — roadmap есть, но много TBD, матрица рисков хорошая, limitations прописаны.
- **Novelty:** 4 — оригинальная концепция, связь Ze с CDATA, spacetime emergence, конкурентное позиционирование vs BrainYears.
- **Risk:** 2 — высокий риск: отсутствие предварительных данных по ключевым гипотезам (χ_Ze vs aging, CDATA longitudinal), незавершённая валидация, fabrication markers в references.

## Checklist (✓/✗ each + explanation)

| # | Condition | Status | Explanation |
|---|-----------|--------|-------------|
| 1 | Operationalised falsifiability (numeric thresholds) | ✗ | Все effect size указаны как TBD (placeholder). Нет конкретных N, p, power для каждой гипотезы. |
| 2 | Pre-registration plan (OSF placeholder + date) | ✓ | Есть OSF ID placeholder (`osf.io/TBD`) и planned dates (2026-09-01, 2026-12-01). |
| 3 | Sample size calc (power analysis) | ✗ | Формула приведена, но effect size и SD — TBD; calculated N — TBD. Power analysis не завершена. |
| 4 | Risk matrix ≥5 rows | ✓ | 6 строк с Probability, Impact, Mitigation. |
| 5 | Limitations section | ✓ | Явный раздел (2.0.1) с 7 пунктами. |
| 6 | Consortium / collaboration plan | ✓ | Placeholder list (TBD institution) для нейрофизиологии, биостатистики, клиники. |
| 7 | All references PubMed/Crossref-verified or pre-print | ✗ | Есть ссылки без PMID/DOI (Nature Communications без DOI; s4me.info; the manual). В KNOWLEDGE.md присутствуют маркеры `[Reference pending]` и `[reference removed]` — не верифицированы. |
| 8 | No fabrication markers ([REF_NEEDED]/[PMID_REMOVED]) | ✓* | Формально `[REF_NEEDED]` и `[PMID_REMOVED]` отсутствуют, но есть `[Reference pending]` и `[reference removed]`, что аналогично по духу. Считаем условно выполненным, но авторам рекомендуется удалить все такие маркеры. |

*Пункт 8 формально пройден, но фактически тексты содержат следы fabrication (незавершённые ссылки). Рекомендуется исправить в REVISE_MAJOR.

## Top 5 text-level fixes (обязательно добавить/изменить)

1. **CONCEPT.md: § Operational falsifiability** — заменить все TBD в effect size, α, power, required N на конкретные числа, обоснованные либо пилотными данными, либо литературой. Для v*_active: указать 95% CI per dataset (bootstrap B=10000, BCa). Для χ_Ze vs aging: r=0.3, α=0.05, power=0.80 → N=84.

2. **CONCEPT.md: § Sample size calculation** — заполнить все TBD (σ, δ, calculated N). Пример: для v*_active: использовать Cohen's d=0.5 (medium effect) из пилота N=12 → σ=0.25 → N=64 per group. Для MCID: SD=0.03 → N≥50 для SEM-based. Все расчёты должны быть численно завершены.

3. **KNOWLEDGE.md: References** — удалить все `[Reference pending]` и `[reference removed]`. Заменить их либо валидными PMID/DOI, либо полностью исключить. Все ссылки на нерецензируемые источники (s4me.info, the manual, bioRxiv) должны быть явно помечены как pre-print или веб-ресурс. Добавить DOI для Nature Communications (10.1038/s41467-025-64275-4).

4. **CONCEPT.md: § Consortium / partners** — заменить "TBD institution" на названия конкретных учреждений, с которыми ведутся переговоры, или хотя бы на типы институтов с указанием географического региона (например, "University of Havana, Cuba (EEG validation lab)"). Без конкретики consortium план выглядит необязательным.

5. **CONCEPT.md: § Pre-registration plan** — указать реальный OSF ID, даже временный (можно зарегистрировать проект с ограниченным доступом). Дата должна быть реалистичной и соответствовать началу финансирования. Если pre-registration ещё не проведён, указать "to be registered before data collection starts (target: DD.MM.YYYY)".

## PACKET

Все замечания выше. Проект имеет потенциал, но в текущем виде не готов к финансированию из-за отсутствия конкретных численных порогов, неполной power analysis и незавершённых ссылок. Рекомендуется REVISE_MAJOR с последующей повторной подачей.