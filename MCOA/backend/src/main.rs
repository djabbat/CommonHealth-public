use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use mcoa_backend::config::Config;
use mcoa_backend::db::DbPool;
use mcoa_backend::error::AppResult;
use mcoa_backend::routes;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "mcoa_backend=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting MCOA backend...");

    // Load configuration
    let config = Config::load()?;
    tracing::debug!("Loaded config: {:?}", config);

    // Initialize database pool
    let db_pool = DbPool::connect(&config.database_url).await?;
    tracing::info!("Database connection established");

    // Apply migrations
    db_pool.run_migrations().await?;
    tracing::info!("Database migrations applied");

    // Build application
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(routes::counter_routes())
        .merge(routes::tissue_routes())
        .merge(routes::subject_routes())
        .merge(routes::damage_measurement_routes())
        .merge(routes::tissue_load_routes())
        .merge(routes::coupling_matrix_routes())
        .with_state(db_pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("Shutdown signal received");
}