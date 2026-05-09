# Review of Ze

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3 (интересная интерпретативная рамка, но слабая связь с данными)
- Method: 2 (формализм неоперационален; power analysis с TBD; fabrications markers)
- Evidence: 1 (пилотные данные N<100, гетерогенность I²=90%, нет независимых репликаций)
- Falsif: 1 (числовые пороги не заданы; все effect sizes – TBD)
- Deliv: 2 (консорциум placeholder; нет прототипов; TRL 2)
- Novelty: 4 (оригинальная идея, но исполнение сырое)
- Risk: 2 (высокий: fabrications, неверифицированные ссылки, нет power)

## Checklist (✓/✗ each + explanation)
1. **Operationalised falsifiability (numeric thresholds)** ✗  
   Все effect sizes TBD; нет конкретных N, p, threshold. Таблица «Operational falsifiability» содержит только placeholder. Требование не выполнено.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   Есть два исследования с placeholder ID и датами. Допустимо для заявки.

3. **Sample size calc (power analysis)** ✗  
   Формула есть, но параметры σ и δ – TBD. Расчёт N не проведён. Power analysis отсутствует.

4. **Risk matrix ≥5 rows** ✓  
   6 строк с probability/impact/mitigation. Выполнено.

5. **Limitations section** ✓  
   7 пунктов в CONCEPT.md 2.0.1. Явно указаны.

6. **Consortium / collaboration plan** ✓  
   Список планируемых партнёров (TBD), структура консорциума. Допустимо.

7. **References PubMed/Crossref-verified** ✗  
   Есть PMID 27330520 и DOI, но также присутствует ссылка на s4me.info (форум, не PubMed/Crossref). Fabrication cleanup маркеры указывают на удалённые/неверифицированные ссылки. Не все верифицированы.

8. **No fabrication markers** ✗  
   В KNOWLEDGE.md явно присутствуют `[Reference removed — citation pending verification…]` и `FABRICATION CLEANUP applied`. Это fabrication markers.

## Top 5 text-level fixes (обязательно)

1. **`KNOWLEDGE.md: все fabrication markers` – удалить полностью**  
   Заменить все `[Reference removed …]` на корректные PMID/DOI или удалить строки. Написать вначале: «All references have been verified against PubMed/Crossref as of <date>».

2. **`CONCEPT.md: "Operational falsifiability" + "Sample size calculation"` – задать конкретные числа**  
   Заменить все TBD на числовые значения на основе пилотных данных или литературы (например, Cohen's d=0.5 для v*_active, σ=0.2). Пересчитать N. Даже если оценки грубые, они должны быть явными.

3. **`CONCEPT.md: v*_active – статистический план` – реализовать bootstrap per dataset**  
   Убрать pooled estimate (I²=90%). Дать per‑dataset bootstrap CI (BCa) для Cuban (N=88). Указать: «We will compute v*_active as the median of Cuban EEG bootstrap distribution (95% CI pending); cross-cohort generalisability uncertain».

4. **`CONCEPT.md: MCID` – удалить числовые пороги (0.05) из всех разделов, кроме рабочей гипотезы**  
   Заменить на «MCID to be determined in WP2 with anchor‑based method (N≥100)». Убрать все упоминания «0.05» как MCID.

5. **`PARAMETERS.md + KNOWLEDGE.md` – привести в соответствие операциональное определение**  
   Явно разграничить теоретическое v*_passive=1−ln2 и эмпирическое v*_active≈0.456 (Cuban). Указать единицы и контекст. Добавить предупреждение: «The value 0.456 is dataset‑specific and not yet validated as universal».

---

**Примечание:** Без выполнения этих исправлений проект не может быть рассмотрен для финансирования. Особенно критично удаление fabrication markers – это делает заявку неприемлемой для ERC/Wellcome. После исправления всех пунктов возможен повторный ревью с повышением до REVISE_MINOR или FUND_AS_IS.