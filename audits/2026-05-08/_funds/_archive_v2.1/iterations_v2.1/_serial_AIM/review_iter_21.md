# Review of AIM

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 1
- Falsif: 4
- Deliv: 3
- Novelty: 4
- Risk: 2
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение)

1. **✓ Операционализированная фальсифицируемость (числовые пороги)** — Пороги заданы: PAM-13 Δ ≥ 5.4, α=0.05, power=0.80, N≥55/группа, правило остановки, промежуточный анализ. Однако дублируются секции с α=0.025 (Bonferroni). Требуется унификация.

2. **✓ Pre-registration plan** — Указан OSF placeholder и дата 2026-09-01. Описан дизайн и анализ. Формально соответствует.

3. **✓ Sample size calculation** — Формула приведена с подстановкой, результат 55/группа, учтён dropout (20%). Анализ чувствительности присутствует. Проблема: σ=10 не обоснован из литературы — ссылка дана на неверный PMID (15527447), хотя должна быть на 15333167.

4. **✓ Risk matrix (≥5 rows)** — Две версии матрицы: одна с 7 строками, другая с 5. Качественно различаются по формату. Формально ≥5 строк есть.

5. **✓ Limitations section** — Присутствуют 8 честных ограничений. Включает упоминание возможной недостаточной мощности и проблем с достоверностью ссылок.

6. **✓ Consortium / collaboration plan** — Перечислены партнёры: Insignia Health (лицензия PAM-13), Fraunhofer IGD (L1), TSU (валидация языка), University of Copenhagen (экономика здоровья). Роли указаны. Lead PI и Co-I обозначены как TBD — приемлемо на этой стадии.

7. **✗ Reference reality + match** — Критическое нарушение. 3 из 5 ключевых ссылок (Tao et al. 2026, Blumenthal-Lee 2024, Tkemaladze 2026) имеют недействительные идентификаторы (DOI TBD). Ссылка Hibbard 2004 в одном месте указана с неверным PMID 15527447, что противоречит правилу. Реальные статьи существуют, но не верифицированы по идентификаторам.

8. **✗ No fabrication markers** — В тексте обнаружены TBD/placeholder в недопустимых контекстах: σ = TBD, sensitivity analysis = TBD, placeholder OSF ID = XXXXX вместо конкретного идентификатора, [REF_NEEDED] в одном из вариантов. Следы коррекции PMID («исправлено с 15527447 на 15333167») — это маркер предыдущей ошибки.

9. **✗ Internal consistency core docs** — Многочисленные противоречия:
   - Secondary outcomes: в CONCEPT.md MMAS-8/EQ-5D-5L/hospitalisations, в THEORY.md добавлено «physician time per visit».
   - α-уровень: в одном месте 0.05, в другом 0.025 (Bonferroni).
   - В README указано, что KIMI/Qwen rejected, а в справочнике KNOWLEDGE.md они ещё упоминаются (хоть и как rejected).
   - Названия провайдеров в таблице языков (Gemini vs DeepSeek) не совпадают между файлами.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Hibbard et al. (2004) PAM development | PMID 15333167 | Да | Да | [OK] |
| 2 | Hibbard et al. (2005) Short-form PAM | PMID 15527447 | Да | Да | [OK] |
| 3 | Hibbard et al. (2009) PAM scoring & MCID | No identifier (proprietary manual) | Не проверяется | Сомнительно — MCID 5.4 может не подтверждаться | [REF_VERIFY] |
| 4 | Tao et al. (2026) *Nature Medicine* – co-design RCT n=2069 | DOI TBD | Нет — DOI не существует | Утверждение «Level I confirmed» может быть не подтверждено публикацией | [REF_VERIFY] REJECT |
| 5 | Blumenthal D., Lee J. (2024) *JAMA* – 4-zone HCI framework | DOI TBD | Нет | Нельзя верифицировать существование статьи | [REF_VERIFY] REJECT |
| 6 | Tkemaladze J. (2026) *Longevity Horizon* – Patient as Project | DOI TBD | Нет | Пока нет публикации, ссылка недействительна | [REF_VERIFY] REJECT |
| 7 | Hibbard 2004 (ошибочно указан PMID 15527447 в тексте) | PMID 15527447 (ошибочно) | Да, но неверно отнесена | Текст утверждает «SD range 9-11», а в PMID 15527447 — совсем другая статья (short-form) | Искажение [REF_VERIFY] |
| 8 | Jaba (2022), Tkemaladze (2023) *Georgian Scientists* | Нет идентификатора | Не проверяется | Неизвестно | [REF_VERIFY] |

**Итого:** 3 невалидных ссылки (REJECT уровня) + 3 сомнительных [REF_VERIFY]. Условие 7 полностью провалено.

## Top 5 text-level fixes

1. **THEORY.md: §8 — заменить все DOI TBD на реальные идентификаторы**  
   Удалить ссылки Tao 2026, Blumenthal 2024, Tkemaladze 2026, если они не опубликованы. Заменить на существующие публикации с PMID/DOI или указать «in preparation».  
   *Файл: THEORY.md, раздел 8*

2. **THEORY.md / CONCEPT.md — унифицировать secondary outcomes**  
   Удалить «physician time per visit» из THEORY.md или добавить его во все файлы.  
   *Файл: THEORY.md, раздел 3.4; CONCEPT.md – Pre-registration plan*

3. **CONCEPT.md: Falsifiability — устранить дублирование α**  
   Оставить единую запись: α=0.05 two-sided, с пометкой о Bonferroni (0.025) для ≤2 сравнений. Удалить повторяющуюся секцию.  
   *Файл: CONCEPT.md – Falsifiability*

4. **CONCEPT.md: Sample size — заменить σ = TBD на обоснованное значение**  
   Указать σ=10 на основе Hibbard 2004 (PMID 15333167) с явной цитатой. Удалить упоминание неправильного PMID.  
   *Файл: CONCEPT.md – Sample size calculation*

5. **THEORY.md / CONCEPT.md – удалить свидетельства правки PMID**  
   Убрать фразу «исправлено с 15527447 на 15333167» — это маркер фабрикации. Сделать чистую версию.  
   *Файл: THEORY.md – Limitations (п.8); CONCEPT.md – Sample size*

## PACKET

```
# AIM

Отправлен на **REVISE_MAJOR**.  
Критические замечания:
- Ссылки на неопубликованные статьи (Tao 2026, Blumenthal 2024, Tkemaladze 2026) недопустимы. Требуется замена на верифицируемые источники или удаление.
- Внутренние противоречия между THEORY.md и CONCEPT.md (secondary outcomes, α-уровень) необходимо устранить.
- Placeholder-ы и следы коррекции (TBD, RR) должны быть заменены на конкретные данные.
Deviation от правил EU Horizon Europe по проверке референций. Требуется полный рефакторинг ссылок.

Повторная проверка после исправлений.
```