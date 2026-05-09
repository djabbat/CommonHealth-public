# Review of EpigeneticDrift

## Verdict
**REVISE_MINOR**

## Scores (1-5)
- Premise: 5 — хорошо обоснована необходимость формализации эпигенетического дрейфа как счётчика.
- Method: 4 — уравнение чётко, параметры обоснованы, но количественные оценки γ₄ⱼ остаются гипотетическими.
- Evidence: 4 — ссылки качественные, но одна вызывает сомнения в реальности (PMID 41289991).
- Falsif: 5 — числовые пороги и количественные критерии для каждого теста прописаны.
- Deliv: 4 — кодовая архитектура и планы экспериментов проработаны, но зависят от доступа к данным и коллаборациям.
- Novelty: 4 — формализация в рамках MCOA нова, но отдельные элементы известны.
- Risk: 3 — высокий риск конфундинга ABL-2, частично учтён.
- RefIntegrity: 4 — один флаг REF_VERIFY, остальные ссылки реальны и соответствуют.

## Checklist (✓/✗ + объяснение по каждому из 9 условий)

1. **✓ Операционализированная фальсифицируемость** — числовые пороги указаны: 95% CI β₄, p<0.05 для α₄, effect size d≥0.3, мощность 80% для γ₄₃, |ΔD₄|>0.15, p<0.01 для тестов OPEN_PROBLEMS.
2. **✓ Pre-registration plan** — OSF placeholder (osf.io/TBD) и дата (2026-09-01) присутствуют.
3. **✓ Sample size calculation** — power analysis для t-test (N=64/group) и регрессии (N=92) с α=0.05, power=0.80.
4. **✓ Risk matrix** — 5 строк с probability, impact, mitigation.
5. **✓ Limitations** — явный раздел с 5 пунктами.
6. **✓ Consortium / collaboration plan** — placeholder список с ролями (Primary PI, Horvath, Brunet).
7. **⚠️ Reference reality + match** — одна ссылка (Arif et al. 2025, PMID: 41289991) помечена как [REF_VERIFY]; остальные 20 PMID реальны и соответствуют тексту.
8. **✓ No fabrication markers** — placeholders допустимы (OSF, consortium), нет [REF_NEEDED] или [PMID_REMOVED].
9. **✓ Internal consistency core docs** — CONCEPT, THEORY, EVIDENCE, PARAMETERS, OPEN_PROBLEMS согласованы, отсылки к CORRECTIONS учтены.

## Reference audit

| # | Цитата (короткая) | DOI/PMID | Реальна? | Соответствует тексту? | Решение |
|---|-------------------|----------|----------|-----------------------|---------|
| 1 | Horvath 2013, DNA methylation age | PMID: 24138928 | ✅ Да | ✅ Да | OK |
| 2 | Lu et al. 2019, GrimAge | PMID: 30669119 | ✅ Да | ✅ Да | OK |
| 3 | Belsky et al. 2022, DunedinPACE | PMID: 35029144 | ✅ Да | ✅ Да | OK |
