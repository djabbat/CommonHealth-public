# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsif: 4
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)
1. **Operationalised falsifiability (numeric thresholds):** ✓  
   Числовые пороги присутствуют (PAM-13 Δ ≥5.4, α=0.05, power=0.80, N≥55 per group, stopping rule, interim analysis). Небольшое дублирование, но суть выполнена.

2. **Pre-registration plan (OSF placeholder + date):** ✓  
   OSF placeholder (osf.io/TBD), дата 2026-09-01, дизайн и анализ указаны.

3. **Sample size calc (power analysis):** ✓  
   Формула, подстановка, результат (n=55 per group, total 132), sensitivity analysis, обоснование σ. Однако в одном блоке CONCEPT.md указан неверный PMID (15527447) для Hibbard 2004 — автор сам это отмечает, но это свидетельствует о внутренней несогласованности.

4. **Risk matrix ≥5 rows:** ✓  
   Представлено несколько матриц, минимум одна содержит 7 строк.

5. **Limitations section:** ✓  
   Есть в CONCEPT.md и THEORY.md, перечислено 8+ ограничений.

6. **Consortium / collaboration plan:** ✓  
   Указаны Lead PI (Dr. Tkemaladze), Co-I, партнёры (Insignia Health, Fraunhofer IGD, TSU, UCPH). Letters of support pending. Placeholder для Co-I (Name TBD) допустим в этом разделе.

7. **Reference reality + match:** ✗ **КРИТИЧЕСКОЕ НАРУШЕНИЕ**  
   Из 8 уникальных ссылок только 2 (PMID 15333167, 15527447) имеют реальные и проверяемые идентификаторы. Остальные 6:
   - Hibbard 2009 (Insignia technical manual) — без DOI/PMID
   - Tao et al. 2026 (Nat Med) — DOI TBD
   - Blumenthal & Lee 2024 (JAMA) — DOI TBD
   - Tkemaladze 2026 (Longevity Horizon) — DOI TBD
   - Ссылки KNOWLEDGE.md на Mayo Clinic, NIH — это URL, не DOI/PMID  
   **Условие не выполнено. Автоматический REJECT.**

8. **No fabrication markers:** ✗  
   В CONCEPT.md в разделе "Consortium / partners" один блок содержит "Lead PI: [Name TBD]" — placeholders в consortium plan не разрешены (разрешены только в pre-reg и risk matrix). Также есть дублирование и противоречивые версии раздела Sample size calculation.

9. **Internal consistency core docs:** ✗  
   - Две версии Sample size calculation в CONCEPT.md (одна с верным PMID, другая с неверным).  
   - Дублирование разделов Falsifiability и Pre-registration.  
   - В одном месте Consortium указывает Dr. Tkemaladze, в другом — Name TBD.  
   - Разные форматы risk matrix.  
   Несогласованность между core-файлами.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|------------------------|---------|
| 1 | Hibbard et al. 2004, PAM development | PMID 15333167 | Да | Да — обоснование σ для PAM-13 | OK |
| 2 | Hibbard et al. 2005, short-form PAM | PMID 15527447 | Да | Да — упоминается как валидация PAM-13 | OK |
| 3 | Hibbard et al. 2009, MCID & technical manual | Нет (proprietary) | Нет идентификатора | Да, но без проверяемого DOI/PMID | REF_VERIFY: нет ID |
| 4 | Tao et al. 2026, co-design RCT (Nat Med) | DOI TBD | Нет (pre-print, не зарегистрирован) | Утверждается, что n=2069 RCT; DOI отсутствует | REJECT |
| 5 | Blumenthal & Lee 2024, 4-zone HCI (JAMA) | DOI TBD | Нет | Утверждается как основа HCI; DOI отсутствует | REJECT |
| 6 | Tkemaladze 2026, Patient as Project (Longevity Horizon) | DOI TBD | Нет | Используется как аксиоматическая рамка; DOI отсутствует | REJECT |
| 7 | Mayo Clinic Lab Reference Values | URL (mayoclinic.org) | Да, но не DOI/PMID | Соответствует | REF_VERIFY: URL не DOI |
| 8 | NIH MedlinePlus lab tests | URL (medlineplus.gov) | Да, но не DOI/PMID | Соответствует | REF_VERIFY: URL не DOI |

**Итого:** 2/8 ссылок имеют корректный PMID. 6 ссылок не удовлетворяют требованию разрешимого идентификатора.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

Для повышения до REVISE_MAJOR потребовалось бы:
1. **Все ссылки без DOI/PMID** — заменить на реальные идентификаторы:  
   - Hibbard 2009 — получить официальный DOI (Insignia manual может иметь ISBN).  
   - Tao 2026 — указать arXiv ID или DOI если pre-print опубликован.  
   - Blumenthal-Lee 2024 — указать реальный DOI из JAMA.  
   - Tkemaladze 2026 — зарегистрировать preprint с DOI.  
   - Для Mayo Clinic и NIH — можно оставить как URL, но добавить оговорку, что это не научные статьи.
2. **Устранить дублирование** в CONCEPT.md — оставить один блок Falsifiability, один Pre-registration, один Sample size calculation.  
3. **Согласовать Consortium** — убрать "Name TBD" и указать реальное имя Lead PI во всех местах.  
4. **Провести внутренний аудит** на противоречия: версия с PMID 15527447 для Hibbard 2004 должна быть удалена.  
5. **Добавить pre-registration placeholder** в соответствии с правилами (OSF ID должен быть зарезервирован, а не просто TBD).

Текущий вердикт — **REJECT** из-за невыполнения условия 7 (множество невалидных идентификаторов) и дополнительных нарушений 8 и 9. Без исправления ссылок компонент не может быть принят ни на каком уровне.