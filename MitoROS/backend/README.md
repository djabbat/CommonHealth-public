# MitoROS Backend

Mitochondrial ROS and mtDNA Damage as Counter #3 in the Multi-Counter Architecture of Aging.

## Overview

This backend implements the MitoROS (Counter #3) subproject for LongevityCommon. It provides REST API endpoints for managing and computing mitochondrial damage accumulation according to the MCOA framework.

## Features

- **Counter #3 Data Management**: Store and retrieve time-series data for mitochondrial damage (D3)
- **Parameter Management**: CRUD operations for MitoROS model parameters (α3, β3, τ3, n3*, etc.)
- **Tissue Registry**: Manage biological tissues with MCOA weights
- **D3 Computation**: Compute D3(n,t) using the kinetic equation with default γ3=0
- **Database Migrations**: PostgreSQL with automatic schema management

## API Endpoints

### Tissues
- `GET /api/tissues` - List all tissues
- `GET /api/tissues/:id` - Get specific tissue
- `POST /api/tissues` - Create new tissue
- `PUT /api/tissues/:id` - Update tissue
- `DELETE /api/tissues/:id` - Delete tissue

### Counter3 Parameters
- `GET /api/counter3_parameters` - List all parameter sets
- `GET /api/counter3_parameters/:id` - Get specific parameters
- `POST /api/counter3_parameters` - Create new parameters
- `PUT /api/counter3_parameters/:id` - Update parameters
- `DELETE /api/counter3_parameters/:id` - Delete parameters

### Counter3 Records
- `GET /api/counter3_records` - List all records (with pagination)
- `GET /api/counter3_records/:id` - Get specific record
- `POST /api/counter3_records` - Create new record
- `PUT /api/counter3_records/:id` - Update record
- `DELETE /api/counter3_records/:id` - Delete record

### Computation
- `POST /api/compute_d3` - Compute D3 value for given n and t

## Canonical Rules

According to LongevityCommon CORRECTIONS_2026-04-22:

1. **γ3 = 0 by default**: Interaction coefficients are set to 0 as the null hypothesis
2. **No Health Score aggregation**: Removed from all scaffold counters
3. **Parameter defaults**: α3=0.001, β3=0.01, n3*=1000, τ3=30 years

## Quick Start

1. **Set up environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

2. **Set up PostgreSQL**:
   ```bash
   createdb mitoros_db
   ```

3. **Run migrations**:
   ```bash
   cargo sqlx migrate run
   ```

4. **Start server**:
   ```bash
   cargo run
   ```

5. **Test API**:
   ```bash
   curl http://localhost:3006/health
   ```

## Database Schema

### Tissues
- `id` (UUID): Primary key
- `name`: Tissue name (unique)
- `mitotic_index`: Proportion of dividing cells (0-1)
- `metabolic_rate`: Relative metabolic rate
- `weight_w3`: MCOA weight for Counter #3

### Counter3Parameters
- `tissue_id`: Foreign key to tissues
- `d3_0`: Basal damage level
- `alpha3`: Division-dependent coefficient
- `n3_star`: Critical division threshold
- `beta3`: Time-dependent coefficient
- `tau3`: Characteristic time constant
- `gamma3`: Interaction coefficient (default 0)

### Counter3Records
- `tissue_id`: Foreign key to tissues
- `n_cell_divisions`: Cell division count (n)
- `t_time`: Chronological time in years (t)
- `d3_value`: Computed D3 value

## Kinetic Equation

```
D3(n,t) = D3_0 + α3 * (n/n3*) + β3 * (t/τ3) + γ3 * I(other_counters)
```

Where:
- `γ3 = 0` by default (null hypothesis of independence)
- `I(other_counters) = 0` for scaffold implementation

## Development

### Build
```bash
cargo build
```

### Run tests
```bash
cargo test
```

### Database operations
```bash
# Create new migration
cargo sqlx migrate add <name>

# Run migrations
cargo sqlx migrate run

# Revert last migration
cargo sqlx migrate revert
```

## Docker Deployment

```bash
docker build -t mitoros-backend .
docker run -p 3006:3006 --env-file .env mitoros-backend