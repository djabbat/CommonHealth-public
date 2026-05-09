# Review of CytogeneticTree

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 2
- Falsif: 1
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

### 1. Operationalised falsifiability (numeric thresholds) ✗
В документе присутствуют **множественные противоречащие друг другу** таблицы с порогами:
- В разделе «Pre-registration plan» (CONCEPT.md) указано: N=43 per arm, α=0.001, power=0.95, d=1.2.
- Там же чуть ниже — таблица с N=24 per arm, α=0.001, power=0.95, h=0.8.
- Далее — ещё одна таблица с N=24 per arm, α=0.001, power=0.90, d=1.2.
- В разделе «Falsifiability» — таблица с N=24, α=0.001, power=0.95, d=1.2, а затем «Unified numeric thresholds» с N=24, power=0.90.
- PARAMETERS.md указывает α=0.05, power=0.80 для log-rank теста — **прямое противоречие**.
- Нет единого, окончательного, связного набора числовых порогов. Заявленный «single binding» блок не является единственным — старые версии не удалены. **Условие не выполнено.**

### 2. Pre-registration plan (OSF placeholder + date) ✓ (с оговоркой)
OSF ID указан как «osf.io/TBD» (корректный placeholder). Дата регистрации 2026-07-01 указана. Содержание плана описано. Однако фактическая регистрация отсутствует (TBD), что допустимо для грантовой заявки. **Условие засчитывается, но слабое.**

### 3. Sample size calc (power analysis) ✗
Присутствуют **не менее шести различных расчётов**:
- n = 10 per arm (α=0.05, power=0.80, d=0.5) → N=24
- n = 15 per arm (α=0.001, power=0.95, h=0.8) → N=24
- n = 24 per arm (α=0.001, power=0.95, d=1.2) (из G*Power)
- n = 34 per arm → N=43 (из ручного расчёта)
- n = 6 per arm → N=10 → N=24
- N=30 per arm (attrition-adjusted)
Расчёты используют разные α, power, effect size, формулы. Нет единого, окончательного расчёта. **Условие не выполнено.**

### 4. Risk matrix ≥5 rows ✓
Присутствуют матрицы рисков с 6 и 7 строками. Риски реальны, предложены вероятности, воздействие и смягчение. **Условие выполнено.**

### 5. Limitations section ✓
Раздел «Limitations» присутствует в CONCEPT.md, перечисляет >5 ограничений, включая «RITE cassette does not exist», «FACS by centriole color not described», «unverified reference». **Условие выполнено.**

### 6. Consortium / collaboration plan ✓ (с оговоркой)
Список партнёров с ролями приведён (Principal Investigator, host institution, RITE design, FACS, computational reconstruction и др.). Однако большинство участников — placeholders (TBD), письма поддержки отсутствуют. Формально «placeholder list» есть. **Условие засчитывается.**

### 7. Reference reality + match ✗
KNOWLEDGE.md утверждает, что все 37 PMID верифицированы через NCBI eutils, **однако**:
- Ссылка **Lee & Luo 1999, Neuron** (PMID 10197526) имеет статус «REFERENCE VERIFICATION PENDING» — не верифицирована.
- В PARAMETERS.md указана ссылка **Parrinello 2003** без DOI/PMID, помечена [REF_VERIFY — DOI/PMID TBD].
- В тексте присутствует отметка об удалённой ссылке «[[Reference removed pending verification]]».
Таким образом, не все цитируемые работы верифицированы. Некоторые утверждения опираются на непроверенные источники. **Условие не выполнено.**

### 8. No fabrication markers ✗
В документе присутствуют следующие недопустимые маркеры:
- `[REF_VERIFY — DOI/PMID TBD]` (PARAMETERS.md)
- `[REFERENCE VERIFICATION PENDING]` (KNOWLEDGE.md)
- `[[Reference removed pending verification]]` (KNOWLEDGE.md)
- `[PREPRINT: https://doi.org/10.1101/...]` — неопределённый статус.
Согласно правилу, любые невалидные ссылки или пометки «не верифицировано» являются фабрикационными маркерами. **Условие не выполнено.**

