# Review of CDATA

## Verdict
REVISE_MAJOR

## Scores (1-5)
- Premise: 4
- Method: 3
- Evidence: 3
- Falsif: 4
- Deliv: 3
- Novelty: 5
- Risk: 3

## Checklist (✓/✗ each + explanation)

| # | Criterion | Status | Explanation |
|---|-----------|--------|-------------|
| 1 | Operationalised falsifiability (numeric thresholds) | ✅ | OPEN_PROBLEMS.md содержит тесты с конкретными порогами: r_spearman > 0.6, p < 0.01; N = 25, 20, 15 с мощностью 0.80. |
| 2 | Pre-registration plan (OSF placeholder + date) | ✗ | Отсутствует. Есть лишь общее упоминание "pre-register analysis plan" в risk matrix, но нет конкретного OSF ID и даты. |
| 3 | Sample size calc (power analysis) | ✅ | Явный раздел "Sample Size Calculation" в OPEN_PROBLEMS.md с α=0.05, power=0.80, effect size r≥0.6 → N≥19. |
| 4 | Risk matrix ≥5 rows | ✅ | 5 строк (Low reproducibility, Negative result, Key personnel, Budget overrun, Data management failure) с probability, impact, mitigation. |
| 5 | Limitations section | ✅ | EVIDENCE.md §3 "Опровергающие свидетельства и ограничения" – 6 пунктов; также Bradford Hill с невыполненными критериями. |
| 6 | Consortium / collaboration plan | ✅ | OPEN_PROBLEMS.md "Consortium / partners" – список из 4 потенциальных партнёров (Yale, Stanford, MPI, Cambridge). Placeholder допустим. |
| 7 | References PubMed/Crossref-verified or marked as pre-print | ✗ | Множественные [PMID_REMOVED] и [pre-print placeholder: DOI TBD] в CONCEPT.md и EVIDENCE.md после аудита фабрикаций. Не все ссылки верифицированы. |
| 8 | No fabrication markers ([REF_NEEDED]/[PMID_REMOVED]) | ✗ | Присутствуют: [PMID_REMOVED 2026-05-08], [pre-print placeholder: DOI TBD], комментарий "FABRICATION CLEANUP applied". |

**Итог:** Не выполнены пункты 2, 7, 8 → минимум один провал → REVISE_MAJOR.

## Top 5 text-level fixes

1. **file:OPEN_PROBLEMS.md** — Add explicit pre-registration plan: OSF Identifier placeholder (e.g., `osf.io/abcde`) and planned date (e.g., `2026-09-01`) for each prospective test (FT1.1, FT2.1, FT6.1, FT6.2).

2. **file:EVIDENCE.md** — Remove all remaining [PMID_REMOVED] and [pre-print placeholder: DOI TBD] entries. Either supply verified PMIDs/DOIs or delete the reference entirely. Ensure every literature statement has a working identifier.

3. **file:CONCEPT.md** — Purge the HTML comment `<!-- REF_AUDIT_2026-05-08: FABRICATION CLEANUP applied -->` and all [PMID_REMOVED]/[pre-print placeholder] markers. Replace with real citations or remove.

4. **file:CONCEPT.md** — The "FABRICATION CLEANUP" backstory must not appear in any grant submission. Rewrite the `CDATA/CONCEPT.md` header to remove audit trail commentary; keep only scholarly content.

5. **file:PARAMETERS.md** — Update tissue-specific `nu` values (isc_nu=70, muscle_nu=4, neural_nu=2) from literature priors to post-MCMC posteriors, with annotation "Round-7 MCMC posterior" as done for α_HSC. (See STATE.md L1 residual.)

---

## PACKET

The submission demonstrates a logically constrained, innovative theory with falsifiable quantitative predictions and a clear computational model. However, three critical procedural issues prevent acceptance:

- **Missing pre-registration plan** (required by ERC/NIH/Wellcome standards).
- **Unresolved fabrication markers** in the reference list – despite a cleanup audit, remnants of fabricated or unverified citations remain, undermining trust in the evidence base.
- **Absence of full reference verification** – several citations are flagged as removed or placeholder.

### Required actions before resubmission (REVISE_MAJOR):

1. Provide a complete pre-registration table with OSF IDs and dates for all planned experimental tests (FT1.1–FT6.2).
2. Remove all traces of fabrication audit from the document body (comments, [PMID_REMOVED], [pre-print placeholder]). Replace with verified PubMed/Crossref identifiers or delete unsupported claims.
3. Ensure every literature reference in EVIDENCE.md and CONCEPT.md has a resolved PMID/DOI and matches the claim it supports (no "concept only, no α published").
4. (Optional but recommended) Update PARAMETERS.md tissue `nu` values to code-pinned posteriors, and run a complete LOO-CV after fixing the ROS equation to improve mean LOO-CV from -0.093.

The scientific core is promising, but the packaging must meet the highest standards of reproducibility and integrity. Resubmit after addressing these points.