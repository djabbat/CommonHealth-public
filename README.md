# LongevityCommon

**An integrative ecosystem for biomarker-guided interventions in aging as Total Chronic Disease.**

LongevityCommon — это hypothesis-stage framework, объединяющий 5 научных подпроектов + тонкий социальный слой:

| Подпроект | Уровень | Что делает |
|-----------|---------|------------|
| **MCOA** | Theoretical (meta-теория) | Multi-Counter Architecture; aging как взвешенная сумма параллельных счётчиков |
| **CDATA** | Molecular-cellular | Hypothesis: centriolar damage в HSC (status: inconclusive) |
| **Ze** | Mathematical | Entropy-geometric ansatz `dτ/dt = −α·I(Z)` |
| **BioSense** | Applied | Wearable platform + χ_Ze биомаркер |
| **FCLC** | Infrastructure | Federated learning + DP + k-anonymity (semi-honest only) |

Plus **Activated** clinical pilot (anemia management cohort, Tbilisi).

Каноническая статья: `~/Desktop/LongevityCommon.md` (v5.6).

## Status (2026-04-28, v5.6)

⚠ **Hypothesis-stage research platform.** Все эмпирические результаты — exploratory (hypothesis-generating), не confirmatory. Pre-registered тесты ранней univariate χ_Ze formulation на Cuban/Dortmund/LEMON cohorts → NULL results (deprecated/superseded). Текущая мультимодальная χ_Ze — post-hoc reformulation. AUC и r² values — exploratory с явным p-hacking risk (Ioannidis 2005, PMID 16060722).

Ключевые публикации (MCOA, Ze, BioSense) — НЕ peer-reviewed на момент v5.6.

## Authority order on conflict

1. `LongevityCommon/CONCEPT.md` (cross-cutting status, falsifiability, threat model)
2. `<subproject>/CONCEPT.md` (internal math)
3. `<subproject>/THEORY.md` (formal derivations)
4. Article (`~/Desktop/LongevityCommon.md`) — full narrative
5. Code — следует за CONCEPT соответствующего уровня

## Repository structure

```
LongevityCommon/
├── *.md # umbrella core (CONCEPT, THEORY, DESIGN, PARAMETERS, MAP, ...)
├── server/ # Rust/axum REST API (social layer)
├── web/ # React+TS PWA (social layer UI)
├── realtime/ # Phoenix Channels (social layer WS)
├── deploy/ # docker-compose-all.yml
├── docs/EIC_PartB_2026/ # active grant track
├── _archive/ # старые версии
├── _audits/ # audit reports
│
└── <subprojects>/ # MCOA, CDATA, Ze, BioSense, ...
```

## Run (subproject backends)

```bash
cd Ze && ./run.sh # :4000 / :4001
cd BioSense && ./run.sh # :4100 / :4101
```

Social layer (server/web/realtime) — отдельный stack, см. `DESIGN.md §7`.

## Tests

```bash
cargo test --release # in any subproject root
mix test # in any Phoenix subproject
```

## Grant track (active)

**EIC Pathfinder Challenges 2026 — "Biotechnology for Healthy Ageing", deadline 2026-10-28.**
LongevityCommon umbrella как заявка по Area #2 (biomarker-based tool, BioSense — центр).
Подробности: `docs/EIC_PartB_2026/`.

## License

MIT (see LICENSE).

## Contact

Jaba Tkemaladze · jaba@longevity.ge · ORCID 0000-0001-8651-7243
Georgia Longevity Alliance (NGO #404506520, Associated Country Horizon Europe)
