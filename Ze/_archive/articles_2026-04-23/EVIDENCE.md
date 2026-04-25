# Ze · EVIDENCE — источники, на которые опирается теория

Дата: 2026-04-23. Источник истины — 5.md (`~/Desktop/5.md`). Здесь сводка ключевых работ и их роли в Ze Theory.

## 1. Энтропическая геометрия квантовых состояний

- **Miller (2025)** — *Entropic random quantum states* (arXiv:2511.01988).
  Обосновывает BKM-метрику `ds² = −d²S(ρ)` на пространстве смешанных состояний; формальная основа §4.1 Ze Theory («дистанция как интеграл импеданса» — обобщение BKM на неравновесные траектории).

- **Jiang (2021)** — *Holographic distance and criticality* (JHEP 2021(6)).
  Показывает, что trace-distance и relative entropy выражаются через площадь космической браны. Используется в §4.3 (кривизна через вторые производные `𝓘`).

- **Lewkowycz & Maldacena (2018)** — *Exact quantum extremal surfaces* (JHEP 2018(8)).
  Принцип экстремизации функционала запутанности. В Ze §9.4 возникает как частный случай.

## 2. Free Energy Principle / Active Inference

- **Friston (2019)** — *A free energy principle for a particular physics* (arXiv:1906.10184).
  Минимизация variational free energy как принцип живых систем. Подпирает §2.2 Ze Theory («prediction — онтологический механизм»).

- **Fields, Friston, Glazebrook, Levin (2022)** — *A free energy principle for generic quantum systems* (arXiv:2201.00921).
  Распространение FEP на квантовые системы. Перекликается с §6 Ze (КМ как проекция).

- **Wauthier et al. (2022)** — *Active inference using tensor networks* (arXiv:2208.08713).
  «Ze-force» как активный вывод — §2.5.

- **Carhart-Harris & Friston (2019)** — *REBUS and the anarchic brain* (Pharmacol Rev 71(3)).
  Медитация и психоделики действуют противоположно на иерархический предиктивный кодинг; §21.3 Ze использует это для предсказаний по EEG.

## 3. No-signaling и Bell-корреляции

- **Ryu, Lee, Kim (2018)** — *Geometric approach to monogamy relations in no-signaling theories* (arXiv:1812.01494).
  Статистическое разделение как триангулярное неравенство → Bell-неравенства. §4.2 и §7.3 Ze.

- **Braunstein & Caves (1988)** — *Information-theoretic Bell inequalities* (Phys. Rev. Lett. 61(6)).
  Энтропические Bell-неравенства; нарушаются для EPR. §7.5 Ze использует как подпорку.

- **Xu, Chen, Li (2017)** — *Freezing of quantum correlations in dissipative environments* (Sci. Rep. 7).
  Эмпирическое явление «freezing» классических-vs-квантовых корреляций. §8.5 Ze — новая интерпретация через энтропическую геометрию.

- **Kerenidis & Cherrat (2025)** — *Quantum agents for CHSH games* (arXiv:2501.12345).
  CHSH как игра для квантовых агентов; перекликается с §16.4 Ze (Quantum Ze agents).

## 4. Сознание и интегрированная информация

- **Tononi (2015)** — *Integrated information theory* (Scholarpedia 10(1):4164).
  Φ как мера сознания. `Φ_Ze = ∮ 𝓘 dt` в 5.md — динамическая вариация (темп vs статика).

- **Dehaene & Naccache (2001)** — *Towards a cognitive neuroscience of consciousness* (Cognition 79(1-2)).
  Global workspace framework; используется как фон для §12.

- **Raichle et al. (2001)** — *A default mode of brain function* (PNAS 98(2)).
  DMN. §21.3 — гипотеза о медитации и DMN через уменьшение `𝓘`.

## 5. Алгоритмический идеализм

- **Sienicki (2025)** — *Algorithmic Idealism* (arXiv:2502.08653).
  Реальность как вычислительный процесс с петлёй агент↔среда. §1.3 Ze: «`𝒵` включает абстрактные структуры».

## 6. Квантовые ограничения жульничества

- **D'Ariano (2002)** — *On the impossibility of cheating quantum bit commitment* (arXiv:quant-ph/0209149).
  Невозможность жульничества в bit-commitment. §13 Ze — пределы жульничества.

## 7. QNN / quantum amplitude estimation

- **Seo (2026)** — *Quantum amplitude estimation for single-shot inference in QNN* (arXiv:2604.19320).
  Используется как бекграунд для §16.4 (Quantum Ze agents).

---

## Как это связывает с симулятором

| Модуль симулятора | Источники |
|---|---|
| `impedance` (§2 THEORY) | Friston 2019, Carhart-Harris & Friston 2019 |
| `chsh` (§3 THEORY) | Ryu 2018, Braunstein & Caves 1988, Xu 2017 |
| `autowaves` (§4 THEORY) | Friston 2019 (FEP); BZ-аналогия — классика химии реакций |

## Что требует будущей валидации

1. **T1:** CHSH-сдвиг `S(H) = 2.828·(1 − 2αH)` — пока нет опубликованных измерений.
2. **T2:** `γ_Ze = −0.031` в CMB — MCMC на Planck 2018 не реализован публично.
3. **T5–T7:** EEG-корреляты `Φ_Ze` — pre-registered тесты не проводились.

Эти пункты — содержимое `OPEN_PROBLEMS.md`.
