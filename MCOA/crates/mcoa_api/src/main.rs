//! mcoa-api — Axum REST + WebSocket server consumed by the Phoenix LiveView frontend.
//!
//! Endpoints (v0.1 + 2026-05-08 additions):
//!   POST /api/simulate                  — run a simulation synchronously
//!   GET  /api/counters                  — list counters, tissues, and defaults
//!   GET  /v1/counters/{id}/D            — compute D_i for given (tissue, n, t)
//!                                          (Phase 1.4 deliverable, 2026-05-08)
//!   GET  /healthz                       — liveness
//!
//! Future (v0.2):
//!   WS   /ws/stream              — stream long simulations + MCOA-vs-CDATA residuals in real time

use axum::{
    extract::{Json, Path, Query},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use mcoa_core::{
    default_drift_rates, default_reference_scales, Counter, Gamma, Tissue, N_COUNTERS,
};
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

// ── /v1/counters/{id}/D — single-counter snapshot (Phase 1.4) ────────

fn parse_counter_id(id: u8) -> Option<Counter> {
    match id {
        1 => Some(Counter::Centriolar),
        2 => Some(Counter::Telomere),
        3 => Some(Counter::Mitochondrial),
        4 => Some(Counter::Epigenetic),
        5 => Some(Counter::Proteostasis),
        _ => None,
    }
}

#[derive(Deserialize)]
struct DQuery {
    tissue:    Option<String>,
    n:         Option<f64>,
    t_seconds: Option<f64>,
    coupling:  Option<f64>,
}

#[derive(Serialize)]
struct DResponse {
    counter_id:   u8,
    counter_name: &'static str,
    tissue:       String,
    n_divisions:  f64,
    t_seconds:    f64,
    d:            f64,
    components:   DComponents,
}

#[derive(Serialize)]
struct DComponents {
    division_term: f64,
    time_term:     f64,
    coupling_term: f64,
    alpha:         f64,
    beta:          f64,
    n_star:        Option<f64>,
    tau_seconds:   f64,
}

/// Compute D_i for counter `id` at (n, t) on `tissue`.
///   D_i = α·(n / n*) + β·(t / τ) + γ·I(others)
async fn counter_d(
    Path(id): Path<u8>,
    Query(q): Query<DQuery>,
) -> Result<Json<DResponse>, (StatusCode, String)> {
    let counter = parse_counter_id(id).ok_or_else(|| {
        (StatusCode::BAD_REQUEST, format!("counter id must be 1..{N_COUNTERS}, got {id}"))
    })?;
    let tissue_name = q.tissue.as_deref().unwrap_or("fibroblast");
    let tissue = parse_tissue(tissue_name)
        .ok_or((StatusCode::BAD_REQUEST, format!("bad tissue {tissue_name}")))?;
    let n         = q.n.unwrap_or(0.0).max(0.0);
    let t_seconds = q.t_seconds.unwrap_or(0.0).max(0.0);
    let coupling  = q.coupling.unwrap_or(0.0).clamp(0.0, 1.0);

    let rates  = default_drift_rates(counter, tissue);
    let scales = default_reference_scales(counter, tissue);

    let division_term = match scales.n_star {
        Some(n_star) if n_star > 0.0 => rates.alpha * (n / n_star),
        _ => 0.0,
    };
    let time_term = if scales.tau_seconds > 0.0 {
        rates.beta * (t_seconds / scales.tau_seconds)
    } else {
        0.0
    };
    let coupling_term = 0.01 * coupling;
    let d = division_term + time_term + coupling_term;

    Ok(Json(DResponse {
        counter_id:   counter.mcoa_number(),
        counter_name: counter.as_str(),
        tissue:       tissue_name.to_string(),
        n_divisions:  n,
        t_seconds,
        d,
        components: DComponents {
            division_term,
            time_term,
            coupling_term,
            alpha:       rates.alpha,
            beta:        rates.beta,
            n_star:      scales.n_star,
            tau_seconds: scales.tau_seconds,
        },
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/api/simulate", post(simulate))
        .route("/api/counters", get(counters))
        .route("/v1/counters/:id/D", get(counter_d))
        .route("/healthz", get(healthz));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    tracing::info!("mcoa-api listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
