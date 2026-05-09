# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1–5)
| Критерий | Оценка |
|----------|--------|
| Premise | 4 |
| Method | 3 |
| Evidence | 2 |
| Falsif | 4 |
| Deliv | 3 |
| Novelty | 3 |
| Risk | 3 |
| RefIntegrity | 3 |
| EvidenceDepth | 1 |
| MethodDepth | 3 |
| Reproducibility | 2 |

---

## Checklist (✓/✗)

### 1. Operationalised falsifiability (numeric thresholds) — ✓
Числовые пороги указаны для M1 (concordance ≥0.80, α=0.05, power=0.80, N=286), uptime (≥0.90), contamination rate (<0.03), cost ($4500). Есть H₀, H₁, test statistic, decision rule.

### 2. Pre-registration plan (OSF placeholder + date) — ✓
Указан OSF (`osf.io/TBD`), дата 2026-06-01. Плейсхолдер допустим, но требуется конкретный ID.

### 3. Sample size calculation (power analysis) — ✓
Для первичного endpoint (concordance) – полный расчёт с формулой (z-тест для одной пропорции). Для CDATA-эксперимента – расчёт по Cohen's d (n=30/group). Для contamination – TBD (плейсхолдер, но не основной). Требование выполнено.

### 4. Risk matrix ≥5 rows — ✓
Две таблицы: первая (6 строк) + вторая (6 строк). Итого >5. Вероятность, влияние, mitigation указаны.

### 5. Limitations section — ✓
Явный раздел в CONCEPT.md (8 пунктов), дублирован в EVIDENCE.md. Честное перечисление.

### 6. Consortium / collaboration plan — ✓
Таблица партнёров с ролями и статусами (LongevityCommon, University of Bristol, Zeiss, FLIR, ThorLabs, OpenTrons). Формально достаточно.

### 7. Reference reality + match — ✓
Все 10 проверенных ссылок (DOI/PMID) реальны и соответствуют утверждениям. См. таблицу ниже.

### 8. No fabrication markers — ✗
**Нарушение.** В CONCEPT.md:
- Раздел "Evidence base & meta-analysis": три плейсхолдера `[Placeholder: e.g., …]` вместо реальных ссылок.
- `Sample size calculation`: `σ² = TBD`, `δ = TBD`, `DE = TBD`.
- `Consortium / partners`: два `TBD (additional partner)`.
- `Risk matrix`: `DE = TBD`, `Sample size: N = TBD`.
- `Pre-registration`: `osf.io/TBD`.
- Код, данные, протоколы – все `TBD`.

Фабрикационные маркеры присутствуют в местах, где должны быть конкретные данные.

### 9. Internal consistency core docs — ✓
Методы (`THEORY.md` → bounded autonomy, PROMPT-driven) согласуются с концепцией (`CONCEPT.md`). Цели (CDATA Phase A) едины. Противоречий между core-файлами не обнаружено.

### 10. Evidence base depth (≥3 indep refs/claim, sys-review, contradictions) — ✗
- **Ни одно ключевое утверждение не подкреплено ≥3 независимыми источниками.** Примеры:
  - "Zeiss IM 35 has C-mount" – 1 источник (manual).
  - "Arduino XY stage achievable with ±5µm" – 1 источник (OpenFlexure).
  - "AI-operated microscopy precedents" – 3 источника (Burger, Boiko, Bran), но это разные работы, а не 3 на одно утверждение.
- **Нет систематического обзора или мета-анализа.** Раздел "Evidence base & meta-analysis" содержит только плейсхолдеры.
- **Противоречия не учтены.** Утверждение "No contradictory results were identified" при отсутствии систематического поиска – грубое нарушение.

### 11. Methodology depth (replication-ready protocol, SAP, controls, replication strategy) — ✓
Формально все элементы присутствуют:
- Step-by-step protocol (5 шагов).
- SAP (primary endpoint, вторичные, Bonferroni, LOCF).
- Controls (positive: human; negative: random AI).
- Replication (split-sample + independent dataset).
- Blinding (оценщики ослеплены) и рандомизация (порядок рандомизирован).
Хотя протокол краткий, для предварительного ревью достаточно.

