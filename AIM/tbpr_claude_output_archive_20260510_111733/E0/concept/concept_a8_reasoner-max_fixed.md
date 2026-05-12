# CONCEPT.md — Experiment 0 (Revised v7.0)

**Версия:** 7.0  
**Дата:** 2026-06-12  
**Статус:** Revised per TBPR Major Revision – Strengthened experimental design (3x48h runs), grounded impact claim, formalized team rota and backup protocol.

---

## 1. Purpose & Vision

### 1.1 Strategic Context

Experiment 0 (E0) is the **final commissioning phase** of an AI‑agent‑driven automated microscopy platform. The platform will be used in Experiment A (iPSC‑organoid centriole dynamics), a funded target of $80k–$200k. E0 aims to **complete the remaining validation** (three 48‑hour continuous autonomous runs, independent API‑independence verification, and release of a hardened reproducible toolchain) before moving to expensive biology.

**Why additional funding is needed now:** All preliminary work (prerequisite static demo, 10‑minute dynamic test, safety monitor validation, independent evaluation) was completed using the PI’s discretionary funds, loaned equipment, and volunteer time. The remaining **three 48‑hour stress tests and final documentation** require dedicated resources: continuous monitoring staffing with formal pager‑duty rota, contingency for unexpected failures (e.g., camera cable replacement, overnight shipping for minor parts), and professional archiving (DOI assignment, protocol registration). The requested $4,484 budget covers these final tasks and ensures the platform is ready for Experiment A with minimal risk.

**Impact of E0 success:** The $4,484 investment is expected to reduce integration risk for Experiment A by 10–20% (cost‑overrun savings of $8k–$16k). This is a conservative heuristic benchmark drawn from institutional cost‑overrun analysis of prior academic automation projects, not a formal statistical prediction. Equally important, E0 will produce a validated, open‑source blueprint for low‑cost AI‑automated microscopy, enabling other labs to replicate the setup. The specific contribution is the **validation framework itself** – the combination of a rigorous synthetic safety‑test suite grounded in published LLM tool‑calling taxonomies, an independently verified inter‑rater reliability protocol, and a deployable fallback controller.

### 1.2 What This Proposal Covers

This document requests funding for the **final validation phase** of Experiment 0. The following milestones are **already completed** using independent resources:

- Hardware integration (Zeiss IM 35, LGY40‑C stage, 450 nm CW laser, Arduino Nano, safety enclosure) – funded by PI’s prior project savings.
- Prerequisite integrated demonstration (static fluorescent beads, 50 consecutive correct moves) – completed 2026‑05‑10.
- Preliminary dynamic test on live *Elodea* (10‑minute autonomous tracking, 96.2% correct decisions) – completed 2026‑06‑02.
- Safety watchdog validated against 500 random and 50 agent‑synthetic unsafe commands (100% detection, mean 38–41 ms abort).
- Independent blinded human evaluation of agent decisions (κ = 0.84, 100 decision points).
- Fallback rule‑based controller validated (89% correct, 0% unsafe moves).
- 1‑hour static autonomous run completed 2026‑06‑05.

**Remaining work to be funded:**

- **Three 48‑hour continuous autonomous stress tests** (engineering triplicates replacing a single 7‑day run to provide failure‑mode replication and tighter statistics).
- **Final failure‑mode analysis and thermal characterization.**
- **API‑independence verification** (test with open‑source models as fallback; containerization and logging for reproducibility).
- **Comprehensive documentation** (SOPs, annotated dataset, agent versioning, Zenodo archiving, protocol publication).
- **Contingency** for component failures during extended runs.

---

## 2. What Is Validated – Current Status

All validation targets below are **already met** with preliminary data. The final three 48‑hour stress tests will confirm long‑term reliability across engineering replicates.

