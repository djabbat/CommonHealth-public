# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: **4** (практически важная идея, но не беспрецедентная)
- Method: **3** (детальность хорошая, но много placeholder/TBD)
- Evidence: **2** (только 1–2 источника на ключевые утверждения, мета-анализ отсутствует)
- Falsif: **4** (основная гипотеза M1 fully operationalised; contamination/uptime неполны)
- Deliv: **3** (бюджет реалистичен, но DIY-качество ±2 мкм под вопросом; AI-hallucination risk не оценён)
- Novelty: **4** (первое применение Claude-class LLM для live-cell microscopy)
- Risk: **3** (несколько высоких рисков с неполной mitigation)
- RefIntegrity: **5** (все проверенные ссылки реальны и соответствуют)
- EvidenceDepth: **1** (нет ≥3 источников на ключевые claims; нет систематического обзора)
- MethodDepth: **4** (SAP, controls, blinding описаны; не хватает step-by-step протокола для hardware)
- Reproducibility: **3** (планы есть, но GitHub/TBD, OSF/TBD, protocols.io/TBD)

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **Operationalised falsifiability** — **✗**  
   Для M1 (concordance) есть полные пороги: N=286, α=0.05, power=0.80, Δ=0.05.  
   Для uptime H₀: ≤0.90, 180 дней — есть.  
   Contamination: N=TBD — не операционализировано.  
   M2–M4 имеют качественные критерии, но не все числовые.  
   *Требуется завершить для всех гипотез.*

2. **Pre-registration plan** — **✓**  
   OSF identifier `osf.io/automicroscopy_cdata` (placeholder) + date 2026-06-01.

3. **Sample size calc (power analysis)** — **✓**  
   Для CDATA: n=(1.96+0.84)²·(0.4²+0.4²)/0.3² = 28.4 → 30 per group, Cohen’s d=0.75.  
   Для contamination — TBD (неполнота, но главная гипотеза покрыта).

4. **Risk matrix ≥5 rows** — **✓**  
   6 строк в CONCEPT.md + 7 строк в EVIDENCE.md.

5. **Limitations section** — **✓**  
   8 пунктов в CONCEPT.md, также в EVIDENCE.md.

6. **Consortium / collaboration plan** — **✓**  
   Таблица с ролями и статусами (LongevityCommon, Bristol, Zeiss, FLIR, ThorLabs, OpenTrons