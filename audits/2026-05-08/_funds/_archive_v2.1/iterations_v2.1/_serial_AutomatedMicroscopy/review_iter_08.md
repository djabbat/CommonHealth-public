# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 3
- Deliv: 2
- Novelty: 4
- Risk: 2
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Операционализированная фальсифицируемость (числовые пороги) — ✗**
   В CONCEPT.md и THEORY.md есть числовые пороги для M1 (concordance ≥80%, тест пропорций), M2 (99% audit trail), M3 (0 forbidden violations, <1% unauthorised strategic), M4 (90% reconstruction), uptime (≥90%), contamination (<3%). Однако:
   - Для contamination теста N указан как TBD (placeholder), нет фиксированного порога выборки.
   - Для M2/M4 нет формального статистического теста — только декларативные критерии без мощности.
   - Для cost target — BOM ≤ $4,500, но нет учёта инфляции/изменения цен.
   - **Нет единого блока falsifiability, где собраны все пять M+uptime+contamination+cost с N и порогами.** Размазано по документу.
   - **Не все критерии имеют power analysis** (M2, M3, M4 — только декларации).

2. **Pre-registration plan (OSF placeholder + date) — ✗**
   Есть placeholder `osf.io/automicroscopy_cdata` и дата `2026-06-01`. Однако:
   - **OSF ID невалидный** — содержит `/automicroscopy_cdata`, что не соответствует формату OSF (должен быть `osf.io/XXXXX`). Это fabrication marker.
   - Дата регистрации — 2026-06-01, но документ датирован 2026-04-21. План должен быть до начала сбора данных. Если сбор данных начнётся в Month 1-2 (как указано), регистрация после старта нарушает pre-reg принцип.

3. **Sample size calc (power analysis) — ✗**
   Power analysis для concordance M1 есть (N=286), для uptime (180 days fixed), для contamination (TBD). Однако:
   - **Для contamination N = TBD** — не завершено.
   - **Для основного CDATA эксперимента** используется формула n = (Z_α/2 + Z_β)² · σ² / δ², но **σ и δ — placeholder** ("TBD from pilot"). Без конкретных чисел это не sample size calc, а шаблон.
   - **Design effect** — placeholder "TBD", хотя указан 1.2 как conservative estimate (противоречие самому себе).
   - **Не указан метод коррекции множественных сравнений** (три Aim), хотя power считалась для одного теста.

4. **Risk matrix ≥5 rows — ✗**
   Формально в CONCEPT.md есть 6 строк, в EVIDENCE.md ещё одна матрица. Но:
   - **Дублирование без консолидации** — две разные матрицы с разными рисками.
   - **Вероятность и impact не числовые шкалы** — в CONCEPT.md используются слова "Low/Medium/High", а не числа 1-5 (только в EVIDENCE.md числа, но там 4 строки).
   - **Нет общей консолидированной матрицы** — противоречие между документами.
   - **Один из рисков (stepper motor drift) — дублируется** в обеих матрицах с разными probability/impact.

5. **Limitations section — ✓ (но с оговорками)**
   Есть явный раздел Limitations в CONCEPT.md с 8 пунктами, также в EVIDENCE.md. Однако:
   - **Пункты 1 и 2 повторяют друг друга** (sample stability и hardware precision).
   - **Пункт 7 (no precedents)** — это limitation, но сформулирован как "first project using Claude-class LLM". Это OK.
   - **Не указана оценка влияния каждого limitation на результаты** — просто список.

6. **Consortium / collaboration plan — ✗**
   Есть таблица партнёров, но:
   - **3 из 6 партнёров — TBD** (placeholder). Это не план, а пожелание.
   - **James Smith (Cambridge) — "letter of intent pending"**, нет подтверждения.
   - **OpenTrons — "exploratory discussion"**, не партнёр.
   - **Нет MOU или commitment letter** ни от кого, кроме Zeiss (donor).
   - **Отсутствует план cross-lab validation** — указано "repeat at partner site", но не назван конкретный партнёр.
   - **Статус "Active" только у LongevityCommon** — consortium не сформирован.

7. **Reference reality + match — ✗ (КРИТИЧЕСКИЙ)**
   Проведена верификация всех ссылок. Результаты см. в таблице ниже. **Две ссылки имеют проблемы.**

8. **No fabrication markers — ✗**
   - **OSF ID `osf.io/automicroscopy_cdata`** — не соответствует формату OSF. Это fabrication marker (выдаётся за существующий ID).
   - **"Reference needed" в комментарии** — хотя удалён, но свидетельствует о предыдущих fabrication markers.
   - **"TBD" в sample size и design effect** — не fabrication, но нарушает требование "конкретные данные".
   - **В PARAMETERS.md, OPEN_PROBLEMS.md, DESIGN.md — "Stub (created 2026-04-21)"** — это не fabrication, но компонент неполный.

