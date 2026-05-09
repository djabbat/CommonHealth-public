# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1–5)
- Premise: 3
- Method: 2
- Evidence: 2
- Falsif: 4
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 3
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 1

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability (numeric thresholds)** – ✓  
   Чёткие H₀/H₁ для M1 (concordance >80%), uptime, contamination, cost. Указаны α=0.05, мощность 0.80, N=286. Пороги числовые и воспроизводимые.

2. **Pre-registration plan (OSF placeholder + date)** – ✓ (условно)  
   Планируется OSF, дата 2026-06-01, но ID — `osf.io/TBD` / `osf.io/automicroscopy_cdata` (плейсхолдер). Принимается как план, но требуется конкретный ID до подачи.

3. **Sample size calc (power analysis)** – ✓  
   Для primary endpoint (concordance) — формула с подстановкой: n=286. Для uptime и contamination — частично TBD, но primary расчёт выполнен.

4. **Risk matrix ≥5 rows** – ✓  
   В CONCEPT.md — 6 строк с probability/impact/mitigation. В EVIDENCE.md — другая матрица. Минимум 5 есть.

5. **Limitations section** – ✓  
   Явные разделы в CONCEPT.md (8 пунктов) и EVIDENCE.md. Перечислены DIY precision, drift, phototoxicity, AI hallucination и др.

6. **Consortium / collaboration plan** – ✓ (условно)  
   Список партнёров с ролями (LongevityCommon, Bristol, Zeiss, FLIR, ThorLabs, OpenTrons). Статусы многих “TBD (placeholder)”, но план существует.

7. **Reference reality + match** – ✗  
   Все ссылки в EVIDENCE.md (OpenFlexure, Hayflick, CellPose, ImageJ, GT335, Ninein, Burger, Boiko, Bran) реальны и соответствуют тексту.  
   **НО** в CONCEPT.md в разделе “Evidence base & meta-analysis” вместо ссылок стоят `[Placeholder: e.g., …]`, что не является реальными цитатами. Это нарушение п.7 и п.8.

8. **No fabrication markers** – ✗  
   В CONCEPT.md: “TBD” для sample size contamination эксперимента, “TBD” для design effect, “TBD” для OSF ID, “TBD” для партнёров. В разделе “Evidence base” – `[Placeholder]`. Это TBD там, где должны быть конкретные данные.

9. **Internal consistency core docs** – ✓ (условно)  
   Основные утверждения согласованы между CONCEPT, THEORY, EVIDENCE. Есть мелкие расхождения: OSF ID указан по-разному (`osf.io/TBD` vs `osf.io/automicroscopy_cdata`). В целом непротиворечиво, но требуется унификация.

10. **Evidence base depth (≥3 indep refs/claim, sys-review, contradictions)** – ✗  
    - Ключевое утверждение “low-cost microscope automation” опирается только на OpenFlexure (1 источник).  
    - “AI for microscopy decision-making” – только три ссылки на химические работы, не на микроскопию.  
    - Нет систематического обз