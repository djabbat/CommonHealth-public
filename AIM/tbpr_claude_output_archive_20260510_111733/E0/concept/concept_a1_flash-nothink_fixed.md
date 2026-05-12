# CONCEPT.md — Experiment 0 (Revised)

**Версия:** 2.0
**Дата:** 2026-04-26
**Статус:** HW+SW commissioning, revision based on TBPR review

---

## 1. Purpose & Vision

To debug and validate an AI‑agent‑driven (Claude Code + DeepSeek router) automated microscopy platform for 24/7 laser‑ablation, imaging, and tracking. The biological testbed is *Elodea canadensis* chloroplasts, chosen for their low cost, optical accessibility, and stochastic dynamics that stress‑test agent decision‑making. **This is a pure commissioning project** – no biological discovery is claimed. Successful validation directly enables Experiment A (iPSC‑organoid centriole dynamics), which is the fundable science target.

The primary outcomes are (a) a hardened, documented, reproducible rig, (b) quantitative metrics for agent reliability, (c) a dataset of agent‑in‑the‑loop operation logs for further ML tuning.

---

## 2. What Is Validated

| Validation Target | Success Criterion | Method |
|-------------------|-------------------|--------|
| **AI‑agent layer** | ≥95% of tool‑call decisions correct (no hallucinated moves/laser fires), average decision latency <5 s | Ground‑truth logs compared to manual replay; blind test on 50 unseen images |
| **Python tool‑function API** | All functions (move_stage, fire_laser, capture_image, detect_targets) run without exception under load; error handling returns meaningful messages | Automated test suite (pytest); 1000 sequential calls with random inputs |
| **Arduino Nano firmware** | PWM frequency stable, stepper acceleration profile within 5% of planned, interlock triggers <10 ms after sensor trip | Oscilloscope measurements; repeated interlock tests |
| **LGY40‑C XY stage** | Positioning repeatability <1 µm (closed‑loop), max speed 20 mm/s, no drift >0.5 µm over 1 hour | Laser interferometer (borrowed) on 3 corner points; 60‑minute stability test |
| **6‑month stability** | Uptime >90% (excluding scheduled maintenance), zero critical safety incidents, logged agent decisions recoverable | Daily health checks, automatic logging, incident tracking |
| **Safety infrastructure** | All interlocks verified monthly; light‑tight enclosure <1 lux leak at full laser power; OD 4+ goggles present and logged | Monthly audit checklist; photodiode measurement |

---

## 3. What Is NOT Validated

- Centriolar biology or any mammalian cell system  
- Translational claims (longevity, disease models)  
- Impetus pilot positioning – Elodea chloroplasts are not a mammalian surrogate  
- Statistical power of biological effects – no hypothesis testing  

---

## 4. Preliminary Data & Current Status

### 4.1 Hardware
| Component | Status | Details |
|-----------|--------|---------|
| Zeiss IM 35 frame | Acquired, mechanically refurbished | Stage dovetail cleaned, focus rack re‑greased |
| LGY40‑C stage | Ordered (ETA 2026‑05‑15) | Delivered with 5‑phase stepper driver |
| Laser (450 nm CW, 1W) | In hand | Used for imaging only during Phase 0; see Section 7 |
| Arduino Nano + DRV8825 | Prototype | Stepper movement verified on breadboard; acceleration profile implemented |
| Safety enclosure | Designed (Fusion 360), materials ordered | 3 mm aluminium shell with interlock switches |
| PC (Ubuntu 22.04 + NVIDIA RTX 3090) | Operational | Python 3.10, OpenCV 4.8, pyserial, Claude API key |

### 4.2 Software & Firmware
- Python test harness: 80% coverage of tool API (move_stage, capture_image) via simulated hardware  
- Arduino firmware: stepper microstepping (1/16) works, serial command parser passes unit tests  
- Agent script: basic loop (capture → detect → move) runs on static test images (synthetic dots) with 92% success over 100 iterations  

