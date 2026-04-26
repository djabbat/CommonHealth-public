# Ontogenesis Frontend

Phoenix 1.7 LiveView frontend for the LongevityCommon subproject "Ontogenesis".

## Overview

Ontogenesis v4.2 is a developmental prequel layer for MCOA (Multicomponent Ontogenetic Architecture), covering human development from zygote to death (0–120 years). This frontend provides interactive visualization and exploration of:

- **Etagenesis**: Three-phase model (Ontogenesis 0–25y, Mesogenesis 25–50y, Gerontogenesis 50–120y)
- **Five Neurobiological Phases**: Based on Nat Commun 2025 DOI 10.1038/s41467-025-65974-8
- **Four Domains**: Morphology, Physiology, Psychology, Sociology
- **Cross-domain Coupling**: LCS (Latent Change Score) modeling with coupling coefficients
- **Age-related Metamorphosis**: Synchronous transitions across ≥2 domains

## Features

- **Dashboard**: Overview of all parameters, phases, and algorithm configuration
- **Detail Views**: Drill-down into individual parameters with related entities
- **Real-time Updates**: LiveView powered automatic refresh
- **Telemetry**: Comprehensive monitoring of backend requests and system metrics
- **Graceful Degradation**: Handles backend failures with informative error states
- **Production Ready**: Proper error handling, logging, and deployment configuration

## Configuration

### Environment Variables

- `PORT`: Server port (default: 4011)
- `PHX_HOST`: Hostname for URL generation
- `SECRET_KEY_BASE`: Phoenix secret key for signing
- `BACKEND_URL`: Rust backend API URL (default: http://localhost:3011)

### Backend API

The frontend expects a RESTful backend providing:

- `GET /api/dashboard` - Dashboard data (parameters, phases, counts)
- `GET /api/entities/:id` - Individual entity with related entities
- `GET /api/parameters?domain=:domain` - Domain-specific parameters
- `GET /health` - Health check endpoint

## Architecture

### Key Modules

- `DashboardLive`: Main dashboard with overview and metrics
- `DetailLive`: Detailed entity exploration
- `BackendClient`: HTTP client for backend communication with telemetry
- `CoreComponents`: Reusable UI components (cards, badges, progress bars)
- `Layouts`: Application layout with navigation

### Telemetry

Metrics collected:
- Backend request duration and count
- LiveView process count
- Phoenix endpoint and router timing

View metrics at `/dashboard` (Phoenix LiveDashboard).

## Development

```bash
cd frontend
mix deps.get
mix assets.setup
mix phx.server
```

Visit `http://localhost:4011`

## Production Deployment

### With Docker

```bash
docker build -t ontogenesis-frontend .
docker run -p 4011:4011 \
  -e BACKEND_URL=http://your-backend:3011 \
  -e SECRET_KEY_BASE=your_secret_key_base \
  ontogenesis-frontend
```

### Without Docker

```bash
MIX_ENV=prod mix release
_build/prod/rel/ontogenesis_frontend/bin/ontogenesis_frontend start
```

## CORRECTIONS_2026‑04‑22 Compliance

- No χ_Ze validated biomarker claims (BioSense shows raw sensor streams only)
- No Health Score widget
- For Ze displays: v = N_S/(N-1)
- For MCOA: counter registry UI integration
- For CDATA: Sobol sensitivity visualization, HSC lineage tracking
- Scaffold subprojects (Telomere, MitoROS, EpigeneticDrift, Proteostasis) have γ_i = 0 by default

## Data Sources

- WHO Growth Charts (CC BY‑NC‑SA)
- British Birth Cohorts (n=27,432)
- ASEBA norms (licensed)
- WISC/WAIS norms
- GSS/ESS survey data
- Frolkis (1999) etagenesis model
- Nat Commun 2025 neurobiological phases

## License

Research Use Only. All data is synthetic and anonymized.

---

*Ontogenesis v4.2 — Developmental prequel layer for MCOA (Tkemalad