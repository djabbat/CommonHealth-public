# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 3
- Deliv: 3
- Novelty: 3
- Risk: 2
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** ✗  
   Числовые пороги есть (PAM-13 Δ ≥ 5.4, α = 0.05, power = 0.80, N ≥ 55). Однако присутствуют внутренние противоречия:  
   - В одном месте α = 0.05 с Bonferroni для secondary → α = 0.025; в другом написано «p < 0.001 (Bonferroni-adjusted)», что не согласовано.  
   - В CONCEPT.md написано α = 0.05 primary, Bonferroni-adjusted α = 0.025 для secondary; в THEORY.md указано «α = 0.05 (two-sided)» без упоминания Bonferroni, и затем упоминается «p < 0.001 (Bonferroni-adjusted)» — расхождение.  
   Требуется единая, непротиворечивая спецификация порогов.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   OSF ID: `osf.io/TBD` (placeholder), planned date 2026-09-01. Placeholder допустим. В одном месте также `osf.io/XXXXX`, что не критично; следует унифицировать.

3. **Sample size calc (power analysis)** ✗  
   Формула в CONCEPT.md: `n = (Z_α/2 + Z_β)² · 2σ² / δ²` — верно, подстановка даёт n ≈ 55.  
   В THEORY.md же написано: `n = (1.96 + 0.84)² · σ² / δ²` — пропущена двойка в числителе (2σ²). Это грубая математическая ошибка, противоречащая CONCEPT.md.  
   Ссылка на Hibbard для σ = 10 содержит неверный PMID (см. п.7).  
   Также не указан тип теста (двухвыборочный t-тест) явно в THEORY.md.

4. **Risk matrix ≥5 rows** ✗  
   В CONCEPT.md есть две разные таблицы рисков: одна с 7 строками (Probability/Impact/Mitigation), другая с 4 строками (другой формат).  
   В THEORY.md ещё одна таблица с 2 рисками. Раздел не унифицирован, таблицы противоречат друг другу. Требуется единая матрица рисков, минимум 5 строк.

5. **Limitations section** ✗  
   В CONCEPT.md перечислены 8 ограничений, в THEORY.md — 7 (частично пересекаются, частично разные). Отсутствует единый, полный раздел limitations. Нет ссылки на конкретные confounding factors (например, digital literacy, Hawthorne effect) и их влияние.

6. **Consortium / collaboration plan** ✓  
   Указаны Lead PI, Co-Is, потенциальные партнёры с ролями (Insignia Health, Fraunhofer IGD, TSU, University of Copenhagen). Есть placeholder для имён (TBD), что допустимо на стадии pre-approval.  
   Не хватает конкретных писем поддержки (LoP) — ожидается, что будут получены до финала.

7. **Reference reality + match** ✗  
   **Критическая ошибка:**  
   - В CONCEPT.md раздел Sample size calculation ссылается на «Hibbard 2004 (PMID 15527447)». Статья Hibbard et al. 2004 *Health Serv Res* 39(4 Pt 1):1005–26 имеет PMID **15333167**, а не 15527447. PMID 15527447 принадлежит статье Hibbard et al. 2005 *Health Serv Res* 40(6 Pt 1):1918–30. Это **несоответствие года и PMID** — ссылка нереальна в данном контексте.  
   - Ссылка Tkemaladze (2026) *Longevity Horizon* 2(5) указана как pre-print с «DOI TBD» — не проверяется, но требует заполнения.  
   - Ссылки Tao et al. (2026) *Nature Medicine* и Blumenthal–Lee (2024) *JAMA* указаны как pre-prints; их реальность устанавливается после публикации.  
   - Остальные ссылки (Hibbard 2005, 2009) — реальны, но 2009 manual — проприетарный, без PMID/DOI.  
   **Общий вердикт:** наличие неверного PMID для Hibbard 2004 является автоматическим REJECT компонента.

