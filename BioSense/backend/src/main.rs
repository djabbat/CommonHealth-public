//! biosense-backend — Rust HTTP server для χ_Ze биомаркера.
//!
//! Endpoints (called by `biosense-web` Phoenix LiveView via BackendClient):
//!     GET  /healthz                     — liveness probe
//!     POST /chi_ze                      — body {v_eeg,v_hrv,v_resp,v_sleep}
//!                                          → {chi_ze, components}
//!     POST /bridge                      — body {d}
//!                                          → {chi_ze} (CDATA bridge stub)
//!     POST /exacerbation                — body {age,sex,chi_now,chi_7d}
//!                                          → {risk, level}
//!     GET  /api/v_star                  — return canonical v* (Article form)
//!
//! Port: 127.0.0.1:4502 (decided 2026-05-07; nginx biosense.longevity.ge
//! already maps /api/ → :4502 and /,/live/ → :4501 biosense-web).
//!
//! Reference: ~/Desktop/LongevityCommon/BioSense/CONCEPT.md.

use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Canonical v*_active. Sourced from `lc-shared-types` to close cross-project
/// drift (audit P0 #6, 2026-05-08).  Python form is used for internal
/// computation; Article form is the canonical inter-subproject value
/// (root PARAMETERS.md § 1).
const V_STAR_ACTIVE_PY: f64 = lc_shared_types::ze::V_STAR_ACTIVE_PYTHON;
const V_STAR_ACTIVE_ARTICLE: f64 = lc_shared_types::ze::V_STAR_ACTIVE_ARTICLE;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".parse().unwrap()),
        )
        .init();

    // Routes are mounted at BOTH `/<name>` (legacy short form) and
    // `/api/<name>` (matches nginx `biosense.longevity.ge.conf` /api/ →
    // :4502/api/). Phoenix biosense-web client uses /api/ prefix, the
    // dropped Docker container `deploy-biosense-backend-1` did the same.
    let app = Router::new()
        .route("/healthz",         get(healthz))
        .route("/api/healthz",     get(healthz))
        .route("/chi_ze",          post(chi_ze))
        .route("/api/chi_ze",      post(chi_ze))
        .route("/bridge",          post(bridge))
        .route("/api/bridge",      post(bridge))
        .route("/exacerbation",    post(exacerbation))
        .route("/api/exacerbation", post(exacerbation))
        .route("/api/v_star",      get(get_v_star))
        .route("/v_star",          get(get_v_star));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4502);
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let addr: SocketAddr = format!("{host}:{port}").parse().expect("HOST:PORT parse");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    tracing::info!(%addr, "biosense-backend listening");
    axum::serve(listener, app).await.expect("serve");
}

// ── handlers ─────────────────────────────────────────────────────

async fn healthz() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "ts": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    }))
}

async fn get_v_star() -> impl IntoResponse {
    Json(serde_json::json!({
        "v_star_active": {
            "article": V_STAR_ACTIVE_ARTICLE,
            "python": V_STAR_ACTIVE_PY,
            "canonical": "article",
        },
        "source": "root PARAMETERS.md § 1 (decided 2026-05-07)",
    }))
}

/// Accept both wire-format conventions (Phase 4.4 reconciliation 2026-05-08):
/// - `v_eeg` / `v_hrv` / `v_resp` / `v_sleep`  — idiomatic Rust (this crate's
///   original shape).
/// - `eeg`   / `hrv`   / `resp`   / `sleep`     — what the live Docker
///   `deploy-biosense-backend-1` container has been receiving from the
///   Phoenix `biosense-web` client.
///
/// `serde(alias)` lets us drop the container without breaking the client.
#[derive(Deserialize)]
struct ChiZeRequest {
    #[serde(alias = "eeg")]
    v_eeg: f64,
    #[serde(alias = "hrv")]
    v_hrv: f64,
    #[serde(alias = "resp")]
    v_resp: f64,
    #[serde(alias = "sleep")]
    v_sleep: f64,
}

