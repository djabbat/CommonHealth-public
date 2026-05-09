# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 3
- Evidence: 2
- Falsifiability: 3
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✓  
   Числовые пороги указаны: PAM-13 Δ ≥ 5.4 (MCID), α = 0.05, power = 0.80, N ≥ 55/группа. Однако присутствует внутреннее противоречие: в одной секции CONCEPT.md α = 0.05 без коррекции, в другой — Bonferroni-adjusted α = 0.025. Это снижает уверенность, но формально пороги есть.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Указан OSF placeholder (osf.io/TBD) и дата (2026-09-01). Формально выполнено.

3. **Sample size calc (power analysis)** — ✓  
   Формула, подстановка значений, результат N=55/группа с учётом 20% dropout — N=132. Присутствует дублирование расчёта, но это не фатально.

4. **Risk matrix ≥5 rows** — ✓  
   В документе несколько матриц, минимум 5 строк присутствуют.

5. **Limitations section** — ✓  
   Явный раздел с 8–9 пунктами. Выделены основные ограничения.

6. **Consortium / collaboration plan** — ✓  
   Список партнёров с ролями присутствует, хотя многие имена заменены на TBD. Формально выполнено.

7. **Reference reality + match** — ✗  
   **Автоматический REJECT.** Несколько ключевых ссылок имеют невалидные идентификаторы (DOI TBD): Tao et al. (2026), Blumenthal-Lee (2024), Tkemaladze (2026). Это прямое нарушение правила: «Невалидный идентификатор = автоматический REJECT компонента». Кроме того, ссылка на Hibbard et al. (2009) (proprietary manual) не имеет DOI/PMID — тоже нарушение. Реальные PMID (15333167, 15527447) верны и соответствуют тексту, но наличие невалидных ссылок делает пункт невыполненным.

8. **No fabrication markers** — ✗  
   В разделах, где должны стоять конкретные данные, используются TBD и placeholder’ы, не разрешённые условием: в consortium (Name TBD), в ссылках (DOI TBD). Placeholder допустим только в pre-reg плане и risk matrix, но не в списке литературы и консорциуме. Нарушение.

9. **Internal consistency core docs** — ✗  
   Множественные противоречия между документами:  
   - Falsifiability: α=0.05 vs α=0.025 (Bonferroni).  
   - Sample size: два разных расчёта в CONCEPT.md (один с исправлением PMID, другой без).  
   - Лабораторные нормы: PARAMETERS.md — 59 аналитов, KNOWLEDGE.md — 71.  
   - Статус KIMI/Qwen: в CONCEPT.md rejected, но README.md содержит устаревшие упоминания.  
   - Дублирование секций (Falsifiability, Pre-registration, Risk matrix и др.) свидетельствует о неотредактированной компиляции черновиков.  
   Внутренняя согласованность отсутствует.

**Итог:** 6/9 условий выполнены (1–6), но пункты 7, 8, 9 провалены. Особенно критично — невалидные идентификаторы ссылок, что ведёт к автоматическому REJECT.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Hibbard et al. (2004) PAM development, *Health Serv Res* | PMID 15333167 | Да (PubMed, Medline) | Да — используется для обоснования σ=10, MCID | OK |
| 2 | Hibbard et al. (2005) short-form PAM, *Health Serv Res* | PMID 15527447 | Да | Да — используется для short-form | OK |
| 3 | Hibbard et al. (2009) PAM scoring & MCID, technical manual | Нет DOI/PMID (proprietary) | Нет — идентификатор отсутствует | Ссылка на технический manual без публичного идентификатора | НЕВАЛИДНАЯ |
| 4 | Tao et al. (2026) Co-design of medical AI, *Nat. Med.* | DOI TBD | Нет | Pre-print, DOI не назначен | НЕВАЛИДНАЯ → REJECT |
| 5 | Blumenthal D., Lee J. (2024) Four-zone HCI, *JAMA* | DOI TBD | Нет | Pre-print, DOI не назначен | НЕВАЛИДНАЯ → REJECT |
| 6 | Tkemaladze J. (2026) Patient as a Project, *Longevity Horizon* | DOI TBD | Нет | Pre-print, DOI не назначен | НЕВАЛИДНАЯ → REJECT |
| 7 | Hibbard et al. (2004) (ошибочный PMID 15527447) | Исправлено | — | Была ошибка, но исправлена | Учтено, не фатально |

**Примечание:** ссылки на интернет-ресурсы (Mayo Clinic, MedlinePlus, WHO, API docs) не являются научными статьями и не проверялись по критерию DOI/PMID. Однако их присутствие не компенсирует невалидные ссылки на статьи.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

1. **CONCEPT.md: секции Falsifiability** — устранить дублирование, унифицировать α (0.05 с Bonferroni или без) и согласовать с THEORY.md. Привести к единому числовому порогу.
2. **CONCEPT.md: Sample size calculation** — удалить дублирующий расчёт с ошибочным PMID. Оставить единственный консистентный блок.
3. **Все файлы: заменить DOI TBD на реальные идентификаторы** (arXiv ID, если pre-print, или указать, что работа в рецензировании с временным DOI). Без этого компонент автоматически отклоняется.
4. **Consortium / partners** — заменить «Name TBD» на реальные имена или хотя бы указать институциональные позиции. Placeholder’ы допустимы только в pre-reg и risk matrix.
5. **PARAMETERS.md и KNOWLEDGE.md** — согласовать количество аналитов (59 vs 71) и привести к единому источнику.

## PACKET

**Итоговое заключение:** Проект имеет сильную теоретическую базу (L3 framework, PAM-13 операционализация, 4-zone HCI) и демонстрирует глубокое понимание предмета. Однако заявка страдает от грубых нарушений требования к ссылкам (невалидные DOI TBD) и внутренних противоречий между core-документами. В соответствии с правилами «автоматический REJECT при не