| Validation Target | Status | Success Criterion | Evidence |
|-------------------|--------|-------------------|----------|
| **AI‑agent layer (static)** | ✅ Complete | ≥95% correct tool calls, mean latency <5 s | 100% correct on 50 static beads; mean latency 3.2 s |
| **AI‑agent layer (dynamic)** | ✅ Complete | ≥95% correct tool calls, mean latency <5 s | 96.2% correct on 100 decisions from *Elodea* run; mean latency 4.1 s |
| **Dynamic AI decision accuracy (inter‑rater reliability)** | ✅ Complete | Cohen’s κ ≥ 0.80 | κ = 0.84 (95% CI: 0.76–0.92) from two blinded evaluators |
| **Python tool API** | ✅ Complete | All functions run without exception under load (2000 sequential calls) | Automated test suite passes |
| **Arduino firmware** | ✅ Complete | PWM stable, stepper acceleration <5% error, interlock <10 ms | Measured interrupt latency 6.2±0.8 ms |
| **Safety watchdog** | ✅ Complete | 100% detection of unsafe commands, abort <50 ms | 500 random + 50 agent‑synthetic injections: 100% detection, mean abort 38–41 ms |
| **LGY40‑C stage** | ✅ Complete | Repeatability <1 μm, drift <0.5 μm/h | 0.6 μm repeatability, <0.3 μm drift over 1 hour |
| **System reliability & reproducibility** | **Pending – core request** | Uptime >90% per run, zero critical safety incidents across all runs, consistent failure modes identified | Three 48‑hour runs provide engineering triplicates. Combined dataset provides ~3,000 decision cycles for robust κ and MTBF estimation. |
| **Safety infrastructure** | ✅ Complete | <1 lux leakage, interlocks verified, OD4 goggles present | Leak <0.5 lux, audit passed 2026‑06‑06 |

---

## 3. What Is NOT Validated (Scope Boundaries)

- **No biological discovery** – E0 is a pure commissioning and de‑risking project.
- **No centriolar biology or mammalian cell systems** – reserved for Experiment A.
- **No pulsed laser ablation** – Phase 0 uses 450 nm CW laser only for imaging and photobleaching of *Elodea* chloroplasts.
- **No statistical power for hypothesis testing** – only engineering metrics.
- **No 6‑month continuous operation** – final validation is three 48‑hour runs; long‑term reliability trends will be extrapolated and specifically addressed in Experiment A.

---

## 4. Preliminary Data – Summary of Completed Work

All preliminary data were generated using the PI’s discretionary funds, loaner hardware, and volunteer contributions. The results demonstrate that the system is mature enough for the final stress tests.

### 4.1 Prerequisite Integrated Demonstration (Completed 2026‑05‑10)

- Camera → agent → stage loop on a static slide of 5 μm fluorescent beads.
- 50 consecutive targets autonomously detected and centered.
- 100% correct tool‑call decisions, mean latency 3.2 s.
- Full video and logs archived (`PREREQUISITE_DEMO_RESULTS/`).

### 4.2 Preliminary Dynamic Test (Completed 2026‑06‑02 – Gating Milestone)

- 10‑minute autonomous run on live *Elodea canadensis* chloroplasts.
- Agent tracked three chloroplasts; longest continuous track 5 min 12 s.
- No unsafe commands; no manual intervention.
- Decision accuracy: 96.2% correct (96/100; 3 ambiguous low‑contrast frames, 1 false positive from specular reflection).
- Mean decision latency: 4.1 s.
- Full logs and video in `DYNAMIC_TEST_GATE/`.

### 4.3 Safety Watchdog & Independent Evaluation

- **Watchdog:** 100% detection on 500 random unsafe injections (mean abort 38 ms). Additional 50 synthetic agent‑generated unsafe commands (e.g., out‑of‑range coordinates, excessive laser power) detected 100% (mean abort 41 ms).
- **Rigorous Synthetic Test Suite:** The synthetic unsafe commands were generated using a structured taxonomy of LLM tool‑calling failure modes (ToolBench, Wan et al. 2023), covering tool hallucination, argument injection, state violation, and redundancy attacks. This ensures the watchdog validation is grounded in realistic failure scenarios, not arbitrary noise.
- **Blinded human evaluation (2026‑06‑04):** 100 decision points from the dynamic run evaluated by two independent trained lab members (not involved in agent development). Cohen’s κ = 0.84 (95% CI: 0.76–0.92). Agent decisions matched consensus on 94/100 points.
- **Fallback rule‑based controller:** Validated on same dynamic video (2026‑06‑03). Achieved 89% correct decisions, 0% unsafe moves, 70% centering accuracy. Deployable via single command‑line switch in 15 minutes.

### 4.4 Additional Measurements

- **Static 1‑hour autonomous run (2026‑06‑05):** 100% uptime, no incidents, stage drift <0.3 μm.
- **Precision/Recall:** Static: 0.92/0.90; Dynamic: 0.88/0.85.
- **False positive rate:** Static 0.02, dynamic 0.05.
- **Self‑recovery:** 12 s for a single missed detection during dynamic run.
- **Human‑operated loop comparison:** Human latency 2.1 s, accuracy 94%; agent latency 4.1 s, accuracy 96%.

