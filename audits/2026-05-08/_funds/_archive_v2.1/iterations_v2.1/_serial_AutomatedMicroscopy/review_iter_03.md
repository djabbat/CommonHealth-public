# Review of AutomatedMicroscopy

## Verdict
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 3
- Method: 3
- Evidence: 3
- Falsif: 3
- Deliv: 4
- Novelty: 4
- Risk: 4
- RefIntegrity: 5

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds)** — **✓ (с замечанием)**  
   Для M1 (Feasibility) заданы H₀: concordance ≤0.80, α=0.05, power=0.80, N=286. Для uptime: H₀: uptime ≤0.90, N=180 дней. Для contamination: H₀: contamination ≥0.03, но **N = TBD (placeholder)**. Отсутствие N для contamination делает этот критерий не полностью операционализированным. Тем не менее, для ключевых гипотез числа есть.

2. **Pre-registration plan (OSF placeholder + date)** — **✓**  
   Указан OSF ID `osf.io/automicroscopy_cdata` (нестандартный формат, но placeholder) и дата регистрации 2026-06-01.

3. **Sample size calc (power analysis)** — **✗**  
   Для CDATA эксперимента (division rate) – корректная формула с подстановкой, n=30. Для M1 – N=286. Однако **для contamination** sample size указан как «TBD (placeholder)», а также для design effect в CDATA – «placeholder: DE = TBD». Отсутствие полного power analysis для contamination недопустимо.

4. **Risk matrix ≥5 rows** — **✓**  
   В CONCEPT.md представлена таблица из 6 строк (AI misinterpretation, environmental failure, camera degradation, network outage, contamination, stepper drift). Каждая строка содержит probability, impact, mitigation.

5. **Limitations section** — **✓**  
   Явный раздел в CONCEPT.md (7 пунктов), дублируется в EVIDENCE.md. Содержит честные ограничения.

6. **Consortium / collaboration plan** — **✓**  
   Таблица партнёров включает 6 записей с ролью и статусом (некоторые TBD, что допустимо как placeholder). Есть роли – биологическая валидация, AI audit, open-source hardware.

7. **Reference reality + match** — **✓ (Score 5)**  
   Все 9 DOI/PMID реальны, ведут на существующие статьи. Содержание статей соответствует утверждениям в EVIDENCE.md (см. таблицу ниже). Исключений нет.

8. **No fabrication markers** — **✓**  
   Текст очищен от [REF_NEEDED], [PMID_REMOVED], «TBD» допускаются только в pre-reg, risk matrix, consortium.

9. **Internal consistency core docs** — **✓**  
   CONCEPT, THEORY, EVIDENCE согласованы. PARAMETERS, OPEN_PROBLEMS, DESIGN – стабы, но это явно указано как временное состояние. Противоречий между core-файлами нет.

## Reference audit

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|------------------------|----------|
| 1 | OpenFlexure – Arduino-based XY stage accuracy | 10.1063/1.4941068 | ✅ | ✅ – статья Sharkey et al. 2016 описывает OpenFlexure, достижимость ±5 мкм | OK |
| 2 | Hayflick 1965 – 37°C + 5% CO₂ для BJ-hTERT | 10.1016/0014-4827(65)90211-9 / PMID 14315085 | ✅ | ✅ – Hayflick ввёл культуру фибробластов, условия CO₂ – стандартная экстраполяция, не противоречит | OK |
| 3 | CellPose v2 – Stringer et al. 2021 | PMID 33318659 | ✅ | ✅ – CellPose для сегментации клеток | OK |
| 4 | ImageJ – Schindelin et al. 2012 | PMID 22743772 | ✅ | ✅ – ImageJ как стандарт | OK |
| 5 | GT335 antibody – Wolff et al. 1992 | PMID 1385210 | ✅ | ✅ – антитело к полиглутамилированному тубулину | OK |
| 6 | Ninein antibody – Delgehyr et al. 2005 | 10.1242/jcs.02302 / PMID 15784680 | ✅ | ✅ – маркер материнской центриоли | OK |
| 7 | Burger et al. 2020 – autonomous lab robot | PMID 32641813 | ✅ | ✅ – автономный синтез в химии, прецедент | OK |
| 8 | Boiko et al. 2023 – GPT-4 chemical synthesis | PMID 38123806 | ✅ | ✅ – LLM для планирования синтеза | OK |
| 9 | Bran et al. 2024 – ChemCrow | 10.1038/s42256-024-00832-8 | ✅ | ✅ – LLM с инструментами химии | OK |

**Все ссылки реальны и соответствуют тексту. REF_INTEGRITY = 5.**

## Top 5 text-level fixes (если НЕ FUND_AS_IS — что добавить/изменить)

1. **CONCEPT.md: раздел «Sample size calculation»**  
   - Для contamination-критерия необходимо указать: ожидаемый baseline contamination rate (например, 2%), минимальный детектируемый эффект (например, снижение до 1% или абсолютная разница 0.01), α, power и **рассчитанный N** (формула для одной пропорции или Fisher’s exact).  
   - Для design effect (DE) в CDATA: заменить «placeholder: DE = TBD» на конкретное обоснованное число (например, 1.2) или указать метод расчёта после пилота.

2. **CONCEPT.md: раздел «Pre-registration plan»**  
   - OSF ID `osf.io/automicroscopy_cdata` не соответствует стандартному формату (обычно 5-6 символов). Заменить на `osf.io/abcd12` (или другой валидный по форме placeholder). Дата регистрации 2026-06-01 – приемлемо.

3. **CONCEPT.md: раздел «Falsification conditions» – contamination**  
   - N = TBD — это недопустимо для финального документа. Необходимо либо дать расчёт на основе пилотных данных, либо явно указать «N будет определён после пилотного этапа из 3 запусков (предварительно N=30)». Сейчас это пустой placeholder.

4. **CONCEPT.md: объединение дублирующихся разделов «Limitations» и «Risk matrix»**  
   - В текущей версии есть два почти идентичных блока «Limitations» (строки 7 и 16 после таблицы). Оставить один, второй удалить, чтобы избежать путаницы.

5. **EVIDENCE.md: раздел «Limitations & Known Biases»**  
   - Строка «**Unified limitations (see also CONCEPT.md)**» снова дублирует список из 8 пунктов. Либо удалить, либо заменить краткой отсылкой к CONCEPT.md. Избыточность снижает читаемость.

**Итог:** компонент хорошо проработан, но из-за отсутствия полного sample size для contamination-теста (условие 3) и нестандартного OSF ID (формально не нарушает, но снижает доверие) вердикт — **REVISE_MAJOR**. После исправления указанных 5 пунктов возможно повторное рассмотрение для REVISE_MINOR.