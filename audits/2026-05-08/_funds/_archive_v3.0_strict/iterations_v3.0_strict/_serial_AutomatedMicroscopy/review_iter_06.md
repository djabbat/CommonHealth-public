# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 3
- Evidence: 2
- Falsif: 4
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 2
- EvidenceDepth: 1
- MethodDepth: 3
- Reproducibility: 3

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Операционализованная фальсифицируемость (числовые пороги)** — ✓  
   Присутствуют числовые пороги: concordance >0.80, uptime >0.95, contamination <0.03. Расчёт N=286 для concordance. Но для contamination N=TBD. Формально выполнено, но с недоработкой.

2. **Pre-registration plan (OSF placeholder + date)** — ✓  
   Указан OSF ID (osf.io/TBD) и planned date (2026-06-01). Плейсхолдер допустим, дата есть.

3. **Sample size calculation (power analysis)** — ✓  
   Приведены формулы и подстановки для concordance (N=286), для uptime (180 days), для contamination (N=TBD). Полнота не идеальная, но условие выполнено.

4. **Risk matrix ≥5 rows** — ✓  
   В CONCEPT.md таблица из 6 рисков; в EVIDENCE.md ещё одна из 7 рисков. ≥5 строк есть.

5. **Limitations section** — ✓  
   Явный раздел в CONCEPT.md (8 пунктов) и в EVIDENCE.md (дополнительно). Присутствует, честный.

6. **Consortium / collaboration plan** — ✓  
   Таблица с партнёрами, ролями, статусами (включая TBD). План есть, формально выполнено.

7. **Reference reality + match** — ✗  
   В EVIDENCE.md ссылки на статьи выглядят валидными (DOI/PMID проверены по тексту). Однако в CONCEPT.md раздел "Evidence base & meta-analysis" содержит явные плейсхолдеры: `[Placeholder: e.g., OpenTrons, µManager...]`, `[Placeholder: e.g., recent work...]`, `[Placeholder: e.g., published protocols...]`. Эти «цитаты» не являются реальными ссылками. Требование реальности не выполнено. Также в CONCEPT.md не указаны источники для многих утверждений (state-of-the-art без ссылок). **Score RefIntegrity: 2.**

8. **Отсутствие фабрикационных маркеров** — ✗  
   Множественные `TBD` в sample size (σ², δ, DE), OSF ID, partner roles, contamination N, effect size. В разделе Limitations слово "placeholder". В Evidence base & meta-analysis прямые плейсхолдеры. Это расценивается как фабрикационные маркеры (отсутствие конкретных данных там, где они должны быть). Условие провалено.

9. **Внутренняя согласованность core-документов** — ✓  
   Противоречий между CONCEPT, THEORY, EVIDENCE, PARAMETERS, OPEN_PROBLEMS, DESIGN, README не обнаружено. Стабы (PARAMETERS, OPEN_PROBLEMS, DESIGN) согласованы. Согласованность есть.

10. **Доказательная база и мета-анализ** — ✗  
    (a) Ключевые утверждения (AI quality, low-cost retrofit, environmental control) не имеют ≥3 независимых источников. Для AI используются косвенные аналоги из химии (Burger, Boiko, Bran) – это 3, но они не про микроскопию. Для low-cost retrofit – только OpenFlexure. Для environmental – Hayflick + Inkbird spec. (b) Нет ни одного систематического обзора или мета-анализа; в CONCEPT.md прямо указано "No systematic review or meta-analysis was performed". (c) Соответственно, Cochrane/PRISMA не цитируется. (d) Противоречия: сказано "No contradictory results were identified, but the search was not systematic" – это не соответствует требованию явно учитывать противоречия, если они есть в литературе. (e) State-of-the-art описан, но без ссылок и без сравнения с альтернативами. **Score EvidenceDepth: 1.**

11. **Глубина методологии** — ✓ (с оговорками)  
    (a) Есть step-by-step protocol из 5 шагов, описанный в CONCEPT.md. (b) Statistical Analysis Plan: primary/secondary endpoints, Bonferroni, missing data strategy (LOCF). (c) Replication strategy: split-sample + independent dataset from second lab (TBD). (d) Controls: positive (human), negative (random AI). (e) Blinding: evaluators blinded; order randomized. Детализация достаточна для репликации, хотя часть указана как TBD. Условно засчитываю.

12. **Воспроизводимость и open science** — ✓ (с оговорками)  
    (a) Code availability: обещание GitHub "upon acceptance". (b) Data deposit plan: Zenodo/OSF. (c) Pre-registration: placeholder OSF ID. (d) Materials transparency: protocols.io, requirements.txt. Всё перечислено, но с плейсхолдерами. Формально выполнено.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Zeiss IM 35 manual (1985) | manufacturer spec | N/A (не статья) | – | Пропустить |
| 2 | FLIR Blackfly S datasheet | flir.com/products/blackfly-s-usb3 | N/A (даташит) | – | Пропустить |
| 3 | OpenFlexure (Sharkey et al. 2016) | 10.1063/1.4941068 | Да | Да | OK |
| 4 | Micro-Manager 2.0 | micro-manager.org | N/A (софт) | – | Пропустить |
| 5 | Hayflick 1965 | 10.1016/0014-4827(65)90211-9 | Да | Да | OK |
| 6 | Inkbird ITC-100 spec | manufacturer spec | N/A | – | Пропустить |
| 7 | CellP