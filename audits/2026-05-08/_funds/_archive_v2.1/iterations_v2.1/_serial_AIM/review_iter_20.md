# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 3
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — ✗ (частично)
   Числовые пороги присутствуют (PAM-13 Δ ≥5.4, α=0.05, power=0.80, N≥55). Однако есть внутренние противоречия: в CONCEPT.md σ указана как TBD в одном месте, как 10 — в другом; разные версии α-коррекции (p<0.025 vs Bonferroni-adjusted α=0.025). Расчёт sample size использует σ=10 без достаточного обоснования (путаница с PMID 15527447). Формально thresholds есть, но неконсистентны.

2. **Pre-registration plan (OSF placeholder + date)** — ✗ (технически есть, но с нарушениями)
   Указан placeholder OSF ID (`osf.io/TBD`, `osf.io/XXXXX`) и planned date 2026-09-01. Однако разные версии плана в CONCEPT.md и THEORY.md содержат разные наборы secondary outcomes (MMAS-8 vs physician time per visit). Placeholder допустим, но неконсистентность между файлами нарушает п.9.

3. **Sample size calc (power analysis)** — ✗ (неполный)
   Формула и подстановка чисел есть (n=55 per group, sensitivity analysis). Но:
   - В одном месте σ указана как TBD, в другом — как 10.
   - Обоснование σ=10 ссылается на Hibbard 2005 (PMID 15527447), который на самом деле не содержит данных PAM-13 SD, а является short-form validation. Это ошибка, которую авторы сами признают, но не исправляют в тексте.
   - Sensitivity analysis не заполнена полностью (range указан, но без конкретных чисел для каждого сценария).
   - В разных версиях документа разная формула (с разными Z-значениями).

4. **Risk matrix ≥5 rows** — ✓
   Несколько таблиц рисков, минимум 5 строк. Содержат probability, impact, mitigation.

5. **Limitations section** — ✓
   Явный раздел есть в THEORY.md и в CONCEPT.md. Перечислено 8+ ограничений. Без приукрашиваний.

6. **Consortium / collaboration plan** — ✗ (неполный)
   Список партнёров есть (Tkemaladze, Samanishvili, Insignia Health, Fraunhofer IGD, TSU, UCPH), но:
   - Lead PI указан как "Name TBD".
   - Роли Co-I и Data management — "TBD".
   - Letters of support — "pending" или "to be obtained".
   Это fabrication markers (см. п.8).

7. **Reference reality + match** — ✗ (критическое нарушение)
   **Множественные неразрешимые ссылки**: Hibbard 2009 (proprietary manual без DOI), Tao et al. 2026 (DOI TBD), Blumenthal-Lee 2024 (DOI TBD), Tkemaladze 2026 (DOI TBD), Tkemaladze 2023 (без идентификатора), Jaba 2022 (без идентификатора).  
   **Несоответствие тексту**: обоснование σ=10 ссылается на Hibbard 2005 (PMID 15527447), хотя должно опираться на Hibbard 2004 (PMID 15333167). Это искажение смысла.  
   **Непроверяемые проприетарные ссылки**: Hibbard 2009 manual.  
   См. таблицу Reference audit ниже.

8. **No fabrication markers** — ✗ (критическое нарушение)
   Присутствуют: `Sensitivity analysis: TBD`, `σ = TBD`, `Placeholder for range of effect sizes`, `Lead PI: Name TBD`, `Co-I (Clinical): Name TBD`, `DOI TBD` для трёх статей, `OSF ID: osf.io/TBD` (допустим, но с оговоркой), `Letters of support pending`.  
   **Особенно критично**: TBD в sample size calculation (σ) и в consortium (ключевые роли) — это не pre-reg и не risk matrix, где placeholder разрешён.

9. **Internal consistency core docs** — ✗ (критическое нарушение)
   **Противоречия между CONCEPT.md и THEORY.md**:
   - Разные версии falsifiability: в CONCEPT.md α=0.05 с Bonferroni 0.025, в THEORY.md — α=0.05 без упоминания