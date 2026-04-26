use anyhow::Result;
use epigeneticdrift_backend::{
    config::Settings,
    db::DbPool,
    error::AppError,
    routes,
};
use std::net::SocketAddr;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let settings = Settings::new().map_err(|e| {
        error!("Failed to load settings: {}", e);
        e
    })?;

    // Connect to database
    let db_pool = DbPool::connect(&settings.database.url).await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        e
    })?;

    // Run migrations
    db_pool.run_migrations().await.map_err(|e| {
        error!("Failed to run migrations: {}", e);
        e
    })?;

    // Build application with routes
    let app = routes::router()
        .with_state(db_pool)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        error!("Failed to bind to address {}: {}", addr, e);
        e
    })?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C, shutting down"),
        _ = terminate => info!("Received SIGTERM, shutting down"),
    }
}