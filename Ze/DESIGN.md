# DESIGN — Ze

## 1. Architecture

```
ze_simulator (Rust, CLI ze_sim)
    ├── impedance/      — гл. 2-3 ODE
    ├── chsh/           — гл. 7-8 quantum
    └── autowaves/      — гл. 13/17 reaction-diffusion
                ↓
ze_backend (Rust axum, port 4001)
    └── REST endpoints: /api/impedance, /api/chsh, /api/autowaves
                ↓
ze_frontend (Phoenix LiveView, port 4000)
    └── 3 вкладки: impedance / chsh / autowaves
                ↓
JSON / графики (Chart.js)
```

## 2. File tree

```
Ze/
├── Ze Theory.pdf · Ze Теория.pdf      ← канон
├── 9 core .md files                    ← документация
├── Cargo.toml · Cargo.lock             ← Rust workspace
├── simulator/
│   ├── Cargo.toml
│   ├── src/lib.rs                      ← модули impedance, chsh, autowaves
│   └── src/bin/ze_sim.rs               ← CLI
├── backend/
│   ├── Cargo.toml
│   └── src/main.rs                     ← axum
├── frontend/                           ← Phoenix LiveView
│   ├── mix.exs · mix.lock
│   ├── config/ · lib/ · assets/ · test/
└── _archive/                           ← старые версии
```

## 3. Workflow книга → код

```
Ze Theory.pdf chapter X.Y
        ↓
PARAMETERS.md (numerical values)
        ↓
simulator/src/lib.rs::module     // комментарий "// Ze Theory.pdf §X.Y"
        ↓
backend/src/main.rs               // REST handler
        ↓
frontend/lib/                     // LiveView component
        ↓
JSON output / Chart.js graph
```

## 4. Module design — `simulator/src/lib.rs`

### 4.1 `impedance` (гл. 2-3)
```rust
pub fn run_impedance(scenario: Scenario, params: ImpedanceParams) -> Vec<Sample>
// Sample { tau, I, t_phys, C, Phi_Ze, K }
```
Scenarios: routine, novelty, meditation, cheating.
Numerical: RK4, h=0.01.

### 4.2 `chsh` (гл. 7-8)
```rust
pub fn compute_chsh(angles: &[f64; 4], delta: f64, h: f64) -> ChshResult
// ChshResult { S_QM, S_Ze, delta_S, sigma_budget }
```
Замкнутая аналитика; Монте-Карло проверка опционально.

### 4.3 `autowaves` (гл. 13/17)
```rust
pub fn run_autowaves(grid_size: usize, steps: usize, params: AutowaveParams) -> Vec<Frame>
// Frame { I[N], x[N], y[N] } at each timestep
```
Numerical: explicit Euler для x/y, Euler для I с 5-point Laplacian (1D).

### 4.4 F-tests (unit tests)
- F1: routine → I(∞) → 0
- F2: novelty → I monotonic increase
- F3: |S_QM − 2√2| < 1e-10
- F4: S_Ze − S_QM = δ·1.7478 ± 1e-9
- F5: autowaves no-source → I статичен
- F6: x, y ∈ [0, 1] всегда

## 5. REST API

### Impedance
```
POST /api/impedance
  body: { scenario, horizon, params }
  → { samples: [{tau, I, t_phys, C, ...}] }
```

### CHSH
```
POST /api/chsh
  body: { angles, delta, h }
  → { S_QM, S_Ze, delta_S, sigma_budget }
```

### Autowaves
```
POST /api/autowaves
  body: { grid_size, steps, params }
  → { frames: [{I, x, y}] }
```

### Scenarios
```
GET /api/scenarios
  → ["routine", "novelty", "meditation", "cheating"]
```

## 6. Что планируется (TODO STATE)

См. STATE.md P1-P3:
- 2D autowaves (спиральные волны)
- Quantum damping произвольная плотность
- GR-интегратор (гл. 9-11)
- Космологический solver (cobaya/MontePython)
- Frontend LiveView вкладки с Chart.js

## 7. Performance

- impedance: ~10 ms per scenario
- chsh: <1 ms (analytic)
- autowaves: ~500 ms per 2000 steps на 1D N=200 решётке

## 8. Что НЕ моделируется

См. OPEN_PROBLEMS.md §3.
