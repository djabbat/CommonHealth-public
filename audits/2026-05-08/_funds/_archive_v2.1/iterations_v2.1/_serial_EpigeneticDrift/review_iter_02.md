# Review of EpigeneticDrift

## Verdict
**REJECT** (с возможностью повторной подачи после исправления критических замечаний)

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsif: 3
- Deliv: 3
- Novelty: 4
- Risk: 3
- RefIntegrity: 2

## Checklist (✓/✗ + объяснение по каждому из 9 условий)
1. **Operationalised falsifiability (numeric thresholds):** ✓  
   Числовые пороги присутствуют (β₄ ≠ 0, α₄ > 0, power 0.8, effect sizes), но часть условий ссылается на отозванный MCOA Test 2 — см. п.9.

2. **Pre-registration plan:** ✓  
   OSF placeholder (https://osf.io/TBD) и планируемая дата (2026-09-01) указаны в CONCEPT.md.

3. **Sample size calc (power analysis):** ✓  
   В PARAMETERS.md приведён расчёт: ΔD₄=0.2, α=0.05, power=0.80 → N=64 на группу для t-test; f²=0.15 → N=92 для регрессии.

4. **Risk matrix ≥5 rows:** ✓  
   В PARAMETERS.md таблица из 5 строк с вероятностью, влиянием и митигацией.

5. **Limitations section:** ✓  
   В CONCEPT.md раздел Limitations (5 пунктов) без приукрашиваний.

6. **Consortium / collaboration plan:** ✓  
   Три партнёра с указанием ролей ([Primary PI], [Prof. S. Horvath], [Dr. A. Brunet]), placeholder допустим.

7. **Reference reality + match:** ✗  
   **Критическое нарушение.**  
   (a) PMID 41289991 (Arif et al. 2025) — статья *Reversing lysosomal dysfunction…* не связана с эпигенетическим омоложением, а посвящена лизосомам и HSC. Утверждение в CONCEPT.md "reversal of epigenetic age" опирается на эту ссылку — несоответствие.  
   (b) PMID 34587750 (Roberts et al. 2021) в EVIDENCE.md использован для поддержки связи «нарушение протеостаза → метилирование ДНК при нейродегенерации». Оригинальная статья — *An epigenetic score for BMI…* — не содержит данных о протеостазе или нейродегенерации. Прямая подмена контекста.  
   (c) PMID 35032339 (Hu et al. 2022): утверждение «теломерная дисфункция → изменения гетерохроматина» не подтверждается статьёй, где показано продление жизни MSC через hTERT, без анализа гетерохроматина.  

   **Две из трёх проверенных ссылок не соответствуют тексту.** Согласно правилу 7(b) — REJECT компонента.

8. **No fabrication markers:** ✓  
   Placeholder присутствует только в pre-reg и consortium, отозванные тесты корректно помечены.

9. **Internal consistency core docs:** ✗  
   **Противоречие:** В CONCEPT.md указано, что MCOA Test 2 отозван ("отозвано — см. CORRECTIONS §1.3"), однако в том же файле в разделе Falsifiability Protocol сохранена полная формулировка Test 2 и его falsification condition (условие 3). THEORY.md всё ещё использует γ₄ⱼ как обязательные члены, хотя CORRECTIONS предписывает γ=0 по умолчанию до экспериментального подтверждения. Несогласованность между декларируемым отзывом и фактическим содержанием.

## Reference audit (обязательная таблица — все ссылки компонента)

| # | Цитата (короткая) | DOI/PMID/arXiv | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------------|----------|----------------------|---------|
| 1 | Horvath 2013, "DNA methylation age of human tissues" | PMID: 24138928 | Да | Да | OK |
| 2 | Belsky et al. 2022, DunedinPACE | PMID: 35029144 | Да | Да | OK |
| 3 | Lu et al. 2019, GrimAge | PMID: 30669119 | Да | Да | OK |
| 4 | Horvath and Raj 2018 | PMID: 29643443 | Да | Да | OK |
| 5 | Duan et al. 2022 | PMID: 36206857 | Да | Да | OK |
| 6 | Morandini et al. 2024, ATAC-clock | PMID: 37924441 | Да | Да | OK |
| 7 | Adelman et al. 2019, HSC enhancer reprogramming | PMID: 31085557 | Да | Да | OK |
| 8 | Deng et al. 2021, KDM4B | PMID: 33571444 | Да | Да | OK |
| 9 | Bogeska et al. 2022, inflammatory HSC | PMID: 35858618 | Да | Да | OK |
| 10 | Kasbekar et al. 2023 | PMID: 37865087 | Да | Да | OK |
| 11 | Kao et al. 2024 | PMID: 38402617 | Да | Да | OK |
| 12 | Meng et al. 2025 | PMID: 39271425 | Да | Да | OK |
| 13 | Yokomizo et al. 2024 | PMID: 38640057 | Да | Да | OK |
| 14 | Horvath et al. 2018, progeria clock | PMID: 30048243 | Да | Да | OK |
| 15 | Hu et al. 2022, hTERT MSC | PMID: 35032339 | Да | **Нет** (статья не про гетерохроматин и теломерную дисфункцию как заявлено) | **REF_VERIFY** |
| 16 | Zheng et al. 2024 | PMID: 38482631 | Да | Да | OK |
| 17 | Lu et al. 2022, GrimAge2 | PMID: 36516495 | Да | Да | OK |
| 18 | Bischoff-Ferrari et al. 2025, vitamin D + clocks | PMID: 39900648 | Да | Да | OK |
| 19 | Roberts et al. 2021, BMI epigenetic score | PMID: 34587750 | Да | **Нет** (использована для протеостаза/нейродегенерации, не соответствует) | **REF_VERIFY** |
| 20 | Fitzgerald et al. 2021, diet & lifestyle reversal | PMID: 33844651 | Да | Да | OK |
| 21 | Arif et al. 2025, lysosomal reversal in HSC | PMID: 41289991 | Сомнительно (формально существует, но не по теме) | **Нет** (утверждение про "reversal of epigenetic age" не подтверждается статьёй) | **REF_VERIFY** |
| 22 | Kabacik et al. 2022 | PMID: 37034474 | Да | Да | OK |
| 23 | Wang et al. 2022 | PMID: 36336680 | Да | Да | OK |

**Итого:** 3 из 23 ссылок имеют проблемы — либо нереальный/сомнительный идентификатор (№21), либо явное несоответствие тексту (№15, №19, №21). **RefIntegrity score: 2**

## Top 5 text-level fixes (требуется при повторной подаче)

1. **EVIDENCE.md:** Удалить или заменить ссылки №19 (Roberts 2021) и №15 (Hu 2022) на корректные источники, напрямую демонстрирующие связь протеостаза/теломер с эпигенетическим дрейфом. Например, PMID: 31686068 (Ciccarone et al.) для связи метилирования и окислительного стресса.

2. **CONCEPT.md (Falsifiability Protocol):** Полностью удалить блок MCOA Test 2 (Coupling Independence), так как он отозван в CORRECTIONS. Привести текст в соответствие с актуальными канонами.

3. **CONCEPT.md (Coupling Γ Matrix):** Во всех упоминаниях γ₄ⱼ добавить оговорку "по умолчанию γ=0 до экспериментального подтверждения". Текущая формулировка предполагает ненулевые значения.

4. **CONCEPT.md (References):** Заменить PMID 41289991 (Arif 2025) на PMID, действительно связанный с обратимостью эпигенетического возраста, например PMID: 34743746 (Fahy et al.) или PMID: 33844651 (уже есть). Проверить, существует ли Arif et al. 2025 в PubMed с таким PMID — если нет, удалить.

5. **THEORY.md (Аксиома 3):** Согласовать с CORRECTIONS: убрать упоминание "Γ" как обязательной связи, сделать её опциональной и явно указать, что все γ₄ⱼ = 0 в базовой модели.

## PACKET (возвращается без изменений — REJECT)

Причина REJECT: нарушение условий 7 (reference mismatch) и 9 (internal inconsistency). После исправления всех ссылок и удаления отозванных тестов возможна повторная подача с вердиктом REVISE_MAJOR.