# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsifiability: 3
- Deliv: 2
- Novelty: 3
- Risk: 2
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✗  
   Числовые пороги присутствуют (PAM‑13 Δ ≥5.4, α=0.05, power=0.80, N≥55), но α несогласована: в одних местах 0.05, в других Bonferroni‑скорректированная 0.025, в третьих p<0.025 per comparison. THEORY.md указывает просто α=0.05 без поправки. Нет единого, непротиворечивого набора порогов. Условие не выполнено из‑за внутреннего противоречия.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Указан `osf.io/TBD` (placeholder) и planned date 2026‑09‑01. План зарегистрированного дизайна описан. Placeholder допустим, но должен быть заменён до начала набора.

3. **Sample size calc (power analysis)** — ✓  
   Формула приведена, параметры подставлены, расчёт выполнен (n=55/group), учтён dropout, сделана sensitivity analysis. Удовлетворяет.

4. **Risk matrix ≥5 rows** — ✓  
   Матрицы есть, включая одну с 7 строками. Формально ≥5 строк. Однако матрицы дублируются и различаются по содержанию — проблема согласованности, но само условие выполнено.

5. **Limitations section** — ✓  
   Раздел присутствует в нескольких копиях. Перечислены основные ограничения. Условие засчитывается.

6. **Consortium / collaboration plan** — ✓  
   Указаны Lead PI, Co‑I, потенциальные партнёры (Insignia Health, Fraunhofer IGD, TSU, Копенгагенский университет). Роли описаны. План есть, хотя многие пункты TBD.

7. **Reference reality + match** — ✗**  
   **Невалидные идентификаторы (DOI TBD) для трёх ссылок → автоматический REJECT компонента.**  
   - Tao et al. 2026 *Nature Medicine* — DOI TBD  
   - Blumenthal‑Lee 2024 *JAMA* — DOI TBD  
   - Tkemaladze 2026 *Longevity Horizon* — DOI TBD  
   Эти ссылки не могут быть проверены на реальность, что прямо запрещено правилом 7(a).  
   Кроме того, путаница с PMID для Hibbard 2004 (в тексте встречается как 15333167, так и ошибочно 15527447) свидетельствует о несоответствии.

8. **No fabrication markers** — ✗  
   Присутствуют множественные TBD и placeholders: `osf.io/TBD`, `[Name TBD]`, `[pre‑print; DOI TBD]`, `Letters of support pending`. Это fabrication markers, нарушающие условие.

9. **Internal consistency core docs** — ✗  
   Серьёзные противоречия между CONCEPT.md, THEORY.md и внутри самих файлов:  
   - α‑уровень различается (0.05 vs 0.025 vs p<0.025 per comparison)  
   - Sample size justification использует разные PMID для одной работы  
   - Risk matrix даны в трёх несовпадающих версиях  
   - Limitations и pre‑registration также дублируются с различиями  
   - Vaporware (KIMI/Qwen) то исключены, то упоминаются в README  
   Документы не прошли единую вычитку, что нарушает условие.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|-----------------------|---------|
| 1 | Hibbard et al. (2004) PAM development | PMID 15333167 | ✅ Да | ✅ Да (разработка PAM‑13) | OK |
| 2 | Hibbard et al. (2005) short‑form PAM | PMID 15527447 | ✅ Да | ✅ Да | OK |
| 3 | Hibbard et al. (2009) PAM scoring/MCID | (proprietary manual) | N/A (не статья) | Упоминается MCID 5.4 | Допустимо, но не проверяемо |
| 4 | Tao et al. (2026) *Nature Medicine* co‑design RCT | DOI TBD | ❌ Нет (не указан) | Утверждается RCT n=2069 | **REJECT** – нереальная ссылка |
| 5 | Blumenthal‑Lee (2024) *JAMA* 4‑zone HCI framework | DOI TBD | ❌ Нет (препринт без ID) | Цитируется как основа HCI | **REJECT** – нереальная ссылка |
| 6 | Tkemaladze (2026) *Longevity Horizon* patient‑as‑project | DOI TBD | ❌ Нет (препринт без ID) | Является концептуальной основой | **REJECT** – нереальная ссылка |
| 7 | Hibbard 2004 (ошибочный PMID в тексте) | PMID 15527447 (ошибочно приписан) | ✅ (PMID реален, но не соответствует тексту) | ❌ Нет – этот PMID относится к 2005 short‑form, а не к 2004 | **REF_VERIFY** – несоответствие |

**Итог:** 3 нереальные ссылки (DOI TBD) → автоматический REJECT. 1 несоответствие (PMID 15527447 приписан неверно).

## Top 5 text-level fixes (для исправления, если бы был шанс на REVISE_MAJOR)

1. **CONCEPT.md:** Заменить все DOI TBD реальными идентификаторами рецензируемых статей (или удалить эти ссылки и заменить на проверяемые).
2. **CONCEPT.md §Falsifiability:** Унифицировать α‑уровень: либо 0.05 без поправки, либо явно описать множественное тестирование с Bonferroni и единым порогом.
3. **CONCEPT.md §Sample size calculation:** Устранить дублирование и конфликт PMID; привести единую обоснованную оценку σ с корректным PMID.
4. **Все файлы:** Удалить все fabrication markers (TBD, placeholder‑OSF, «pre‑print; DOI TBD»); заменить на конкретные значения или явно указать «будет определено до подачи».
5. **CONCEPT.md / THEORY.md:** Вычистить дублирующиеся секции (Falsifiability, Pre‑registration, Sample size, Risk matrix, Limitations, Consortium) до единственной непротиворечивой версии; согласовать с THEORY.md.

---

**Финальное заключение:** Пакет содержит нереальные ссылки (DOI TBD), fabrication markers и внутренние противоречия. Вердикт REJECT. Для повторной подачи необходимо: (1) заменить все неверифицируемые ссылки на реальные публикации с корректными DOI/PMID; (2) провести жёсткую вычитку и унификацию всех core‑документов; (3) удалить все placeholders.