use axum::{
    routing::{get, post, put, delete},
    Router,
};
use hap_backend::{
    config::Config,
    db::DbPool,
    error::AppError,
    routes::{
        taxa, steroids, parameters, bhcacriteria, experiments, knowledges,
        biomarkers, counter_registry,
    },
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting HAP backend...");

    // Load configuration
    let config = Config::from_env().map_err(|e| {
        tracing::error!("Failed to load config: {}", e);
        AppError::ConfigError(e.to_string())
    })?;

    // Initialize database connection pool
    let pool = DbPool::connect(&config.database_url).await.map_err(|e| {
        tracing::error!("Failed to connect to database: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to run migrations: {}", e);
            AppError::MigrationError(e.to_string())
        })?;

    tracing::info!("Database migrations completed");

    // Build application with routes
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/taxa", get(taxa::list_taxa).post(taxa::create_taxon))
        .route(
            "/api/taxa/:id",
            get(taxa::get_taxon)
                .put(taxa::update_taxon)
                .delete(taxa::delete_taxon),
        )
        .route("/api/steroids", get(steroids::list_steroids).post(steroids::create_steroid))
        .route(
            "/api/steroids/:id",
            get(steroids::get_steroid)
                .put(steroids::update_steroid)
                .delete(steroids::delete_steroid),
        )
        .route("/api/parameters", get(parameters::list_parameters).post(parameters::create_parameter))
        .route(
            "/api/parameters/:id",
            get(parameters::get_parameter)
                .put(parameters::update_parameter)
                .delete(parameters::delete_parameter),
        )
        .route("/api/bhcacriteria", get(bhcacriteria::list_bhcacriteria).post(bhcacriteria::create_bhcacriterion))
        .route(
            "/api/bhcacriteria/:id",
            get(bhcacriteria::get_bhcacriterion)
                .put(bhcacriteria::update_bhcacriterion)
                .delete(bhcacriteria::delete_bhcacriterion),
        )
        .route("/api/experiments", get(experiments::list_experiments).post(experiments::create_experiment))
        .route(
            "/api/experiments/:id",
            get(experiments::get_experiment)
                .put(experiments::update_experiment)
                .delete(experiments::delete_experiment),
        )
        .route("/api/knowledges", get(knowledges::list_knowledges).post(knowledges::create_knowledge))
        .route(
            "/api/knowledges/:id",
            get(knowledges::get_knowledge)
                .put(knowledges::update_knowledge)
                .delete(knowledges::delete_knowledge),
        )
        .route("/api/biomarkers", get(biomarkers::list_biomarkers).post(biomarkers::create_biomarker))
        .route(
            "/api/biomarkers/:id",
            get(biomarkers::get_biomarker)
                .put(biomarkers::update_biomarker)
                .delete(biomarkers::delete_biomarker),
        )
        .route("/api/counter/registry", get(counter_registry::get_registry))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    // axum 0.7+ uses tokio TcpListener + axum::serve (Server removed)
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| AppError::ServerError(e.to_string()))?;
    axum::serve(listener, app)
        .await
        .map_err(|e| {
            tracing::error!("Server error: {}", e);
            AppError::ServerError(e.to_string())
        })?;

    Ok(())
}