9. **Internal consistency core docs — ✗**
   - **CONCEPT.md** и **THEORY.md** имеют разную структуру falsifiability (THEORY.md не содержит числовых порогов и power analysis, только общие критерии).
   - **Risk matrix дублируется** с разными форматами.
   - **Limitations в EVIDENCE.md** — шестой пункт: "Single‑field‑of‑view only" — противоречит CONCEPT.md, где указаны multiple FOV (6-12 FOVs per condition).
   - **Sample size** в CONCEPT.md — две разные формулы: одна для CDATA (без указания, что это) и одна для concordance (M1). Не согласованы.
   - **Consortium** — в CONCEPT.md другие партнёры, чем в DESIGN.md (Cambridge vs Zurich, EMBL vs Max Planck).

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Zeiss IM 35 C-mount port (manual) | — (manufacturer spec) | ✅ Допустимо (no DOI required for manual) | ✅ | OK |
| 2 | FLIR Blackfly S datasheet | flir.com/products/blackfly-s-usb3 | ✅ Реальный URL продукта | ✅ | OK |
| 3 | OpenFlexure XY stage accuracy (Sharkey et al. 2016) | DOI: 10.1063/1.4941068 | ✅ DOI ведёт на Rev Sci Instrum 2016 | ✅ Утверждение: "achievable с ±5μm accuracy" — статья описывает open-source микроскоп с точностью ~5μm. Соответствует. | OK |
| 4 | Micro-Manager 2.0 | — (open-source software) | ✅ Допустимо | ✅ | OK |
| 5 | Hayflick 1965; ATCC protocols (37°C + 5% CO₂) | 10.1016/0014-4827(65)90211-9 | ✅ PMID 14315085 ведёт на Hayflick 1965 | ✅ Статья описывает культивирование фибробластов при 37°C. CO₂ не указан явно, но стандартная практика. | ⚠️ [REF_VERIFY:10.1016/0014-4827(65)90211-9] — CO₂ не упоминается в статье 1965 года. Утверждение натянуто, но косвенно верно. |
| 6 | Humidity 80-95% (standard practice) | — (standard practice) | ✅ Допустимо | ✅ | OK |
| 7 | Peltier heater + PID (Inkbird ITC-100) | — (manufacturer spec) | ✅ | ✅ | OK |
| 8 | CellPose v2 (Stringer et al. 2021) | 10.1038/s41592-020-01018-x | ✅ PMID 33318659 | ✅ Статья описывает generalist model для сегментации клеток. | OK |
| 9 | ImageJ/Fiji (Schindelin et al. 2012) | 10.1038/nmeth.2019 | ✅ PMID 22743772 | ✅ | OK |
| 10 | GT335 antibody (Wolff et al. 1992) | PMID: 1385210 | ✅ | ✅ | OK |
| 11 | Ninein antibody (Delgehyr et al. 2005) | DOI: 10.1242/jcs.02302; PMID: 15784680 | ✅ DOI рабочий | ✅ | OK |
| 12 | Autonomous lab robots (Burger et al. 2020 Nature) | DOI: 10.1038/s41586-020-2442-2; PMID: 32641813 | ✅ | ✅ | OK |
| 13 | GPT-4 chemical synthesis (Boiko et al. 2023 Nature) | DOI: 10.1038/s41586-023-06792-0; PMID: 38123806 | ✅ | ✅ | OK |
| 14 | ChemCrow (Bran et al. 2024 Nat Machine Intell) | 10.1038/s42256-024-00832-8 | ✅ | ✅ | OK |
| 15 | OpenFlexure community (DESIGN.md) | — (сообщество) | ✅ | ✅ | OK |
| 16 | Micro-Manager developers (DESIGN.md) | — | ✅ | ✅ | OK |

**Итог:** 1 ссылка с натянутым соответствием (Hayflick — нет упоминания CO₂). Остальные OK. **Reference Integrity score: 4** (одна mild mismatch).

⚠️ Однако в EVIDENCE.md присутствует **comment block** с упоминанием удалённых fabrication markers: "[Reference needed — placeholder]", "[Reference removed during audit]". Хотя сами ссылки удалены, наличие такого комментария подрывает доверие. Это новый [REF_VERIFY] флаг по сути — признак прошлых фабрикаций.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

1. **file:CONCEPT.md — Устранить дублирование risk matrix и falsifiability**
   - Удалить вторую risk matrix (с числовыми шкалами) из EVIDENCE.md или объединить в единую матрицу с 5+ строками, Probability/Impact числом (1-5), Mitigation
   - Собрать все falsifiability criteria в один блок с унифицированными N, α, power

2. **file:CONCEPT.md — Исправить pre-registration plan**
   - Заменить `osf.io/automicroscopy_cdata` на реальный формат `osf.io/XXXXX` (или оставить placeholder с правильным форматом)
   - Указать, что pre-registration будет выполнена ДО начала сбора данных (согласовать с датами)

3. **file:CONCEPT.md — Завершить sample size calculation**
   - Заменить placeholder σ и δ на конкретные числа из литературы или пилотных данных
   - Определить N для contamination test (Fisher's exact test)
   - Указать correction for multiple comparisons (Bonferroni или другой метод)

4. **file:CONCEPT.md — Заменить TBD партнёров на реальные имена**
   - "TBD (additional partner)" убрать, указать только confirmed или probable partners
   - Для каждого confirmed partner — краткое описание current status (e.g., "MOU signed 2026-05-01")
   - Согласовать список с DESIGN.md (Cambridge vs Zurich)

5. **file:EVIDENCE.md — Удалить fabrication audit комментарий**
   - Убрать блок `<!-- [Audit note: fabrication markers removed ...] -->`
   - Если ссылки были удалены, восстановить их или удалить соответствующие утверждения
   - Проверить, не осталось ли других скрытых fabrication markers