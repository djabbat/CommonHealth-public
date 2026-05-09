# Review of AutomatedMicroscopy

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 1
- Falsif: 3
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 1
- EvidenceDepth: 1
- MethodDepth: 2
- Reproducibility: 1

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **✗ Operationalised falsifiability (numeric thresholds)** — Частично выполнено. Есть числовые пороги для concordance (>80%), uptime (>95%), contamination (<3%). Однако для contamination-claimed sample size — TBD (placeholder), power analysis для второго endpoint не завершён. В одном месте формула sample size использует σ² и δ = TBD. Требуемые пороги указаны, но не все операционализированы с числовыми ссылками на pilot data.

2. **✗ Pre-registration plan (OSF placeholder + date)** — Указан placeholder «osf.io/TBD» и дата 2026-06-01. Реальный ID не создан. План зарегистрирован не будет до исправления. Нарушение: placeholder ≠ plan. Требуется реальный номер до подачи.

3. **✗ Sample size calc (power analysis)** — Есть power analysis для основного endpoint (concordance: n=286), для CDATA эксперимента (n=30 per group, Cohen's d=0.75). Но для contamination endpoint: «N = TBD», design effect = TBD. Также есть «pilot data placeholders». Неполная калькуляция для всех первичных и вторичных endpoint'ов.

4. **✓ Risk matrix ≥5 rows** — Присутствует несколько таблиц, содержащих ≥5 строк. Probability и Impact указаны, mitigation есть. Однако в одном месте числовая шкала (1-5) не согласована с другой таблицей (словесная). Счёт как выполненный, но качество среднее.

5. **✓ Limitations section** — Отдельный раздел присутствует; перечислены 8 пунктов. Дублируется в нескольких местах, но это не нарушение. Ограничения честные.

6. **✗ Consortium / collaboration plan** — Таблица партнёров есть, но статус многих — «TBD (placeholder)», «Exploratory discussion», «To be identified». Реальные signed agreements отсутствуют. Требуется минимум 2-3 confirmed partner с конкретными ролями.

7. **✗ Reference reality + match** — **Множественные фабрикационные маркеры:** в тексте встречаются «[Reference needed — placeholder: replace with DOI or PMID before submission]», «[Reference removed during audit — placeholder: verify and restore or delete sentence]», «[Author(s), Year, Journal, DOI TBD]», «Placeholder: e.g., OpenTrons, µManager…». Это прямое указание на незавершённые ссылки. Даже после «Audit note: fabrication markers removed» эти строки остались. Также есть цитаты, для которых авторы сами признают «only one peer-reviewed source; additional independent validation needed» — это несоответствие требованию «≥3 независимых источников». **Автоматический REJECT** по условию 7(b) и 7(a) из-за невалидных/отсутствующих идентификаторов.

8. **✗ No fabrication markers** — **Фабрикационные маркеры присутствуют:** «osf.io/TBD», «Repository: TBD», «protocols.io link TBD», «[Author(s), Year, Journal, DOI TBD]», «N = TBD», «DE = TBD», «σ² = TBD», «δ = TBD». Многочисленные TBD в тех местах, где должны быть конкретные данные. Условие нарушено.

9. **✓ Internal consistency core docs** — Методы (концепция AI-operated microscopy) соответствуют целям (сбор данных для CDATA). Теория, evidence, концептуальные документы не противоречат друг другу. Небольшие дублирования (несколько risk matrices, limitations) не нарушают согласованность.

10. **✗ Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed)** — Критически не выполнено.
    - Ключевое утверждение «Low-cost microscope retrofit is feasible»: всего 1 peer-reviewed source (OpenFlexure) + 1 manufacturer datasheet.
    - «Cell segmentation with CellPose»: 1 источник (Stringer et al. 2021).
    - «Environmental control for long-term imaging»: 1 peer-reviewed source (Hayflick 1965) + 1 datasheet.
    - «AI-operated microscopy has precedents»: 3 источника, но все из химии, не из биомедицинской микроскопии; прямых аналогов LLM-управления микроскопом нет — авторы сами это признают.
    - Систематический обзор или мета-анализ по теме не представлен. Вместо этого сказано: «No systematic review or meta-analysis was identified» — но требование п.10(b): «ссылки на ≥1 систематический обзор или мета-анализ». Невыполнение.
    - Противоречия в литературе не обсуждаются. Есть лишь пустое «No contradictory results were identified», без анализа возможных источников несоответствий (например, AI error rates в низкоконтрастных образцах). Нарушение 10(d).

