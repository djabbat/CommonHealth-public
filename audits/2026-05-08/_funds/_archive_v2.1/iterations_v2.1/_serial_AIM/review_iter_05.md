# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 1
- Falsifiability: 2
- Deliv: 2
- Novelty: 3
- Risk: 2
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** ✗  
   Даны пороги (PAM-13 Δ ≥ 5.4, α=0.05, power=0.80, N≥55), но α противоречив: в одном месте указано α=0.05, в другом α=0.025 (Bonferroni), в THEORY.md фигурирует p<0.001 (хотя потом удалено), а в CONCEPT.md §0 говорится о едином α=0.05. Нет единого непротиворечивого порога. Требуются числа, и они должны быть согласованы.

2. **Pre-registration plan (OSF placeholder + date)** ✗  
   В CONCEPT.md и THEORY.md даны разные OSF ID: `osf.io/TBD`, `osf.io/XXXXX`. Нет конкретного placeholder, который можно проверить. Дата 2026-09-01 — плановая, но placeholder не уникален и не соответствует стандарту OSF (должен быть 5-символьный код или "TBD" один раз). Наличие двух разных записей — нарушение согласованности.

3. **Sample size calc (power analysis)** ✗  
   Формула и расчёт есть (N=55 per group), но стандартное отклонение σ=10 обосновано ссылкой на Hibbard 2004 с неверным PMID (15527447 вместо 15333167). Правильный PMID указан в другом месте, но дублирование с ошибкой — fabrication marker. Кроме того, формула продублирована дважды в CONCEPT.md, что избыточно, а в THEORY.md расчёт повторён с теми же параметрами. Отсутствует единый авторитетный расчёт.

4. **Risk matrix ≥5 rows** ✗  
   В CONCEPT.md приведено несколько разных матриц (одна 7 строк, другая 7 строк, третья 5 строк). Они не согласованы: различаются вероятности, воздействия, митигации. Например, "LLM hallucination" оценивается как Medium/High, Low/Critical, Medium/High в разных местах. Нет единого документа; матрицы несовместимы.

5. **Limitations section** ✗  
   Раздел Limitations присутствует в CONCEPT.md (8 пунктов) и THEORY.md (7 пунктов), но формулировки и количество пунктов различаются. Нет единого согласованного списка. Например, в CONCEPT.md упоминается "reference integrity: one reference (Hibbard 2004) had incorrect PMID", что является признанием ошибки, но не исправлением в самом документе. В THEORY.md этого пункта нет.

6. **Consortium / collaboration plan** ✗  
   В CONCEPT.md есть список партнёров с ролью "Lead PI: [Name TBD]", "Co-I: [Name TBD]" и "Potential partners (letters of support pending)". В THEORY.md дан другой список с Dr. Tkemaladze и Dr. Samanishvili. Нет единого документа с указанием ролей каждого. Placeholder TBD в именах — fabrication marker. Письма поддержки не получены.

7. **Reference reality + match** ✗  
   Проведён референс-аудит (см. таблицу ниже). Большинство ссылок невалидны: многие имеют "DOI TBD" или "pre-print; DOI TBD", один PMID неверен, один источник проприетарный (не верифицируется). Ни одна ссылка не соответствует требованию "реальная и соответствующая тексту" для всех цитат.

8. **No fabrication markers** ✗  
   Fabrication markers присутствуют массово:  
   - `[Name TBD]` в consortium (3 раза)  
   - `DOI TBD` для Tkemaladze (2026), Tao et al. (2026), Blumenthal-Lee (2024)  
   - `[pre-print; DOI TBD]`  
   - `osf.io/TBD` и `osf.io/XXXXX`  
   - `PMID_REMOVED` не обнаружено, но `15527447` вместо `15333167` — ошибка, эквивалентная fabrication.  
   - `letter of support pending`  
   - `sigma estimate off by >20% from pilot` — неопределённость, но не fabrication.

9. **Internal consistency core docs** ✗  
   Фундаментальные противоречия между CONCEPT.md и THEORY.md:  
   - α: 0.05 vs. 0.025 vs. 0.001 (удалено)  
   - OSF ID: два разных placeholder  
   - Sample size: дублирование, неверный PMID  
   - Risk matrix: разные версии, несовместимые  
   - Limitations: разный состав  
   - Consortium: разные PI и Co-I.  
   - Sample size justification: CONCEPT.md ссылается на Hibbard 2004 (PMID 15527447 — ошибка), THEORY.md — на тот же PMID? В THEORY.md нет явной ссылки, но есть запись про ошибку. Это нарушение.

