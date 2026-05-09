# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 2
- Falsif: 3
- Deliv: 3
- Novelty: 3
- Risk: 4
- RefIntegrity: 4
- EvidenceDepth: 1
- MethodDepth: 2
- Reproducibility: 1

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds) — ✓/✗**  
   *Частично.* Есть H₀/H₁ с α=0.05, power=0.80, N=286 для concordance; uptime, contamination. Однако для contamination N=TBD (зависит от пилота), а для sample size в power analysis σ² и δ = TBD. Количественные пороги есть, но не все подкреплены полным расчётом.

2. **Pre-registration plan (OSF placeholder + date) — ✓**  
   *С замечанием.* Указан OSF ID (`osf.io/TBD` и `osf.io/automicroscopy_cdata` — несогласованность), дата 2026-06-01. Placeholder есть, но требуется унификация.

3. **Sample size calc (power analysis) — ✗**  
   *Не выполнено.* Для первичной гипотезы M1 (concordance) расчёт: n = (1.96+0.84)²·σ²/δ², но σ² и δ = TBD. Формула есть, но подстановка отсутствует. Для contamination N=TBD. Без численных значений расчёт не является завершённым.

4. **Risk matrix ≥5 rows — ✓**  
   Представлены минимум две таблицы по 5–6 строк каждая. Критерий выполнен.

5. **Limitations section — ✓**  
   Явный раздел в CONCEPT.md (дважды, но это технический недочёт), также в EVIDENCE.md. Выполнено.

6. **Consortium / collaboration plan — ✓**  
   В CONCEPT.md таблица партнёров с ролями и статусом (University of Bristol, Zeiss, FLIR, ThorLabs, OpenTrons). Отдельные ссылки на James Smith, Lena Zhang в EVIDENCE.md. План есть, хотя часть статусов "TBD" или "placeholder".

7. **Reference reality + match — ✓/**  
   Все проверенные DOI/PMID реальны (Sharkey, Hayflick, Stringer, Schindelin, Wolff, Delgehyr, Burger, Boiko, Bran). Утверждения соответствуют тексту. Однако ссылки на "low-cost retrofit" — только 1 peer-reviewed источник (OpenFlexure) + manufacturer specs, что недостаточно. Есть [Placeholder: ...] — не фабрикация, но слабо.

8. **No fabrication markers — ✗**  
   В нескольких местах стоят TBD там, где должны быть конкретные данные: sample size (σ², δ), OSF ID, репозиторий для кода/данных. Это нарушает требование "никаких TBD в местах, требующих конкретных данных".

9. **Internal consistency core docs — ✗**  
   Противоречия между CONCEPT.md и EVIDENCE.md: разные OSF ID, дублирование limitations с разными формулировками, множественные версии risk matrix без унификации. Цели (CONCEPT) и методы (EVIDENCE) не полностью согласованы.

10. **Evidence base depth — ✗**  
    (a) Ключевые утверждения не подкреплены ≥3 независимыми источниками: "low-cost retrofit" — 1 статья (OpenFlexure); "environmental control" — 1 статья (Hayflick) + datasheet; "AI-operated microscopy" — 3 статьи из химии, но microscopy-specific — 0.  
    (b) Нет ссылки на систематический обзор или мета-анализ; в EVIDENCE.md указано "no systematic review identified", но не приведён хотя бы один обзор по смежной теме.  
    (c) Cochrane/PRISMA-обзор отсутствует.  
    (d) Противоречия упомянуты только как "will be monitored", без анализа.  
    (e) State-of-the-art описан (Nikon, Zeiss, OpenFlexure, µManager) — частично выполнено.

11. **Methodology depth — ✗**  
    (a) Step-by-step protocol описан лишь на уровне общих шагов, недостаточно для независимой репликации; нет ссылки на protocols.io или детальный протокол.  
    (b) SAP присутствует (primary endpoint, secondary, Bonferroni, LOCF).  
    (c) Replication strategy указана (split-sample + independent dataset).  
    (d) Controls: positive (human), negative (random) — есть.  
    (e) Blinding/randomisation — описаны.  
    *Итог: (a) не выполнено → условие в целом не выполнено.*

12. **Reproducibility & open science — ✗**  
    (a) Code: обещано "GitHub upon acceptance", но без ссылки.  
    (b) Data: "TBD" для платформы (Zenodo/OSF — не конкретно).  
    (c) Pre-registration: placeholder, но без конкретного ID.  
    (d) Materials: protocol ссылка "TBD", requirements.txt "to be included".  
    *Ни один из четырёх пунктов не выполнен полностью.*

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|----------------------|---------|
| 1 | Sharkey et al. 2016 OpenFlexure | 10.1063/1.4941068 | ✅ | ✅ | Принять |
| 2 | Hayflick 1965 | 10.1016/0014-4827(65)90211-9 | ✅ | ✅ | Принять |
| 3 | Stringer et al. 2021 CellPose | 10.1038/s41592-020-01018-x | ✅ | ✅ | Принять |
| 4 | Schindelin et al. 2012 Fiji | 10.1038/nmeth.2019 | ✅ | ✅ | Принять |
| 5 | Wolff et al. 1992 GT335 | PMID 1385210 | ✅ | ✅ | Принять |
| 6 | Delgehyr et al. 2005 Ninein | 10.1242/jcs.02302 | ✅ | ✅ | Принять |
| 7 | Burger et al. 2020 Nature | 10.1038/s41586-020-2442-2 | ✅ | ✅ (химия, не microscopy, но acknowledged) | Принять |
| 8 | Boiko et al. 2023 Nature | 10.1038/s41586-023-06792-0 | ✅ | ✅ | Принять |
| 9 | Bran et al. 2024 ChemCrow | 10.1038