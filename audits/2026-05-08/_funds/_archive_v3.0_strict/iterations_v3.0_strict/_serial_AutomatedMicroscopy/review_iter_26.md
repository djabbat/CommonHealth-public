# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

---

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsif: 5
- Deliv: 3
- Novelty: 5
- Risk: 3
- RefIntegrity: 2
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 1

---

## Checklist (✓/✗ + объяснение)

1. **Operationalised falsifiability (numeric thresholds)** → ✓  
   Чёткие H₀/H₁, α=0.05, мощность 0.80, N=286 для M1, uptime H₀≤0.90, contamination H₀≥0.03, cost ≤$4500. Все пороги числовые.

2. **Pre-registration plan (OSF placeholder + date)** → ✓  
   OSF placeholder `osf.io/TBD` и `osf.io/automicroscopy_cdata`, дата 2026-06-01. Placeholder допустим.

3. **Sample size calc (power analysis)** → ✓  
   Для CDATA: Cohen’s d=0.75, α=0.05, power=0.80 → n=30/group, DE=1.2. Для M1: N=286 decisions. Формула с подстановкой.

4. **Risk matrix ≥5 rows** → ✓  
   Несколько таблиц, минимум 5 строк (6 в одном месте, 5 в другом). Риски адекватны.

5. **Limitations section** → ✓  
   Явные разделы в CONCEPT.md и EVIDENCE.md. Перечислены 6–8 ограничений.

6. **Consortium / collaboration plan** → ✓  
   Таблица партнёров (LongevityCommon, Bristol, Zeiss, FLIR, ThorLabs, OpenTrons) с ролями и статусами. Дополнительно в DESIGN.md.

7. **Reference reality + match** → ✗  
   Реальные ссылки (Burger, Boiko, Bran, Stringer, Hayflick, Wolff, Delgehyr, Schindelin, OpenFlexure) соответствуют утверждениям. **Однако** в EVIDENCE.md и CONCEPT.md присутствуют **placeholders** с `[Author(s) TBD]` и `DOI TBD` — это не реальные ссылки. Нарушение требования «реальность + соответствие» для каждой цитаты.

8. **No fabrication markers** → ✗  
   В конце EVIDENCE.md раздел «Evidence base & meta-analysis» содержит: `[Author(s), Year, Journal, DOI TBD]` — явный fabrication marker (TBD там, где должны быть конкретные данные). Аналогично в CONCEPT.md. Также остались `[Reference needed — placeholder]` и `[Reference removed during audit]` (хотя они помечены как удалённые, сам факт наличия маркеров указывает на незавершённость).

9. **Internal consistency core docs** → ✓  
   CONCEPT.md ⇔ THEORY.md ⇔ EVIDENCE.md согласованы: аксиомы, цели, методы, интерфейсы. Нет противоречий.

10. **Evidence base depth** → ✗  
    - Ключевые утверждения: «CellPose v2 сегментирует клетки» — только 1 источник (Stringer 2021).  
    - «Low-cost retrofit feasible» — только OpenFlexure (1 источник) + manufacturer specs.  
    - «AI-operated microscopy имеет прецеденты» — 3 статьи, но все из химии, не из микроскопии; прямых аналогов нет.  
    - **Систематический обзор / мета-анализ отсутствует.** Сами авторы признают «no systematic review or meta-analysis».  
    - Противоречия: сказано «No contradictory results were identified», но не обсуждаются потенциальные (variability in AI accuracy, batch effects) и не приведены ссылки на работы, показывающие проблемы AI-фокусировки.  
    - State-of-the-art описан, но без количественного сравнения performance.

11. **Methodology depth** → ✗  
    - Step-by-step protocol **отсутствует** (есть только высокоуровневое описание из 5 шагов в конце EVIDENCE.md).  
    - SAP: primary endpoint указан (Cohen’s kappa), но не указан конкретный статистический тест (предположительно z-тест для пропорции? Не ясно). Secondary endpoints не детализированы. Missing data strategy — LOCF (устаревший метод). Multiple comparisons correction упомянута (Bonferroni), но не для всех secondary.  
    - Replication strategy: split-sample 70/30 — это для проверки AI, а не для репликации биологического результата. Independent dataset только обещан (partner lab TBD).  
    - Controls: positive/negative указаны, но не детализированы (какой именно human expert? Какие random decisions?).  
    - Blinding/randomisation: evaluators blinded, но для in vitro работы blinding не описан (кто и как рандомизирует?).

12. **Reproducibility & open science** → ✗  
    - Code availability: только обещание «will be released on acceptance», нет ссылки на репозиторий.  
    - Data deposit: «Zenodo or OSF» — не указан конкретный депозит, нет идентификатора.  
    - Pre-registration: placeholder `osf.io/TBD` — не зарегистрировано.  
    - Materials transparency: protocols.io link = TBD, requirements.txt не предоставлен.  

**Итого:** ✗ по 5 пунктам (7, 8, 10, 11, 12). Вердикты FUND_AS_IS и REVISE_MINOR невозможны.

---

## Reference audit

| # | Цитата | DOI/PMID | Реальна? | Соответствует тексту? | Решение |
|---|--------|----------|----------|-----------------------|---------|
| 1 | OpenFlexure (Sharkey et al. 2016) | DOI:10.1063/1.4941068 | ✅ (Rev Sci Instrum) | ✅ Утверждение о точности ±5μm | OK |
| 2 | Hayflick 1965 | PMID 14315085 | ✅ (Exp Cell Res) | ✅ 37°C/5% CO₂ для фибробластов | OK |
| 3 | Stringer et al. 2021 (CellPose) | DOI:10.1038/s41592-020-01018-x | ✅ (Nat Methods) | ✅ Generalist segmentation | OK |
| 4 | Schindelin et al. 2012 (Fiji) | DOI:10.1038/nmeth.2019 | ✅ (Nat Methods) | ✅ Batch processing | OK |
| 5 | Wolff et al. 1992 (GT335) | PMID 1385210 | ✅ (Eur J Cell Biol) | ✅ Антитело к полиглутамилированному тубулину | OK |
| 6 | Delgehyr et al. 2005 (Ninein) | DOI:10.1242/jcs.02302 | ✅ (J Cell Sci) | ✅ Mother centriole marker | OK |
| 7 | Burger et al. 2020 (автономная химия) | DOI:10.1038/s41586-020-2442-2 | ✅ (Nature) | ✅ Прецедент AI-управляемого эксперимента | OK |
| 8 | Boiko et al. 2023 (GPT-4 химия) | DOI:10.1038/s41586-023-06792-0 | ✅ (Nature) | ✅ Прецедент | OK |
| 9 | Bran et al. 2024 (ChemCrow) | DOI:10.1038/s42256-024-00832-8 | ✅ (Nat Mach Intell) | ✅ LLM + инструменты | OK |
| 10 | Inkbird ITC-100 спецификация | manufacturer spec | ✅ (даташит) | ✅ ±0.3°C | OK (не peer-reviewed) |
| 11 | Zeiss IM 35 manual | manufacturer spec | ✅ | ✅ C-mount | OK |
| 12 | FLIR Blackfly S datasheet | flir.com | ✅ | ✅ Характеристики | OK |
| 13 | Micro-Manager | micro-manager.org | ✅ | ✅ Open-source |