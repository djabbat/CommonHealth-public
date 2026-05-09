# Review of UMBRELLA

## Verdict  
**REVISE_MINOR**

## Scores (1-5)
- Premise: 3  
- Method: 2  
- Evidence: 1  
- Falsifiability: 3  
- Deliv: 2  
- Novelty: 4  
- Risk: 2  

## Checklist (✓/✗ each + explanation)
1. **Operationalised falsifiability (numeric thresholds)** ✓  
   *M4, v* и χ_Ze pre‑registration задают N≥2000, α=0.001, partial r²<0.05. Числовые пороги есть.*

2. **Pre-registration plan (OSF placeholder + date)** ✓  
   *Есть placeholder `https://osf.io/TBD`, дата 2026-12-31, первичный исход указан.*

3. **Sample size calc (power analysis)** ✓  
   *Приведена мощность 80% для R²=0.3 → N=1875, N≥2000. Расчёт присутствует, хотя эффект для самого χ_Ze не указан.*

4. **Risk matrix ≥5 rows** ✓  
   *5 строк с вероятностью, влиянием, митигацией. Достаточно.*

5. **Limitations section** ✓  
   *6 явных ограничений в CONCEPT.md.*

6. **Consortium / collaboration plan** ✓  
   *Названы Geiger, Janke, González Ballester; есть обсуждение EIC. План – список потенциальных партнёров, что удовлетворяет условию.*

7. **References PubMed/Crossref-verified** ✓  
   *EVIDENCE.md показывает верификацию DOI/PMID; непроверенные удалены.*

8. **No fabrication markers** ✓  
   *Нет [REF_NEEDED] или [PMID_REMOVED].*

**Все 8 условий выполнены.**  
Однако множественные открытые проблемы, слабая доказательная база и неконкретный план консорциума требуют доработок – **REVISE_MINOR**, а не FUND_AS_IS.

## Top 5 text-level fixes (что добавить/изменить)

1. **`UMBRELLA/CONCEPT.md` §2 (Pre‑registration plan)**  
   *Заменить `TBD` на конкретный OSF‑идентификатор (например, `https://osf.io/abcde`) и добавить детали: точный ковариатный набор, правила остановки, метод коррекции множественности.*

2. **`UMBRELLA/CONCEPT.md` §5 (Falsifiability)**  
   *Явно сформулировать фальсификационный порог для **χ_Ze как первичной метрики** – какую partial r² или AUC ожидаете и при каком N отклоняется гипотеза (сейчас фальсификация только для MCOA).*

3. **`UMBRELLA/CONCEPT.md` §6 (Consortium plan)**  
   *Добавить отдельный раздел "Consortium partnerships" с минимум тремя подтверждёнными институтами и их ролями. Даже без подписанных LoI укажите статус переговоров.*

4. **`UMBRELLA/PARAMETERS.md` §5 (Power analysis)**  
   *Привести power analysis для **первичного исхода pre‑registration** (partial r² χ_Ze vs. all‑cause mortality): задайте ожидаемый effect size (r² или partial r²), α, мощность, полученное N.*

5. **`UMBRELLA/CONCEPT.md` §9 (Limitations)**  
   *Добавить ограничение: "Multimodal weights (0.30,0.30,0.20,0.20) получены на N=150 без внешней репликации; статус CDATA – inconclusive (Sobol p=0.12)."*

---

## PACKET
# UMBRELLA

*(Весь предоставленный пакет принят к рассмотрению. Конкретные замечания – в Top 5 fixes.)*