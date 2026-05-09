# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 1
- Falsif: 2
- Deliv: 2
- Novelty: 4
- Risk: 2
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)
1. **Operationalised falsifiability (numeric thresholds):** ✗  
   Формально пороги есть (MCID 5.4, α=0.05, power 0.80, N≥55). Однако внутри одного документа (`CONCEPT.md`) и в `THEORY.md` встречается противоречащее утверждение: «p < 0.001 (Bonferroni-adjusted)» для того же самого primary endpoint. Это нарушает однозначность фальсифицируемости. Плюс Bonferroni применяется к вторичным endpoint'ам, но primary анализ указан как two-sided α=0.05. Наличие p<0.001 создаёт неразрешимое противоречие – какой порог на самом деле. Условие не выполнено.

2. **Pre-registration plan (OSF placeholder + date):** ✓  
   Указан OSF ID `osf.io/TBD` (single placeholder), запланированная дата 2026-09-01, дизайн, первичный исход, анализ. Все остальные OSF placeholders удалены. Соответствует.

3. **Sample size calc (power analysis):** ✓  
   Формула, подстановка (Z_α/2=1.96, Z_β=0.84, σ=10, δ=5.4 → n≈55), учтён 20% dropout → total N=132. Приведена sensitivity analysis. Обоснование σ=10 через литературу. OK.

4. **Risk matrix ≥5 rows:** ✓  
   В файле представлено несколько вариантов матрицы, одна содержит 7 строк (с вероятностью/влиянием словами), другая – 5 строк с числовыми scores. Минимум есть. Однако дублирование и несогласованность между ними (разные риски, разные метрики) снижает качество.

5. **Limitations section:** ✓  
   Присутствует явный раздел, перечислено 8 ограничений (single-centre, self-report, digital literacy, Hawthorne, placebo, σ assumption, reference integrity и т.д.). Без приукрашиваний.

6. **Consortium / collaboration plan:** ✓  
   Указаны Lead PI, Co-I (клинический, технический), потенциальные партнёры (Insignia Health, Fraunhofer IGD, TSU, UCPH) с указанием статуса (в переговорах, подтверждено). Placeholder для имён допустим.

7. **Reference reality + match:** ✗  
   **Критическое нарушение.** Несколько ключевых ссылок не имеют реального идентификатора:  
   - Tao et al. (2026) *Nature Medicine* – «pre-print; DOI TBD»  
   - Blumenthal-Lee (2024) *JAMA* – «pre-print; DOI TBD»  
   - Tkemaladze J. (2026) *Longevity Horizon* – «DOI TBD»  
   - Hibbard et al. (2009) *PAM scoring & MCID* – proprietary manual, без DOI/PMID.  
   Кроме того, для Hibbard 2004 в разных местах указаны два разных PMID: 15333167 (корректный) и 15527447 (на самом деле Hibbard 2005 – несоответствие утверждению). Таким образом, условие не выполнено.

8. **No fabrication markers:** ✗  
   Обнаружено внутреннее противоречие: в разделе Falsifiability указано «p < 0.001», что не соответствует заявленному α=0.05. Это похоже на остаточную ошибку (возможно, от старой версии). Также имеется неверный PMID для Hibbard 2004 (15527447 вместо 15333167). Формально это не [REF_NEEDED], но является маркером недоделанности/фабрикации. Условие не выполнено.

9. **Internal consistency core docs:** ✗  
   Противоречия между `CONCEPT.md` и `THEORY.md`:  
   - Разный α-порог (0.05 vs 0.001).  
   - Разный PMID для одной и той же работы.  
   - Risk matrix и Limitations представлены в нескольких несогласованных версиях (дублирование, разные формулировки, разный набор рисков).  
   - Статус KIMI/Qwen: в `CONCEPT.md` явно указано «rejected 2026-05-07», в `README.md` они всё ещё упоминаются как rejected, но без единой ссылки на решение. Хотя это не критическое противоречие, общая несогласованность документов высока.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Hibbard JH et al. (2004). Development of the Patient Activation Measure. *Health Serv Res* 39(4 Pt 1):1005–26. | PMID 15333167 | Да | В одном месте указан корректный PMID, в другом – PMID 15527447 (статья 2005 г.). Несоответствие: текст ссылается на 2004, а PMID ведёт на 2005. | [REF_VERIFY:15333167] – реальна, но несоответствие утверждению в части документа. |
