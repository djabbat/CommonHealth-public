# Review of AutomatedMicroscopy

## Вердикт
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4 (интересная и практически значимая идея, но не нова — low-cost retrofit микроскопов известен)
- Method: 4 (подробное описание техники, power analysis, risk matrix — но есть пробелы в детализации некоторых расчётов)
- Evidence: 3 (ссылки на ключевые технологии есть, но часть источников не имеет проверяемых идентификаторов; отсутствуют рецензируемые источники для части технических утверждений)
- Falsif: 5 (чёткие числовые пороги с α, power, N для всех гипотез — отлично)
- Deliv: 4 (перечислены deliverables, но не привязаны к конкретным датам/вехам)
- Novelty: 4 (использование LLM для управления микроскопом в режиме `/overnight` — ново, но precedents в химии есть)
- Risk: 5 (подробная матрица с probability, impact, mitigation — 6 строк минимум)
- RefIntegrity: 3 (несколько ссылок без DOI/PMID; наличие [audit note] о ранее удалённых фабрикациях — хотя сейчас чисто)

## Checklist (9 условий)

1. **Операционализированная фальсифицируемость (числовые пороги)** — ✓  
   Четкие H₀/H₁ для всех критериев: concordance (>0.80, N=286), uptime (>0.90, N=180), contamination (<0.03), cost (<$4,500). Все с α=0.05, power=0.80.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   OSF ID: `osf.io/automicroscopy_cdata` (placeholder), дата регистрации 2026-06-01.

3. **Sample size calc (power analysis)** — ✓  
   Для concordance: n = (1.645+0.84)²×0.85×0.15/0.05² ≈ 286. Для CDATA: n = 30/group, формула с подстановкой. Для contamination — placeholder (TBD), но это допустимо на текущем этапе.

4. **Risk matrix ≥5 rows** — ✓  
   В CONCEPT.md таблица из 6 строк (AI misinterpretation, env chamber, camera, network, contamination, stepper drift) с P×I×Mitigation.

5. **Limitations section** — ✓  
   В CONCEPT.md явный раздел из 8 пунктов; в EVIDENCE.md дополнительно — без приукрашиваний.

6. **Consortium / collaboration plan** — ✓  
   Таблица партнёров (LongevityCommon, University of Bristol, Zeiss, FLIR, ThorLabs, OpenTrons) с ролями и статусами. Некоторые TBD, но план есть.

7. **Reference reality + match** — ✗ (частично)  
   - Большинство DOI/PMID (OpenFlexure, Hayflick, CellPose, ImageJ, GT335, Ninein, Burger, Boiko, Bran) — реальны и соответствуют тексту.  
   - Проблемные ссылки (см. таблицу ниже): Zeiss IM 35, FLIR Blackfly S, Micro-Manager, стандартная практика культуры клеток, Inkbird — не имеют DOI/PMID. Отсутствие проверяемого идентификатора для нескольких «работ» нарушает требование 7. Это >2 флагов, поэтому автоматический REJECT компонента не ставим, но REVISE_MAJOR обязателен.

8. **Отсутствие фабрикационных маркеров** — ✓  
   В текущем тексте нет `[REF_NEEDED]`, `[PMID_REMOVED]` или `TBD` там, где должны быть данные (placeholders только в pre-reg и risk matrix).

9. **Внутренняя согласованность core-документов** — ✓  
   CONCEPT, THEORY, EVIDENCE согласованы: axioms, falsifiability, партнёры, ограничения повторяются без противоречий. DESIGN/PARAMETERS/OPEN_PROBLEMS — стабы, но не противоречат.

**Итог по условиям:**  
- 1–6, 8, 9 — выполнены.  
- 7 — НЕ выполнено (отсутствие идентификаторов для ≥3 ссылок).  
- Вердикт: REVISE_MAJOR.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Zeiss IM 35 inverted microscope C-mount port | — (manufacturer spec, 1985) | Не проверяемо | Да (известный факт) | [REF_VERIFY: нет идентификатора] |
| 2 | FLIR Blackfly S BFS-U3-63S4M-C | flir.com/products/blackfly-s-usb3