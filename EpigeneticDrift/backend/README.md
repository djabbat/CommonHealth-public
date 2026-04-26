# Epigenetic Drift Backend

REST API backend for the Epigenetic Drift Counter (#4) in the Multi-Counter Architecture of Organismal Aging (MCOA).

## Features

- Complete CRUD operations for Epigenetic Drift entities:
  - **Counters**: Time-series tracking of epigenetic drift state `D₄`
  - **Measurements**: Raw epigenetic measurements (DNA methylation, ATAC-seq)
  - **Parameters**: Tissue-specific kinetic parameters for the drift equation
- PostgreSQL database with proper migrations
- RESTful API with JSON serialization
- Comprehensive error handling and tracing
- Graceful shutdown
- CORS enabled

## API Endpoints

### Health Check
- `GET /health` - Service health status

### Counters
- `GET /counters` - List all counters
- `GET /counters/:id` - Get specific counter
- `POST /counters` - Create new counter
- `PUT /counters/:id` - Update counter
- `DELETE /counters/:id` - Delete counter

### Measurements
- `GET /measurements` - List all measurements
- `GET /measurements/:id` - Get specific measurement
- `POST /measurements` - Create new measurement
- `PUT /measurements/:id` - Update measurement
- `DELETE /measurements/:id` - Delete measurement

### Parameters
- `GET /parameters` - List all parameter sets
- `GET /parameters/:id` - Get specific parameters
- `POST /parameters` - Create new parameters
- `PUT /parameters/:id` - Update parameters
- `DELETE /parameters/:id` - Delete parameters

## Data Model

### Epigenetic Drift Equation
```
D₄(n, t) = D₄,₀ + β₄·(t / τ₄) + α₄·(n / n₄*) + γ₄ · I(other counters)
```

### Key Parameters
- `D₄`: Epigenetic drift state (normalized)
- `D₄,₀`: Baseline epigenetic state
- `β₄`: Time-dominant linear coefficient
- `τ₄`: Characteristic time constant (~10 years)
- `α₄`: Replication-associated coefficient
- `n₄*`: Characteristic number of divisions
- `γ`: Interaction coefficients (default 0 per canonical rules)

## Getting Started

### Prerequisites
- Rust 1.70+ (2021 edition)
- PostgreSQL 14+
- Cargo

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd backend
   ```

2. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. Set up database:
   ```bash
   createdb epigeneticdrift_db
   psql epigeneticdrift_db -c "CREATE USER cn WITH PASSWORD 'cn';"
   psql epigeneticdrift_db -c "GRANT ALL PRIVILEGES ON DATABASE epigeneticdrift_db TO cn;"
   ```

4. Run migrations:
   ```bash
   sqlx migrate run
   ```

5. Build and run:
   ```bash
   cargo build --release
   cargo run
   ```

### Docker

```bash
docker build -t epigeneticdrift-backend .
docker run -p 3007:3007 epigeneticdrift-backend
```

## Configuration

Environment variables (see `.env.example`):

- `DATABASE_URL`: PostgreSQL connection string
- `PORT`: Server port (default: 3007)
- `RUN_MODE`: Environment mode (development/production)

## Development

### Testing
```bash
cargo test
```

### Database Migrations
```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert migration
sqlx migrate revert
```

### Code Structure
- `src/main.rs`: Application entry point
- `src/routes.rs`: API route definitions
- `src/models.rs`: Data models and database operations
- `src/db.rs`: Database connection pool
- `src/error.rs`: Error types and handling
- `src/config.rs`: Configuration management
- `migrations/`: Database migrations

## Canonical Rules Applied

1. All interaction coefficients (γ) default to 0
2. No health score aggregation (removed from MCOA)
3. Counter parameters follow defaults from PARAMETERS.md:
   - τ₄ = 10.0 years (estimated)
   - n₄* = 50 divisions (hypothetical)
   - β₄ = 1.0 (normalization factor)
   - α₄ = 0.0 (requires experimental determination)

## License

Proprietary - LongevityCommon Project