### 9. Internal consistency core docs ✗
Основные документы (CONCEPT.md, PARAMETERS.md, KNOWLEDGE.md) содержат внутренние противоречия:
- CONCEPT.md предлагает α=0.001, power=0.95, PARAMETERS.md — α=0.05, power=0.80.
- В CONCEPT.md несколько версий sample size (24, 43, 30, 15, 10).
- PARAMETERS.md не согласован с CONCEPT.md по статистическим параметрам.
- Цели и методы не согласованы: CONCEPT.md говорит о «complete DAG from zygote», но методология ограничивается in vitro клеточной линией на несколько месяцев.
- Отсутствует согласование между заявленными гипотезами и расчётными размерами эффекта. **Условие не выполнено.**

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|------------------------|---------|
| 1 | Lee & Luo 1999, Neuron | PMID 10197526 | **Не верифицирована** (статус PENDING) | Невозможно оценить | [REF_VERIFY] |
| 2 | Parrinello 2003 | DOI/PMID отсутствует | **Неизвестна** (помечена REF_VERIFY) | Невозможно оценить | [REF_VERIFY] |
| 3 | Yamashita 2007, Science | PMID 17255513 | Да (верифицирована) | Да | OK |
| 4 | Royall 2023, eLife | PMID 37882444 | Да (верифицирована) | Да | OK |
| 5 | Verzijlbergen 2010, PNAS | PMID 20018668 | Да (верифицирована) | Да | OK |
| 6 | Chan 2019, Nature | PMID 31086336 | Да (верифицирована) | Да | OK |
| 7 | Loeffler 2019, Nature | PMID 31485073 (исправлено) | Да (верифицирована) | Да | OK |
| 8 | Januschke 2011, Nat Commun | PMID 21407209 | Да (верифицирована) | Да | OK |
| 9 | Wang 2009, Nature | PMID 19829375 | Да (верифицирована) | Да | OK |
| 10 | Thayer 2014, PNAS | PMID 25228775 | Да (верифицирована) | Да | OK |
| ... | Прочие 30+ PMID | Все заявлены верифицированными | По утверждению автора — да | Предположительно да | OK (кроме 2 выше) |

**Итого: 2 неверифицированные ссылки, 0 фабрикованных, но статус PENDING недопустим.**

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

1. **CONCEPT.md: разделы «Falsifiability», «Sample size calculation», «Pre-registration plan»** — удалить все старые версии расчётов. Оставить **единственный, связный блок** с согласованными значениями (например, α=0.001, power=0.95, N=43 per arm, d=1.2). Убедиться, что этот блок не противоречит PARAMETERS.md. Обновить PARAMETERS.md, убрав α=0.05 и power=0.80.

2. **PARAMETERS.md: строка «Statistical parameters»** — изменить α с 0.05 на 0.001, β с 0.20 на 0.05 (power=0.95). Согласовать с CONCEPT.md. Удалить ссылку на Parrinello 2003 без DOI; либо добавить корректный PMID, либо удалить.

3. **KNOWLEDGE.md: заменить «REFERENCE VERIFICATION PENDING» для Lee & Luo 1999** — выполнить ручную верификацию через PubMed, подтвердить PMID 10197526. Если ссылка недоступна или не соответствует контексту, удалить утверждение или заменить на проверенный источник. Удалить все пометки [REF_VERIFY] и [Reference removed pending verification].

4. **CONCEPT.md: раздел «Consortium/partners»** — заменить placeholders «TBD» на хотя бы инициалы реально заинтересованных учёных или указать статус «to be confirmed» с конкретными критериями отбора. Добавить письма поддержки от подтверждённых участников.

5. **CONCEPT.md: устранить внутренние противоречия** — проверить, что все ссылки на N per arm (например, в таблицах и тексте) ведут к одному числу. Убедиться, что в разделе «Falsifiability» таблица согласована с «Unified numeric thresholds» и с расчётом размера выборки. Удалить устаревшие версии (N=6, N=10, N=15, N=24, N=30, N=48). Итоговое значение должно быть одно.

**После исправлений — повторный review с возможным повышением до REVISE_MINOR, если все 9 условий будут выполнены.**