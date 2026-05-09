# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3 – идея L3-валидации через PAM-13 интересна, но плохо операционализирована
- Method: 1 – расчёт выборки содержит TBD, множественные внутренние противоречия
- Evidence: 1 – большинство цитируемых работ не имеют разрешающихся идентификаторов (DOI/PMID); core reference (Tao 2026) не верифицируем
- Falsifiability: 2 – числовые пороги есть, но σ placeholder, sensitivity TBD → не полностью фальсифицируемо
- Deliv: 1 – документация содержит дублирующиеся/конфликтующие версии, placeholder TBD в недопустимых местах
- Novelty: 3 – концепция L3 новая, но не подкреплена ни одним проверяемым источником
- Risk: 3 – матрицы риска есть, но перегружены дубликатами
- RefIntegrity: 1 – см. Reference audit (только 2 из ≥7 ссылок имеют реальные PMID; остальные без идентификатора)

## Checklist (✓/✗ + объяснение по каждому из 9 условий)
1. **✗ Operationalised falsifiability (numeric thresholds)** — пороги объявлены (Δ≥5.4, α=0.05, power=0.80, N≥55), но расчёт выборки содержит «σ (pooled SD): TBD» и «Sensitivity analysis: TBD», что делает числовые пороги не полностью операционализированными. Присутствие TBD в ключевых параметрах — грубое нарушение.
2. **✓ Pre-registration plan (OSF placeholder + date)** — есть «osf.io/TBD / XXXX» и дата 2026-09-01. Плейсхолдер допустим.
3. **✗ Sample size calc (power analysis)** — формула и подстановка есть, но σ=10 имеет пометку «pending confirmation» и «TBD» в одном из блоков. Sensitivity analysis не завершена. Не выполнено.
4. **✓ Risk matrix ≥5 rows** — присутствуют несколько матриц по 7 строк (все ≥5).
5. **✓ Limitations section** — есть отдельные разделы с 7–8 пунктами. OK.
6. **✓ Consortium / collaboration plan** — есть список партнёров (Lead PI TBD, Co-I TBD, Insignia, Fraunhofer, TSU, Copenhagen) с ролями. Плейсхолдеры допустимы.
7. **✗ Reference reality + match** — см. таблицу ниже. Из 7 идентифицируемых ссылок только 2 PMID реальны и соответствуют тексту. Остальные не имеют DOI/PMID, одна ссылка (Hibbard 2005) использована для обоснования σ=10, хотя в тексте написано Hibbard 2004 (несоответствие). REJECT по этому пункту однозначно.
8. **✗ No fabrication markers** — в разделе «Sample size calculation» CONCEPT.md присутствуют «TBD» для σ и sensitivity analysis, а также в консорциуме «[Name TBD]» – последнее допустимо, но σ TBD – нет. Также в тексте «Hypothetical example» и «Example only» для расчёта (косвенный маркер). Fabrication marker налицо.
9. **✗ Internal consistency core docs** — заметные противоречия:  
   - CONCEPT.md vs THEORY.md: набор secondary outcomes различается (MMAS-8/EQ-5D-5L/hospitalisations vs physician time per visit); сами авторы отмечают это как проблему.  
   - В CONCEPT.md дублируются секции с разными значениями (σ TBD vs σ=10, разные PF).  
   - README.md утверждает, что KIMI/Qwen rejected, но в PARAMETERS.md они всё ещё в таблице (хотя с пометкой not implemented).  
   - Falsifiability thresholds указаны минимум трижды с различиями (α-corrected vs uncorrected).  
   Внутренняя связанность документов нарушена.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Hibbard JH et al. (2004) PAM development, *Health Serv Res* 39(4 Pt 1):1005–26 | PMID 15333167 | Да (PubMed) | Tекст ссылается на PAM-13 и SD от 8 до 12. DOI: в PubMed есть, PMID ведёт на статью. Соответствует. | ✓ |
| 2 | Hibbard JH et al. (2005) Short-form PAM, *Health Serv Res* 40(6 Pt 1):1918–30 | PMID 15527447 | Да (PubMed) | Используется в одном блоке для обоснования σ=10, хотя в том же абзаце указано “Hibbard 2004” – неконсистентно. Соответствие частичное. | ⚠ [REF_VERIFY:PMID 15527447] – бумага 2005, а в тексте написано Hibbard 2004. Ошибка. |
| 3 | Hibbard JH et al. (2009) PAM scoring & MCID, Insignia Health technical manual | Нет DOI/PMID | Нет идентификатора | Утверждается MCID=5.4. Прямая проверка невозможна. | ✗ – отсутствие идентификатора |
| 4 | Tao W. et al. (2026) Co-design of medical AI… *Nature Medicine*, n=2069 RCT | Нет DOI/PMID (указан “pre-print; DOI TBD”) | Не верифицируется | Используется в cornerstone как подтверждение co-design > fine-tuning. Без идентификатора проверить нельзя. | ✗ – отсутствие идентификатора |
| 5 | Blumenthal D., Lee J. (2024) Four-zone framework… *JAMA* | Нет DOI/PMID (указан “pre-print; DOI TBD”) | Не верифицируется | Утверждение про 4-zone HCI. | ✗ – отсутствие идентификатора |
| 6 | Tkemaladze J. (2026) Patient as a Project, *Longevity Horizon* 2(5) | Нет DOI/PMID (указа