11. **✗ Methodology depth (replication-ready protocol, SAP, controls, replication strategy)** — Недостаточно детально.
    - Step-by-step protocol представлен на уровне «Setup, Configuration, Execution, Monitoring, Analysis» — не содержит конкретных параметров, порогов, шагов для независимой репликации.
    - SAP есть, но primary endpoint (concordance rate) использует Cohen's kappa — хорошо. Missing data strategy: LOCF — сомнительно для time-series; не указана sensitivity analysis. Multiple comparison correction: Bonferroni для secondary endpoints — приемлемо.
    - Replication strategy: split-sample (70/30) и independent dataset, но independent dataset указан как «TBD» — не специфицирован.
    - Controls: positive (human expert) и negative (random decisions) — хорошо, но не указано, как они встроены в сам эксперимент (например, каждое ли решение сопровождается контрольным).
    - Blinding/randomization: evaluators blinded, order randomized — достаточно.

12. **✗ Reproducibility & open science (code, data, pre-reg, materials)** — Полностью не выполнено.
    - Code availability: «will be made available on GitHub upon acceptance. Repository: TBD» — не принято.
    - Data deposit: «TBD» для платформы (Zenodo/OSF).
    - Pre-registration: «osf.io/TBD» — placeholder.
    - Materials transparency: protocols.io link TBD, requirements.txt только обещан.
    - Ни одной конкретной ссылки или deposit plan.

**Итого:** не выполнены пункты 1 (частично), 2, 3 (частично), 6 (частично), 7, 8, 10, 11, 12. Полностью выполнены только 4, 5, 9. В соответствии с правилами, FUND_AS_IS или REVISE_MINOR невозможны. Наличие фабрикационных маркеров и placeholder'ов в критических разделах однозначно требует REJECT. Даже если технически некоторые пункты исправимы, текущее состояние не соответствует минимальным требованиям к серьёзной заявке.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|---|---|---|---|---|
| 1 | Sharkey et al. 2016, Rev Sci Instrum | 10.1063/1.4941068 | Да (Crossref) | Да — XY stage accuracy | OK |
| 2 | Stringer et al. 2021, Nat Methods | 10.1038/s41592-020-01018-x | Да (PubMed) | Да — CellPose v2 | OK |
| 3 | Schindelin et al. 2012, Nat Methods | 10.1038/nmeth.2019 | Да (PubMed) | Да — ImageJ batch processing | OK |
| 4 | Wolff et al. 1992, Eur J Cell Biol | PMID: 1385210 | Да (PubMed) | Да — GT335 antibody | OK |
| 5 | Delgehyr et al. 2005, J Cell Sci | 10.1242/jcs.02302; PMID: 15784680 | Да (PubMed) | Да — Ninein antibody | OK |
| 6 | Burger et al. 2020, Nature | 10.1038/s41586-020-2442-2; PMID: 32641813 | Да (PubMed) | Да — autonomous lab robot | OK |
| 7 | Boiko et al. 2023, Nature | 10.1038/s41586-023-06792-0; PMID: 38123806 | Да (PubMed) | Да — GPT-4 chemistry | OK |
| 8 | Bran et al. 2024, Nat Mach Intell | 10.1038/s42256-024-00832-8 | Да (Crossref) | Да — ChemCrow LLM | OK |
| 9 | Hayflick 1965, Exp Cell Res | 10.1016/0014-4827(65)90211-9 | Да (PubMed) | Да — 37°C/5% CO2 | OK |
| 10 | Zeiss IM 35 manual (manufacturer spec) | нет DOI | Нет (нецифровой) | Да — C-mount | Недоказуемо, но не фабрикация |
| 11 | FLIR Blackfly S datasheet | flir.com/products/blackfly-s-usb3 | Да (сайт производителя) | Да | OK |
| 12 | Inkbird ITC-100 spec | нет DOI (manufacturer datasheet) | Нет (нет DOI) | Да | Приемлемо для datasheet |
| 13 | OpenTrons / µManager (placeholders) | нет идентификатора | Нет | Нет — placeholders | **FABRICATION MARKER** |
| 14 | [Author(s), Year, Journal, DOI TBD] | нет идентификатора | Нет | Нет | **FABRICATION MARKER (multiple occurrences)** |
| 15 | [Reference needed — placeholder: replace with DOI or PMID before submission] | нет | Нет | Нет | **FABRICATION MARKER** |
| 16 | [Reference removed during audit — placeholder: verify and restore or delete sentence] | нет | Нет | Нет | **FABRICATION MARKER** |

Примечание: ссылки #13–16 являются прямыми фабрикационными маркерами. Даже если они будут удалены, отсутствие их содержательного эквивалента делает заявку неготовой.

## Evidence depth audit (новое v3.0)

| # | Ключевое утверждение