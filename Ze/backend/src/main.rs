//! ze_backend — axum REST API for Ze Theory simulators.
//!
//!   GET  /health
//!   GET  /api/scenarios
//!   GET  /api/impedance?scenario=novelty&horizon=50
//!   GET  /api/chsh?h=0.5&alpha=0.03&delta=0.05
//!   GET  /api/autowaves?n=200&steps=2000&snapshot_every=200

use std::net::SocketAddr;

use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};
use ze_simulator::{autowaves, chsh, impedance};

const IMPEDANCE_SCENARIOS: &[&str] = &["routine", "novelty", "meditation", "cheating"];

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/scenarios", get(scenarios))
        .route("/api/impedance", get(imp))
        .route("/api/chsh", get(ch))
        .route("/api/autowaves", get(aw))
        .layer(cors);

    let addr: SocketAddr = "127.0.0.1:4001".parse().unwrap();
    tracing::info!("ze_backend listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn scenarios() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "impedance": IMPEDANCE_SCENARIOS,
        "modes": ["impedance", "chsh", "autowaves"],
    }))
}

#[derive(Debug, Deserialize)]
struct ImpQuery {
    #[serde(default)]
    scenario: Option<String>,
    #[serde(default)]
    horizon: Option<f64>,
    #[serde(default)]
    lambda: Option<f64>,
}

async fn imp(Query(q): Query<ImpQuery>) -> impl IntoResponse {
    let mut cfg = impedance::RunConfig::default();
    if let Some(s) = q.scenario {
        if !IMPEDANCE_SCENARIOS.contains(&s.as_str()) {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "unknown scenario", "allowed": IMPEDANCE_SCENARIOS})),
            )
                .into_response();
        }
        if s == "cheating" {
            cfg.cheating_spike = Some((10.0, 0.2));
        }
        cfg.params.scenario = s;
    }
    if let Some(v) = q.horizon {
        if v <= 0.0 || v > 500.0 {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "horizon out of range"})),
            )
                .into_response();
        }
        cfg.t_end = v;
    }
    if let Some(v) = q.lambda {
        cfg.params.lambda = v;
    }
    Json(impedance::simulate(&cfg)).into_response()
}

#[derive(Debug, Deserialize)]
struct ChshQuery {
    #[serde(default)]
    h: Option<f64>,
    #[serde(default)]
    alpha: Option<f64>,
    #[serde(default)]
    delta: Option<f64>,
}

async fn ch(Query(q): Query<ChshQuery>) -> impl IntoResponse {
    let mut p = chsh::Params::default();
    if let Some(v) = q.h {
        p.h = v.clamp(0.0, 1.0);
    }
    if let Some(v) = q.alpha {
        p.alpha = v.clamp(0.0, 1.0);
    }
    if let Some(v) = q.delta {
        p.delta0 = v.clamp(-1.0, 1.0);
    }
    Json(chsh::run(p)).into_response()
}

#[derive(Debug, Deserialize)]
struct AwQuery {
    #[serde(default)]
    n: Option<usize>,
    #[serde(default)]
    steps: Option<usize>,
    #[serde(default)]
    snapshot_every: Option<usize>,
}

async fn aw(Query(q): Query<AwQuery>) -> impl IntoResponse {
    let mut p = autowaves::Params::default();
    if let Some(n) = q.n {
        if n < 10 || n > 1000 {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "n must be 10..1000"})),
            )
                .into_response();
        }
        p.n = n;
    }
    let steps = q.steps.unwrap_or(2000).min(20_000);
    let snap = q.snapshot_every.unwrap_or(steps / 10).max(1);
    Json(autowaves::simulate(p, steps, snap)).into_response()
}
