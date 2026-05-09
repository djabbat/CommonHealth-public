# Review of Ze

## Verdict
**REVISE_MAJOR** — ни один из 8 обязательных пунктов checklist не провален полностью, но пункты 1, 7, 8 имеют критические недочёты, не позволяющие присвоить FUND_AS_IS/REVISE_MINOR. Совокупность placeholder-значений, непроверенных ссылок и оставленных fabrication-маркеров делает заявку неприемлемой в текущем виде. Требуется доработка по всем трём пунктам до подачи в ERC/Wellcome.

## Scores (1-5)
- Premise: 3 (интересная, но чрезмерно амбициозная интерпретативная рамка без эмпирической базы)
- Method: 2 (формализм фрагментарен: ρ_Z → P_Z без обоснования перехода, v* имеет два противоречащих значения, power analysis с TBD)
- Evidence: 1 (N=196 с I²=90.3% — pooling некорректен, нет ни одной независимой репликации, все данные из одной лаборатории)
- Falsif: 2 (таблица есть, но effect sizes и требуемые N помечены TBD; операциональные пороги для MCID и χ_Ze — TBD)
- Deliv: 2 (консорциум и pre-registration — placeholder, нет конкретных писем поддержки, нет прототипа BioSense)
- Novelty: 4 (оригинальная теоретическая конструкция, связь Ze-счёта с энтропией и aging — новая)
- Risk: 1 (чрезвычайно высокий: ни одного валидированного биомаркера, отсутствие данных по сравнению с эпигенетическими часами, высокая гетерогенность, fabrication-маркеры)

## Checklist (✓/✗ each + explanation)

| # | Пункт | Статус | Объяснение |
|---|-------|--------|------------|
| 1 | **Operationalised falsifiability (numeric thresholds)** | ✗ | Таблица с α=0.05, power=0.80 есть, но все effect sizes, σ и требуемые N помечены **TBD**. Для гипотез «χ_Ze клинические пороги (0.80/0.60/0.40)» и «MCID_χZe = 0.05» прямо указано, что они статистически недействительны. Фальсифицируемость не операционализирована в числах — это недопустимо. |
| 2 | **Pre-registration plan (OSF placeholder + date)** | ✓ | OSF ID `osf.io/TBD` и даты 2026-09-01, 2026-12-01 указаны. Пусть placeholder, но формально есть. |
| 3 | **Sample size calc (power analysis)** | ✓ | Формула и таблица с δ, σ, calculated N приведены. Однако δ и σ помечены TBD или «placeholders». Нельзя считать расчёт полноценным, но структура присутствует. |
| 4 | **Risk matrix ≥5 rows** | ✓ | 6 строк с probability, impact, mitigation. |
| 5 | **Limitations section** | ✓ | Явный раздел с 7 пунктами (обобщаемость, математический формализм, размер выборки, независимая репликация, pre-registration, конфаундеры, publication bias). |
| 6 | **Consortium / collaboration plan** | ✓ | Перечислены планируемые партнёры (neurophysiology, biostatistics, clinical center, data management) — все с пометкой TBD. Формально план есть. |
| 7 | **References PubMed/Crossref-verified** | ✗ | 1) Ссылка на BrainYears — bioRxiv (препринт, не verified). 2) Ссылка на Wearable Aging Clock — Nature Communications (verified). 3) Ссылка на HRV-CV — s4me.info (форум, не научный источник). 4) Ссылка на WHOOP — коммерческий сайт. 5) PMID 27330520 (Koo & Mae) — verified. Остальные ссылки (Shannon 1948) без PMID. Не все references verified. Требуется, чтобы каждая ссылка была либо PMID, либо DOI c Crossref для рецензируемых статей; препринты допустимы только с явной пометкой «preprint». |
| 8 | **No fabrication markers** | ✗ | В KNOWLEDGE.md присутствуют строки: `[Reference pending — placeholder; will be replaced with valid PMID or removed entirely before submission. As of 2026-05-08, no fabrication markers remain.]` и `[reference removed — to be replaced with valid PMID or removed entirely]`. Это прямые fabrication-маркеры: они показывают, что ссылки были удалены/заменены. Даже если автор утверждает, что fabrication отсутствует, наличие этих меток свидетельствует о незавершённой верификации. До их полного удаления пункт не выполнен. |

**Итог:** пункты 1, 7, 8 не выполнены → REVISE_MAJOR.

## Top 5 text-level fixes

### 1. Заменить все TBD в operational falsifiability на конкретные числа
- **file:** `CONCEPT.md` → таблица «Operational falsifiability for each hypothesis»
- **что вписать:** Для каждой гипотезы указать effect size (Cohen's d или r), рассчитанный N, и явный источник этого effect size (например, из пилотных данных, литературы или консервативная оценка). Если effect size неизвестен — указать minimal detectable effect size при N, достижимом в рамках гранта (например, d=0.5 при N=64). **Удалить все TBD.**

### 2. Удалить fabrication-маркеры из KNOWLEDGE.md
- **file:** `KNOWLEDGE.md` (строки с `[Reference pending...]`, `[reference removed...]`)
- **что сделать:** Полностью удалить эти строки. Для каждой такой ссылки либо вставить валидный PMID/DOI, либо убрать упоминание работы. **Никаких следов «pending» в финальном тексте.**

### 3. Заменить непроверенные ссылки на verified
- **file:** `CONCEPT.md`, `KNOWLEDGE.md`
- **что сделать:** Ссылку на BrainYears (bioRxiv) заменить на arXiv или PMID (если статья принята). Ссылку на s4me.info удалить, заменить на рецензируемый источник (если нет — просто убрать). Ссылку на WHOOP заменить на публикацию в рецензируемом журнале (если есть) или убрать. Для всех оставшихся ссылок проверить, что они ведут на рецензируемые статьи с PMID или DOI Crossref.

### 4. Устранить противоречие v*_passive vs v*_active
- **file:** `PARAMETERS.md` (строки про v*_passive = 0.3069 и v*_active ≈ 0.456) и `README.md` (упоминание 0.456 как empirical approximation)
- **что сделать:** Чётко указать, что v*_passive = 1−ln2 — теоретическое значение для пассивного счётчика (Shannon-оптимум), а v*_active ≈ 0.456 — эмпирическая оценка для активного агента в Cuban EEG (I²=90.3%, pooling некорректен). **Обязательно** добавить 95% ДИ для v*_active, рассчитанный bootstrap per dataset (не pooling). Пока ДИ не получен, писать «empirical estimate from Cuban cohort (N=88), 95% CI pending».

### 5. Добавить явное указание на статус MCID и клинических порогов
- **file:** `CONCEPT.md` → разделы «Клиническая интерпретация χ_Ze» и «MCID»
- **что сделать:** Заменить все числовые пороги (0.80/0.60/0.40, MCID=0.05) на «TBD — not yet validated». Написать: «All clinical thresholds are theoretical and require anchor-based validation (N≥100). Until then, χ_Ze is described as an exploratory biomarker in analytical validation phase. No clinical decisions should be based on this index.» Убрать любые таблицы с конкретными числами.