=== THEORY.md ===
# THEORY.md — Experiment 0: Commissioning Framework

**Версия:** 1.0  
**Статус:** Formal theory for HW+SW validation rig

## 1. Formal Framework

### 1.1 Scope
Experiment 0 (E0) is a **commissioning theory** — not a biological hypothesis. It defines the formal requirements for validating an AI‑agent‑controlled precision microscopy system under 24/7 autonomous operation.

### 1.2 Mathematical Preliminaries
Let $S$ be the system state space:  

$$
S = \{ \text{stage\_position}, \text{laser\_state}, \text{camera\_state}, \text{environmental\_sensors}, \text{interlock\_status} \}
$$

Let $A$ be the set of agent actions (tool functions):  

$$
A = \{ \text{move\_stage}(x,y), \text{fire\_laser}(t), \text{capture\_image}(params), \text{detect\_targets}(img), \text{log\_event}(msg) \}
$$

The agent $G$ (Claude Code + DeepSeek router) implements a decision policy $\pi: S \rightarrow A$.

### 1.3 Validation Objective
For a test period $T = 6$ months, we require:

$$
\forall t \in [0, T] : \text{system\_safe}(t) \land \text{agent\_responsive}(t) \land \text{data\_integrity}(t)
$$

where  

- **system\_safe**($t$) = all interlock conditions satisfied at time $t$  
- **agent\_responsive**($t$) = agent completes action within $\tau_{\max}$  
- **data\_integrity**($t$) = captured images stored without corruption

## 2. Core Axioms

### Axiom 1: Layered Autonomy
The system comprises three independent layers:  

- **L0 (Realtime):** Arduino Nano firmware — guarantees deterministic response (< 1 ms) for safety‑critical paths.  
- **L1 (Controller):** Python tool‑function API — mediates between agent and hardware.  
- **L2 (Decision):** AI agent — executes high‑level plans and adapts to observations.

### Axiom 2: Fail‑Safe Default
In absence of agent command within $\tau_{\text{watchdog}}$, every subsystem reverts to a passive safe state: stage stops, laser turns off, camera idle.

### Axiom 3: Measurement Fidelity
All sensor readings (temperature, humidity, vibration, laser power) shall be logged with uncertainty $\pm \epsilon$ and timestamped to the system clock with drift $< 1$ s/day.

## 3. Derived Properties

### 3.1 Stability Condition
The rig must maintain thermal equilibrium such that:

$$
\Delta T_{\text{objective}} < 0.1\,^\circ\text{C/min}
$$

to prevent focal drift.

### 3.2 Laser Budget Constraint
Maximum cumulative laser exposure per sample per hour:

$$
E_{\text{max}} = \frac{P_{\text{laser}} \cdot t_{\text{on}}}{\text{area}_{\text{spot}}} \leq 10\, \text{J/cm}^2
$$

(empirical limit for Elodea chloroplast viability under 450 nm CW).

### 3.3 Agent Circle Time
Expected agent decision latency:

$$
\tau_{\text{agent}} = \tau_{\text{LLM}}(G) + \tau_{\text{API}} + \tau_{\text{hardware}}
$$

must be $< 10$ s for real‑time tracking performance.

## 4. Falsifiable Predictions

| Prediction | Test | Falsification |
|------------|------|---------------|
| P1: Agent completes 1000 consecutive autonomous cycles without safety override | Run $N=1000$ cycle test | Any manual interlock trigger |
| P2: Stage positioning repeatability $< 1\,\mu$m over 24 h | Measure 20 positions before/after 24 h drift test | $\sigma > 1\,\mu$m |
| P3: Data pipeline stores $> 10^5$ images without corruption | Write‑and‑read hash verification | Any hash mismatch |
| P4: Laser power stability $\pm 5\%$ over 1 h | 3600 measurements at 1 Hz | $\text{CV} > 5\%$ |

## 5. Connection to Existing Theories