---

## 5. Requested Funding – Remaining Work

The core request is **$4,484** to execute and document the final validation phase (three 48‑hour stress tests). Budget breakdown:

| Item | Cost (USD) | Justification |
|------|------------|---------------|
| Continuous monitoring staffing (PI + intern, shared pager duty for 3 runs) | 2,400 | Covers three 48‑hour runs plus 24‑hour setup/teardown windows. 15‑minute response SLA. Formal rota (Section 6). |
| Contingency components (cables, sensors, fasteners, overnight shipping) | 776 | Replace degraded camera cable, spare Arduino Nano, UPS battery replacement. Risk spread across 3 runs. |
| Professional archiving (DOI minting, protocol.io preprint fee, Zenodo storage) | 250 | Ensure FAIR data compliance: 2 DOIs (software + dataset), protocol registration. |
| Supplies for extended run (cleanroom wipes, immersion oil, replacement *Elodea* cultures) | 500 | Maintain sample quality over 3 independent runs. |
| Miscellaneous (cables, connectors, labeling) | 150 | – |
| **Subtotal** | **4,076** | – |
| Contingency (10%) | 408 | Reduced from v6.0 due to improved experimental design (failures in one run do not lose entire experiment). |
| **Grand total** | **4,484** | – |

*Note: The budget covers only the final validation phase. All prior milestones were self‑funded.*

---

## 6. Team & Roles

| Role | Person | Expertise | Commitment |
|------|--------|-----------|------------|
| **PI / Lead Developer** | Alexei D. | Optics, firmware, Python, AI agent; prior successful automated microscopy integrations (C. elegans laser‑ablation, PID autofocus platform). AI training: RL specialization (Coursera 2025‑Q4), 6 months collaboration with Dr. A. Wang (DeepSeek community). | 20 h/week during stress test |
| **Hardware Consultant / Backup Operator** | Dr. Elena K. (institute neighbour) | 20+ years precision microscopy, Zeiss service; signed commitment for 4 h/week remote support. On‑call pager duty backup for PI. | 4 h/week + pager |
| **Facility Backup Operator** | Prof. J. Müller | Concrete‑floor lab with optical table; written agreement for monthly booking windows + ad‑hoc emergency access. Trained on system shutdown and fallback controller. | On call |
| **Firmware Consultant** | Dr. Oleksandr H. (online collaborator) | Arduino real‑time control; contributed to watchdog validation. | 1 h/week (ad‑hoc); intern cross‑trained for basic debugging |
| **Biology Consultant** | Dr. Maria V. (retired, plant physiology) | *Elodea* culture protocols. | 1 h/week |
| **Undergraduate Intern** | Recruited (started 2026‑05‑10) | Data collection, testing, logging. Currently enrolled in university ML course. Primary pager duty (days 5–8). | 15 h/week (10 during semester, full‑time summer) |

**Key‑person risk mitigation:** All procedures are fully documented in living SOPs (currently 80% complete; final 20% will be completed during stress tests). Intern will be capable of independent troubleshooting by M2 (already passed). A formal pager‑duty rota covers 24/7/12‑day coverage:
- **Primary:** PI (Days 1–4), Intern (Days 5–8)
- **Secondary:** Dr. Elena K. (full period, remote)
- **Tertiary:** Prof. J. Müller (facility access, emergency stop trained)

If PI is unavailable, the intern or Dr. Elena K. can execute the fallback controller and emergency shutdown. Remote assistance via screen‑share. The rota is published in the SOP and pre‑tested during the 24h shakedown.

---

## 7. Risk Mitigation – Updated for Final Phase

### 7.1 AI API Dependency & Reproducibility

| Concern | Mitigation | Status |
|---------|------------|--------|
| Proprietary APIs (Claude, DeepSeek) could change or become unavailable | **Plan in place:** (1) All agent decisions are logged with full context; any API change can be re‑evaluated offline. (2) Fallback rule‑based controller is validated and deployable immediately. (3) **Containerization** of the entire agent environment (Docker image with pinned API versions) ensures exact reproducibility. (4) **Open‑source model evaluation** will be formally tested during one entire 48‑hour run to validate its reliability as a primary or parallel agent. Preliminary tests with Llama‑3‑70B show 91% correct tool calls (n=100 static decisions). | Containerization in progress (Dockerfile available in repo). Open‑source model test scheduled for Run #2 during stress test sequence. |
| Real hallucination patterns may differ from synthetic tests | Synthetic tests were designed based on known hallucination patterns from published LLM‑tool‑calling literature (ToolBench taxonomy). Additionally, the dynamic run itself generated no unsafe hallucinations. The stress tests will involve continuous operation where any novel hallucination will be caught by the watchdog (validated to 100% detection). | Low residual risk. |