#[derive(Serialize)]
struct ChiZeResponse {
    chi_ze: f64,
    components: ChiZeComponents,
    v_star_active: f64,
    convention: &'static str,
}

#[derive(Serialize)]
struct ChiZeComponents {
    chi_eeg: f64,
    chi_hrv: f64,
    chi_resp: f64,
    chi_sleep: f64,
}

/// χ_Ze per modality is `1 − |v − v*| / max(v*, 1 − v*)`, then composed
/// with weights from `BioSense/PARAMETERS.md § "χ_Ze composition weights"`
/// (default 0.30/0.30/0.20/0.20). Returns BOTH article and python forms
/// of v*; chi_ze itself is convention-neutral (depends only on |v−v*|).
async fn chi_ze(Json(req): Json<ChiZeRequest>) -> Result<Json<ChiZeResponse>, AppError> {
    let v_star = V_STAR_ACTIVE_PY; // input vs are in Python form
    let chi = |v: f64| chi_ze_single(v, v_star);
    let chi_eeg = chi(req.v_eeg);
    let chi_hrv = chi(req.v_hrv);
    let chi_resp = chi(req.v_resp);
    let chi_sleep = chi(req.v_sleep);
    // Default weights from BioSense/PARAMETERS § 2.
    let composed = 0.30 * chi_eeg + 0.30 * chi_hrv + 0.20 * chi_resp + 0.20 * chi_sleep;
    Ok(Json(ChiZeResponse {
        chi_ze: composed,
        components: ChiZeComponents { chi_eeg, chi_hrv, chi_resp, chi_sleep },
        v_star_active: V_STAR_ACTIVE_ARTICLE,
        convention: "article",
    }))
}

#[derive(Deserialize)]
struct BridgeRequest {
    d: f64,
}

#[derive(Serialize)]
struct BridgeResponse {
    chi_ze: f64,
    d: f64,
    note: &'static str,
}

/// CDATA → χ_Ze bridge — placeholder polynomial stub.
/// Real bridge constants (a, b, c, g₀, g₁) live in root PARAMETERS § 3
/// and are flagged as "underpowered fit (5 params on N=196)". This stub
/// returns a clamped sigmoid: χ ≈ 1 − tanh(d). Replace when bridge fit
/// is upgraded with new dataset.
async fn bridge(Json(req): Json<BridgeRequest>) -> Result<Json<BridgeResponse>, AppError> {
    let d = req.d.max(0.0);
    let chi = 1.0 - d.tanh();
    Ok(Json(BridgeResponse {
        chi_ze: chi,
        d,
        note: "bridge is stub; underpowered constants in root PARAMETERS §3",
    }))
}

#[derive(Deserialize)]
struct ExacerbationRequest {
    age: f64,
    sex: String,    // "M" or "F"
    chi_now: f64,
    chi_7d: f64,    // 7-day moving average
}

#[derive(Serialize)]
struct ExacerbationResponse {
    risk: f64,
    level: &'static str, // "low" | "medium" | "high"
    drop_pct: f64,
}

/// Exacerbation risk — placeholder. Computes 7-day χ drop, scales with
/// age, applies modest sex offset. NOT a clinical model — replace with
/// validated logistic per BioSense/CONCEPT.md once cohort N≥2000 ready.
async fn exacerbation(
    Json(req): Json<ExacerbationRequest>,
) -> Result<Json<ExacerbationResponse>, AppError> {
    let drop_pct = if req.chi_7d.abs() < 1e-9 {
        0.0
    } else {
        ((req.chi_7d - req.chi_now) / req.chi_7d.max(1e-9)).max(0.0)
    };
    let age_factor = (req.age / 80.0).clamp(0.0, 1.5);
    let sex_factor = if req.sex.eq_ignore_ascii_case("M") { 1.05 } else { 1.0 };
    let risk = (drop_pct * age_factor * sex_factor).clamp(0.0, 1.0);
    let level = if risk < 0.15 {
        "low"
    } else if risk < 0.40 {
        "medium"
    } else {
        "high"
    };
    Ok(Json(ExacerbationResponse {
        risk,
        level,
        drop_pct,
    }))
}