- **Real‑Time Control Theory:** The L0 firmware implements a finite state machine (FSM) with bounded response time.  
- **Multi‑Agent Systems:** The DeepSeek router acts as a meta‑layer for task decomposition (not implemented in E0 – single agent).  
- **Uncertainty Quantification:** All sensor logs include measurement uncertainty – future Bayesian calibration.

---

**Note:** No biological theory is proposed. See `PEER_REVIEW_DRAFT.md` for surrogate gap discussion.

---

=== DESIGN.md ===
# DESIGN.md — Experiment 0: Architecture & Implementation

**Версия:** 1.0  
**Статус:** Pre‑commissioning design

## 1. Architecture Overview
```
┌──────────────────────┐
│   AI Agent Layer     │
│  Claude Code (LLM)   │
│  DeepSeek Router     │
└────────┬─────────────┘
         │ JSON RPC
┌────────▼─────────────┐
│  Python API Layer    │
│  Tool Functions      │
│  (move_stage, ...)   │
└────────┬─────────────┘
         │ Serial USB
┌────────▼─────────────┐
│  Arduino Nano Layer  │
│  (Realtime FSM)      │
│  PWM, Stepper, Int.  │
└────────┬─────────────┘
         │ Physical
┌────────▼─────────────┐
│  Zeiss IM 35 +       │
│  LGY40-C XY Stage    │
│  Laser 450nm CW      │
│  Camera (USB)        │
└──────────────────────┘
```

## 2. Components

### 2.1 Hardware
- **Microscope:** Zeiss IM 35 / ICM 405 (inverted) — **no modification** of original mechanics.  
- **Stage:** LGY40‑C motorized XY stage stacked on top of original manual stage.  
- **Laser:** 450 nm CW diode laser, collimated, with TTL modulation.  
- **Camera:** USB CMOS/CCD sensor (generic, to be specified in BOM).  
- **Arduino Nano:** Firmware implementing real‑time control:  
  - Stepper driver (A4988) for LGY40‑C  
  - PWM for laser power  
  - Interlock circuit (door, temperature, emergency stop)  
- **Enclosure:** Light‑tight box, OD 4+ filtered windows.  
- **Safety:** UPS, hardware kill switch, thermal cutoff.

### 2.2 Software
- **Host OS:** Ubuntu 22.04 LTS (headless)  
- **Agent:** Claude Code (Anthropic) via API; DeepSeek router as fallback router (not primary).  
- **API Language:** Python 3.10+, using `pyserial`, `numpy`, `opencv`, `json`.  
- **Tool Functions** (see 3.1).  
- **Data Pipeline:** Local SSD → encrypted backup → optional cloud (not in scope).

### 2.3 Firmware (Arduino)
- Language: C++ (Arduino IDE / PlatformIO)  
- State Machine: `IDLE → MOVING → LASERING → IMAGING → LOGGING → IDLE`  
- Watchdog timer: 500 ms – if no serial command, enter SAFE.

## 3. Data Flow

```
Agent (L2) ──JSON RPC──> Python API (L1) ──Serial──> Arduino (L0) ──PWM──> Laser
                                        ──GPIO──> Stepper
                                        ──ADC──> Sensors
                                        <── Serial ── Status
Camera ──USB──> Python API ──Base64──> Agent (Image stored locally)
```

### Control Flow (Example: fire laser)
1. Agent sends `{"action": "fire_laser", "duration_ms": 100}` via REST/stdio.  
2. Python validates input (bounds).  
3. Sends serial command `LASER 100\n` to Arduino.  
4. Arduino checks interlock: if SAFE, energises laser via PWM for 100 ms.  
5. Arduino returns `OK` or `ERROR` with code.  
6. Python logs and returns response to Agent.

## 4. API Specification

### 4.1 Tool Functions