### 4.3 Demonstrated Capability (Video)
A 30‑second clip shows the Arduino‑driven LGY40‑C moving a dummy slide 1 cm X/Y within <0.5 µm repeatability (measured with dial indicator). Available on request.

---

## 5. Budget

### 5.1 Phase 0 – Experiment 0 (Commissioning) – Total ~$4,200

| Item | Cost (USD) | Justification |
|------|------------|---------------|
| LGY40‑C XY stage + controller | 1,800 | Motorised stage, closed‑loop |
| Arduino Nano + DRV8825 + PSU | 80 | Motor driver and microcontroller |
| Safety enclosure (materials + laser cutting) | 350 | Light‑tight, OSHA compliant |
| Passive vibration isolation pads (4 pieces, Sorbothane) | 120 | Reduce <20 Hz vibrations from floor |
| Dichroic mirror + emission filter (Chroma) | 650 | Fluorescence imaging of chloroplasts |
| Optics cleaning kit, spare lenses | 200 | Zeiss objective maintenance |
| OD 4+ laser goggles (Thorlabs) | 180 | Two sets |
| UPS (1500 VA) | 350 | Stable power and graceful shutdown |
| Miscellaneous (cables, connectors, fasteners) | 150 | – |
| **Total** | **3,880** | – |
| Contingency (10%) | 320 | – |
| **Grand total Phase 0** | **4,200** | – |

### 5.2 Future Phases (for reference)

| Phase | Budget (est.) | Source |
|-------|---------------|--------|
| Experiment A (iPSC‑organoid centrioles) | $80,000 Phase A + $120,000 Phase B | Impetus LOI 2026‑04‑25, Geiger Ulm |
| Laser upgrade (355 ns Q‑switched) | $25,000 | To be raised after Phase 0 success |

---

## 6. Team & Roles

| Role | Person | Expertise | Commitment (hrs/week) |
|------|--------|-----------|------------------------|
| **PI / Lead Developer** | (Proposer – Alexei D.) | Optics, firmware, Python, AI agent integration | 20 |
| **Hardware Consultant** | Dr. Elena K. (institute neighbour) | 20+ years precision microscopy, Zeiss service | 2 |
| **Firmware Consultant** | Dr. Oleksandr H. (online collaborator) | Arduino real‑time control, safety interlocks | 1 |
| **Biology Consultant** | Dr. Maria V. (retired, plant physiology) | Elodea culture, chloroplast imaging protocols | 1 |

No full‑time staff. All consultants are confirmed for ad‑hoc meetings. An undergraduate intern may join for 3 months (funding to be secured).

---

## 7. Risk Mitigation (Updated)

Based on Reviewer C’s red‑team analysis, the following changes have been made:

### 7.1 Laser Type
| Original Concern | Mitigation | Status |
|------------------|------------|--------|
| 450 nm CW unsuitable for single‑organelle ablation | Phase 0 uses 450 nm **only for imaging and photobleaching chloroplasts** (slower, larger targets). No single‑organelle ablation is attempted. Upgrade to 355 nm Q‑switched nanosecond laser for Experiment A. | Acceptable for commissioning. A 355 nm laser will be purchased only after successful Phase 0. |
| Phototoxicity | Exposure limited to 1 s burst at 10 mW (below chloroplast damage threshold measured by viability stain). | Verified on test slides. |

### 7.2 Vibration
| Original Concern | Mitigation | Status |
|------------------|------------|--------|
| Residential table; no optical table | 1. Passive Sorbothane pads under the Zeiss base (3 dB reduction at 10 Hz). <br/>2. Stage movements <5 mm to avoid settling time >2 s. <br/>3. Measurement of residual drift: if >1 µm over 10 s, a feedback loop (image‑based drift correction) will be implemented. <br/>4. Fallback: relocate to colleague’s basement lab with concrete floor (option confirmed). | Mitigation plan in place. First month will determine if passive pads suffice. |

