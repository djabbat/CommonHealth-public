# Review of Ze

## Verdict
**REJECT**

## Scores (1-5)
- Premise: **2** — оригинальная спекулятивная идея, но без операционализации и доказательной базы
- Method: **1** — плейсхолдеры вместо параметров, гетерогенность I²=90% игнорируется, MCID на N=12, отсутствие конкретного power-анализа
- Evidence: **1** — пилотные данные статистически невалидны, ни одна гипотеза не проверена, есть следы удалённых фабрикованных ссылок
- Falsif: **1** — thresholds все TBD, числовые пороги отсутствуют, схемы есть, но без чисел
- Deliv: **1** — roadmap не представлен, deliverables не определены, даты плейсхолдерные
- Novelty: **3** — новая концептуальная рамка, но без методологической строгости
- Risk: **1** — риски перечислены (6 строк), но mitigation неконкретные; плюс этический риск фабрикации

## Checklist (✓/✗ each + explanation)
1. ❌ **Operationalised falsifiability (numeric thresholds)** — все effect size «TBD», целевые N «TBD», α=0.05 is fine, power=0.80 fine, но отсутствуют конкретные δ, σ, рассчитанные N. Плейсхолдеры ≠ операционализация.
2. ✓ **Pre-registration plan (OSF placeholder + date)** — формально есть: `osf.io/TBD` и дата 2026-09-01. Принято как минимальное.
3. ❌ **Sample size calc (power analysis)** — формула приведена, но все входные параметры «TBD», рассчитанные N тоже «TBD». Нет конкретного обоснованного размера выборки.
4. ✓ **Risk matrix ≥5 rows** — 6 строк, Probability/Impact/Mitigation. Выполнено.
5. ✓ **Limitations section** — 7 пунктов, включая отсутствие пре-регистрации, малую выборку, confounding. Выполнено.
6. ✓ **Consortium / collaboration plan** — перечислены типы партнёров (пусть TBD institutions). Минимально принято.
7. ❌ **All references PubMed/Crossref-verified или явно помечены как pre-print** — в KNOWLEDGE.md есть маркеры `[Reference removed — citation pending verification]` и `[reference removed — to be replaced with valid PMID or removed entirely]`. Это означает, что часть ссылок ранее была фабрикована. Оставшиеся ссылки не верифицированы (например, s4me.info, manual.com). Не выполнено.
8. ❌ **No fabrication markers** — в KNOWLEDGE.md присутствуют явные метки `[Reference removed — citation pending verification]`. Это fabrication markers. Пункт нарушен.

**Итого:** 3/8 пунктов не выполнены (1,3,7,8 — 4 невыполненных). Fabrication markers — критическое нарушение.

## Top 5 text-level fixes (если бы проект был принят к ревизии, но по факту REJECT)

1. **CONCEPT.md: Operational falsifiability → заменить все «TBD» на конкретные числа**
   - В строках таблицы «Effect size (Cohen's d or r)» заменить placeholder на обоснованные значения из пилотных данных или литературы (например, d=0.5 → d=0.8 на основе Cuban когорты). Указать 95% ДИ для каждого effect size. После этого пересчитать N и поставить конкретные числа (например, N=64 → N_calculated=88). Без этого ни один рецензент не воспримет проект всерьёз.

2. **KNOWLEDGE.md: Удалить все fabrication markers полностью**
   - Строки `[Reference removed — citation pending verification]` и аналогичные должны быть убраны. Вместо них либо вставить реальные PMID/DOI после проверки, либо убрать весь абзац. Следы фабрикации недопустимы в грантовой заявке. Дополнительно: провести полный аудит всех ссылок на PubMed/Crossref и приложить таблицу верификации.

3. **CONCEPT.md: v*_active — заменить единое число 0.456 на per-dataset оценки с CI**
   - Указать: Cuban EEG: v*=0.456 [95% CI bootstrap]; Dortmund HRV: v*=0.385 [CI]; MPI-LEMON: v*=0.472 [CI]. Заменить pooled median на отдельные оценки. Добавить Cochran Q с I²=90.3% и признать, что pooling некорректен. Это честно и повышает доверие.

4. **CONCEPT.md: MCID — удалить цифру 0.05, оставить только плановые методы**
   - Полностью убрать строку `MCID_χZe = 0.05 — ТОЛЬКО РАБОЧАЯ ГИПОТЕЗА`. Заменить на: «MCID will be determined using SEM-based and anchor-based methods with N≥50 (SEM) and N≥100 (anchor). No preliminary estimate is available.» Без этого рецензенты увидят статистическую безграмотность.

5. **PARAMETERS.md: Удалить клинические пороги (0.35–0.45 и др.) как необоснованные**
   - Таблица «Ze HRV Clinical Thresholds» содержит утверждения (Healthy 0.35–0.45, Stress v→0 или v→1), не подкреплённые данными. Удалить или переформулировать: «Hypothetical ranges based on theoretical reasoning; not validated clinically.» Аналогично в README.md.

---

**Заключение:** проект содержит признаки фабрикации ссылок, отсутствие операционализируемых численных порогов, невалидные статистические процедуры и не может быть принят к финансированию в текущем виде. Рекомендуется полная переработка с соблюдением стандартов научной добросовестности, после чего возможна повторная подача.