| Function | Arguments | Returns | Description |
|----------|-----------|---------|-------------|
| `move_stage(x, y)` | `x`: μm (–5000..5000), `y`: μm (–5000..5000) | `{"status", "position"}` | Relative move (μm) |
| `fire_laser(duration_ms)` | `duration_ms`: int (1..10000) | `{"status", "energy_mJ"}` | Continuous wave pulse |
| `capture_image(exposure_ms)` | `exposure_ms`: int (10..5000) | `{"image_base64", "metadata"}` | Returns image |
| `detect_targets(image)` | `image`: base64 | `{"targets": [x,y,size,...]}` | Chloroplast detection |
| `get_status()` | none | `{"stage", "laser", "temp", "interlock"}` | Full system state |
| `set_laser_power(percent)` | `percent`: 0..100 | `{"status"}` | Calibrated power |

### 4.2 Serial Protocol (L1→L0)
- Baud: 115200  
- Format: `CMD [arg]\n`  
- Responses: `ACK` or `ERR <code>\n`  
- Commands: `MOVE X Y\n`, `LASER DURATION\n`, `STATUS\n`, `CALIBRATE\n`, `STOP\n`

## 5. Safety Infrastructure
- **Hardware Interlock:** Door switch → cuts laser power supply directly.  
- **Firmware Watchdog:** If no valid command for 500 ms, stage stops, laser off.  
- **Software Watchdog:** Python monitors Arduino response; if missing, kills agent process.  
- **Agent Check:** Claude Code sends periodic heartbeat; if fails, Python shuts down safely.

## 6. Deployment
- Rig assembled on standard desk (no optical table – risk accepted).  
- Remote access via SSH + reverse tunnel (for monitoring).  
- Logging: all actions, errors, images saved with UTC timestamp.

---

=== PARAMETERS.md ===
# PARAMETERS.md — Experiment 0: All Constants, Variables, Thresholds

**Версия:** 1.0  
**Последнее обновление:** 2026-04-23  
**Единицы:** SI (with exceptions noted)

## 1. Physical Constants

| Parameter | Symbol | Value | Unit | Source/Justification |
|-----------|--------|-------|------|---------------------|
| Laser wavelength | λ | 450 | nm | CONCEPT.md – CW diode |
| Stage step size (LGY40‑C) | $d_{\text{step}}$ | 0.1 | μm/step | Manufacturer specification (не указано точное, оценка) |
| Camera pixel size (typ.) | $p_{\text{px}}$ | 2.2 | μm | Generic CMOS, will calibrate |
| Objective NA (Zeiss Plan 40×) | NA | 0.65 | – | Zeiss IM 35 typical |
| Thermal expansion coefficient (aluminium) | α | 23×10⁻⁶ | /°C | Al 6061, for drift estimate |

## 2. Model Parameters

### 2.1 Laser System
| Parameter | Symbol | Range | Default | Unit | Justification |
|-----------|--------|-------|---------|------|---------------|
| Laser power (max) | $P_{\text{max}}$ | 0–1000 | 500 | mW | TBC in BOM |
| Pulse duration | $t_{\text{on}}$ | 1–10000 | 100 | ms | Empirical for chloroplast bleaching |
| Duty cycle (max) | $DC_{\text{max}}$ | 0–50 | 10 | % | Thermal limit (no active cooling) |

### 2.2 Stage
| Parameter | Symbol | Range | Default | Unit | Justification |
|-----------|--------|-------|---------|------|---------------|
| Travel range X | $X_{\text{range}}$ | –5000…5000 | – | μm | LGY40‑C typical |
| Travel range Y | $Y_{\text{range}}$ | –5000…5000 | – | μm | same |
| Max velocity | $v_{\text{max}}$ | 0–1000 | 500 | μm/s | Stepper torque limit |
| Acceleration | $a_{\text{max}}$ | 0–2000 | 1000 | μm/s² | Avoid lost steps |

