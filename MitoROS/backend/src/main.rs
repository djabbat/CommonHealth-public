use axum::{routing, Router};
use std::net::SocketAddr;
use tracing_subscriber;
use mitoros_backend::{config::Config, db::DbPool, routes, error::AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting MitoROS backend...");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Loaded configuration, database URL: {}", config.database_url);

    // Initialize database pool
    let db_pool = DbPool::connect(&config.database_url).await?;
    tracing::info!("Database pool initialized");

    // Run migrations
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    tracing::info!("Database migrations applied");

    // Build application routes
    let app = Router::new()
        .route("/health", routing::get(health_check))
        .nest("/api", routes::api_routes())
        .with_state(db_pool)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await
            .map_err(|e| AppError::ServerError(e.to_string()))?,
        app
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .map_err(|e| AppError::ServerError(e.to_string()))?;

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