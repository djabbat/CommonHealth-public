# Review of MCOA

## Verdict
**REVISE_MAJOR** — проект содержит серьёзные пробелы в верификации литературы, placeholder-значения без обоснования и несколько неоперационализированных тестов. Для перехода к FUND_AS_IS / REVISE_MINOR требуется исправление пункта 7 (верификация ссылок) и доработка ряда количественных спецификаций.

## Scores (1-5)
- **Premise: 4** — сильная формальная рамка; априорные веса — ключевая инновация, но пока не реализована.
- **Method: 3** — формализм чёткий, но не хватает complete specification всех функций преобразования `f_i` и протокола для `w_i` prediction.
- **Evidence: 3** — частично подтверждено, но множественные fabrication markers (исправленные) и отсутствие verified-статуса для ряда ключевых ссылок.
- **Falsif: 4** — Axiom M4 задаёт жёсткий порог; тесты 1A–5 описаны, но не все имеют numeric thresholds для sample size.
- **Deliv: 4** — Nature Aging submission, Rust reference implementation, EIC Pathfinder integration.
- **Novelty: 4** — мульти-счётная архитектура с априорными весами и обязательной фальсифицируемостью оригинальна.
- **Risk: 3** — высокие риски (априорные веса не калиброваны, измерение Γ требует новых экспериментов, финансирование не обеспечено).

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)**  
   **✓** — Axiom M4 содержит конкретный порог (N ≥ 2000, α = 0.001, partial r² < 0.05). Тесты 1A–5 имеют качественные критерии, но для 1A, 2A, 3A не указаны N и power.

2. **Pre-registration plan (OSF placeholder + date)**  
   **✓** — osf.io/TBD, planned date 2026-07-01. Пока placeholder, но план присутствует.

3. **Sample size calc (power analysis)**  
   **✓** — для глобального теста: α=0.05, power=0.80, δ=0.3 → N=1875, threshold N≥2000. Для остальных тестов нет.

4. **Risk matrix ≥5 rows**  
   **✓** — 6 строк с Probability, Impact, Mitigation.

5. **Limitations section**  
   **✓** — 5 пунктов в CONCEPT.md.

6. **Consortium / collaboration plan**  
   **✓** — lead institution + 4 proposed partners с ролями.

7. **References PubMed/Crossref-verified**  
   **✗** — не все ссылки верифицированы.  
   - В THEORY.md используется `Schultz & Sinclair Cell 2019, PMID 30982602`, но в EVIDENCE.md эта запись удалена (DELETED) и указана как suggested replacement. Статус verified отсутствует.  
   - В PARAMETERS.md ссылки `PMID 2342578, 1631178` помечены как CORRECTED, но не verified.  
   - Нет явной пометки "pre-print" ни для одной ссылки.  
   - В EVIDENCE.md осталась строка с `Reference NEEDED` для связи Mito→EpiDrift, что нарушает требование.

8. **No fabrication markers**  
   **✓ (условно)** — fabrication markers (DELETED, CORRECTED) присутствуют, но они явно обозначены как исправления и не оставлены без внимания. Требование отсутствия [REF_NEEDED] или [PMID_REMOVED] в основном тексте выполняется, но в EVIDENCE.md есть "NEEDS REPLACEMENT", что borderline. Требуется полная очистка.

## Top 5 text-level fixes

1. **EVIDENCE.md / таблица Γ**: заменить строку с "NEEDS REPLACEMENT" на реальную верифицированную ссылку (Schultz & Sinclair 2019 удалена; найти и проверить другую, например, обзор по NAD+/сиртуинам). Если нет — удалить утверждение из THEORY.md.

2. **PARAMETERS.md**: все значения с пометкой "Placeholder" (D_CP₀, D_Mito₀, α_Epi, α_Prot, β_Prot и др.) должны быть заменены на литературно обоснованные числа или содержать пояснение, почему они placeholder и когда будут калиброваны. Например, D_CP₀ = 0.05 ± 0.02 из независимых данных.

3. **OPEN_PROBLEMS.md / Test 1A**: добавить power analysis (эффект, α, мощность → N) для тестов 1A, 2A, 3A. Сейчас только качественные R² пороги. Также для теста 4A указать N экспериментальных повторов.

4. **THEORY.md / раздел 5**: явно указать, что ссылка на Schultz & Sinclair 2019 (PMID 30982602) является корректной и верифицированной, либо заменить на verified источник. Добавить строку в EVIDENCE.md с ✅ для этой ссылки.

5. **CONCEPT.md / Limitations**: явно указать, что OSF pre-registration ещё не выполнен (placeholder), а реальная выборка для теста на смертность может быть меньше N≥2000. Также отметить, что timetable для априорного предсказания весов (Test 1A) не указан — добавить ожидаемую дату завершения.