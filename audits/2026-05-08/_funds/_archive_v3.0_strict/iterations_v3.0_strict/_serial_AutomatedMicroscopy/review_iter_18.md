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
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 2

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds)** ✗  
   Concordance H₀: ≤0.80, N=286 — есть. Uptime H₀: ≤0.90, N=180 — есть. Но для contamination H₀: ≥0.03, N=TBD — не определён. M2/M3/M4 имеют критерии, но без power. В CONCEPT.md в одном месте N=286, в другом «Sample size: TBD». Несогласованность. Требуются числовые пороги для **всех** ключевых предсказаний. Не выполнено.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   OSF ID: `osf.io/TBD` (или `osf.io/automicroscopy_cdata`), дата 2026-06-01. План включён. Формально присутствует. OK.

3. **Sample size calc (power analysis)** ✗  
   Есть расчёт для CDATA (n=30/group, Cohen's d=0.75) и для concordance (n=286, one-proportion test). Но:  
   - Расчёт для concordance использует δ=0.05, σ² не указана — формула для пропорции, не для среднего.  
   - Для contamination N=TBD.  
   - В разделе «Sample size calculation» (CONCEPT.md) есть противоречащая формула с TBD для σ² и δ.  
   - Не все ключевые сравнения имеют power analysis. Не выполнено.

4. **Risk matrix ≥5 rows** ✓  
   Есть три разные таблицы рисков: 6, 6 и 7 строк. Каждая ≥5. Формально OK.

5. **Limitations section** ✓  
   Отдельный раздел Limitations в CONCEPT.md (8 пунктов), также в EVIDENCE.md. Присутствует.

6. **Consortium / collaboration plan** ✓  
   Таблица с партнёрами (LongevityCommon, Bristol, Zeiss, FLIR, ThorLabs, OpenTrons) + роли + статусы. Есть placeholder TBD для недостающих. OK.

7. **Reference reality + match** ✗  
   В EVIDENCE.md перечислены реальные DOI/PMID (OpenFlexure 10.1063/1.4941068, Hayflick PMID 14315085, CellPose 10.1038/s41592-020-01018-x, ImageJ 10.1038/nmeth.2019, GT335 PMID 1385210, Ninein 10.1242/jcs.02302, Burger 10.1038/s41586-020-2442-2, Boiko 10.1038/s41586-023-06792-0, Bran 10.1038/s42256-024-00832-8). Эти реальны и соответствуют утверждениям.  
   **Однако** в разделе «Evidence base & meta-analysis» (CONCEPT.md) присутствуют три пункта с надписями `[Author(s), Year, Journal, DOI TBD]` — несуществующие ссылки. Это нарушение condition 7 (нереальные).  
   Плюс есть ссылка на «Inkbird ITC-100 spec» — не peer-reviewed. Не все ключевые утверждения подкреплены рецензированными источниками.

8. **No fabrication markers** ✗  
   Множество `TBD`, `[Placeholder]`, `σ² = TBD`, `δ = TBD`, `DE = TBD`, `Sample size: TBD`, `osf.io/TBD` в ключевых разделах. Несмотря на пометку в EVIDENCE.md об удалении маркеров, в CONCEPT.md они остались. Присутствуют явные fabrication markers.

9. **Internal consistency core docs** ✗  
   Противоречия:  
   - Два разных варианта Limitations (один после Risk matrix, другой после Consortium).  
   - Разные версии falsifiability/sample size: один раз «n = TBD», другой — конкретные числа.  
   - Pre-registration ID: то `osf.io/TBD`, то `osf.io/automicroscopy_cdata`.  
   - Risk matrix повторён трижды с разным оформлением.  
   - PARAMETERS.md, OPEN_PROBLEMS.md, DESIGN.md — заглушки (stubs).  
   Цели–концепт согласованы, но методы и данные — нет. Не выполнено.

10. **Evidence base depth (≥3 indep refs/claim, sys-review or meta-analysis cited, contradicting results addressed)** ✗  
    - Утверждение «Low-cost retrofit feasible»: всего 1-2 источника (OpenFlexure + Zeiss manual).  
    - «Environmental control»: 1 источник (Hayflick) + 1 нерецензированный (Inkbird).  
    - «Cell segmentation»: 1 источник (CellPose).  
    - «AI-operated microscopy»: 3 источника (Burger, Boiko, Bran) — выполнено, но это химия, не microscopy.  
    - Систематический обзор / мета-анализ: явно не процитирован, сказано «не найден» — это не замена.  
    - Противоречия: не обсуждаются (упомянуты только как будут мониториться).  
    - State-of-the-art раздел есть, но слабый.  
    Не выполнено.

11. **Methodology depth (replication-ready protocol, SAP, controls, replication strategy)** ✗  
    - Step-by-step protocol: общее описание (Setup, Configuration,