### 2.3 Camera
| Parameter | Symbol | Range | Default | Unit | Justification |
|-----------|--------|-------|---------|------|---------------|
| Exposure time | $t_{\text{exp}}$ | 10–5000 | 100 | ms | Depends on illumination |
| Gain | $G$ | 0–48 | 12 | dB | Trade‑off noise/bleaching |
| Resolution | $R$ | 1280×1024, 1920×1080 | 1280×1024 | px | Balance speed/detail |

### 2.4 Agent
| Parameter | Symbol | Range | Default | Unit | Justification |
|-----------|--------|-------|---------|------|---------------|
| Agent timeout | $\tau_{\text{agent}}$ | 5–30 | 10 | s | Empirical; >10s breaks loop |
| Watchdog timer (firmware) | $\tau_{\text{wd}}$ | 200–1000 | 500 | ms | Safety margin |
| Max consecutive actions | $N_{\text{max}}$ | 1–10000 | 1000 | – | Test endurance |

## 3. Thresholds

| Parameter | Threshold | Unit | Consequence if exceeded |
|-----------|-----------|------|-------------------------|
| Temperature (stage) | $> 40$ | °C | Pause, cool down |
| Temperature (laser diode) | $> 35$ | °C | Laser off |
| Humidity | $> 80$ | %RH | Abort, dehumidify |
| Vibration RMS | $> 10$ | μm/s | Flag in log, no abort (risk accepted) |
| Stage position error | $> 1$ | μm | Re‑calibrate |
| Laser power deviation from setpoint | $\pm 5$ | % | Log warning; >10% → stop laser |
| Number of corrupted images / 1000 | $> 0$ | – | Investigate storage |
| Agent consecutive failures | $> 3$ | – | Switch to safe mode, notify |

## 4. Calibration Constants (pre‑commissioning)

| Parameter | Description | Value | Unit | Calibration method |
|-----------|-------------|-------|------|-------------------|
| $k_{\text{laser}}$ | Laser power setpoint → mW | TBC | mW/% | Power meter reference |
| $k_{\text{stage\_X}}$ | Steps → μm | 0.1 | μm/step | Micrometer slide |
| $k_{\text{stage\_Y}}$ | Steps → μm | 0.1 | μm/step | Micrometer slide |
| $k_{\text{camera\_um\_px}}$ | μm per pixel | TBC | μm/px | Stage move + image correlation |
| $k_{\text{temp\_offset}}$ | Arduino temp sensor offset | TBC | °C | Against calibrated thermistor |

## 5. Environmental Limits (design)

| Parameter | Min | Ideal | Max | Unit |
|-----------|-----|-------|-----|------|
| Ambient temperature | 15 | 22 | 30 | °C |
| Relative humidity | 20 | 40 | 70 | %RH |
| Illuminance (inside enclosure) | 0 | 0 | 0.01 | lux |

---

**Note:** All "TBC" values will be filled after component procurement and first calibration runs (see `BOM.md` and commissioning logs).

---

=== EVIDENCE.md ===
# EVIDENCE.md — Experiment 0: Existing Evidence & Gaps

**Версия:** 1.0  
**Статус:** Pre‑commissioning literature review

## 1. Direct Evidence for System Components

### 1.1 Motorised XY Stage (LGY40‑C)
- **Manufacturer datasheet:** Accuracy ±5 μm, repeatability ±1 μm (unverified).  
- **Open‑source usage:** Low‑cost XY stages for microscopy have been demonstrated with stepper motors (e.g., OpenFlexure, 10.1016/j.ohx.2020.e00110).  
- **Zeiss IM 35 compatibility:** No published work stacking a motorised stage on the IM 35 manual stage. Risk of increased vibration and backlash.

### 1.2 450 nm CW Laser for Plant Cell Ablation
- **Chloroplast photobleaching:** 450 nm light effectively damages chloroplasts in *Elodea* (10.1093/jxb/erz154, 2019). CW mode causes cumulative heating – not single‑organelle precision (risk noted in CONCEPT).  
- **Laser safety:** OD 4+ at 450 nm corresponds to attenuation $10^4$ – confirmed adequate by ANSI Z136.1.

