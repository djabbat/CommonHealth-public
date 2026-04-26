# Telomere Backend

MCOA Counter #2: Telomere Shortening Counter backend service.

## Overview

This service implements the Telomere Shortening Counter as defined in the Multi-Counter Architecture of Organismal Aging (MCOA). It provides REST APIs for storing and retrieving telomere measurements, managing kinetic equation parameters, and computing tissue-specific aging loads.

## Features

- **Telomere Measurement CRUD**: Store and retrieve telomere length measurements with associated metadata
- **Parameter Management**: Manage kinetic equation parameters (D₂,₀, α₂, β₂, n₂*, τ₂) per subject or using defaults
- **Counter Registry**: MCOA-compliant counter metadata endpoint
- **Tissue Load Computation**: Compute L_tissue(n,t) = w₂(tissue)·f₂(D₂(n,t)) for tissue aging assessment
- **Database Migrations**: SQLx-powered PostgreSQL migrations
- **Comprehensive Error Handling**: Structured error responses with tracing
- **Graceful Shutdown**: Clean shutdown on SIGTERM/SIGINT

## Technology Stack

- **Runtime**: Tokio 1.0 + Async/Await
- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL 14+ with SQLx 0.7
- **Serialization**: Serde 1.0 + JSON
- **Validation**: Validator 0.16
- **Tracing**: Tracing + JSON logs
- **Configuration**: Config crate with environment variable support

## API Endpoints

### Health & Metadata
- `GET /health` - Service health check
- `GET /api/v1/counters` - List all MCOA counters (only Telomere #2)
- `GET /api/v1/counters/:id` - Get counter metadata

### Telomere Measurements
- `GET /api/v1/measurements` - List measurements with filtering
- `GET /api/v1/measurements/:id` - Get specific measurement
- `POST /api/v1/measurements` - Create new measurement
- `PUT /api/v1/measurements/:id` - Update measurement
- `DELETE /api/v1/measurements/:id` - Delete measurement
- `GET /api/v1/subjects/:subject_id/measurements` - List subject measurements

### Telomere Parameters
- `GET /api/v1/parameters` - List all parameter sets
- `GET /api/v1/parameters/:id` - Get specific parameters
- `POST /api/v1/parameters` - Create new parameters
- `PUT /api/v1/parameters/:id` - Update parameters
- `DELETE /api/v1/parameters/:id` - Delete parameters
- `GET /api/v1/subjects/:subject_id/parameters` - Get subject parameters

### Tissue Load Computation
- `POST /api/v1/compute-tissue-load` - Compute tissue aging load

## Kinetic Equation

The telomere shortening counter follows the MCOA kinetic equation:

```
D₂(n, t) = D₂,₀ + α₂·(n / n₂*) + β₂·(t / τ₂)
```

Where:
- `D₂,₀`: Baseline telomere length (bp)
- `α₂`: Division-dependent erosion coefficient (bp/PD)
- `β₂`: Time/stress-dependent erosion coefficient (bp/year)
- `n`: Cumulative population doublings
- `n₂*`: Hayflick limit/critical replicative limit (PD)
- `t`: Time elapsed (years)
- `τ₂`: Turnover timescale (years)

## Default Parameters

Default values from PARAMETERS.md:
- `D₂,₀` = 12500 bp (10-15kb range midpoint)
- `α₂` = 125 bp/PD (50-200 bp/PD range midpoint)
- `β₂` = 35 bp/year (20-50 bp/year range midpoint)
- `n₂*` = 50 PD (40-60 PD range midpoint)
- `τ₂` = 1 year (default)
- `γ_i` = 0 (all coupling coefficients, per CORRECTIONS_2026-04-22)

## Setup & Deployment

### Prerequisites
- Rust 1.70+ (2021 edition)
- PostgreSQL 14+
- Cargo

### Environment Variables
Copy `.env.example` to `.env` and configure:
```bash
DATABASE_URL=postgres://cn:cn@localhost/telomere_db
PORT=3005
RUST_LOG=telomere_backend=info,tower_http=debug
TELOMERE__CORS_ORIGINS=["http://localhost:3000"]
```

### Database Setup
```bash
# Create database
createdb telomere_db

# Run migrations (automatically on startup)
cargo sqlx migrate run
```

### Development
```bash
# Install dependencies
cargo build

# Run with cargo
cargo run

# Run tests
cargo test

# Run with specific config
RUN_MODE=production cargo run
```

### Docker
```bash
# Build image
docker build -t telomere-backend .

# Run container
docker run -p 3005:3005 \
  -e DATABASE_URL=postgres://user:pass@host/telomere_db \
  telomere-backend
```

## Architecture

```
src/
├── main.rs           # Application entry point, server setup
├── config.rs         # Configuration loading
├── error.rs          # Error types and handling
├── models.rs         # Data structures and validation
├── routes.rs         # HTTP route handlers
├── db.rs             # Database operations
└── lib.rs            # Library exports

migrations/
└── 001_initial.sql   # Database schema
```

## Testing

```bash
# Run unit tests
cargo test --lib

# Run integration tests (requires test database)
DATABASE_URL=postgres://cn:cn@localhost/telomere_test cargo test
```

## Monitoring

- Health endpoint: `GET /health`
- Structured JSON logging via Tracing
- Request tracing with Tower HTTP

## MCOA Compliance

This service implements Counter #2 according to the MCOA specification:
- Counter registry endpoint with metadata
- Kinetic equation parameter storage
- Tissue load computation endpoint
- Coupling coefficients (γ) default to 0 per CORRECTIONS_2026-04-22
- No health score aggregation (removed per corrections)

## License

MIT License - See LICENSE file for details.