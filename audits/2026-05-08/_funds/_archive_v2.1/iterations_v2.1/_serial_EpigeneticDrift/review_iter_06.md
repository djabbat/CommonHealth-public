# Review of EpigeneticDrift

## Вердикт
**REVISE_MAJOR**

## Scores (1-5)
- Premise: 5 — убедительная концептуализация Epigenetic Drift как количественного счётчика в MCOA.
- Method: 4 — формальное уравнение, параметры и кинетика обоснованы; не хватает плана пре-регистрации.
- Evidence: 5 — качественный мета-анализ, все ссылки проверены, внутренние данные и честное освещение контрпримеров.
- Falsifiability: 5 — чёткие численные пороги, тесты с количественными критериями, матрица рисков.
- Deliverables: 3 — детальный код и архитектура есть, но нет явного плана пре-регистрации и sample size calculation не встроен в pre-registration.
- Novelty: 4 — формализация дрейфа как автономного счётчика интересна, но парадокс ABL-2 снижает новизну (требуется проверка).
- Risk: 4 — риски идентифицированы, смягчение предложено, но зависимость от ABL-2 остаётся высокой.

## Checklist (✓/✗ each + explanation)

1. ✗ **Pre-registration plan** — Отсутствует. Нет placeholder OSF идентификатора и запланированной даты пре-регистрации. В проекте упоминаются «MCOA Tests», но не как pre-registration. **Это критическое нарушение.**  
   *Требование:* указать OSF (или аналог) и дату регистрации.

2. ✓ **Operationalised falsifiability** — Выполнено. Numeric thresholds для p, effect size, power приведены в CONCEPT.md (p<0.05, d≥0.3, power 80% и т.д.) и в OPEN_PROBLEMS.md.

3. ✓ **Sample size calculation** — Выполнено. В PARAMETERS.md есть расчёт N=64 для t-test и N=92 для регрессии с указанием α=0.05, power=0.80, effect size, и инструмента G*Power.

4. ✓ **Risk matrix ≥5 rows** — Выполнено. В OPEN_PROBLEMS.md приведена таблица из 5 рисков с probability, impact и mitigation.

5. ✓ **Limitations section** — Выполнено. В EVIDENCE.md раздел «Опровергающие свидетельства» и в CONCEPT.md раздел «Open Questions» служат явным ограничением. Рекомендую переименовать или сделать отдельный раздел.

6. ✓ **Consortium / collaboration plan** — Выполнено. В CONCEPT.md перечислены три группы (Primary PI, Horvath, Brunet) с указанием их ролей.

7. ✓ **References PubMed/Crossref-verified** — Выполнено. В EVIDENCE.md все ссылки помечены ✅ с датой проверки. Нет непроверенных препринтов.

8. ✓ **No fabrication markers** — Выполнено. Нет [REF_NEEDED] или [PMID_REMOVED].

## Top 5 text-level fixes (добавить/изменить)

1. **CONCEPT.md: Добавить pre-registration plan**  
   После раздела «Consortium / partners» вставить блок:
   ```
   Pre-registration: The primary experimental test (ABL-2 modulation, OPEN_PROBLEMS §1) will be pre-registered on OSF (placeholder: https://osf.io/xxxxx) prior to data collection. Planned registration date: 2026-09-01. All sample size calculations will be included in the pre-registration.
   ```

2. **PARAMETERS.md: Увязать sample size calculation c pre-registration**  
   Перенести существующий расчёт в pre-registration plan и указать, что он будет зафиксирован до сбора данных.

3. **CONCEPT.md / EVIDENCE.md: Выделить явный раздел «Limitations»**  
   Создать отдельный раздел «Limitations» в CONCEPT.md (или в EVIDENCE.md) и перенести туда все ограничения (каузация, обратимость, несоответствие между слоями, парадокс ABL-2). Это повысит формальную строгость.

4. **OPEN_PROBLEMS.md: Указать, что все тесты будут пре-зарегистрированы**  
   Перед описанием тестов добавить: «All proposed tests (A–D) will be pre-registered with detailed statistical analysis plans (OSF ID pending). Sample size calculations are provided in PARAMETERS.md.»

5. **README.md: Добавить ссылку на pre-registration**  
   В разделе «Key features» или «Project goals» указать: «All experimental predictions are pre-registered on OSF (link TBD after revision).»

---

# PACKET

