## Review of CDATA

## Verdict
**REJECT**

## Scores (1-5)
- **Premise**: 4  
- **Method**: 3  
- **Evidence**: 3  
- **Falsif**: 3  
- **Deliv**: 2  
- **Novelty**: 5  
- **Risk**: 2  

## Checklist (✓/✗ each + explanation)
1. **Operationalised falsifiability (numeric thresholds)** – ✓  
   OPEN_PROBLEMS.md содержит конкретные количественные критерии (r_spearman > 0.6, p < 0.01; r < -0.5 и т.д.) для тестов FT1.1–FT6.1. Предсказания P1–P10 в THEORY.md также имеют числовые пороги.

2. **Pre-registration plan (OSF placeholder + date)** – ✗  
   Ни OSF‑идентификатора, ни запланированной даты пререгистрации нет. В risk matrix упомянуто "Pre‑register analysis plan", но без конкретного идентификатора или срока.

3. **Sample size calc (power analysis)** – ✗  
   Отсутствует расчёт необходимого размера выборки для экспериментальных тестов. Нет указания effect size, α, power → N. Критерии даны качественно или как корреляционные пороги, но без обоснования мощности.

4. **Risk matrix ≥5 rows** – ✓  
   5 строк (Low reproducibility, Negative result, Key personnel turnover, Budget overrun, Data management failure) с Probability, Impact, Mitigation.

5. **Limitations section** – ✓  
   В EVIDENCE.md раздел «Опровергающие свидетельства и ограничения» (6 пунктов). В CONCEPT.md есть «Ограничение (для Aging Cell §Limitations)».

6. **Consortium / collaboration plan** – ✓  
   В OPEN_PROBLEMS.md перечислены 4 потенциальных партнёра с указанием учреждений (Yale, Stanford, Max Planck, Cambridge). Placeholder достаточен для данной стадии.

7. **References PubMed/Crossref-verified** – ✗  
   Не все ссылки верифицированы. В начале файлов указано «REF_AUDIT_2026-05-08» с маркерами [REF_NEEDED 2026-05-08] и [PMID_REMOVED 2026-05-08]. Некоторые утверждения (например, «Nat Chem Biol 2024 (positive feedback)») не имеют PMID/DOI. Часть ссылок помечена как требующая замены.

8. **No fabrication markers** – ✗  
   Fabrication markers присутствуют: [REF_NEEDED 2026-05-08], [PMID_REMOVED 2026-05-08], [pre‑print placeholder: DOI TBD]. Хотя проведена чистка, наличие этих маркеров свидетельствует о неустранённых fabrication следах. Требование считается невыполненным.

**Итого:** выполнено 4/8 пунктов. Fabrication markers и отсутствие пререгистрации с power analysis — блокирующие недостатки.

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)
- `CONCEPT.md` — добавить раздел **Pre‑registration Plan**: «We will preregister the analysis plan for Tests P1–P6 on OSF (placeholder: https://osf.io/XXXXX) prior to data collection, with planned date 2026-09-01.»
- `OPEN_PROBLEMS.md` — для тестов FT1.1, FT2.1, FT6.1, FT6.2 добавить **power analysis**: указать ожидаемый effect size (например, r = 0.6), α = 0.05, power = 0.80 → расчёт N. Пример: «Для обнаружения корреляции r ≥ 0.6 при α = 0.05 и power 0.80 требуется N ≥ 19 парных наблюдений (Fisher’s z‑transformation).»
- `EVIDENCE.md` — заменить все маркеры `[REF_NEEDED 2026-05-08]` на валидные PMID/DOI. Для ссылок без PMID (например, «Nat Chem Biol 2024») указать конкретный DOI или удалить утверждение.
- `CONCEPT.md` и `EVIDENCE.md` — удалить fabrication markers (`[REF_NEEDED]`, `[PMID_REMOVED]`, `[pre-print placeholder]`) после их замены на реальные ссылки. Привести полный список ссылок к стандарту (все верифицированы через PubMed/Crossref).
- `ROADMAP.md` или `CONCEPT.md` — добавить явный раздел **Sample Size Calculation** для ключевых экспериментов (Peters-Hall репликация, CCP1 KO, LDC10 ингибирование). Указать, как количество животных/клеток определено, и включить обоснование статистической мощности.

## PACKET
Пакет содержит все 8 файлов (CONCEPT.md, THEORY.md, EVIDENCE.md, PARAMETERS.md, OPEN_PROBLEMS.md, DESIGN.md, STATE.md, README.md). Текстовый уровень высок: формальная теория с ODE, 32 параметра, Sobol анализ, falsification tests. Однако критические недостатки (pre‑registration, power analysis, fabrication markers) делают заявку неприемлемой для ERC AdG / NIH R01 / Wellcome Discovery / EIC Pathfinder. Рекомендуется доработка с обязательным устранением fabrication markers и добавлением количественного обоснования выборки.