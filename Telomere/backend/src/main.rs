use axum::{
    Router,
    routing::{get, post, put, delete},
};
use std::net::SocketAddr;
use telomere_backend::config::Config;
use telomere_backend::db::Database;
use telomere_backend::routes;
use telomere_backend::error::AppError;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    compression::CompressionLayer,
};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "telomere_backend=info,tower_http=debug".into()),
        )
        .json()
        .init();

    info!("Starting Telomere Backend (MCOA Counter #2)");

    // Load configuration
    let config = Config::load()?;
    info!("Loaded configuration: {:?}", config);

    // Initialize database pool
    let db = Database::new(&config.database_url).await?;
    info!("Database connection pool initialized");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db.pool)
        .await
        .map_err(|e| {
            error!("Failed to run migrations: {}", e);
            AppError::DatabaseError(e.to_string())
        })?;
    info!("Database migrations completed");

    // Build application routes
    let app = Router::new()
        // Health check
        .route("/health", get(routes::health))
        // Counter registry (MCOA)
        .route("/api/v1/counters", get(routes::list_counters))
        .route("/api/v1/counters/:id", get(routes::get_counter))
        // Telomere measurements
        .route("/api/v1/measurements", 
            get(routes::list_measurements)
            .post(routes::create_measurement)
        )
        .route("/api/v1/measurements/:id",
            get(routes::get_measurement)
            .put(routes::update_measurement)
            .delete(routes::delete_measurement)
        )
        .route("/api/v1/subjects/:subject_id/measurements", 
            get(routes::list_subject_measurements)
        )
        // Telomere parameters
        .route("/api/v1/parameters",
            get(routes::list_parameters)
            .post(routes::create_parameters)
        )
        .route("/api/v1/parameters/:id",
            get(routes::get_parameters)
            .put(routes::update_parameters)
            .delete(routes::delete_parameters)
        )
        .route("/api/v1/subjects/:subject_id/parameters",
            get(routes::get_subject_parameters)
        )
        // Tissue load computation (MCOA)
        .route("/api/v1/compute-tissue-load", post(routes::compute_tissue_load))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::new())
        .with_state(db.clone());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            error!("Server error: {}", e);
            AppError::ServerError(e.to_string())
        })?;

    info!("Server shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    info!("Received shutdown signal");
}