```
# EpigeneticDrift

=== EpigeneticDrift/CONCEPT.md ===
# Epigenetic Drift as a Quantifiable Counter in the Multi-Counter Architecture of Organismal Aging (MCOA): Counter #4

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


**Authors:** [Author List]



## Consortium / partners

This project will be executed by a consortium of three groups:
- [Primary PI], [Institution], responsible for theoretical model development and simulation.
- [Potential collaborator 1, e.g., Prof. S. Horvath, UCLA] – for access to DNA methylation clock validation in human cohorts.
- [Potential collaborator 2, e.g., Dr. A. Brunet, Stanford] – for stem cell aging experiments and CRISPR-barcoding lineage tracing.

Letters of intent will be obtained prior to funding. The consortium will meet bi-monthly to coordinate the experimental tests outlined in OPEN_PROBLEMS.

**Correspondence:** [Corresponding Author Email]
**Date:** October 26, 2023

## Abstract

The erosion of epigenetic information is a hallmark of aging, observable as predictable drift in DNA methylation patterns and chromatin states. Within the Multi-Counter Architecture of Organismal Aging (MCOA), this process is formalized as a discrete, quantifiable "counter" (Counter #4: Epigenetic Drift). This conceptual paper provides the formal kinetic and biological definition of this counter. We propose a time-dominant equation, *D₄(n, t) = D₄,₀ + β₄·(t / τ₄) + α₄·(n / n₄\*) + γ₄ · I(others)*, where *D₄* represents the epigenetic drift state, parameterized by a baseline (*D₄,₀*), a time-driven linear coefficient (*β₄*), a replication-associated coefficient (*α₄*), and a coupling term to other aging processes (*γ₄*). Each parameter is grounded in empirical evidence from peer-reviewed meta-analyses of epigenetic clocks (e.g., Horvath, GrimAge, DunedinPACE) and stem cell aging. We detail its primary measurement via DNA methylation arrays and chromatin accessibility assays, its proposed bidirectional couplings with other MCOA counters (Centriolar, Telomere, MitoROS, Proteostasis), and explicit, quantitative falsifiability conditions. Finally, we position Epigenetic Drift within the integrative MCOA framework, where tissue-specific aging is modeled as a weighted sum of counter states, and outline critical open questions regarding causality, mechanistic drivers, and the universality of epigenetic aging signals.

## 1. Introduction: Epigenetic Information Loss as a Core Aging Process

Aging is characterized by a progressive loss of physiological integrity, driven by the accumulation of diverse forms of molecular damage and deregulation. Among the twelve proposed hallmarks of aging, "epigenetic alterations" stand out due to their upstream position in regulating gene expression programs and their established quantifiability (Horvath and Raj 2018, PMID: 29643443). Epigenetic drift refers to the cumulative, often stochastic, changes in epigenetic marks—including DNA methylation, histone modifications, and chromatin accessibility—that deviate from the youthful, tissue-specific epigenetic landscape. This drift is not random noise; it forms highly predictable patterns that can be used to estimate chronological and biological age with remarkable accuracy using epigenetic "clocks" (Horvath 2013, PMID: 24138928; Belsky et al. 2022, PMID: 35029144; Duan et al. 2022, PMID: 36206857).

The Multi-Counter Architecture of Organismal Aging (MCOA) is a meta-theoretical framework that models organismal aging as the integrated output of several discrete, semi-autonomous, and quantifiable degenerative processes termed "counters." Each counter tracks the accumulation of a specific type of molecular or cellular dysfunction (e.g., telomere shortening, mitochondrial ROS burden). The integrative tissue age *L_tissue(n,t)* is calculated as a weighted sum of the normalized states of all counters.

This document provides the formal conceptual definition, kinetic model, and validation criteria for **MCOA Counter #4: Epigenetic Drift**. We move beyond describing epigenetic drift as merely a biomarker and instead formalize it as a dynamic, quantifiable aging process with its own kinetics, drivers, and interactions. We ground every aspect of this model in the current evidence base, citing only from two pre-conducted meta-analyses encompassing 24 peer-reviewed publications.

## 2. Counter Identity and Biological Foundations

**2.1. Definition of the Counter**
Counter #4, Epigenetic Drift, quantifies the progressive, age-associated deviation from a youthful epigenetic state. Its readout (*D₄*) is a composite metric of epigenetic integrity, where an increase signifies greater drift and biological age. The primary molecular layers captured include:
* **DNA Methylation:** The most established layer, characterized by hypermethylation at specific CpG islands (often polycomb group target genes) and hypomethylation at others, forming the basis of most epigenetic clocks (Horvath 2013, PMID: 24138928; Lu et al. 2019, PMID: 30669119).
* **Chromatin Accessibility and Architecture:** Age-related changes in the opening and closing of chromatin regions, which can be quantified independently of methylation (e.g., ATAC-clock) and may offer more direct functional insights (Morandini et al. 2024, PMID: 37924441).
* **Histone Modification Landscapes:** Drift in the genomic distribution of activating (e.g., H3K4me3, H3K27ac) and repressive (e.g., H3K9me3, H3K27me3) histone marks, which is particularly pronounced in aging stem cells (Adelman et al. 2019, PMID: 31085557; Deng et al. 2021, PMID: 33571444).

**2.2. Biological Mechanisms Driving the Counter**
Epigenetic drift arises from the interplay of stochastic errors, directional biochemical pressures, and environmental exposures:
* **Replication-Dependent Errors:** With each cell division, the epigenetic landscape must be faithfully copied. Imperfect maintenance by DNA methyltransferases (DNMTs) and histone-modifying complexes leads to the accumulation of small, stochastic errors over time, contributing to the divisional component of drift (α₄).
* **Enzyme Imbalance and Deregulation:** Age-related changes in the expression and activity of epigenetic writers (e.g., DNMTs), erasers (e.g., TETs, KDMs), and readers disrupt the dynamic equilibrium of epigenetic marks. For example, loss of KDM4B in mesenchymal stem cells drives senescence and bone-fat imbalance (Deng et al. 2021, PMID: 33571444).
* **Environmental and Metabolic Insults:** Chronic inflammation is a potent driver of long-term epigenetic reprogramming in hematopoietic stem cells (Bogeska et al. 2022, PMID: 35858618; Kasbekar et al. 2023, PMID: 37865087). Metabolic dysregulation, including iron homeostasis, can also alter the epigenetic state of stem cells (Kao et al. 2024, PMID: 38402617).
* **Stem Cell Exhaustion and Lineage Infidelity:** In stem cell compartments, age-associated epigenetic drift is directly linked to functional decline. Profound enhancer reprogramming alters lineage priming, favoring myeloid over lymphoid output in aged H