### 7.2 Long‑Term Reliability

| Concern | Mitigation |
|---------|------------|
| Failures during a single run could compromise the whole validation | **Three 48‑hour runs provide engineering triplicates.** A failure in one run is an independent data point; the other runs remain valid. Downtime between runs allows for analysis and proactive maintenance. |
| 48‑hour runs may reveal thermal drift, network outages, or software bugs | **Pre‑run burn‑in:** 24‑hour shakedown test (already scheduled). **Monitoring:** automated health checks every 10 minutes; all metrics logged. **Graceful degradation:** if agent fails, fallback controller activates automatically. UPS ensures 30‑minute graceful shutdown on power loss. |
| Network or API outage during continuous run | Local caching of agent responses for up to 10 min. If API unreachable for >5 min, switch to fallback controller. After API restored, resume AI agent after human review. |

### 7.3 Vibration & Environmental Stability

Vibration mitigation already validated: concrete‑floor lab + Sorbothane pads → drift <0.3 μm/h. Backup location (Prof. Müller’s optical table) available.

### 7.4 Experimental Design Weaknesses Addressed

- **Single 7‑day run lacking replicates:** Replaced with three 48‑hour independent runs (engineering triplicates). This directly provides failure‑mode replication, tighter statistical bounds, and operational resilience. If a failure occurs in one run, the other runs are unaffected and the failure itself becomes a valuable data point.
- **Wide confidence interval for κ (0.76–0.92):** The combined dataset from three runs (~3,000 decision points) will provide a high‑precision estimate, definitively confirming κ > 0.80 with targeted 95% CI width < 0.05.
- **No biological replicates:** The three 48‑hour runs constitute formal engineering replicates, which is the appropriate standard for this commissioning phase. This structure mirrors the multi‑replicate design required for Experiment A.

---

## 8. Experimental Design – Final Validation Plan

### 8.1 Timeline (Start: funding receipt, estimated 2026‑06‑10)

| Week | Milestone | Deliverable | Status |
|------|-----------|-------------|--------|
| **Pre‑funding** | All preliminary work completed (Sections 1–4) | Evidence submitted | ✅ Complete |
| Week 1 | 24‑hour shakedown test (static + dynamic sample) | Go/No‑Go for 48h run sequence; resolve any early failures | Planned |
| Week 2 | **Run #1 (48h)** + Fallback controller test (static beads) | Full event log; preliminary failure analysis | **Core request** |
| Week 3 | **Run #2 (48h)** + API‑independence test (open‑source model) | Comparative performance report; Docker image published | Scheduled |
| Week 4 | **Run #3 (48h)** + Final documentation, archiving, and handover | SOPs, annotated dataset, agent version, protocol on protocols.io, Zenodo DOI | Planned |

### 8.2 Success Criteria for Final Phase

- **Uptime >90% per run** (excluding scheduled maintenance windows of 1 h/day for sample replacement and log backup).
- **Zero critical safety incidents across all three runs** (any event requiring emergency stop or causing physical damage).
- **All agent decisions logged and recoverable** across all runs (no data loss).
- **Failure‑mode report** categorizes all incidents by type and frequency across runs and proposes fixes for Experiment A.
- **API‑independence plan demonstrated** (Docker image runs with open‑source models achieving ≥85% correct decisions on static test set).
- **Final κ > 0.80 with 95% CI width < 0.10** on evaluation from combined stress test data.

### 8.3 Controls & Replicates

- **Engineering triplicates:** Three 48‑hour runs constitute the primary experimental units. This addresses the TBPR critique regarding lack of replication and statistical power.
- **Static slide** of fluorescent beads is imaged for 1 h at the start and end of the entire sequence, and between runs, to track long‑term instrument drift and agent drift.
- **Fallback controller** runs for 1 h within each 48‑hour run (separate sample) to compare AI vs. rule‑based performance longitudinally.

### 8.4 Pre‑Registration & Reproducibility

- Protocol pre‑registered on protocols.io (DOI assigned before stress test start).
- All code, firmware, and logs version‑controlled and archived on Zenodo.
- Docker container for full agent environment will be published.
- Agent decision logs include timestamp, input frame hash, tool call, parameter list, watchdog verdict, and human override flag.

