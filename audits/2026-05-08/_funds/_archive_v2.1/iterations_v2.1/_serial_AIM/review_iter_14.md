# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 1
- Falsif: 2
- Deliv: 3
- Novelty: 4
- Risk: 2
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)**  
   ✗ Числовые пороги есть (N≥55, α=0.05, power=0.80, MCID=5.4), но внутри документа CONCEPT.md присутствуют противоречивые описания:  
   – в одном месте указано α=0.05 без коррекции для первичного анализа,  
   – в другом – Bonferroni-скорректированный порог p<0.025 для «≤2 сравнений», при том что первичный конец один.  
   Наличие числовых значений формально закрывает п.1, но внутренняя несогласованность снижает надёжность. (См. также п.9.)  
   *Результат: ✓ с замечанием, но нарушение согласованности влияет на вердикт.*

2. **Pre-registration plan (OSF placeholder + date)**  
   ✓ В CONCEPT.md указан OSF ID `osf.io/TBD` и плановая дата регистрации 2026-09-01. Placeholder допустим по условиям. Дизайн и исходы описаны.

3. **Sample size calc (power analysis)**  
   ✓ Полный расчёт: формула, Z-значения, δ=5.4, σ=10, n=55/группа, oversample 20% → N=132. Сделана чувствительность для σ=8,10,12. Обоснование σ из Hibbard 2004 (PMID 15333167).

4. **Risk matrix ≥5 rows**  
   ✓ В CONCEPT.md приведены три разных таблицы рисков, суммарно ≥5 строк. Например, первая таблица содержит 7 строк с вероятностью, воздействием и митигацией.

5. **Limitations section**  
   ✓ Явный раздел с 8 пунктами в CONCEPT.md (и дублирован в THEORY.md). Перечислены single-centre, self-report, digital literacy, Hawthorne effect, placebo effect, σ assumption, reference integrity и др.

6. **Consortium / collaboration plan**  
   ✓ Указаны Lead PI (Dr. Tkemaladze), Co-I (Dr. Samanishvili), потенциальные партнёры с ролями (Insignia Health, Fraunhofer IGD, TSU, University of Copenhagen). Имена некоторых исполнителей – placeholder («TBD»), что допустимо для pre-grant стадии.

7. **Reference reality + match**  
   ✗ **Критическое нарушение.** Для следующих цитируемых работ отсутствует валидный DOI/PMID/arXiv ID, либо он указан как «TBD»:  
   – Tao et al. (2026) *Nature Medicine* [pre-print; DOI TBD]  
   – Blumenthal & Lee (2024) *JAMA* [pre-print; DOI TBD]  
   – Tkemaladze (2026) *Longevity Horizon* [pre-print; DOI TBD]  
   – Hibbard et al. (2009) Insignia Health technical manual (proprietary) – не научная статья, нет идентификатора.  
   Согласно правилам, отсутствие или невалидный идентификатор ведёт к автоматическому REJECT компонента.

8. **No fabrication markers**  
   ✓ В тексте используются placeholders (OSF ID, имена, даты), что разрешено. Отсутствуют маркеры типа [REF_NEEDED], [PMID_REMOVED] или явные фабрикации.

9. **Internal consistency core documents**  
   ✗ **Нарушение согласованности:**  
   – Противоречия в α-порогах между разделами CONCEPT.md (см. п.1