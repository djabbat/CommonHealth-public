# Review of AIM

## Verdict
**REVISE_MAJOR**

## Scores (1–5)
- Premise: 3  
- Method: 4  
- Evidence: 2  
- Falsif: 3  
- Deliv: 3  
- Novelty: 4  
- Risk: 4  
- RefIntegrity: 2  

## Checklist (✓/✗ + объяснение)

1. **Operationalised falsifiability (numeric thresholds)**  
   **✗**  
   Первичный порог PAM-13 Δ ≥ 5.4, α = 0.05, power = 0.80, N ≥ 55 — присутствуют. Однако внутри раздела Falsifiability есть прямое противоречие: сначала указано α = 0.05 (two-sided), Bonferroni-adjusted α = 0.025 для вторичных конечных точек, а затем написано, что H₀ будет отвергнута при p < 0.001. Два разных α для одного и того же анализа — недопустимо. Несогласованность числовых порогов.

2. **Pre-registration plan (OSF placeholder + date)**  
   **✓**  
   Указан OSF ID (osf.io/XXXXX и osf.io/TBD — два разных плейсхолдера, что мелкая небрежность, но не нарушение). Дата 2026-09-01 присутствует. Условие формально выполнено.

3. **Sample size calc (power analysis)**  
   **✓**  
   Формула, подстановка чисел, результат 55 на группу (132 с отсевом). Чувствительность для разных σ. Обоснование σ = 10, хотя привязка к PMID путаная. Условие выполнено.

4. **Risk matrix ≥ 5 rows**  
   **✓**  
   В CONCEPT.md представлена таблица из 7 строк с Probability, Impact, Mitigation. Условие выполнено.

5. **Limitations section**  
   **✓**  
   Есть явный раздел с 8 пунктами. Условие выполнено.

6. **Consortium / collaboration plan**  
   **✓**  
   Указаны Lead PI, Co-I, потенциальные партнеры (Insignia Health, Fraunhofer IGD, TSU, Univ. Copenhagen). Есть статус переговоров. Условие выполнено.

7. **Reference reality + match**  
   **✗**  
   - **Hibbard 2004** (PMID 15333167) — реальна, но в тексте Sample size calculation ошибочно приписана к PMID 15527447 (который относится к Hibbard 2005). Путаница: один и тот же факт (SD PAM-13 range 9–11) приписывается то 15333167, то 15527447. Несоответствие тексту.  
   - **Tao et al. (2026) *Nature Medicine*** — указан «DOI TBD». Невалидный идентификатор. Статья не проверяема.  
   - **Blumenthal-Lee (2024) *JAMA*** — указан «DOI TBD». Невалидный идентификатор.  
   - **Tkemaladze J. (2026) *Longevity Horizon*** — указан «DOI TBD». Невалидный идентификатор.  
   - **Hibbard 2009 (technical manual)** — без PMID/DOI, указано как «proprietary».  
   - Все три препринта не имеют реального идентификатора → автоматический REJECT по условию 7(a).  
   - Кроме того, утверждение «Co-design > fine-tuning (Tao et al., Nat Med 2026 [pre-print; DOI TBD])» не может быть проверено.  
   → Условие не выполнено.

8. **No fabrication markers**  
   **✓ (условно)**  
   Нет скрытых [REF_NEEDED] или [PMID_REMOVED]. Плейсхолдеры (osf.io/XXXXX, DOI TBD, Name TBD) явно обозначены. Однако дублирование целого блока Sample size calculation (дважды повторяется один текст) свидетельствует о неаккуратном редактировании, но формально не является фабрикационным маркером.

9. **Internal consistency core docs**  
   **✗**  
   Два критических противоречия:  
   - Falsifiability: α = 0.05 vs p < 0.001 для первичного анализа.  
   - Sample size: дважды повторён блок с разными PMID для обоснования σ (один раз с правильным PMID 15333167, другой с ошибочным 15527447).  
   - Pre-registration: два разных плейсхолдера OSF (XXXXX и TBD).  
   Эти противоречия нарушают согласованность между CONCEPT.md и THEORY.md.

## Reference audit

| # | Цитата | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|--------|----------------|----------|-----------------------|---------|
| 1 | Hibbard et al. (2004) *Development of the Patient Activation Measure* | PMID 15333167 | Да | Да (для PAM-13 концепции) | OK |
| 2 | Hibbard et al. (2005) *Development and testing of a short form* | PMID 15527447 | Да | Да (для short-form PAM) | OK |
| 3 | Hibbard et al. (2004) – упоминание в Sample size calculation как PMID 15527447 | PMID 15527447 | Да, но неверно приписана к 2004 году | Нет – 2004 работа имеет PMID 15333167, а не 15527447 | Mismatch |
| 4 | Tao W. et al. (2026) *Co-design of medical AI improves patient activation* | DOI TBD | Нет | Не проверяемо | Reject |
| 5 | Blumenthal D., Lee J. (2024) *Four-zone framework* | DOI TBD | Нет | Не проверяемо | Reject |
| 6 | Tkemaladze J. (2026) *Patient as a Project* | DOI TBD | Нет | Не проверяемо | Reject |
| 7 | Hibbard et al. (2009) *PAM scoring & MCID* | Proprietary, нет PMID | Не проверяемо | Не проверяемо | Flag (proprietary) |

**Итог:** 4 из 7 ссылок не имеют разрешающегося идентификатора или не соответствуют тексту.

## Top 5 text-level fixes

1. **CONCEPT.md:§Falsifiability** – устранить противоречие между α = 0.05 и p < 0.001. Выбрать единый порог (рекомендуется α = 0.05 с Bonferroni-коррекцией для вторичных; p < 0.001 удалить).  
2. **CONCEPT.md:Sample size calculation** – удалить дублирующий блок. Указать единый PMID для σ = 10 (правильный 15333167) и добавить примечание, что в ранних версиях была опечатка.  
3. **CONCEPT.md:Pre-registration plan** – унифицировать OSF placeholder (оставить один `osf.io/TBD` или `osf.io/XXXXX`).  
4. **CONCEPT.md:§ References / THEORY.md:§ Ссылки** – заменить все «DOI TBD» на реальные идентификаторы (минимум arXiv ID или DOI препринтов, если они существуют). Для препринтов без присвоенного DOI – указать arXiv ID. Если таковых нет, удалить ссылку или заменить на рецензированные работы с валидным PMID/DOI.  
5. **CONCEPT.md:Risk matrix** – проверить, что все риски уникальны (в документе приведены две разные таблицы – осталась только одна).

## PACKET
AIM

**Verdict:** REVISE_MAJOR.  
**Требуется:** исправить все перечисленные несоответствия в Core-документах, обеспечить валидные идентификаторы для каждой цитируемой работы, устранить внутренние противоречия. После повторной проверки возможен переход к REVISE_MINOR или FUND_AS_IS.