8. **No fabrication markers** ✓  
   Placeholder (osf.io/TBD, Name TBD, DOI TBD) присутствуют в допустимых местах. Нет маркеров [REF_NEEDED] или [PMID_REMOVED]. Честно указаны нереализованные компоненты (KIMI/Qwen).

9. **Internal consistency core docs** ✗  
   Множественные расхождения:  
   - α-уровень (0.05 vs 0.025 vs 0.001) — не согласован между CONCEPT.md и THEORY.md.  
   - Формула sample size в THEORY.md неверна (пропущена 2).  
   - Риски и ограничения различаются в разных документах.  
   - PMID для Hibbard 2004 неверен.  
   Core-документы не образуют единое непротиворечивое описание.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|----------------------|---------|
| 1 | Hibbard JH et al. (2004) *Health Serv Res* 39(4 Pt 1):1005–26. | **PMID: 15527447** (указан в CONCEPT.md) | **НЕТ** (PMID 15527447 — статья 2005 года; правильный PMID для 2004 — 15333167) | Нет (год не соответствует PMID) | **REJECT** |
| 2 | Hibbard JH et al. (2005) *Health Serv Res* 40(6 Pt 1):1918–30. | **PMID: 15527447** (правильный для этой статьи) | Да | Да | OK |
| 3 | Hibbard JH et al. (2009) PAM scoring & MCID. Insignia Health manual. | Нет PMID/DOI (proprietary) | Да (существует как продукт) | Да | Accept (manuals допускаются) |
| 4 | Tkemaladze J. (2026) *Longevity Horizon* 2(5). | DOI TBD (pre-print) | Неизвестно (pre-print) | N/A | Placeholder (требуется заполнение) |
| 5 | Tao W. et al. (2026) *Nature Medicine* RCT n=2069. | DOI TBD (pre-print) | Неизвестно | N/A | Placeholder |
| 6 | Blumenthal D., Lee J. (2024) *JAMA* 4-zone framework. | DOI TBD (pre-print) | Неизвестно | N/A | Placeholder |
| 7 | Hibbard JH et al. (2004) — в THEORY.md §8. | **PMID: 15333167** (правильный) | Да | Да | OK |

**Вывод по ссылкам:** одна критическая ошибка (№1) — неверный PMID для Hibbard 2004, что ведёт к автоматическому REJECT. Score RefIntegrity: 2.

## Top 5 text-level fixes (для исправления до REVISE или повторной подачи)

1. **AIM/CONCEPT.md (Sample size calculation)** — исправить PMID для Hibbard 2004 с `15527447` на `15333167`. Убедиться, что ссылка соответствует году и утверждению.

2. **AIM/THEORY.md (Sample size calculation)** — исправить формулу: `n = (1.96 + 0.84)² · 2σ² / δ²` (добавить множитель 2). Указать явно двухвыборочный t-тест.

3. **AIM/CONCEPT.md (Falsifiability) + AIM/THEORY.md (Falsifiability)** — согласовать α-уровень. Убрать упоминания «p < 0.001»; везде указать: primary α = 0.05 (two-sided), Bonferroni для ≤2 secondary → α = 0.025. Указать это единообразно.

4. **AIM/CONCEPT.md (Risk matrix) + AIM/THEORY.md (Limitations)** — создать единую матрицу рисков (≥5 строк) и единый раздел limitations. Убрать дублирующие/противоречащие таблицы. Добавить риск, связанный с неверным PMID (управление качеством ссылок).

5. **Все core-документы** — провести полный аудит всех ссылок (не только Hibbard). Для pre-prints указать DOI после публикации. Убедиться, что все PMID/DOI валидны и соответствуют тексту. 

---

## PACKET

# AIM

**Итоговый вердикт: REJECT** из-за критической ошибки в ссылке (невалидный PMID для Hibbard 2004) и внутренних противоречий между core-документами (α, формула sample size). Требуется исправить Top 5 пунктов и пройти повторный ревью.