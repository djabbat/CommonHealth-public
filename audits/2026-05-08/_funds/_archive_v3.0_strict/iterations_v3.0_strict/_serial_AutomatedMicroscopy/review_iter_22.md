# Review of AutomatedMicroscopy

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 2
- Falsif: 3
- Deliv: 2
- Novelty: 4
- Risk: 2
- RefIntegrity: 1
- EvidenceDepth: 2
- MethodDepth: 2
- Reproducibility: 1

## Checklist (✓/✗ + объяснение по каждому из 12 условий)

1. **✗ Operationalised falsifiability (numeric thresholds)**  
   Для M1 (concordance) есть N=286, α=0.05, power=0.80, Δ=0.05. Для uptime N=180. Однако для contamination N=TBD (плейсхолдер), для cost нет формальной гипотезы с мощностью. Не все falsification criteria численно заданы.

2. **✓ Pre-registration plan (OSF placeholder + date)**  
   OSF ID: `osf.io/TBD`, planned date 2026-06-01. Плейсхолдер допустим, дата указана.

3. **✓ Sample size calc (power analysis)**  
   Для CDATA: n = (1.96+0.84)² × (0.4²+0.4²) / 0.3² = 28.4 → 30 cells/group, α=0.05, power=0.80, Cohen's d=0.75. Формула и подстановка приведены.

4. **✓ Risk matrix ≥5 rows**  
   В CONCEPT.md две матрицы по 6 рисков с Probability/Impact и mitigation. ≥5.

5. **✓ Limitations section**  
   Два явных раздела "Limitations" в CONCEPT.md, перечислены конкретные ограничения.

6. **✓ Consortium / collaboration plan**  
   Таблица партнёров (LongevityCommon, Univ. Bristol, Zeiss, FLIR, ThorLabs, OpenTrons) с ролями и статусами. Дополнительно в DESIGN.md ещё список. План есть.

7. **✗ Reference reality + match**  
   В разделе "Evidence base & meta-analysis" CONCEPT.md присутствуют фабрикационные маркеры: `[Author(s), Year, Journal, DOI TBD]` — невалидный идентификатор. Другие ссылки (Burger, Boiko, Bran) имеют DOI, но их соответствие утверждениям не проверено нами. Наличие невалидного DOI/PMID = автоматический REJECT по условию.

8. **✗ No fabrication markers**  
   Помимо [DOI TBD], в CONCEPT.md есть `[Reference needed — placeholder: replace with DOI or PMID before submission]` (удалён, но след остался) и `Placeholder: e.g., OpenTrons, µManager` вместо конкретных ссылок. Это прямые фабрикационные маркеры.

9. **✓ Internal consistency core docs**  
   THE