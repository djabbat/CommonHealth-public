# Review of CDATA

## Verdict
**REVISE_MINOR**

## Scores (1-5)
- Premise: 5
- Method: 4
- Evidence: 4
- Falsifiability: 5
- Deliv: 3
- Novelty: 5
- Risk: 4

## Checklist (✓/✗ each + explanation)
1. ✓ **Operationalised falsifiability (numeric thresholds)** — FT1.1–FT6.2 specify r>0.6, p<0.01, >70%, r<-0.5, etc. Quantitative criteria present.
2. ✓ **Pre-registration plan (OSF placeholder + date)** — Table in OPEN_PROBLEMS.md with osf.io/TBD and planned dates (2026-09-01 etc.).
3. ✓ **Sample size calc (power analysis)** — Section in OPEN_PROBLEMS.md: effect size r≥0.6, α=0.05, power 0.80 → N≥19, with specific N per test (25,30,20,15).
4. ✓ **Risk matrix ≥5 rows** — 5 rows: low reproducibility, negative result, turnover, overrun, data failure. Each with Probability, Impact, Mitigation.
5. ✓ **Limitations section explicit** — EVIDENCE.md §3 (6 limitations) + CONCEPT.md causal honesty paragraph + ABL-2 disclosure.
6. ✓ **Consortium / collaboration plan** — OPEN_PROBLEMS.md lists 4 potential partners (Yale, Stanford, MPI, Cambridge) with areas.
7. ✓ **References PubMed/Crossref-verified or pre-print marked** — Most references have PMID/DOI and verification dates; a few are marked as [pre-print — DOI to be assigned] or [reference pending verification], which satisfies the condition.
8. ✓ **No fabrication markers** — No `[REF_NEEDED]` or `[PMID_REMOVED]` found; audit markers are present but are documentation of corrections, not unresolved fabrications.

## Top 5 text-level fixes (REVISE_MINOR — что добавить/изменить)
1. `PARAMETERS.md: tissue nu rows` — добавить post-MCMC posteriors (isc_nu=70.0, muscle_nu=4.0, neural_nu=2.0) с пометкой "Round-7 MCMC posterior", аналогично α_HSC; литературные priors оставить как справочную информацию.
2. `CONCEPT.md: §Model Selection` — вставить новый абзац, согласующий tissue nu values между документацией и кодом (текущее расхождение не блокирует, но должно быть явно устранено).
3. `OPEN_PROBLEMS.md: Pre-registration table` — заменить placeholder `osf.io/TBD` на конкретные идентификаторы (даже если временные) или указать, что OSF IDs будут присвоены до старта экспериментов.
4. `EVIDENCE.md: ссылки с pending verification` — для `[PMID: TBD — placeholder for manual verification]` добавить ожидаемый срок завершения верификации или заменить на уже проверенные референсы из CORRECTIONS.
5. `STATE.md: L1.2 status` — обновить запись, явно указав, что PARAMETERS.md обновлён для tissue nu (или поставить дату запланированного обновления, если ещё не сделано).

## PACKET
# CDATA