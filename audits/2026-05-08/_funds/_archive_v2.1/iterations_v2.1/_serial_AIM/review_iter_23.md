# Review of AIM

## Verdict  
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4  
- Method: 2  
- Evidence: 2  
- Falsif: 3  
- Deliv: 2  
- Novelty: 4  
- Risk: 3  
- RefIntegrity: 1  

## Checklist (✓/✗ + объяснение)

1. **Операционализированная фальсифицируемость (числовые пороги)**  
   ✗ — пороги присутствуют (PAM-13 Δ ≥5.4, α=0.05, power=0.80, N≥55, stopping rule, interim analysis), но **внутренне противоречивы**: в одном месте указано Bonferroni-скорректированное α=0.025 для ≤2 secondary endpoints, в другом — α=0.05 без коррекции. Набор secondary outcomes различается между CONCEPT.md и THEORY.md (в THEORY.md добавлено "physician time per visit"). Консенсус не достигнут.

2. **Pre-registration plan (OSF placeholder + дата)**  
   ✓ (погранично) — есть OSF placeholder (`osf.io/TBD`, `osf.io/XXXXX`) и дата 2026-09-01. Но идентификатор не унифицирован (TBD vs XXXXX) и не заменён реальным. Пока проходит.

3. **Sample size calculation (power analysis с формулой и подстановкой)**  
   ✗ — формула и числовая подстановка даны, но **обоснование σ = 10 ссылается на неверный PMID**: текст использует PMID 15527447 (Hibbard 2005, short-form PAM) вместо PMID 15333167 (Hibbard 2004, original PAM-13). SD range 9–11 из Hibbard 2004, но указан не тот PMID. Это фатальная ошибка reference integrity в самом чувствительном месте. Кроме того, в одном из блоков σ обозначен как "TBD (placeholder)" — что недопустимо для этого раздела.

4. **Risk matrix (≥5 строк)**  
   ✓ — минимум 5 строк присутствуют, хотя формат и содержимое различаются в разных частях документа.

5. **Limitations section**  
   ✓ — есть отдельный раздел с 8 пунктами.

6. **Consortium / collaboration plan**  
   ✓ — перечислены роли Lead PI, Co-I, potential partners (Insignia Health, Fraunhofer IGD, TSU, University of Copenhagen) с указанием функций. Placeholder допускается.

7. **Reference reality + match**  
   ✗ — **Критический провал**.  
   - Hibbard 2004 в одном месте указан с правильным PMID 15333167, но в обосновании sample size использован неверный PMID 15527447 (Hibbard 2005) — прямая ошибка.  
   - Hibbard 2009 (proprietary manual) — нет идентификатора, неверифицируем.  
   - Tao et al. (2026) Nature Medicine — pre-print, **нет DOI/PMID**, неверифицируем.  
   - Blumenthal & Lee (2024) JAMA — pre-print, **нет DOI**, неверифицируем.  
   - Tkemaladze (2026) Longevity Horizon — pre-print, **нет DOI**, неверифицируем.  
   Для 4 из 6 ключевых ссылок отсутствуют разрешимые идентификаторы. Condition не выполнен.

8. **Отсутствие фабрикационных маркеров**  
   ✓ — прямых фабрикаций (фальшивых DOI, [REF_NEEDED]) не обнаружено. Placeholders присутствуют в допустимых разделах, хотя sample size block содержит несогласованные "TBD" в одном из повторений.

9. **Внутренняя согласованность core-документов**  
   ✗ — **Систематическая несогласованность**:  
   - Разные наборы secondary outcomes (CONCEPT.md: MMAS-8, EQ-5D-5L, hospitalisations; THEORY.md: + physician time per visit).  
   - Дублирующиеся и противоречивые разделы Falsifiability в CONCEPT.md.  
   - Множественные версии pre-reg плана с разными OSF ID.  
   - Расхождение в α-уровнях (0.05 vs 0.025).  
   - README.md всё ещё упоминает KIMI и Qwen как провайдеров, хотя CONCEPT.md их удалил (vapor cleanup).

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|------------------------|---------|
| 1 | Hibbard et al. (2004) PAM development, Health Serv Res 39(4 Pt 1):1005–26 | PMID 15333167 | Да | Используется с неверным PMID в некоторых местах | [REF_VERIFY: PMID 15527447 used incorrectly] |
| 2 | Hibbard et al. (2005) Short-form PAM, Health Serv Res 40(6 Pt 1):1918–30 | PMID 15527447 | Да | В тексте ошибочно приписан Hibbard 2004 | [REF_VERIFY: mismatch] |
| 3 | Hibbard et al. (2009) PAM scoring & MCID, Insignia Health technical manual | Нет идентификатора | Нет | Частично (MCID=5.4 общеизвестен) | Неверифицируем |
| 4 | Tao W. et al. (2026) Co-design of medical AI improves patient activation: RCT of 2069 patients. Nature Medicine | Нет DOI/PMID | Неверифицируем | Препринт без идентификатора | Неверифицируем |
| 5 | Blumenthal D., Lee J. (2024) Four-zone framework for human-AI clinical collaboration. JAMA | Нет DOI/PMID | Неверифицируем | Препринт без идентификатора | Неверифицируем |
| 6 | Tkemaladze J. (2026) Patient as a Project. Longevity Horizon 2(5) | Нет DOI/PMID | Неверифицируем | Самоцитирование без идентификатора | Неверифицируем |

## Top 5 text-level fixes

1. **file:CONCEPT.md, THEORY.md** — Исправить обоснование sample size: заменить неверный PMID 15527447 на PMID 15333167 для σ из Hibbard 2004. Проверить, чтобы все ссылки на Hibbard 2004 использовали корректный идентификатор. Удалить все "TBD" из раздела sample size calculation, оставив только консистентные числа.

2. **file:CONCEPT.md, THEORY.md** — Унифицировать пороги фальсифицируемости: выбрать единый α (0.05 или 0.025 после коррекции), согласовать набор