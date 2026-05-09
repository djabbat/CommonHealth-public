# Review of Ze

## Verdict
**REVISE_MAJOR**

---

## Scores (1-5)

- **Premise:** 3 — интересная объединяющая идея, но напоминает FEP/IIT. Не хватает чёткой новизны.
- **Method:** 2 — математический формализм есть, но эмпирические реализации страдают от неопределённости, effect sizes TBD.
- **Evidence:** 1 — пилотные данные N<50, I²=90.3%, отсутствие независимой репликации, все гипотезы не валидированы.
- **Falsifiability:** 2 — таблицы есть, но все числовые пороги — TBD. Фактически фальсифицируемость не операционализирована.
- **Deliverability:** 2 — консорциум описан, но все партнёры TBD. Нет подтверждённых писем поддержки.
- **Novelty:** 4 — оригинальная попытка свести к бинарному счётчику, хотя пересечение с FEP снижает новизну.
- **Risk:** 2 — высокий: неопределённые размеры эффекта, отсутствие пре-регистрации, фабрикационные маркеры в ссылках.

---

## Checklist (✓/✗ each + explanation)

1. **Operationalised falsifiability (numeric thresholds)** → ✗  
   Таблицы есть, но все effect size/δ/σ помечены `TBD`. Для каждого теста требуется конкретная числовая граница (например, `d≥0.5`, `r≥0.3`, `MCID≥0.05`). Без этого фальсифицируемость не реализована.

2. **Pre-registration plan (OSF placeholder + date)** → ✗  
   В CONCEPT.md указано: *"No studies are pre-registered"*. Нет ни OSF-идентификатора, ни планируемой даты регистрации. Требуется хотя бы один зарегистрированный протокол (например, `osf.io/XXXXX`, дата Q3 2026).

3. **Sample size calc (power analysis)** → ✗  
   Таблица `Sample size calculation` содержит формулу и плановые N, но σ и δ везде `TBD`. Без обоснованных effect sizes расчёт не принимается. Необходимо указать хотя бы ожидаемые значения (например, из литературы или пилота).

4. **Risk matrix ≥5 rows** → ✗  
   Раздел `Limitations` (7 пунктов) — это не risk matrix. Требуется таблица с колонками: риск → вероятность (0–1) → влияние (0–1) → митигация. Пример: *Недостаточная мощность — P=0.7, I=0.9, mitigation: увеличить N до расчётного.*

5. **Limitations section** → ✓  
   Есть явный список из 7 ограничений в CONCEPT.md §2.0.1.

6. **Consortium / collaboration plan** → ✓ (условно)  
   Структура описана, институты перечислены. Но все партнёры `TBD`, нет ни одного подтверждённого письма поддержки. Для ERC это слабо, но формально пункт считается выполненным.

7. **References PubMed/Crossref-verified or explicitly labelled pre-print** → ✗  
   В `KNOWLEDGE.md` есть fabrication markers (`[reference removed — to be replaced with valid PMID or removed entirely]`). Ссылка на Cuban EEG (figshare) — ок, но ссылка на HRV-CV (s4me.info) не верифицирована. Не все источники проверены.

8. **No fabrication markers ([REF_NEEDED] / [PMID_REMOVED])** → ✗  
   В `KNOWLEDGE.md` в начале файла явно указано `REF_AUDIT_2026-05-08: FABRICATION CLEANUP applied ... [reference removed — to be replaced with valid PMID or removed entirely]`. Это недопустимо в заявке/рукописи. Должно быть полностью очищено.

**Итого:** выполнено 2/8 (пункты 5 и 6), частично пункт 4 (но не как risk matrix). Остальные — критический провал.

---

## Top 5 text-level fixes (REVISE_MAJOR — что добавить/изменить)

### 1. CONCEPT.md : Operational falsifiability — заменить TBD на конкретные числа
- **file:section** `CONCEPT.md` → `#### Operational falsifiability for each hypothesis`  
  **что вписать:**  
  - Для `v*_active > v*_passive`: установить `Cohen's d = 0.5` (обоснование: средний эффект по литературе EEG clocks), `α=0.05`, `power=0.80`, `N_per_group=64`.  
  - Для `χ_Ze vs aging clock`: `r = 0.3` (минимальный клинически значимый), `N=84`.  
  - Убрать все `TBD` и заменить на обоснованные консервативные оценки с указанием источника (например, *"effect size d=0.5 based on meta-analysis of EEG biomarkers, PMID 12345678"*).  
  - Для каждой гипотезы добавить строку: *"Criterion for falsification: if 95% CI of effect excludes threshold."*

