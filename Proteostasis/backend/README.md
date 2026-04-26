# Proteostasis Backend

Backend service for Proteostasis Counter #5 in the Multi-Counter Architecture of Aging (MCOA).

## Features

- REST API for managing proteostasis parameters (D₅,₀, α₅, n₅*, β₅, τ₅, γ couplings)
- Time series storage for proteostasis damage measurements (D₅)
- Damage computation endpoint using the kinetic equation:
  ```
  D₅(n, t) = D₅,₀ + α₅ · (n / n₅*) + β₅ · (t / τ₅)
  ```
- PostgreSQL database with automatic migrations
- Graceful shutdown and structured logging

## API Endpoints

### Health Check
- `GET /health` - Service health status

### Parameters Management
- `GET /proteostasis/parameters` - List all parameter sets
- `GET /proteostasis/parameters/:id` - Get specific parameter set
- `POST /proteostasis/parameters` - Create new parameter set
- `PUT /proteostasis/parameters/:id` - Update parameter set
- `DELETE /proteostasis/parameters/:id` - Delete parameter set

### Time Series Management
- `GET /proteostasis/time_series` - List all time series entries
- `GET /proteostasis/time_series/:id` - Get specific time series entry
- `POST /proteostasis/time_series` - Create new time series entry
- `PUT /proteostasis/time_series/:id` - Update time series entry
- `DELETE /proteostasis/time_series/:id` - Delete time series entry

### Computation
- `POST /proteostasis/compute` - Compute D₅ value given n and t

## Environment Variables

```bash
PORT=3008
DATABASE_URL=postgres://cn:cn@localhost/proteostasis_db
LOG_LEVEL=info
```

## Quick Start

1. Clone the repository
2. Copy `.env.example` to `.env` and adjust values
3. Start PostgreSQL database
4. Run migrations: `sqlx migrate run`
5. Build and run: `cargo run --release`

## Database Schema

### proteostasis_parameters
- Stores tissue-specific parameters for counter #5
- Includes all coefficients from the kinetic equation
- Default values from PARAMETERS.md pre-loaded

### proteostasis_time_series
- Stores time-series measurements of proteostasis damage
- Links to parameter sets via foreign key
- Includes metadata JSON field for additional context

## Development

### Prerequisites
- Rust 1.70+ with Cargo
- PostgreSQL 13+
- SQLx CLI: `cargo install sqlx-cli`

### Running Migrations
```bash
sqlx database create
sqlx migrate run
```

### Testing
```bash
cargo test
```

### Building Docker Image
```bash
docker build -t proteostasis-backend .
```

## Default Parameters

Based on PARAMETERS.md:
- α₅ = 0.05 (damage per normalized division)
- n₅* = 50 (critical divisions)
- β₅ = 0.1 (damage per year)
- τ₅ = 10 years (aggregation time constant)
- All γ coefficients = 0.0 (per CORRECTIONS_2026-04-22 §1.3)

Tissue weights:
- Neuron: 0.4
- Muscle: 0.2
- Liver: 0.05