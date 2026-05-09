# Review of CytogeneticTree

## Verdict
**REJECT**

## Scores (1-5)
- Premise: 4
- Method: 2
- Evidence: 2
- Falsif: 1
- Deliv: 2
- Novelty: 4
- Risk: 3
- RefIntegrity: 1

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **Operationalised falsifiability (numeric thresholds) ✗**  
   В документе приведено **множество противоречащих друг другу наборов** порогов:  
   - N=24 per arm, α=0.001, power=0.95, d=1.2 (один расчёт)  
   - N=43 per arm (другой расчёт)  
   - N=10, N=15, N=6 (предыдущие версии)  
   - В PARAMETERS.md α=0.05, β=0.20 (противоречит CONCEPT.md)  
   Отсутствует единый **согласованный** финальный набор. Утверждение «this supersedes all earlier draft values» не подкреплено удалением старых; они остались в тексте. Условие не выполнено.

2. **Pre-registration plan (OSF placeholder + date) ✓**  
   Указан OSF ID `osf.io/TBD` (placeholder допустим) и дата `2026-07-01`. Формально выполнено.

3. **Sample size calc (power analysis) ✗**  
   Формула и подстановка есть, но присутствуют **разные финальные значения** (N=24, N=43, N=15 и т.д.). Нет единого расчёта, согласованного для всех гипотез. Расчёт в CONCEPT.md утверждает «single binding calculation», но через несколько абзацев — другой. Условие не выполнено.

4. **Risk matrix ≥5 rows ✓**  
   В CONCEPT.md приведена таблица рисков из 7 строк с probability, impact и mitigation. Формально выполнено.

5. **Limitations section ✓**  
   Явный раздел Limitations присутствует (несколько блоков). Не приукрашен. Формально выполнено.

6. **Consortium / collaboration plan ✓**  
   Таблица с ролями и статусами (большинство TBD — placeholder). Это допустимо для ранней стадии. Формально выполнено.

7. **Reference reality + match ✗**  
   - В KNOWLEDGE.md есть **неверифицированная ссылка** `Lee & Luo 1999 Neuron` (`REFERENCE VERIFICATION PENDING`). Это недопустимо в компоненте, претендующем на FUND_AS_IS/REVISE_MINOR.  
   - Есть `[Reference removed pending verification – see Limitations section for details]` — реальная ссылка отсутствует, но утверждение осталось.  
   - DOI/PMID должны быть реальными и соответствовать тексту; для Lee & Luo проверка не пройдена.  
   Условие не выполнено.

8. **No fabrication markers ✗**  
   В тексте присутствуют:  
   - `REFERENCE VERIFICATION PENDING` (аналог `[REF_NEEDED]`)  
   - `[Reference removed pending verification]` (аналог `[PMID_REMOVED]`)  
   - `TBD` в pre-reg плане допустимо, но fabrication marker явно есть.  
   Условие не выполнено.

9. **Internal consistency core docs ✗**  
   - CONCEPT.md содержит противоречивые значения N, α, power.  
   - PARAMETERS.md задаёт α=0.05, β=0.20, что конфликтует с CONCEPT.md (α=0.001, power=0.95).  
   - Falsifiability table не соответствует sample size calculation (разные N).  
   - Цели и методы не полностью согласованы между файлами.  
   Условие не выполнено.

**Итого:** выполнено 4 из 9 (пп.2,4,5,6). Не выполнены пп.1,3,7,8,9. Условия для FUND_AS_IS/REVISE_MINOR не соблюдены.