### 2. CONCEPT.md — Pre-registration plan (добавить OSF ID + дату)
- **file:section** `CONCEPT.md` → after `#### Sample size calculation`  
  **что вписать:**  
  ```markdown
  **Pre-registration plan:**
  - Study 1 (v*_active replication): OSF ID placeholder `osf.io/abc12`, registration date: 2026-09-01.
  - Study 2 (χ_Ze vs BrainYears): OSF ID placeholder `osf.io/def34`, registration date: 2026-12-01.
  - All primary analyses and stopping rules will be pre-registered before data collection. 
  ```

### 3. CONCEPT.md — Risk matrix (добавить таблицу ≥5 строк)
- **file:section** `CONCEPT.md` → после `### 2.3.0. Статус эмпирической валидации`  
  **что вписать:**
  ```markdown
  **Risk matrix:**
  | Risk | Probability (0–1) | Impact (0–1) | Mitigation |
  |------|-------------------|--------------|------------|
  | Insufficient power for primary endpoint | 0.7 | 0.9 | Increase N by 20% above calc; interim analysis at 80% N |
  | Confounding by age/sex/medication | 0.8 | 0.7 | Stratified randomization; include covariates in LMM |
  | Replication failure due to cohort heterogeneity | 0.6 | 0.8 | Multi-site consortium (3 independent labs) |
  | EEG hardware failure / data quality | 0.4 | 0.6 | Dual recording (BioSense + Emotiv); 10% oversampling |
  | Inability to validate MCID anchor-based | 0.5 | 0.7 | Use distribution-based MCID (0.5 SD) as backup |
  | Publication bias (only positive reported) | 0.9 | 0.5 | Pre-register all analyses; report null results in supplement |
  ```

### 4. KNOWLEDGE.md — Удалить все fabrication markers
- **file:section** `KNOWLEDGE.md` — весь файл  
  **что сделать:**  
  - Заменить `[reference removed ...]` на корректную ссылку или полностью удалить строку.  
  - Удалить блок комментария `<!-- REF_AUDIT_2026-05-08: FABRICATION CLEANUP applied ... -->`.  
  - Для отсутствующих ссылок использовать `[citation needed]` только если будет добавлен PMID до подачи. В текущем виде это disqualifying для ERC.

### 5. CONCEPT.md — Завершить power analysis (TBD → конкретные)
- **file:section** `CONCEPT.md` → `#### Sample size calculation (power analysis)`  
  **что вписать:**  
  - Для каждой гипотезы: указать δ (например, `δ = MCID = 0.05` для MCID, `δ = 0.3 r-to-z` для корреляции).  
  - σ: взять из пилотных данных или литературы (например, `σ_χZe = 0.03` из N=12).  
  - Итоговые N: пересчитать в явную формулу.  
  - Если данных нет, указать conservative estimate: *"effect size d=0.5 assumed per Cohen's convention for medium effect; N=64 per group"*

---

**Дополнительно:**  
- В `CONCEPT.md` раздел `v*_active — Статистический план` — методологически верно указано, что pooling при I²=90.3% некорректен. Это похвально. Однако для гранта требуется уже исправленный анализ, а не описание ошибки. Рекомендую выполнить bootstrap per dataset и представить `v*` с 95% BCa CI для каждого датасета отдельно.  
- Ссылка на BrainYears (bioRxiv) — ок, но нужно явно пометить как pre-print.  
- Все файлы (`PARAMETERS.md`, `README.md`) содержат внутренние константы, но не касаются грантовых требований — их можно не править для данного ревью.

**Вывод:** проект имеет амбициозную концепцию и частично проработанный формализм, но критически не соответствует стандартам ERC/Wellcome по фальсифицируемости, пре-регистрации, power analysis и чистоте ссылок. Без исправления перечисленных 5 пунктов — **REJECT**. После исправления — может быть рассмотрен как **REVISE_MINOR** → FUND_AS_IS при условии дополнительной валидации.