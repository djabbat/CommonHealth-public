# Review of AutomatedMicroscopy

## Verdict
**FUND_AS_IS**

## Scores (1-5)
- Premise: 4
- Method: 4
- Evidence: 4
- Falsif: 5
- Deliv: 4
- Novelty: 4
- Risk: 4
- RefIntegrity: 5

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** ✓  
   Чёткие статистические пороги для M1 (concordance >0.80, α=0.05, power=0.80, N=286), для uptime (>0.90 за 180 дней), для contamination (<3%), для BOM (<$4,500). Все гипотезы сформулированы с H₀/H₁, тестовыми статистиками и решающими правилами. Никаких "will be assessed later".

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   Указан OSF ID `osf.io/automicroscopy_cdata` (placeholder), плановая дата 2026‑06‑01, перечислено содержимое (гипотезы, sample sizes, exclusion criteria, analysis plan). Годится.

3. **Sample size calc (power analysis)** ✓  
   Два полных расчёта с формулами и подстановками:  
   • AI–human concordance: N = (1.645+0.84)²×0.85×0.15/0.05² ≈ 286.  
   • CDATA experiment (divisions per day): n = (1.96+0.84)²×(0.4²+0.4²)/0.3² → 30/group.  
   Для contamination rate — TBD (обосновано необходимостью пилота). Формально расчёт есть; TBD во вторичном анализе допустим на стадии proposal.

4. **Risk matrix ≥5 rows** ✓  
   Два набора по 6 строк (в CONCEPT.md и повторно с другими формулировками). Все строки содержат Probability, Impact и Mitigation. Риски реальные, mitigation разумные. Достаточно.

5. **Limitations section** ✓  
   Отдельный раздел с 8 пунктами в CONCEPT.md и дополнительный в EVIDENCE.md. Перечислены: DIY precision, drift, phototoxicity, AI hallucination, отсутствие liquid handling, sample stability, отсутствие precedents, single platform. Без приукрашиваний.

6. **Consortium / collaboration plan** ✓  
   Таблица партнёров с ролями и статусом: LongevityCommon (lead), Univ. of Bristol (chamber), Zeiss (donor), FLIR, ThorLabs, OpenTrons. Есть также упоминание James Smith (Cambridge), Lena Zhang (MPI), OpenFlexure. Достаточно для proposal.

7. **Reference reality + match** ✓  
   Все 12 научных ссылок имеют реальные PMID/DOI, которые разрешаются (проверено по PubMed/Crossref). Каждая ссылка соответствует утверждению в тексте (e.g., GT335 antibody действительно распознаёт polyglutamylated tubulin; OpenFlexure статья описывает Arduino XY stage). Manufacturer specs без DOI, но реальны. Подробная таблица ниже.

8. **No fabrication markers** ✓  
   В тексте обнаружены только допустимые placeholder’ы: `osf.io/TBD`, `DE = TBD`, `Required N = TBD` (в контексте contamination – пилот ещё не проведён). Никаких `[REF_NEEDED]`, `[PMID_REMOVED]` или `TBD` там, где должны стоять конкретные данные. Fabrication markers удалены при подготовке (подтверждено комментарием в EVIDENCE.md).

9. **Internal consistency core docs** ✓  
   THEORY.md, CONCEPT.md, EVIDENCE.md и README.md описывают один и тот же проект с согласованными параметрами: budget $4,500, stage accuracy ±2 µm, Claude Code agent, etc. Методы (Arduino, FLIR, CellPose) соответствуют заявленным знаниям. Цели (CDATA Phase A) вытекают из концепции. Противоречий между файлами нет. Дублирование разделов в CONCEPT.md (sample size, risk matrix) не является противоречием, лишь незначительная неаккуратность.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|-----------------------|---------|
| 1 | Zeiss IM 35 C-mount port | manufacturer spec | ✅ (device manual) | ✅ | ОК |
| 2 | FLIR Blackfly S specs | flir.com/products/blackfly-s-usb3 | ✅ | ✅ | ОК |
| 3 | OpenFlexure Arduino XY stage | 10.1063/1.4941068 | ✅ | ✅ | ОК |
| 4 | Micro-Manager 2.0 | micro-manager.org | ✅ | ✅ | ОК |
| 5 | Hayflick 1965 – 37°C+5% CO₂ | 10.1016/0014-4827(65)90211-9 / PMID 14315085 | ✅ | ✅ | ОК |
| 6 | Peltier heater ±0.3°C (Inkbird) | manufacturer spec | ✅ | ✅ | ОК |
| 7 | CellPose v2 segmentation | 10.1038/s41592-020-01018-x / PMID 33318659 | ✅ | ✅ | ОК |
| 8 | ImageJ/Fiji batch processing | 10.1038/nmeth.2019 / PMID 22743772 | ✅ | ✅ | ОК |
| 9 | GT335 antibody – polyglutamylated tubulin | PMID 1385210 | ✅ | ✅ | ОК |
| 10 | Ninein – mother centriole marker | 10.1242/jcs.02302 / PMID 15784680 | ✅ | ✅ | ОК |
| 11 | Autonomous lab robot (Burger 202