## Reference audit (обязательная таблица – все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|----------------------|---------|
| 1 | Yamashita 2007, Science | PMID 17255513 | Да (верифицировано) | Да (основное утверждение о наследовании старого центриоля) | OK |
| 2 | Rebollo 2007, Dev Cell | PMID 17336911 | Да | Да | OK |
| 3 | Wang 2009, Nature | PMID 19829375 | Да | Да | OK |
| 4 | Stearns 2009, Nature | PMID 19829363 | Да | Да | OK |
| 5 | Conduit 2010, Curr Biol | PMID 21145745 | Да | Да | OK |
| 6 | Januschke 2011, Nat Commun | PMID 21407209 | Да | Да (контрпример) | OK |
| 7 | Pelletier 2012, Curr Opin Cell Biol | PMID 22683192 | Да | Да | OK |
| 8 | Paridaen 2013, Cell | PMID 24120134 | Да | Да | OK |
| 9 | Reina 2014, Phil Trans B | PMID 25047620 | Да | Да | OK |
| 10 | Loeffler 2019, Nature | PMID 31485073 | Да (исправлено с 31485075) | Да | OK |
| 11 | Royall 2023, eLife | PMID 37882444 | Да | Да | OK |
| 12 | Gönczy 2023, Genetics | PMID 36988082 | Да | Да | OK |
| 13 | Verzijlbergen 2010, PNAS | PMID 20018668 | Да | Да (RITE оригинал) | OK |
| 14 | Radman-Livaja 2011, PLoS Biol | PMID 21666805 | Да | Да | OK |
| 15 | Terweij 2013, G3 | PMID 23708297 | Да | Да | OK |
| 16 | Thayer 2014, PNAS | PMID 25228775 | Да | Да (методологический предшественник) | OK |
| 17 | Lee & Luo 1999, Neuron | PMID 10197526 / DOI 10.1016/S0896-6273(00)80701-1 | **Не верифицирована** (помечена REFERENCE VERIFICATION PENDING) | Н/Д (утверждение о MARCM) | **REJECT** |
| 18 | Zong 2005, Cell (MADM) | PMID 15882628 | Да | Да | OK |
| 19 | Gerlach 2013, Science | PMID 23493421 | Да | Да | OK |
| 20 | Naik 2013, Nature | PMID 23552896 | Да | Да | OK |
| 21 | McKenna 2016, Science (GESTALT) | PMID 27229144 | Да | Да | OK |
| 22 | Frieda 2017, Nature (MEMOIR) | PMID 27869821 | Да | Да | OK |
| 23 | Raj 2018, Nat Biotechnol (scGESTALT) | PMID 29608178 | Да | Да | OK |
| 24 | Spanjaard 2018, Nat Biotechnol (LINNAEUS) | PMID 29644996 | Да | Да | OK |
| 25 | Plass 2018, Science | PMID 29674432 | Да | Да | OK |
| 26 | Kalhor 2018, Science (MARC1) | PMID 30093604 | Да | Да | OK |
| 27 | Raj 2018, Nat Protoc | PMID 30353175 | Да | Да | OK |
| 28 | Chan 2019, Nature (молекулярная запись) | PMID 31086336 | Да | Да | OK |
| 29 | Weinreb 2020, Science (LARRY) | PMID 31974159 | Да | Да | OK |
| 30 | Bowling 2020, Cell (CARLIN) | PMID 32413320 | Да | Да | OK |
| 31 | Weinreb 2020, PNAS | PMID 32632001 | Да | Да | OK |
| 32 | Molina 2021, Biomolecules | PMID 34680165 | Да | Да | OK |
| 33 | Lange 2024, Genome Biol (moslin) | PMID 39434128 | Да | Да | OK |
| 34 | Bowling 2025, Methods Mol Biol | PMID 39745646 | Да | Да | OK |
| 35 | Hughes 2015, Angew Chem | PMID 26418181 | Да | Да | OK |
| 36 | Haar 2019, Methods Enzymol | PMID 31155059 | Да | Да | OK |
| 37 | Stringer 2021, Nat Methods (Cellpose) | PMID 33318659 | Да | Да | OK |
| 38 | Thomsen 2020, eLife (DeepFRET) | PMID 33138911 | Да | Да | OK |
| 39 | Aspert 2022, eLife (DetecDiv) | PMID 35976090 | Да | Да | OK |
| 40 | Mahecic 2022, Nat Methods | PMID 36076039 | Да | Да | OK |
| 41 | Togninalli 2023, NPJ Regen Med | PMID 36639373 | Да | Да | OK |
| 42 | Zhang 2023, Nat Methods | PMID 37770712 | Да | Да | OK |
| 43 | Schindelin 2012, Nat Methods (Fiji) | PMID 22743772 | Да | Да | OK |
| 44 | Edelstein 2014, J Biol Methods (µManager) | PMID 25606571 | Да | Да | OK |

**Примечание:** Ссылка #17 (Lee & Luo 1999) помечена как `REFERENCE VERIFICATION PENDING` и не верифицирована. Это автоматический REJECT компонента согласно правилу 7. Кроме того, в тексте присутствует `[Reference removed pending verification]` – ссылка удалена, но утверждение осталось; это является fabrication marker.

**Итоговая оценка RefIntegrity: 1 из 5.**

## Top 5 text-level fixes (если НЕ FUND_AS_IS)

1. **CONCEPT.md:§Falsifiability, §Sample size calculation** — Устранить все противоречия: оставить **единый** набор порогов (α=0.001, power=0.95, N=XX per arm, effect size d=1.2). Удалить все старые значения (N=6,10,15,24,43). Согласовать с PARAMETERS.md (сейчас там α=0.05, β=0.20 – исправить).

2. **KNOWLEDGE.md:§2 Block 1 (MARCM Lee & Luo 1999)** — Либо **верифицировать** PMID 10197526 через PubMed и подтвердить, что содержание соответствует тексту (асимметричное наследование центриолей), либо **удалить** эту ссылку и сопутствующее утверждение. Fabrication marker `[Reference removed pending verification]` также необходимо удалить или заменить на реальную ссылку.

3. **CONCEPT.md:§Risk matrix** — Внести риск, связанный с неверифицированными ссылками и fabrication markers (вероятность 5, impact 5, mitigation: устранить до подачи). Учесть также риск несоответствия PARAMETERS.md.

4. **PARAMETERS.md:§Statistical parameters** — Изменить α=0.001, power=0.95 (а не 0.05 / 0.20) для согласованности с CONCEPT.md. Указать, что это единые параметры для всех гипотез.

5. **CONCEPT.md:§Consortium / partners** — Заменить `TBD` на реальные имена и институты хотя бы для ключевых ролей (PI, host institution). Без этого consortium plan остаётся фиктивным.

**Дополнительно:** Удалить все дублирующиеся и противоречащие блоки по falsifiability и sample size (в тексте есть по 2–3 копии). Привести к единой структуре.

---

**Заключение:** Компонент содержит фундаментальные нарушения: несогласованность ключевых параметров, неверифицированную ссылку, fabrication markers. Вердикт — REJECT. Для повторного ревью требуется полная переработка с устранением всех замечаний.