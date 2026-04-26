# BioSense Frontend

Phoenix 1.7 LiveView frontend for the BioSense measurement layer subproject of LongevityCommon.

## Overview

BioSense is the measurement layer in the MCOA (Multi‑Channel Organism Assessment) framework, providing raw sensor streams from three complementary biosignal modalities:

1. **EEG** – Electroencephalography (brain rhythms)
2. **HRV** – Heart Rate Variability (autonomic nervous system)
3. **Olfaction** – Volatile Organic Compound detection (molecular spectroscopy)

The frontend displays real‑time sensor data, validated dataset results, technical parameters, and domain knowledge without making biomarker claims, in accordance with CORRECTIONS 2026‑04‑22.

## Architecture

- **Phoenix 1.7** with LiveView for real‑time updates
- **Tailwind CSS** for styling
- **Req** for HTTP communication with the Rust backend
- **Telemetry** for monitoring and observability
- Graceful degradation when backend is unavailable

## Key Features

### Dashboard
- Real‑time visualization of EEG, HRV, and Olfaction sensor streams
- Status overview of validated datasets (Cuban, Dortmund, MPI‑LEMON)
- Display of key technical parameters with canonical status
- MCOA counter registry interface
- Sobol sensitivity analysis visualization

### Detail Views
- Drill‑down into individual datasets, parameters, and knowledge entries
- Interactive MCOA counter management (γ_i linkage parameters)
- Sensitivity analysis charts for CDATA models

### CORRECTIONS 2026‑04‑22 Compliance
- No χ_Ze validated biomarker claims
- Display of raw sensor streams only
- Default γ_i = 0 for counter independence hypothesis
- Canonical Ze speed formula: v = N_S/(N−1)
- Clear labeling of exploratory vs confirmatory results

## Development

### Prerequisites
- Elixir 1.15+
- Phoenix 1.7+
- Node.js 18+ (for Tailwind)
- Rust backend running on port 3004

### Setup

```bash
cd frontend
mix deps.get
npm install --prefix assets
mix phx.server
```

Visit `http://localhost:4004`

### Environment Variables

- `BACKEND_URL` – URL of the Rust backend API (default: `http://localhost:3004`)
- `PORT` – Phoenix server port (default: `4004`)
- `SECRET_KEY_BASE` – Required for production

### Testing

```bash
mix test
mix coveralls
```

## Deployment

### Docker

```bash
docker build -t biosense-frontend .
docker run -p 4004:4004 \
  -e BACKEND_URL=http://backend:3004 \
  -e SECRET_KEY_BASE=... \
  biosense-frontend
```

### Production

1. Set environment variables:
   ```bash
   export BACKEND_URL=http://your-backend:3004
   export SECRET_KEY_BASE=$(mix phx.gen.secret)
   export PORT=4004
   ```

2. Build assets:
   ```bash
   mix assets.deploy
   ```

3. Start server:
   ```bash
   MIX_ENV=prod mix phx.server
   ```

## API Integration

The frontend communicates with the Rust backend via REST API. All API calls are handled through `BackendClient` with retry logic, timeout handling, and telemetry instrumentation.

## Monitoring

Telemetry metrics are exposed for:
- Request duration to backend
- Error rates
- Health check status
- Counter parameter updates

Use `:observer_cli` or configure your preferred telemetry consumer.

## License

Part of the LongevityCommon project. See main project repository for licensing information.