### 12. Reproducibility & open science — ✗
- **Code:** Обещание "will be released on acceptance" – допустимо, но без репозитория и без `requirements.txt`.
- **Data:** План депонирования (Zenodo/OSF) указан, но все идентификаторы `TBD`.
- **Pre-registration:** `osf.io/TBD`.
- **Materials:** protocols.io – `DOI: TBD`.
Все элементы – плейсхолдеры без конкретных ссылок. Полный open‑science compliance отсутствует.

---

## Reference audit

| # | Цитата | DOI/PMID | Реальна? | Соответствует тексту? | Решение |
|---|--------|----------|----------|----------------------|---------|
| 1 | OpenFlexure (Sharkey et al., 2016) | 10.1063/1.4941068 | ✅ | ✅ XY stage accuracy | OK |
| 2 | Hayflick (1965) | 10.1016/0014-4827(65)90211-9; PMID 14315085 | ✅ | ✅ 37°C/5% CO₂ for fibroblasts | OK |
| 3 | Inkbird ITC-100 (manufacturer spec) | — | ✅ (manufacturer doc) | ✅ ±0.3°C stability | OK (non‑peer source) |
| 4 | CellPose v2 (Stringer et al., 2021) | 10.1038/s41592-020-01018-x; PMID 33318659 | ✅ | ✅ Segmentation | OK |
| 5 | ImageJ (Schindelin et al., 2012) | 10.1038/nmeth.2019; PMID 22743772 | ✅ | ✅ Batch processing | OK |
| 6 | GT335 antibody (Wolff et al., 1992) | PMID 1385210 | ✅ | ✅ Polyglutamylated tubulin | OK |
| 7 | Ninein antibody (Delgehyr et al., 2005) | 10.1242/jcs.02302; PMID 15784680 | ✅ | ✅ Mother centriole marker | OK |
| 8 | Autonomous chemistry robot (Burger et al., 2020) | 10.1038/s41586-020-2442-2; PMID 32641813 | ✅ | ✅ AI‑driven lab automation | OK |
| 9 | GPT-4 chemical synthesis (Boiko et al., 2023) | 10.1038/s41586-023-06792-0; PMID 38123806 | ✅ | ✅ LLM for chemistry | OK |
| 10 | ChemCrow (Bran et al., 2024) | 10.1038/s42256-024-00832-8 | ✅ | ✅ LLM with tools | OK |

---

## Evidence depth audit

| # | Ключевое утверждение | Источников цитировано | Включён ли мета‑анализ/систематический обзор? | Противоречия учтены? |
|---|----------------------|-----------------------|-----------------------------------------------|----------------------|
| 1 | Low‑cost microscope retrofit возможен | 1 (OpenFlexure) + 1 (manufacturer) | Нет | Нет |
| 2 | AI‑operated microscopy имеет прецеденты | 3 (Burger, Boiko, Bran) – разные домены | Нет | Нет |
| 3 | Environmental control для long‑term imaging | 2 (Hayflick; Inkbird) | Нет | Нет |
| 4 | Cell segmentation возможна с CellPose | 1 | Нет | Нет |
| 5 | Антитела GT335/Ninein специфичны | 2 | Нет | Нет |

**Вывод:** ни одно утверждение не подкреплено ≥3 независимыми peer‑reviewed источниками. Мета‑анализ отсутствует. Противоречия не обсуждаются.

---

## Top 5 text‑level fixes

1. **CONCEPT.md → Evidence base & meta-analysis** – заменить три плейсхолдера `[Placeholder: …]` на реальные DOI/PMID, описывающие: (a) AI‑assisted microscopy, (b) low‑cost microscope retrofit, (c) CDATA protocol. Добавить ссылку на хотя бы один систематический обзор или мета‑анализ (например, PRISMA по AI в микроскопии).

2. **CONCEPT.md → Sample size calculation** – заменить `σ² = TBD`, `δ = TBD`, `DE = TBD` на конкретные численные значения, обоснованные пилотными данными или литературой. Удалить все `