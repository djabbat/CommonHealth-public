# Review of AIM

## Verdict
**REJECT**

## Scores (1-5)
- **Premise:** 2 – концепция (L3 validation через PAM-13) имеет потенциальную новизну, но неоперационализирована и размыта избыточными деталями об инфраструктуре.
- **Method:** 1 – методологический план фрагментирован, содержит противоречивые версии (α, N, sample size), отсутствует единый когерентный протокол.
- **Evidence:** 1 – большинство ключевых ссылок (`Tao 2026`, `Blumenthal-Lee 2024`, `Tkemaladze 2026`) не имеют валидного DOI/PMID; цитируемые источники не подтверждены.
- **Falsifiability:** 2 – числовые пороги указаны, но не согласованы между core-документами (разные α, разный N), нет порогов для secondary endpoints.
- **Deliverables:** 1 – roadmap не содержит конкретных вех, нет плана по software delivery, deployment или clinical integration.
- **Novelty:** 3 – трёхуровневая модель L1/L2/L3 и концепция "patient as project" оригинальны, но не подкреплены реалистичными данными/ссылками.
- **Risk:** 2 – несколько версий risk matrix, несогласованных между собой; не все риски количественно оценены.
- **RefIntegrity:** 1 – см. Reference Audit; 7 из 9 научных ссылок невалидны (отсутствие DOI/PMID или "DOI TBD").

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **✗ Operationalised falsifiability (numeric thresholds):**  
   – В разных секциях указаны разные α (0.05 без поправки, 0.025 с Bonferroni), разный N (110, 132, 110+20% dropout).  
   – Для secondary endpoints (e.g., MMAS-8, EQ-5D-5L) falsifiability thresholds не заданы.  
   – Не указано, как проверяется условие "PAM-13 Δ < 5.4 points → L3 hypothesis not supported" с учётом множественных сравнений.

2. **✗ Pre-registration plan (OSF placeholder + date):**  
   – В CONCEPT.md присутствуют как минимум три различных pre-registration блока (c `osf.io/TBD`, `osf.io/XXXXX`, и планируемой датой 2026-09-01). Они не консолидированы.  
   – Placeholder `osf.io/TBD` пока не зарезервирован; допустим, но требует замены до подачи.  
   – Отсутствует описание анализа secondary outcomes и плана коррекции на множественность.

3. **✗ Sample size calc (power analysis):**  
   – Формула n = (Zα/2+Zβ)²·2σ²/δ² приведена, но подстановка даёт n=53.8, а округление до 55 не объяснено.  
   – В других версиях указан N=110 + 20% dropout = 132, что не согласуется с n=55/group (110 total).  
   – Чувствительный анализ (σ=8,10,12) имеется, но не указан α-spending для interim analysis.  
   – Для secondary outcomes power analysis отсутствует.

4. **✗ Risk matrix (≥5 rows):**  
   – Представлены минимум три разных risk matrix (одна с probability/impact 1-5, другая с Low/Medium/High, третья с 5 строками).  
   – Ма