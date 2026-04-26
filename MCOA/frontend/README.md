# MCOA Phoenix frontend

Phoenix LiveView frontend for the MCOA simulator. Consumes the Axum backend at
`http://127.0.0.1:3030/api/simulate`.

## Quickstart

```bash
# 1. Start the Rust backend (in another terminal)
cd ..
cargo run --release --bin mcoa-api

# 2. Start Phoenix
cd frontend
mix setup
mix phx.server
```

Open http://localhost:4000 — dashboard with tissue selector and counter trajectories.

## Scope (v0.2)

- `DashboardLive` — tissue × divisions selector, per-counter table.
- TODO: `ComparisonLive` — MCOA vs CDATA side-by-side plot, residual panel (consumes the same
  `compare_mcoa_cdata.py` logic on the server side).
- TODO: LiveView hooks for Chart.js trajectories.

## Why Phoenix

Per user's canonical stack rule: Rust for backend, Phoenix for frontend. Consistent with
LongevityCommon's realtime stack.

Long-form rule: `../CLAUDE.md` §"Language / stack".
