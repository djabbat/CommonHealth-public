# Review of UMBRELLA

## Verdict
**TOXIC_WITHDRAW** — проект не готов к финансированию ни в одном из указанных форматов (ERC AdG, EIC Pathfinder, NIH R01, Wellcome, Impetus). Научная строгость недопустимо низка, доказательная база практически отсутствует, управленческая зрелость нулевая. Рекомендую авторам полностью пересмотреть подход: сфокусироваться на одном подпроекте (BioSense + FCLC), провести пре-регистрированное конфирматорное исследование и только после этого подавать заявку.

## Scores (1-5)
| Критерий | Оценка | Обоснование |
|---|---|---|
| **Premise** | 2 | Идея создания интегративной платформы старения как total chronic disease — разумна, но не нова (аналоги: Aging.AI, Insilico Medicine). Раздута до 5+ подпроектов без единого подтверждённого результата. |
| **Method** | 1 | Основные методы — либо постулированные ansatz (Ze dτ/dt = -αI, без биологического обоснования), либо пост-хок подгонки (weights, CDATA bridge) на underpowered выборках. Отсутствие пре-регистрации, признанное p-hacking. |
| **Evidence** | 1 | Единственные "подтверждения" — swept-v* на N=500 (post-hoc), остальное: NULL результаты для v1, inconclusive CDATA (p=0.12), AUC 0.81 на N=2222 без пре-регистрации (Ioannidis 2005 — автор сам цитирует!), отсутствие хоть одного peer-reviewed подтверждения. |
| **Falsifiability** | 2 | M4 threshold (N≥2000, α=0.001, partial r²<0.05) — правильно сформулирован, но не выполнен. CDATA falsifiability не operationalised (Sobol deferred). Ze ansatz не фальсифицируем на практике. |
| **Deliverability** | 2 | 0 подписанных EU LoI для EIC, GDPR blocker (FCLC semi-honest), отсутствие CI, порт-конфликты, зависимость от одного PhD-студента (Lezhava). Архитектура перегружена. |
| **Novelty** | 2 | Комбинация free-energy principle + CHSH + federated learning — не нова. Утверждения о квантовых аналогах в старении (CHSH deformation 1.7478) не обоснованы и выглядят как наукообразная спекуляция. |
| **Risk** | 5 | Крайне высокий риск: отсутствие конфирматорных данных, юридические риски GDPR, зависимость от неполученного гранта (EIC), ключевой персонал с единой точкой отказа. Финансирование такого проекта равносильно сжиганию денег. |

## Top 3 P0 issues

### 1. Научная несостоятельность: нет ни одного пре-регистрированного конфирматорного результата
Весь framework построен на пост-хок анализе (p-hacking, как признаёт сам автор), underpowered выборках (N=196 для CDATA bridge с 5 параметрами — 39 obs/param при Harrell rule 10), и NULL результатах v1, которые "deprecated/superseded" без объяснения механизма исправления. Пока не будет хотя бы одного пре-регистрированного исследования N ≥ 2000 с α = 0.001, всё это — гипотезы, а не наука.

**Действие:** отозвать заявку до проведения такого исследования.

### 2. Отсутствие EU консорциума и юридические проблемы с данными
EIC Pathfinder требует ≥1 EU-MS + ≥2 разных MS/AC — ноль подписанных LoI на дату пакета. GDPR Article 9 blocker (FCLC semi-honest, не защищает от active server collusion) — в текущем виде использование медицинских данных незаконно. Никакого плана миграции v14 с конкретными сроками и бюджетом нет (Q1 2027 расплывчато).

**Действие:** нужно либо отказаться от EIC, либо собрать консорциум + завершить malicious-secure инфраструктуру.

### 3. Фундаментальные дефекты в теоретическом ядре (Ze ansatz + CDATA)
`dτ_Ze/dt = −α·I(Z)` — постулирован по аналогии с физическими часами (Burgholzer 2015, Pearson 2021), без вывода для биологических систем. Автор честно переименовал в "ansatz", но не предлагает механизма, почему энтропия информации должна управлять темпом старения. CDATA — "inconclusive" с p=0.12, при этом остаётся в основном тексте как "Counter #1". CHSH deformation (константа 1.7478) — непонятно, из какой физической модели следует, как измерять. Без ответа на эти вопросы framework — пустая обёртка.

**Действие:** убрать Ze и CDATA из ядра до получения чётких механистических или экспериментальных подтверждений.

## Top 5 text-level fixes (что можно исправить редактированием core docs)

1. **`LongevityCommon/CONCEPT.md: §4`** — Удалить строку "WHO recognized aging as a disease". ICD-11 MG2A "ageing-associated decline in intrinsic capacity" — это не disease, а risk factor. Overclaim — убивает доверие. Заменить на: "WHO classifies age-related decline in intrinsic capacity (MG2A) as a health condition, but not aging as a disease; our framework treats aging as a syndrome of chronic processes."

2. **`LongevityCommon/CONCEPT.md: §3`** — Переместить CDATA и Ze из основного списка 5 компонентов в "supporting hypotheses" с явным указанием "not validated": статус inconclusive для CDATA, ansatz для Ze. Оставить только 3 pillar: MCOA (theoretical framework), BioSense (applied platform), FCLC (infrastructure).

3. **`LongevityCommon/THEORY.md: §4`** — Текущая таблица "Old vs New framing" не меняет факта, что эти утверждения — speculation. Убрать полностью. Вместо этого добавить: "All mathematical claims in this document are speculative (ansatz) until empirical validation on pre-registered cohort N≥2000. See `EVIDENCE.md` for status of each claim."

4. **`LongevityCommon/EVIDENCE.md: §4`** — Строка "`v* = 0.45631` empirical (BioSense)" помечена как "Pilot (All-of-Us N=500, 95% CI 0.443-0.459)" — это не подтверждение, а ещё один пост-хок. Поменять статус с "Confirmed" на "Exploratory (post-hoc, single dataset; replication on independent pre-registered cohort N≥500 required)".

5. **`LongevityCommon/OPEN_PROBLEMS.md: §2.3, §2.5, §3.1, §4.1`** — Каждый из этих пунктов должен быть переведён из "open" в "P0 blocker" с указанием конкретных сроков и ресурсов:
   - §2.3 Реализация EEGLAB/EDF парсера — оценить стоимость (человеко-часы) и альтернативы (shell out to MNE).
   - §2.5 Port conflict — немедленно исправить в actual config; это не open, а баг.
   - §3.1 CI — добавить как обязательный пункт в roadmap: Q3 2026.
   - §4.1 EU LoI — добавить критерии fail: "если к 2026-08-01 не подписано ≥2 LoI от разных MS, заявку отозвать".

**Общее замечание:** после этих правок проект всё равно TOXIC, но станет чуть более честным. В текущем виде это science fiction под маской хардкорной математики, неприемлемое для любого рецензируемого агентства.