// ── helpers ──────────────────────────────────────────────────────

fn chi_ze_single(v: f64, v_star: f64) -> f64 {
    let denom = v_star.max(1.0 - v_star);
    if denom <= 0.0 {
        return 0.0;
    }
    let raw = 1.0 - (v - v_star).abs() / denom;
    raw.clamp(0.0, 1.0)
}

// ── error type ───────────────────────────────────────────────────

#[derive(Debug)]
enum AppError {
    BadInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::BadInput(msg) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": msg, "status": 400})),
            )
                .into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chi_ze_request_accepts_both_field_conventions() {
        // Idiomatic v_eeg shape:
        let v: ChiZeRequest = serde_json::from_str(
            r#"{"v_eeg":0.45,"v_hrv":0.46,"v_resp":0.44,"v_sleep":0.47}"#,
        )
        .unwrap();
        assert_eq!(v.v_eeg, 0.45);
        // Legacy `eeg` shape (Docker container compat):
        let v: ChiZeRequest = serde_json::from_str(
            r#"{"eeg":0.45,"hrv":0.46,"resp":0.44,"sleep":0.47}"#,
        )
        .unwrap();
        assert_eq!(v.v_eeg, 0.45);
        assert_eq!(v.v_sleep, 0.47);
    }

    #[test]
    fn chi_ze_single_perfect_match() {
        let chi = chi_ze_single(V_STAR_ACTIVE_PY, V_STAR_ACTIVE_PY);
        assert!((chi - 1.0).abs() < 1e-9);
    }

    #[test]
    fn chi_ze_single_far_from_v_star() {
        // v=0 → far from 0.456; chi should be small
        let chi = chi_ze_single(0.0, V_STAR_ACTIVE_PY);
        assert!(chi >= 0.0 && chi < 0.2);
    }

    #[test]
    fn chi_ze_single_clamped_to_unit_interval() {
        for v in [-1.0, 0.0, 0.5, 1.0, 2.0] {
            let chi = chi_ze_single(v, V_STAR_ACTIVE_PY);
            assert!(
                (0.0..=1.0).contains(&chi),
                "chi out of range for v={v}: {chi}"
            );
        }
    }

    #[test]
    fn v_star_python_to_article_conversion_correct() {
        // article = 2 · python - 1
        let derived = 2.0 * V_STAR_ACTIVE_PY - 1.0;
        assert!((derived - V_STAR_ACTIVE_ARTICLE).abs() < 1e-4);
    }

    #[test]
    fn weighted_chi_ze_composition_is_convex() {
        // If all per-modality chi=1, composed must be 1.
        let composed: f64 = 0.30 + 0.30 + 0.20 + 0.20;
        assert!((composed - 1.0_f64).abs() < 1e-9);
    }

    #[test]
    fn exacerbation_drop_pct_zero_when_chi_constant() {
        // chi_now == chi_7d → drop_pct == 0 → risk == 0.
        let req = ExacerbationRequest {
            age: 50.0,
            sex: "F".to_string(),
            chi_now: 0.5,
            chi_7d: 0.5,
        };
        let r = futures_test_block_on(async { exacerbation(Json(req)).await.unwrap().0 });
        assert_eq!(r.drop_pct, 0.0);
        assert_eq!(r.level, "low");
    }

    #[test]
    fn bridge_stub_at_zero_d_returns_one() {
        let req = BridgeRequest { d: 0.0 };
        let r = futures_test_block_on(async { bridge(Json(req)).await.unwrap().0 });
        assert!((r.chi_ze - 1.0).abs() < 1e-9);
    }

    fn futures_test_block_on<F: std::future::Future<Output = T>, T>(f: F) -> T {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(f)
    }
}
