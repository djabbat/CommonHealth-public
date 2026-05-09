# AUDIT PACKET — LC_server

Path: `/home/oem/Desktop/LongevityCommon/server`  Date: 2026-05-08

## Size & file counts
```
704K	/home/oem/Desktop/LongevityCommon/server
```
**Extensions:** .json=61, .rs=36, .sql=4, .sh=2, .toml=1, (noext)=1, .lock=1, .md=1, .example=1, .service=1
## Tree (depth=2, max 200 entries)
```
.
./migrations
./migrations/001_initial.sql
./migrations/003_health_factors.sql
./migrations/004_add_hrv_sdnn_columns.sql
./migrations/002_otp_attempts_and_indexes.sql
./deploy
./deploy/scripts
./deploy/systemd
./Cargo.toml
./tests
./tests/auth_integration_tests.rs
./tests/ze_compute_tests.rs
./tests/feed_ranker_tests.rs
./src
./src/services
./src/models
./src/db
./src/lib.rs
./src/routes.rs
./src/middleware
./src/config.rs
./src/main.rs
./src/handlers
./build.rs
./Dockerfile
./Cargo.lock
./AUDIT.md
```
## Detected stack: **Rust**
## Core files

### `Cargo.toml` (1775 chars)
```toml
[package]
name = "longevitycommon-server"
version = "0.1.0"
edition = "2021"
authors = ["Jaba Tkemaladze"]
description = "LongevityCommon REST API — longevity social network backend"

[lib]
name = "longevitycommon_server"
path = "src/lib.rs"

[[bin]]
name = "server"
path = "src/main.rs"

[dependencies]
# Web framework
axum = { version = "0.7", features = ["macros", "multipart"] }
tower = { version = "0.4", features = ["limit", "buffer"] }
tower-http = { version = "0.5", features = ["cors", "trace", "compression-gzip"] }
tokio = { version = "1", features = ["full"] }
hyper = { version = "1", features = ["full"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "json"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Auth
jsonwebtoken = "9"
argon2 = "0.5"
rand = "0.8"

# Rate limiting
# tower_governor = { version = "0.8", features = ["axum"] }  # post-MVP: per-IP rate limiting

# Regex (for Ze·Guide DOI/file extraction)
regex-lite = "0.1"

# IDs & time
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# HTTP client (DOI validation, DeepSeek API)
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }

# Config
dotenvy = "0.15"
config = "0.14"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Error handling
thiserror = "1"
anyhow = "1"

# Validation
validator = { version = "0.18", features = ["derive"] }

[build-dependencies]
chrono = "0.4"

[dev-dependencies]
tokio-test = "0.4"
axum-test = "19"
# `util` enables tower::ServiceExt::oneshot used by integration tests.
tower = { version = "0.4", features = ["util"] }
http-body-util = "0.1"

```
### `Dockerfile` (649 chars)
```
FROM rust:1.84-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/server /usr/local/bin/longevitycommon-server
COPY --from=builder /app/migrations /app/migrations
EXPOSE 8080
ENV RUST_LOG=info LONGEVITYCOMMON_VERSION=v5.6
CMD ["longevitycommon-server"]

```
### code `src/main.rs`
```
use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// All modules are defined in lib.rs and re-exported.
// main.rs only contains the binary entry point.
use longevitycommon_server::{AppState, AppConfig, db, routes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "longevitycommon_server=debug,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let config = AppConfig::from_env()?;

    let db = db::connect(&config.database_url).await?;
    db::run_migrations(&db).await?;
    tracing::info!("Database connected and migrations applied");

    let cors = build_cors(&config.allowed_origins);

    let state = AppState { db, config: config.clone() };

    let app = routes::all_routes(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", config.app_host, config.app_port).parse()?;
    tracing::info!("LongevityCommon API listening on {}", addr);
    tracing::info!("Allowed origins: {:?}", config.allowed_origins);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_cors(origins: &[String]) -> CorsLayer {
    use axum::http::{HeaderValue, Method, header};

    let allowed: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|o| HeaderValue::from_str(o).ok())
        .collect();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed))
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true)
}

```
### code `src/lib.rs`
```
// Library entry point — exposes modules for integration tests.
// Binary entry point is src/main.rs.

pub mod config;
pub mod db;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;

pub use config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: AppConfig,
}

```
## Code volume
| ext | files | bytes |
|---|---|---|
| .rs | 36 | 153883 |