**Итог:** 0 из 9 условий выполнено. Вердикт REJECT.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|------------------------|---------|
| 1 | Hibbard JH et al. (2004) "Development of the Patient Activation Measure" | PMID 15333167 (Health Serv Res 39(4 Pt 1):1005–26) | Да | Да (используется для обоснования SD) | ✅ |
| 2 | Hibbard JH et al. (2005) "Development and testing of a short form of the patient activation measure" | PMID 15527447 (Health Serv Res 40(6 Pt 1):1918–30) | Да | Упоминается в тексте как "short-form PAM development" – соответствует | ✅ |
| 3 | Hibbard JH et al. (2009) "PAM scoring & MCID" | Insignia Health technical manual (proprietary) | Нет (не общедоступна, нет DOI/PMID) | Утверждается, что MCID = 5.4; но ссылка непроверяема | ❌ REF_VERIFY |
| 4 | Tao W. et al. (2026) "Co-design of medical AI improves patient activation: RCT of 2069 patients" | "Nature Medicine" + "pre-print; DOI TBD" | Нет (DOI TBD, не опубликована) | Утверждается, что co-design > fine-tuning – не может быть подтверждено | ❌ REF_VERIFY |
| 5 | Blumenthal D., Lee J. (2024) "Four-zone framework for human-AI clinical collaboration" | "JAMA" + "pre-print; DOI TBD" | Нет (DOI TBD, не опубликована) | Утверждается 4-zone HCI – не может быть подтверждено | ❌ REF_VERIFY |
| 6 | Tkemaladze J. (2026) "Patient as a Project: Three-level framework for AI-assisted integrative medicine" | "Longevity Horizon 2(5)" + "DOI TBD" | Нет (DOI TBD, не опубликована) | Утверждается трехуровневая модель – не может быть подтверждено | ❌ REF_VERIFY |
| 7 | Hibbard JH et al. (2004) – ошибочный PMID | PMID 15527447 (в CONCEPT.md образец #2) | Неверно: PMID 15527447 – это Hibbard 2005, не 2004 | Не соответствует тексту (2004 paper утверждается, но дан PMID 2005) | ❌ REJECT |

**Вывод:** 2 из 7 ссылок реальны и соответствуют тексту (Hibbard 2004 правильный PMID, Hibbard 2005). Остальные 5 – невалидны (DOI TBD, проприетарный источник, ошибочный PMID). Reference Integrity Score: 1.

## Top 5 text-level fixes (обязательные для любого future submission)

1. **CONCEPT.md §0 / THEORY.md §1: Устранить противоречие по α**  
   Выбрать единый уровень значимости (рекомендуется α=0.05 two-sided первичный, Bonferroni α=0.025 для вторичных) и удалить все упоминания p<0.001. Сделать это во всех core-документах.

2. **CONCEPT.md: Pre-registration plan – заменить на единый OSF placeholder**  
   Использовать один идентификатор, например `osf.io/xxxxx` (5 случайных символов), и указать, что он будет зарезервирован до 2026-08-01. Удалить второй placeholder.

3. **THEORY.md: Заменить ссылки с "DOI TBD" на реальные публикации или удалить**  
   Для Tao et al. 2026, Blumenthal-Lee 2024, Tkemaladze 2026 – указать либо опубликованные DOI, либо удалить эти утверждения, заменив их на рецензированные источники. Если статьи ещё не опубликованы, нельзя на них ссылаться как на доказательства.

4. **CONCEPT.md: Исправить неверный PMID для Hibbard 2004**  
   Заменить PMID 15527447 на 15333167 во всех местах. Удалить дублирующийся блок Sample size calculation.

5. **CONCEPT.md: Привести Risk matrix и Limitations к единому виду**  
   Оставить одну согласованную таблицу рисков (≥5 строк), одну секцию Limitations (≥5 пунктов), совпадающую в CONCEPT.md и THEORY.md. Имена в Consortium заменить на конкретные (не TBD) или явно указать "to be named".