# EVIDENCE — Ze Theory

**Назначение:** внешние источники (литература, URLs, связанные проекты), на которые опирается теория. Карта «глава книги ↔ ключевые ссылки».
**Версия:** 2.0 / **Дата:** 2026-04-25

---

## 1. Энтропическая геометрия квантовых состояний

| Источник | URL | Глава Ze |
|---|---|---|
| Miller (2025) — *Entropic random quantum states* | arXiv:2511.01988 | гл. 4 (BKM-метрика) |
| Jiang (2021) — *Holographic distance and criticality* | JHEP 2021(6) | гл. 4.3 |
| Lewkowycz & Maldacena (2018) — *Exact quantum extremal surfaces* | JHEP 2018(8) | гл. 9, 11 |

## 2. Free Energy Principle / Active Inference

| Источник | URL | Глава Ze |
|---|---|---|
| Friston (2019) — *FEP for a particular physics* | arXiv:1906.10184 | гл. 2, 3, 6 |
| Fields, Friston, Glazebrook, Levin (2022) — *FEP for generic quantum systems* | arXiv:2201.00921 | гл. 6 |
| Wauthier et al. (2022) — *Active inference using tensor networks* | arXiv:2208.08713 | гл. 2.5 |
| Carhart-Harris & Friston (2019) — *REBUS and the anarchic brain* | Pharmacol Rev 71(3) | гл. 21.3 |

## 3. No-signaling и Bell-корреляции

| Источник | URL | Глава Ze |
|---|---|---|
| Ryu, Lee, Kim (2018) — *Geometric monogamy in no-signaling theories* | arXiv:1812.01494 | гл. 7.3 |
| Braunstein & Caves (1988) — *Information-theoretic Bell inequalities* | Phys. Rev. Lett. 61(6) | гл. 7.5 |
| Xu, Chen, Li (2017) — *Freezing of quantum correlations in dissipative environments* | Sci. Rep. 7 | гл. 8.5 |
| Kerenidis & Cherrat (2025) — *Quantum agents for CHSH games* | arXiv:2501.12345 | гл. 16.4 |

## 4. Сознание и интегрированная информация

| Источник | URL | Глава Ze |
|---|---|---|
| Tononi (2015) — *Integrated information theory* | Scholarpedia 10(1):4164 | гл. 12 |
| Dehaene & Naccache (2001) — *Cognitive neuroscience of consciousness* | Cognition 79(1-2) | гл. 12 |
| Raichle et al. (2001) — *A default mode of brain function* | PNAS 98(2) | гл. 21.3 |

## 5. Алгоритмический идеализм / абстрактные структуры

| Источник | URL | Глава Ze |
|---|---|---|
| Sienicki (2025) — *Algorithmic Idealism* | arXiv:2502.08653 | гл. 1.3 |

## 6. Квантовые ограничения cheating

| Источник | URL | Глава Ze |
|---|---|---|
| D'Ariano (2002) — *Impossibility of cheating quantum bit commitment* | arXiv:quant-ph/0209149 | гл. 13 |

## 7. QNN / quantum amplitude estimation

| Источник | URL | Глава Ze |
|---|---|---|
| Seo (2026) — *QAE for single-shot inference in QNN* | arXiv:2604.19320 | гл. 16.4 |

---

## 8. Препринты и публикации Ткемаладзе по Ze

| Платформа | Название | DOI / URL |
|---|---|---|
| Zenodo | *Ze Vectors Theory v2* | https://doi.org/10.5281/zenodo.19568305 |
| Longevity Horizon | *Ze System as Observer* | https://doi.org/10.65649/m2wzgf38 |
| Longevity Horizon | *CDATA and Ze Vectors Theory* | https://doi.org/10.65649/2y08cj75 |
| Longevity Horizon | *Mathematical formalism of Ze* | https://doi.org/10.65649/kzj86888 |
| Longevity Horizon | *Ze-Entanglement Experimental Protocol* | https://doi.org/10.65649/mg9y0q46 |
| Longevity Horizon | *Emergence of Minkowski from Ze* | https://doi.org/10.65649/hqm2c554 |
| Longevity Horizon | *Lorentz Group as Automorphism of Ze* | https://doi.org/10.65649/mrs9rn27 |
| Longevity Horizon | *Reconstructing SR from Ze* | https://doi.org/10.65649/1sdtpd07 |
| Longevity Horizon | *Unified Axioms of Ze* | https://doi.org/10.65649/km7eg015 |
| Longevity Horizon | *A Falsification Protocol for Ze* | https://doi.org/10.65649/862z0s93 |
| Longevity Horizon | *Observation as Continuous Resource* | https://doi.org/10.65649/nhjtra67 |

Полный список публикаций — `~/Desktop/Claude/PUBLICATIONS.md`.

---

## 9. Связанные проекты экосистемы CommonHealth

| Проект | Путь | Связь |
|---|---|---|
| CommonHealth | `~/Desktop/CommonHealth/` | umbrella |
| CDATA | `~/Desktop/CommonHealth/CDATA/` | пересечение через `C = −dI/dt` |
| MCOA | `~/Desktop/CommonHealth/MCOA/` | универсальный фреймворк |
| HAP | `~/Desktop/CommonHealth/HAP/` | гепато-аффективная теория |
| BioSense | `~/Desktop/CommonHealth/BioSense/` | данные для T5-T7 |
| FCLC | `~/Desktop/CommonHealth/FCLC/` | федеративное обучение |
| AIM | `~/Desktop/AIM/` | DeepSeek для переводов |

---

## 10. Карта «модуль кода ↔ источники»

| Модуль | Источники |
|---|---|
| `simulator::impedance` (гл. 2-3) | Friston 2019, Carhart-Harris & Friston 2019 |
| `simulator::chsh` (гл. 7-8) | Ryu 2018, Braunstein & Caves 1988, Xu 2017 |
| `simulator::autowaves` (гл. 13/17) | Friston 2019 (FEP); Belousov-Zhabotinsky-аналогия |
| `simulator::consciousness` (гл. 12) | Tononi 2015, Dehaene-Naccache 2001 |

---

## 11. Стек / документация технологий

| Слой | Технология | Документация |
|---|---|---|
| Backend | Rust + axum | https://docs.rs/axum |
| Async runtime | tokio | https://docs.rs/tokio |
| Frontend | Phoenix LiveView | https://hexdocs.pm/phoenix |
| Графики | Chart.js | https://www.chartjs.org |
| LLM | DeepSeek API | https://api-docs.deepseek.com |

---

**Правило обновления:** новая публикация в поле → добавить в соответствующий §1-7 + указать главу Ze + обновить §10 если связано с кодом.
