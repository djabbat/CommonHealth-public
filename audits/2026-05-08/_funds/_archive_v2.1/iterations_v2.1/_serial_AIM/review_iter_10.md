# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsifiability: 3
- Deliv: 2
- Novelty: 4
- Risk: 2
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds): ✗**
   - Пороги указаны (PAM-13 Δ ≥ 5.4, α = 0.05, power 0.80, N ≥ 55), но присутствуют множественные противоречивые формулировки α: то 0.05 (two-sided), то 0.025 (Bonferroni-corrected). В одном блоке написано «принято α=0.05, Bonferroni-adjusted α=0.025», в другом — «p < 0.025 per comparison» без указания первичного. Отсутствует единая, однозначная запись. Несоответствие требованиям однозначности.

2. **Pre-registration plan (OSF placeholder + date): ✓**
   - Присутствует placeholder OSF ID (`osf.io/TBD` и `osf.io/XXXXX`) и planned date (2026-09-01). Наличие нескольких placeholders не критично, формально условие выполнено.

3. **Sample size calc (power analysis): ✓**
   - Формула, подстановка значений, результат n=55 per group, sensitivity analysis, учёт drop-out (20%). Выполнено.

4. **Risk matrix ≥5 rows: ✓**
   - Есть матрицы с 5, 7, 5 строками. Формально ≥5 строк есть. Однако матрицы дублируются, содержат разные стили записи (Low/Medium/High vs числа), что нарушает внутреннюю согласованность.

5. **Limitations section: ✓**
   - Явный раздел, перечислено 7–8 пунктов. Выполнено.

6. **Consortium / collaboration plan: ✓**
   - Указаны потенциальные партнёры (Insignia Health, Fraunhofer IGD, TSU, University of Copenhagen) с ролями, хотя много TBD. Формально выполнено.

7. **Reference reality + match: ✗ → АВТОМАТИЧЕСКИЙ REJECT**
   - Три ключевые научные ссылки (`Tao et al. 2026`, `Blumenthal-Lee 2024`, `Tkemaladze 2026`) указаны как «pre-print; DOI TBD» — невалидный идентификатор. Согласно правилу: «Невалидный идентификатор = автоматический REJECT компонента». Также в CONCEPT.md присутствует дублирующаяся ссылка на Hibbard (2004) с неверным PMID 15527447 (ошибка, признанная автором, но не исправленная в тексте). Одна ссылка имеет реальный PMID (15333167) и соответствует тексту. Другая (Hibbard 2005, PMID 15527447) реальна и соответствует. Однако наличие невалидных DOI по трём ссылкам достаточно для REJECT.

8. **No fabrication markers: ✗**
   - «DOI TBD» для трёх статей — это placeholder там, где должны стоять конкретные данные (идентификаторы). По условию 8 placeholder допустим только в pre-registration плане и risk matrix. Здесь же placeholder в списке литературы, что нарушает правило.

9. **Internal consistency core docs: ✗**
   - Множественные противоречия между разделами CONCEPT.md: разные значения α (0.05 vs 0.025), дублирующиеся блоки с разными PMID для одной статьи, разные версии risk matrix (7 строк vs 5 строк), различные варианты limitations и consortium. Отсутствует единая согласованная версия.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Hibbard et al. (2004) PAM development, Health Serv Res | PMID 15333167 | Да | Да | OK |
| 2 | Hibbard et al. (2005) short-form PAM, Health Serv Res | PMID 15527447 | Да | Да | OK |
| 3 | Hibbard et al. (2009) PAM scoring & MCID, Insignia manual | Нет идентификатора | Н/П | Соответствует | Условно OK (технический мануал) |
| 4 | Tao et al. (2026) Co-design RCT, Nature Medicine | DOI TBD | Нет (невалидный ID) | Заявлено, но проверить нельзя | **REJECT** (нарушение п.7) |
| 5 | Blumenthal D., Lee J. (2024) 4-zone HCI framework, JAMA | DOI TBD | Нет (невалидный ID) | Заявлено, но проверить нельзя | **REJECT** (нарушение п.7) |
| 6 | Tkemaladze J. (2026) Patient as a Project, Longevity Horizon | DOI TBD | Нет (невалидный ID) | Заявлено, но проверить нельзя | **REJECT** (нарушение п.7) |
| 7 | Hibbard (2004) с ошибочным PMID (в блоке с ошибкой) | PMID 15527447 | Да (но неверно приписан) | Нет – статья 2005, а не 2004 | Ошибка, но исправлена в другом месте; снижает оценку |

**Комментарий:** Три ссылки с невалидными идентификаторами → автоматический REJECT. Дополнительно присутствует ошибка в PMID, хотя автор её признаёт, но текст содержит оба варианта.

## Top 5 text-level fixes (если бы не REJECT)

1. **CONCEPT.md:Falsifiability** — Удалить все дублирующиеся блоки и оставить единственную формулировку: α=0.05 (two-sided) for primary, Bonferroni-adjusted α=0.025 for ≤2 secondary endpoints. Убрать противоречащие записи (p<0.001, p<0.025 как первичный порог).

2. **CONCEPT.md:Sample size calculation** — Удалить повторяющийся блок с formula и невер