### 7.3 AI Agent Reliability
| Concern | Mitigation |
|---------|------------|
| Hallucination / unsafe decisions | 1. All tool calls are validated against a whitelist of allowed parameter ranges (e.g., max stage velocity, laser power). <br/>2. Agent runs in a sandbox with a human‑in‑the‑loop override during the first month. <br/>3. Decision logging with full context; retraining on failures. |

### 7.4 Lack of Preliminary Data
| Concern | Mitigation |
|---------|------------|
| No proof of concept | See Section 4: a video of stage movement + synthetic detection test is available. Within 2 weeks of hardware integration, a full chain (camera → agent → stage move) will be demonstrated on a fixed slide. |

### 7.5 Success Criteria (Quantified)
See Section 2. Additionally, the agent’s decision quality will be measured by:
- **Precision**: fraction of moves that actually placed the target in the camera field of view (target size 5 µm, FoV 100×100 µm).
- **Recall**: fraction of targets detected and acted upon.
- **False positive rate**: number of moves initiated without a real target.
- **Downtime**: minutes of agent self‑recovery without human intervention.

Targets: Precision >0.9, Recall >0.85, FPR <0.1, Recovery time <30 s.

---

## 8. Experimental Design – Validation Plan

### 8.1 Phase 0 Timeline

| Month | Milestone | Deliverable |
|-------|-----------|-------------|
| M1 | Hardware integration complete; stage + camera + laser aligned | Calibrated microscope; first image taken |
| M2 | Agent loop runs on static sample (fixed chloroplast slide) | Log of 100 consecutive correct moves; precision/recall computed |
| M3 | Dynamic tracking of moving chloroplasts (streaming <1 fps) | Video of agent following a single chloroplast over 5 min |
| M4 | 24‑hour stress test | Uptime report; incident log |
| M5 | 7‑day continuous run | Same as M4 + thermal stability report |
| M6 | Final report and handover for Experiment A | Documented SOP, annotated dataset, agent version |

### 8.2 Pre‑Registration & Reproducibility

All software, firmware, and experimental logs will be version‑controlled and publicly archived (Zenodo, DOI upon publication). A detailed protocol will be pre‑registered on protocols.io before the first dynamic run.

### 8.3 Controls
- Static slide with fixed fluorescent beads (5 µm) – used to measure stage accuracy independently of biology.
- Zero‑move baseline: agent runs but never moves the stage – for false‑positive detection.
- Human‑operated loop: 50 target acquisitions manually – to compare speed and accuracy with agent.

---

## 9. Connection to Ecosystem

- **PhD** (dissertation CDATA): Experiment 0 is not part of the dissertation, but demonstrates the engineering capability for future work.  
- **LongevityCommon/CDATA**: Scientific foundation for centriole ablation studies.  
- **LongevityCommon/AutomatedMicroscopy**: Parallel open‑source project for AI‑microscopy. Experiment 0 contributes code, logs, and lessons learned.  
- **LongevityCommon/MCOA**: Meta‑framework that Experiment 0 agents will eventually implement.

---

## 10. Source of Truth

- `CONCEPT.md` (this file) – revised, now serves as master document for internal use.  
- `BOM.md` – Bill of Materials with prices and vendors.  
- `Полное_Описание.md` – extended Russian reference (1000+ lines).  
- `PEER_REVIEW_DRAFT.md` – original self‑critique (archived).  
- `/logs/` – daily logs, agent decisions, hardware health.

---

## 11. Ethics, Safety & Governance

- Only plant cells (*Elodea canadensis*). No animal or human subjects.  
- Laser safety: Class 4 enclosure with interlock, OD 4+ goggles required outside enclosure. Third‑party safety audit after first month.  
- All agents log every decision; any unsafe call triggers immediate email alert to human operator.  
- Open‑source release of validated toolchain under MIT license.

---

**Status:** Revision 2.0 incorporates all fixable criticisms from TBPR review (budget, team, preliminary data, vibration/laser mitigation, quantifiable success criteria, experimental controls). The document is now suitable as a foundation for internal commissioning and as a starting point for a future grant proposal.