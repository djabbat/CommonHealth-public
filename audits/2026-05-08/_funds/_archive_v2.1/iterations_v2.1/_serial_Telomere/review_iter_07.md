# Review of Telomere

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- **Premise:** 4 — концепция теломер как количественного счётчика в MCOA обоснована, актуальна, чётко выделено деление на α (деления) и β (стресс). Однако не хватает обсуждения альтернативных гипотез (например, что теломеры — не причинный механизм, а эпифеномен).
- **Method:** 5 — безупречная формализация: кинетическое уравнение выведено, аксиомы T1–T3 сформулированы, параметры обоснованы, для неопределённых параметров предложены протоколы измерения, есть power analysis, pre-registration, risk matrix. Лучший образец среди всех counter-документов проекта.
- **Evidence:** 5 — 21 верифицированный PMID, таблицы с указанием силы и источника, честное раскрытие противоречащих данных (refuting evidence). Полное соответствие стандартам PubMed/Crossref verification.
- **Falsifiability:** 5 — четыре явных условия фальсификации с числовыми порогами (10 bp/PD, 5 bp/year), power analysis (effect size, α, power → N), risk matrix с вероятностью и действиями. Образец для подражания.
- **Deliverability:** 3 — кодовая архитектура (DESIGN.md) продумана, но отсутствует план сотрудничества (consortium). Нет списка потенциальных партнёров, лабораторий, институтов. Это бросает тень на реализуемость в крупном проекте. Оценка снижена.
- **Novelty:** 4 — интеграция теломерного укорочения с окислительным стрессом через BER (8-oxoG) и связь с протеостазом (RIOK2–TRiC) действительно свежие элементы. Но базовая идея «теломеры — часы старения» не нова. Не хватает сравнения с существующими моделями (например, Stochastic Telomere Loss model).
- **Risk Management:** 5 — risk matrix на 8 строк (две таблицы по 4), probability × impact × action, decision tree. Высший балл.

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** ✓  
   CONCEPT.md §6, Falsification Conditions 1–4, каждый с числовыми порогами (α₂ ≤ 10 bp/PD, β₂ ≤ 5 bp/year). Дополнительно OPEN_PROBLEMS.md содержит power analysis с effect size, α, power, N.

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   CONCEPT.md: placeholder `https://osf.io/abcde`, planned date 2026-09-30. План соответствует требованиям.

3. **Sample size calc (power analysis)** ✓  
   CONCEPT.md (Falsification Condition 1) и OPEN_PROBLEMS.md (OP-T1, OP-T2): effect size, α=0.05, power=0.80 → N≥10–12 независимых образцов.

4. **Risk matrix ≥5 rows** ✓  
   OPEN_PROBLEMS.md содержит две таблицы (OP-T1 и OP-T2), суммарно 8 строк, каждая с probability, impact, action. Хотя матрица не вынесена в отдельный раздел, формально условие выполнено.

5. **Limitations section** ✓  
   CONCEPT.md §7 (6 open questions), EVIDENCE.md (Refuting Evidence). Честно перечислены ограничения: неопределённость τ₂, сложность разделения α₂ и β₂ in vivo, игнорирование распределения длин.

6. **Consortium / collaboration plan** ✗  
   Ни один из документов (CONCEPT.md, README.md, DESIGN.md и др.) не содержит перечня потенциальных партнёров, институтов, коллабораторов. Отсутствует даже placeholder. Это критическое нарушение для заявок в ERC AdG / EIC Pathfinder / NIH R01.

7. **References PubMed/Crossref-verified** ✓  
   Все 21 PMID верифицированы через NCBI E-utilities (дата 2026-04-22), указан статус в EVIDENCE.md. Есть раздел PMID verification status.

8. **No fabrication markers** ✓  
   [REF_NEEDED] или [PMID_REMOVED] не обнаружены. Все утверждения имеют корректные идентификаторы.

## Top 5 text-level fixes

1. **file:CONCEPT.md — добавить раздел "Consortium and Collaboration Plan"**  
   Вписать хотя бы placeholder:  
   > *Potential collaborators: [Laboratory of Telomere Biology, University of X]; [Bioinformatics group at Institute Y]; [Clinical partner Z for longitudinal cohort access]. Formal agreements to be established post-funding.*  
   Без этого план нежизнеспособен для крупного гранта.

2. **file:CONCEPT.md — объединить risk assessment в единую таблицу ≥5 строк**  
   Перенести таблицы из OPEN_PROBLEMS.md в §6 или новый §8, добавив строки для общих рисков (например, "нет воспроизводимости результатов", "сложность перехода in vitro → in vivo"). Текущие 4+4 строки формально проходят, но лучше иметь единую матрицу.

3. **file:CONCEPT.md — добавить sample size calculation для всех тестов в §6**  
   Сейчас power analysis есть только для Falsification Condition 1. Для Condition 2–4 и для OP-T2 требуется явный расчёт N. Перенести из OPEN_PROBLEMS.md или дополнить.

4. **file:CONCEPT.md — обсудить альтернативные модели теломерного старения**  
   Например, модель Stochastic Telomere Loss (Steenstrup et al., 2013) или модель "shortest telomere as trigger". Это повысит научную зрелость.

5. **file:OPEN_PROBLEMS.md — уточнить критерии остановки для OP-T1 (Outcome 4)**  
   Сейчас "If outcome 4 occurs, initiate Model Update Protocol". Но не указано, какая статистическая процедура используется для решения "no significant change". Добавить: "Bayesian factor < 3 or frequentist p > 0.10 with equivalence test".

---

## PACKET
(всё содержимое файлов возвращается без изменений, так как вердикт — REVISE_MAJOR, а не REJECT или TOXIC_WITHDRAW).