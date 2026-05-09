# Review of Ze

## Verdict
**REVISE_MAJOR** — ни один из критических требований не выполнен в полном объёме. Проект представляет собой концептуальный каркас с многочисленными TBD-заполнителями, что делает его непригодным для рассмотрения в ERC AdG / Wellcome Discovery / NIH R01. Без конкретных числовых порогов фальсифицируемости и расчёта выборки рецензенты воспримут это как отсутствие методологической готовности.

## Scores (1-5)
- Premise: 3 — оригинальная объединяющая идея, но эмпирической базы нет.
- Method: 2 — формализм не операционализирован, критичные параметры TBD.
- Evidence: 2 — пилотные данные N=12–196, гетерогенность I²=90%, ни одна гипотеза не подтверждена.
- Falsif: 2 — формально есть таблица, но все effect sizes и N — placeholder. Условие не выполнено.
- Deliv: 2 — roadmap размыт, клинические пороги не валидированы, MCID не определён.
- Novelty: 4 — Ze-теория как интерпретативная рамка для QM и биомаркер — несомненно нова.
- Risk: 2 — крайне высокий риск из-за отсутствия предрегистрации, неверифицированных ссылок и неопределённой статистической базы.

## Checklist (✓/✗ each + explanation)
1. **Operationalised falsifiability (numeric thresholds)** ✗ — таблица фальсифицируемости содержит `TBD` для effect size, σ, N. Нет ни одного конкретного порога. Условие не выполнено.
2. **Pre-registration plan (OSF placeholder + date)** ✓ — есть OSF ID placeholder `osf.io/TBD` и планируемая дата (2026-09-01). Формально выполнено, но требуется конкретный ID.
3. **Sample size calc (power analysis)** ✗ — таблица power analysis заполнена `TBD`, расчёт не произведён. Условие не выполнено.
4. **Risk matrix ≥5 rows** ✓ — представлено 6 строк с probability/impact/mitigation.
5. **Limitations section** ✓ — раздел содержит 7 пунктов, хотя некоторые неполны.
6. **Consortium / collaboration plan** ✓ — есть перечень планируемых партнёров (все TBD). Placeholder accepted.
7. **References PubMed/Crossref-verified or clearly marked pre-print** ✗ — часть ссылок не верифицирована (напр., Revicki et al. 2008 без PMID), нет систематического аудита. Пометка "FABRICATION CLEANUP" не заменяет верификации каждой ссылки.
8. **No fabrication markers [REF_NEEDED] / [PMID_REMOVED]** ✓ — в текущей версии маркеры отсутствуют.

## Top 5 text-level fixes (для перехода к REVISE_MINOR)

1. **CONCEPT.md: "Operational falsifiability" — заменить все TBD на конкретные числа**

   Впишите для каждой гипотезы ожидаемый effect size (Cohen's d / r), α=0.05, power=0.80, расчётный N. Источники: пилотные данные (если есть), литературные аналоги, консервативные оценки. Пример:
   ```
   | v*_active > v*_passive | d=0.5 (Cohen’s medium) | α=0.05 | 0.80 | 64 | 12 |
   ```
   Без этого проект не имеет фальсифицируемости.

2. **CONCEPT.md: "Sample size calculation" — рассчитать N для хотя бы одной первичной гипотезы**

   Используйте предполагаемое SD из пилотных данных или литературы. Даже если SD грубый — укажите. Формула: n = (1.96+0.84)²·σ²/δ². Результат — не TBD.

3. **KNOWLEDGE.md: привести все ссылки к PubMed/arXiv или явно пометить как «unverified, to be replaced»**

   Конкретно: Revicki et al. 2008 — нужен PMID или отказ; убрать или подтвердить все удалённые ссылки из fabrication cleanup. Провести полную ревизию.

4. **CONCEPT.md: "Consortium / partners" — заменить TBD institution на реальные названия или хотя бы конкретные специальности**

   Вместо "Neurophysiology group (TBD institution)" напишите: "Dr. X (University Y), expert in EEG source localization — letter of support pending".

5. **CONCEPT.md: "MCID" — удалить раздел или заменить на "рабочая гипотеза без валидации"**

   Текущий текст содержит статистически недействительные утверждения (N=12). Минимум: перенесите в Limitations и добавьте: "MCID will be estimated in WP2 using anchor-based and distribution-based methods (target N≥100)". Не оставляйте голой цитаты.

**Дополнительное замечание:** BUG-v8-1 fix (EEG Application Disclaimer) — единственное корректное признание ограничения, но оно должно быть во всех копиях, а не только в CONCEPT.md. Рекомендую распространить на PARAMETERS.md и README.md.

Проект имеет научную ценность, но в текущем виде не проходит минимальные требования. После исправления указанных 5 пунктов — повторное рассмотрение с возможным апгрейдом до REVISE_MINOR.