| 2 | Hibbard JH et al. (2005). Development and testing of a short form of the patient activation measure. *Health Serv Res* 40(6 Pt 1):1918–30. | PMID 15527447 | Да | Корректно – это 2005 год. | OK |
| 3 | Hibbard JH et al. (2009). PAM scoring & MCID. Insignia Health technical manual. | Нет DOI/PMID (proprietary) | Нет (не публичная запись) | Используется для обоснования MCID=5.4. Непроверяемая ссылка. | [REF_VERIFY:proprietary] – не принимается как научная ссылка. |
| 4 | Tao W. et al. (2026). Co-design of medical AI improves patient activation: RCT of 2069 patients. *Nature Medicine*. | DOI TBD (pre-print) | Нет | Утверждается, что это подтверждение L2. Нет публичной записи. | REJECT (нереальная ссылка) |
| 5 | Blumenthal D., Lee J. (2024). Four-zone framework for human-AI clinical collaboration. *JAMA*. | DOI TBD (pre-print) | Нет | Утверждается как основа 4-zone HCI. Нет публичной записи. | REJECT (нереальная ссылка) |
| 6 | Tkemaladze J. (2026). Patient as a Project. *Longevity Horizon* 2(5). | DOI TBD | Нет | Является cornerstone framework для всей AIM. Не опубликована. | REJECT (нереальная ссылка) |
| 7 | Hibbard 2004 (в другом месте с PMID 15527447) | PMID 15527447 | Да (но это 2005, а не 2004) | Не соответствует – указан 2004 год, но PMID ведёт на 2005. | [REF_VERIFY:15527447] – несоответствие тексту. |

**Итог:** 3 ссылки полностью нереальны (TBD), 2 ссылки имеют несоответствие утверждению, 1 ссылка непроверяема (proprietary). Reference Integrity score = 1.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)
1. **AIM/CONCEPT.md:Раздел Falsifiability** – Убрать противоречащее утверждение «p < 0.001». Оставить единый порог α=0.05 (two-sided) для primary endpoint. Указать, что Bonferroni применяется только к secondary endpoints с α=0.025; никаких других α не используется.  
2. **AIM/CONCEPT.md:Ссылка на Hibbard 2004** – Заменить неверный PMID 15527447 на правильный 15333167 во втором вхождении. Провести полную сверку всех PMID по документам.  
3. **AIM/THEORY.md / CONCEPT.md** – Заменить все ссылки с «DOI TBD» на реальные идентификаторы опубликованных работ. Если работы ещё не опубликованы, убрать ссылки или указать их как «в печати/подготовке» с обоснованием, но не как подтверждающие evidence.  
4. **AIM/CONCEPT.md:Risk matrix и Limitations** – Унифицировать: оставить одну согласованную таблицу рисков (5–7 строк) и один список ограничений. Удалить дублирующиеся разделы. Привести к единому стилю (probability/impact словами или числами).  
5. **AIM/CONCEPT.md §0 / THEORY.md** – Согласовать все числовые константы (α, power, MCID) между core-документами. Провести automated audit на соответствие `config.py` и текстовых описаний.

## PACKET
Полученные core-документы (CONCEPT, THEORY, PARAMETERS, KNOWLEDGE, README) демонстрируют амбициозную концепцию гибридного AI-ассистента с фреймворком уровней пациент-объект/нарратор/проект. Однако заявленная научная строгость подрывается:

- неразрешёнными внутренними противоречиями