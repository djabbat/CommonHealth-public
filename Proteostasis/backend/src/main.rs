use axum::{
    Router,
    routing::{get, post, put, delete},
};
use proteostasis_backend::config::Config;
use proteostasis_backend::db::get_pool;
use proteostasis_backend::routes;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let config = Config::from_env()?;
    let pool = get_pool(&config.database_url).await?;
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/proteostasis/parameters", get(routes::list_parameters))
        .route("/proteostasis/parameters/:id", get(routes::get_parameter))
        .route("/proteostasis/parameters", post(routes::create_parameter))
        .route("/proteostasis/parameters/:id", put(routes::update_parameter))
        .route("/proteostasis/parameters/:id", delete(routes::delete_parameter))
        .route("/proteostasis/time_series", get(routes::list_time_series))
        .route("/proteostasis/time_series/:id", get(routes::get_time_series))
        .route("/proteostasis/time_series", post(routes::create_time_series))
        .route("/proteostasis/time_series/:id", put(routes::update_time_series))
        .route("/proteostasis/time_series/:id", delete(routes::delete_time_series))
        .route("/proteostasis/compute", post(routes::compute_damage))
        .with_state(pool);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Starting server on {}", addr);
    
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
        .expect("Failed to install CTRL+C handler");
    tracing::info!("Shutting down gracefully...");
}