### 1.3 Arduino Real‑Time Control
- **Stepper control:** Proven reliability for microscopy stages (10.1371/journal.pone.0180564).  
- **Watchdog timers:** Standard safety practice in embedded systems (IEC 61508‑3).  
- **No existing public repository for Zeiss IM 35 + Arduino + AI agent** – E0 is novel.

### 1.4 AI Agent (Claude Code) for Scientific Instrumentation
- **Prior art:** AI agents for automated labs (e.g., "Coscientist" for chemistry, 10.1038/s41586-023-06792-0; "BioAutoMATED" for ML).  
- **No published work using Claude Code or DeepSeek router for real‑time laser ablation microscopy.** Gap exists.

## 2. Supporting Evidence for Testing

### 2.1 Stability Rig (6‑month)
- **Long‑term drift:** Mechanical drift of inverted microscopes over months < 5 μm for temperature‑controlled labs (10.1111/j.1365-2818.2004.01265.x).  
- **Vibration tolerance:** Desk‑mounted rig may exceed 1 μm RMS – acceptable for commissioning but not for single‑organelle work.

### 2.2 Data Pipeline
- **Image corruption rate:** Modern SSDs have bit error rates < $10^{-15}$ – undetected corruption probability negligible over $10^5$ images. CRC checks recommended.

## 3. Methodological Gaps (from CONCEPT + `PEER_REVIEW_DRAFT.md`)

| Gap | Impact | Mitigation in E0 |
|-----|--------|-----------------|
| 1. **Biological surrogate**: *Elodea* chloroplasts ≠ mammalian centrioles | Not relevant – E0 is commissioning only | No biological claims |
| 2. **Laser type**: 450 nm CW, not Q‑switched UV | Cannot perform single‑organelle ablation; phototoxicity risk | Use low duty cycle, monitor cell viability |
| 3. **Optics UV coating**: Zeiss IM 35 objectives <30% transmission at 450 nm | Reduced laser power at sample | Calibrate effective power; may need larger spot |
| 4. **Statistics**: No power calculation, pre‑registration, blinding | Invalid for any biological inference | E0 does not produce inferential statistics |
| 5. **Vibration**: Household desk, no optical table | Stage jitter >1 μm possible, affects imaging | Accept for commissioning; upgrade before Experiment A |
| 6. **Agent reliability**: No peer‑reviewed benchmark for Claude Code in real‑time microscopy | Unknown failure modes | Extensive fault injection testing |

## 4. Planned Validation Steps (Fill Gaps)

### 4.1 Pre‑Commissioning Tests
1. **Stage calibration** (micrometer slide, repeated 20x) – verify $< 1\ \mu$m repeatability.  
2. **Laser power calibration** (Ophir power meter) – map PWM duty vs. mW.  
3. **Optical transmission** of objective at 450 nm – measure with power meter.  
4. **Agent simulation** – replace real hardware with mock; test 1000 cycles.

### 4.2 Commissioning Tests (first month)
1. **24‑h stability test** – log temperature, vibration, focus drift.  
2. **1000‑cycle autonomous run** – agent controls stage/laser/camera, no human intervention.  
3. **Fault injection** – introduce fake sensor errors, verify agent response.

### 4.3 Long‑Term (6 months)
1. **Monitored operation** – log all failures and anomalies.  
2. **Data integrity** – weekly checksum verification of all stored images.

## 5. References

- (No PMIDs for hardware components; datasheets and open‑source literature only.)  
- `PEER_REVIEW_DRAFT.md` – detailed gap analysis (internal).  
- `BOM.md` – component specs and supplier links.  
- `Полное_Описание.md` – extended reference (1000 lines).  

---

**Conclusion:** No direct experimental evidence exists for the integrated E0 system. All components have prior art but not in combination. Validation will generate the evidence required for Experiment A design.