---

## 9. Connection to Experiment A

Experiment A (iPSC‑organoid centriole dynamics) requires:

- **Reliable autonomous centering** of centrioles (diameter ~200 nm) with precision <100 nm.
- **Coordinated laser ablation** (355 nm nanosecond pulsed laser) – not yet integrated.
- **Continuous operation** over days (organoid development timescales).
- **AI‑agent decision accuracy** >95% to avoid sample waste.
- **Statistical rigor in replicates** for valid biological inference.

How E0 guarantees these:

| Experiment A Requirement | E0 Validation | Status |
|--------------------------|---------------|--------|
| Sub‑100 nm centering | Stage repeatability 0.6 μm. E0 will add closed‑loop image‑based centering correction (already prototyped). Precision target for E0 final report: <0.5 μm. | In progress; stress tests will measure centering precision on static beads. |
| Pulsed laser coordination | Not validated in E0, but the safety monitor will be extended to handle laser fire commands. E0 watchdog framework is transferable. | E0 produces the watchdog API; laser module will be added in Experiment A. |
| Long‑term reliability | E0 stress tests directly validate uptime, drift, and failure modes over three 48‑hour windows. Engineering triplicates provide failure rate data. | Core deliverable. |
| AI decision robustness | E0 validates decision accuracy on dynamic *Elodea* (96.2%). For centrioles, accuracy may drop; E0 will establish retraining pipeline. | Decision logs from E0 will be used to fine‑tune agent for centriole images. |
| Statistical rigor | E0 demonstrates engineering replicates (3 runs) and provides the analytical framework (reliability analysis, failure mode classification) directly applicable to handling biological replicates in Experiment A. | Inherent in experimental design. |

**Quantified de‑risking:** The $4,484 E0 investment reduces Experiment A’s integration risk by an estimated 10–20% (conservative benchmark from academic lab integrations). This corresponds to $8k–$16k in avoided cost overruns. Additionally, E0’s open‑source toolchain (MIT license) allows other labs to replicate the setup for under $5k, accelerating the field.

---

## 10. Source of Truth

- `CONCEPT.md` (this file) – version 7.0, master document.
- `BOM.md` – Bill of Materials.
- `Полное_Описание.md` – extended Russian reference.
- `/logs/` – daily logs from stress test sequence.
- `PREREQUISITE_DEMO_RESULTS/` – video/logs from static demo (2026‑05‑10).
- `DYNAMIC_TEST_GATE/` – video/logs from dynamic test (2026‑06‑02).
- `FALLBACK_CONTROLLER/` – rule‑based controller code and validation.
- `INDEPENDENT_EVALUATION/` – blinded evaluation data (2026‑06‑04).

---

## 11. Ethics, Safety & Governance

- Only plant cells (*Elodea canadensis*). No animal or human subjects.
- Laser safety: Class 4 enclosure with interlock; OD 4+ goggles outside enclosure. Third‑party safety audit passed 2026‑06‑06.
- All agent decisions logged; watchdog validates every command before execution (<50 ms abort).
- Any unsafe call or watchdog alert triggers immediate email and SMS alert to human operator (pager‑duty rota ensures 15‑minute response).
- **Reproducibility:** All data from the three runs will be made publicly available (Zenodo, MIT license) to facilitate external validation.
- Open‑source release under MIT license.

---

## 12. Summary

**This proposal requests $4,484 to complete the final validation phase of Experiment 0.** Responding directly to TBPR recommendations:
1. **Experimental design strengthened** – single 7‑day run replaced with three 48‑hour engineering triplicates, providing failure‑mode replication and tighter statistics.
2. **Risk‑reduction claim grounded** – impact quantified at 10–20% heuristic cost‑overrun mitigation ($8k–$16k).
3. **Team thinness addressed** – formal pager‑duty rota established with backup operators (Dr. Elena K., Prof. J. Müller) and cross‑trained intern.

All high‑risk technical components are already validated with preliminary data from self‑funded work. The remaining three 48‑hour stress tests will confirm long‑term reliability, establish API‑independence, and produce comprehensive documentation. Successfully completing Experiment 0 will enable Experiment A (iPSC‑organoid centriole dynamics), the fundable science target, with dramatically reduced integration risk. The open‑source toolchain will also benefit the wider automated microscopy community.

**Status:** Version 7.0 addresses all Major Revision recommendations from the TBPR review.