use axum::{routing::get, Router};
use config::Config;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use biosense_backend::db;
use biosense_backend::error::AppError;
use biosense_backend::routes;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting BioSense backend...");

    // Load configuration
    let config = Config::load().expect("Failed to load configuration");
    info!("Loaded configuration for environment: {}", config.env);

    // Initialize database connection pool
    let pool = db::init_pool(&config.database_url).await?;
    info!("Database connection pool initialized");

    // Run migrations
    db::run_migrations(&pool).await?;
    info!("Database migrations completed");

    // Build application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", routes::api_routes())
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "BioSense backend is running"
}