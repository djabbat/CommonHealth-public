//! mcoa-api — Axum REST + WebSocket server consumed by the Phoenix LiveView frontend.
//!
//! Endpoints (v0.1):
//!   POST /api/simulate           — run a simulation synchronously
//!   GET  /api/counters           — list counters, tissues, and defaults
//!   GET  /healthz                — liveness
//!
//! Future (v0.2):
//!   WS   /ws/stream              — stream long simulations + MCOA-vs-CDATA residuals in real time

use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use mcoa_core::{Counter, Gamma, Tissue};
use mcoa_simulation::{run, SimulationRecord};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct SimulateRequest {
    tissue: String,
    divisions: Option<usize>,
    seconds_per_division: Option<f64>,
}

#[derive(Serialize)]
struct SimulateResponse {
    tissue: String,
    records: Vec<SimulationRecord>,
}

fn parse_tissue(s: &str) -> Option<Tissue> {
    match s {
        "fibroblast" => Some(Tissue::Fibroblast),
        "hsc" => Some(Tissue::Hsc),
        "neuron" => Some(Tissue::Neuron),
        "hepatocyte" => Some(Tissue::Hepatocyte),
        "beta_cell" => Some(Tissue::BetaCell),
        "cd8_t_memory" => Some(Tissue::CD8TMemory),
        _ => None,
    }
}

async fn simulate(
    Json(req): Json<SimulateRequest>,
) -> Result<Json<SimulateResponse>, (StatusCode, String)> {
    let tissue =
        parse_tissue(&req.tissue).ok_or((StatusCode::BAD_REQUEST, format!("bad tissue {}", req.tissue)))?;
    let divisions = req.divisions.unwrap_or(100);
    let seconds = req.seconds_per_division.unwrap_or(604800.0);
    let gamma = Gamma::default();
    let records = run(tissue, divisions, seconds, &gamma);
    Ok(Json(SimulateResponse { tissue: req.tissue, records }))
}

async fn counters() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "counters": Counter::ALL.iter().map(|c| c.as_str()).collect::<Vec<_>>(),
        "tissues": ["fibroblast","hsc","neuron","hepatocyte","beta_cell","cd8_t_memory"],
    }))
}

async fn healthz() -> &'static str { "ok" }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/api/simulate", post(simulate))
        .route("/api/counters", get(counters))
        .route("/healthz", get(healthz));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    tracing::info!("mcoa-api listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
