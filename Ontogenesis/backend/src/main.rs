use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use ontogenesis_backend::{
    config::Config,
    db::DbPool,
    routes,
    error::AppError,
};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting Ontogenesis backend v4.2");
    
    // Load configuration
    let config = Config::from_env().map_err(|e| {
        error!("Failed to load config: {}", e);
        AppError::ConfigError(e.to_string())
    })?;
    
    info!("Database URL: {}", config.database_url);
    
    // Initialize database pool
    let db_pool = DbPool::connect(&config.database_url)
        .await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;
    
    info!("Database connection established");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .map_err(|e| {
            error!("Failed to run migrations: {}", e);
            AppError::MigrationError(e.to_string())
        })?;
    
    info!("Database migrations completed");
    
    // Build application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", routes::create_routes())
        .with_state(db_pool)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);
    
    // Run server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        error!("Failed to bind to address: {}", e);
        AppError::ServerError(e.to_string())
    })?;
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            error!("Server error: {}", e);
            AppError::ServerError(e.to_string())
        })
}

async fn health_check() -> &'static str {
    "Ontogenesis backend v4.2 - OK"
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    info!("Shutdown signal received");
}