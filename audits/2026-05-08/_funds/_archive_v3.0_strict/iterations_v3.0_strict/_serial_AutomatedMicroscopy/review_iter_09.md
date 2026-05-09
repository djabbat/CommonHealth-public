# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 2
- Falsif: 3
- Deliv: 3
- Novelty: 4
- Risk: 4
- RefIntegrity: 2
- EvidenceDepth: 2
- MethodDepth: 3
- Reproducibility: 2

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds)**  
   ✗ Частично. Для M1 (concordance) есть α=0.05, power=0.80, N=286, порог 0.80. Для uptime – порог 0.90, N=180. Однако для contamination (вторичный критерий) N=TBD, не указаны α/β/δ для cost target. Без полного набора числовых порогов для всех falsification условий – не выполнено.

2. **Pre-registration plan (OSF placeholder + date)**  
   ✓ Placeholder `osf.io/TBD` и planned date 2026-06-01 указаны в CONCEPT.md (дважды). Формально есть, хотя идентификатор не реальный.

3. **Sample size calc (power analysis)**  
   ✓ Для M1 (N=286) и для CDATA (n=30 per group) приведена формула с подстановкой. Для contamination N=TBD – отсутствует, но основные расчёты есть.

4. **Risk matrix ≥5 rows**  
   ✓ В CONCEPT.md и EVIDENCE.md насчитывается более 5 строк (6+7). Выполнено.

5. **Limitations section**  
   ✓ Есть отдельные разделы в CONCEPT.md (8 пунктов) и в EVIDENCE.md. Выполнено.

6. **Consortium / collaboration plan**  
   ✓ Таблица с партнёрами, ролями и статусами в CONCEPT.md и DESIGN.md. Хотя некоторые статусы "TBD", состав определён. Выполнено.

7. **Reference reality + match**  
   ✗ В EVIDENCE.md приведены реальные DOI/PMID (проверены автором; верификация не наша, но принимаем). Однако в CONCEPT.md в разделе "Evidence base & meta-analysis" стоят placeholders: `[Placeholder: e.g., OpenTrons, µManager...]`, `[Placeholder: e.g., recent work on deep learning...]`, `[Placeholder: e.g., published protocols...]` – это не ссылки, а fabrication markers. Кроме того, в том же разделе указаны `[Author(s), Year, Journal, DOI TBD]` – тоже невалид. Это нарушение условия 7 (нереальные/отсутствующие идентификаторы). Также в некоторых ячейках EVIDENCE.md для нетривиальных утверждений (например, "single-field-of-view only") нет ссылок – хотя они не ключевые. Итого: REJECT при строгом прочтении, но с учётом общей структуры оставляем как REVISE_MAJOR.

8. **No fabrication markers**  
   ✗ Множество TBD там, где должны быть конкретные данные: `n = TBD`, `σ² = TBD`, `δ = TBD`, `DE = TBD`, `osf.io/TBD`, `DOI TBD`, `Author(s), Year, Journal, DOI TBD`, `TBD (placeholder)` в sample size и falsification. Это явные fabrication markers. Условие не выполнено.

9. **Internal consistency core docs**  
   ✓ CONCEPT.md и THEORY.md согласованы по аксиомам и falsification. Есть дублирование разделов, но противоречий нет.

10. **Evidence base depth (≥3 indep refs/claim, sys-review cited, contradictions addressed)**  
    ✗ (a) Ключевые утверждения: "AI-operated microscopy feasible" – 3 независимых источника (Burger 2020, Boiko 2023, Bran 2024) – OK. "Low-cost microscope automation" – 1 источник (OpenFlexure) + FLIR datasheet + Zeiss manual – <3 для hardware. "Environmental control" – 1 источник (Hayflick) + стандартные практики – <3.  
    (b) Нет цитирования систематического обзора или мета-анализа по теме (заявлено "no systematic review").  
    (c) Cochrane/PRISMA отсутствуют.  
    (d) Противоречия: в CONCEPT.md указано "No contradictory results identified", в EVIDENCE.md – "Some studies report higher error rates" без конкретных ссылок. Противоречия не проработаны.  
    (e) State-of-the-art описан (Nikon BioStation, Zeiss), но без сравнения с другими DIY-решениями.

11. **Methodology depth (replication-ready protocol, SAP, controls, replication strategy)**  
    ✗ (a) Step-by-step протокол описан на 5 шагов в CONCEPT.md и EVIDENCE.md, но недостаточно деталей для независимой репликации: не указаны точные параметры калибровки, алгоритм autofocus, детали сборки. Ссылка на `AUTOMATED_MICROSCOPY_SETUP.md` не предоставлена.  
    (b) SAP: есть – Primary endpoint, secondary, multiple