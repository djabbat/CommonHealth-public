use axum::{
    extract::{Path, Query, State},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;
use std::collections::HashMap;

use crate::{
    db::Database,
    models::{
        TelomereMeasurement, TelomereParameters, CounterRegistry,
        TissueLoadRequest, TissueLoadResponse,
        MeasurementQuery, Pagination,
    },
    error::{AppError, AppResult},
};

// Health check endpoint
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "status": "healthy",
        "service": "telomere_backend",
        "version": env!("CARGO_PKG_VERSION"),
        "counter_id": 2,
        "counter_name": "Telomere Shortening Counter"
    })))
}

// Counter registry endpoints (MCOA)
pub async fn list_counters() -> AppResult<Json<Vec<CounterRegistry>>> {
    let counters = vec![
        CounterRegistry {
            id: 2,
            name: "Telomere Shortening Counter".to_string(),
            symbol: "D₂".to_string(),
            description: "Progressive loss of telomeric DNA repeats at chromosome ends, functioning as a mitotic clock and stress integrator".to_string(),
            equation: "D₂(n, t) = D₂,₀ + α₂·(n / n₂*) + β₂·(t / τ₂)".to_string(),
            parameters: vec![
                "D₂,₀".to_string(),
                "α₂".to_string(),
                "β₂".to_string(),
                "n₂*".to_string(),
                "τ₂".to_string(),
            ],
            units: "bp (base pairs)".to_string(),
            coupling: vec![
                ("Γ₂,₁".to_string(), "Centriolar aberrations → Telomere shortening".to_string()),
                ("Γ₂,₃".to_string(), "MitoROS → Telomere shortening (oxidative damage)".to_string()),
                ("Γ₂,₄".to_string(), "Epigenetic drift → Telomere shortening".to_string()),
                ("Γ₂,₅".to_string(), "Proteostasis → Telomere dysfunction".to_string()),
            ],
            default_gamma: 0.0,
        }
    ];
    Ok(Json(counters))
}

pub async fn get_counter(Path(id): Path<i32>) -> AppResult<Json<CounterRegistry>> {
    if id == 2 {
        Ok(Json(CounterRegistry {
            id: 2,
            name: "Telomere Shortening Counter".to_string(),
            symbol: "D₂".to_string(),
            description: "Progressive loss of telomeric DNA repeats at chromosome ends, functioning as a mitotic clock and stress integrator".to_string(),
            equation: "D₂(n, t) = D₂,₀ + α₂·(n / n₂*) + β₂·(t / τ₂)".to_string(),
            parameters: vec![
                "D₂,₀".to_string(),
                "α₂".to_string(),
                "β₂".to_string(),
                "n₂*".to_string(),
                "τ₂".to_string(),
            ],
            units: "bp (base pairs)".to_string(),
            coupling: vec![
                ("Γ₂,₁".to_string(), "Centriolar aberrations → Telomere shortening".to_string()),
                ("Γ₂,₃".to_string(), "MitoROS → Telomere shortening (oxidative damage)".to_string()),
                ("Γ₂,₄".to_string(), "Epigenetic drift → Telomere shortening".to_string()),
                ("Γ₂,₅".to_string(), "Proteostasis → Telomere dysfunction".to_string()),
            ],
            default_gamma: 0.0,
        }))
    } else {
        Err(AppError::NotFound(format!("Counter with id {} not found", id)))
    }
}

// Telomere measurement endpoints
pub async fn list_measurements(
    State(db): State<Database>,
    Query(query): Query<MeasurementQuery>,
    Query(pagination): Query<Pagination>,
) -> AppResult<Json<Vec<TelomereMeasurement>>> {
    let measurements = db.get_measurements(&query, &pagination).await?;
    Ok(Json(measurements))
}

pub async fn get_measurement(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<TelomereMeasurement>> {
    let measurement = db.get_measurement(id).await?;
    Ok(Json(measurement))
}

pub async fn create_measurement(
    State(db): State<Database>,
    Json(mut measurement): Json<TelomereMeasurement>,
) -> AppResult<Json<TelomereMeasurement>> {
    measurement.validate()?;
    let measurement = db.create_measurement(measurement).await?;
    Ok(Json(measurement))
}

pub async fn update_measurement(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(mut measurement): Json<TelomereMeasurement>,
) -> AppResult<Json<TelomereMeasurement>> {
    measurement.id = id;
    measurement.validate()?;
    let measurement = db.update_measurement(measurement).await?;
    Ok(Json(measurement))
}

pub async fn delete_measurement(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    db.delete_measurement(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_subject_measurements(
    State(db): State<Database>,
    Path(subject_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> AppResult<Json<Vec<TelomereMeasurement>>> {
    let query = MeasurementQuery {
        subject_id: Some(subject_id),
        ..Default::default()
    };
    let measurements = db.get_measurements(&query, &pagination).await?;
    Ok(Json(measurements))
}

// Telomere parameters endpoints
pub async fn list_parameters(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
) -> AppResult<Json<Vec<TelomereParameters>>> {
    let parameters = db.get_parameters(&pagination).await?;
    Ok(Json(parameters))
}

pub async fn get_parameters(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<TelomereParameters>> {
    let parameters = db.get_parameters_by_id(id).await?;
    Ok(Json(parameters))
}

pub async fn create_parameters(
    State(db): State<Database>,
    Json(mut parameters): Json<TelomereParameters>,
) -> AppResult<Json<TelomereParameters>> {
    parameters.validate()?;
    let parameters = db.create_parameters(parameters).await?;
    Ok(Json(parameters))
}

pub async fn update_parameters(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(mut parameters): Json<TelomereParameters>,
) -> AppResult<Json<TelomereParameters>> {
    parameters.id = id;
    parameters.validate()?;
    let parameters = db.update_parameters(parameters).await?;
    Ok(Json(parameters))
}

pub async fn delete_parameters(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    db.delete_parameters(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_subject_parameters(
    State(db): State<Database>,
    Path(subject_id): Path<Uuid>,
) -> AppResult<Json<TelomereParameters>> {
    let parameters = db.get_parameters_by_subject(subject_id).await?;
    Ok(Json(parameters))
}

// Tissue load computation (MCOA)
pub async fn compute_tissue_load(
    State(db): State<Database>,
    Json(request): Json<TissueLoadRequest>,
) -> AppResult<Json<TissueLoadResponse>> {
    // Fetch latest measurement for subject
    let query = MeasurementQuery {
        subject_id: Some(request.subject_id),
        ..Default::default()
    };
    let pagination = Pagination {
        limit: Some(1),
        offset: Some(0),
        order_by: Some("measured_at DESC".to_string()),
    };
    
    let measurements = db.get_measurements(&query, &pagination).await?;
    
    if measurements.is_empty() {
        return Err(AppError::NotFound("No measurements found for subject".to_string()));
    }
    
    let measurement = &measurements[0];
    
    // Fetch parameters (subject-specific or default)
    let parameters = match db.get_parameters_by_subject(request.subject_id).await {
        Ok(params) => params,
        Err(_) => {
            // Use default parameters if none exist
            TelomereParameters::default()
        }
    };
    
    // Compute D₂ using the kinetic equation
    let d2 = compute_d2(measurement, &parameters);
    
    // Get tissue-specific weight
    let weight = get_tissue_weight(&request.tissue_type);
    
    // Apply scaling function f₂ (linear for scaffold)
    let tissue_load = weight * d2;
    
    let response = TissueLoadResponse {
        subject_id: request.subject_id,
        tissue_type: request.tissue_type,
        measurement_id: measurement.id,
        d2_value: d2,
        tissue_weight: weight,
        tissue_load,
        computed_at: Utc::now(),
        parameters_used: parameters.id,
    };
    
    Ok(Json(response))
}

// Helper functions
fn compute_d2(measurement: &TelomereMeasurement, parameters: &TelomereParameters) -> f64 {
    // D₂(n, t) = D₂,₀ + α₂·(n / n₂*) + β₂·(t / τ₂)
    // n: population doublings, t: time in years
    
    let n = measurement.population_doublings.unwrap_or(0.0);
    let t = measurement.time_elapsed_years.unwrap_or(0.0);
    
    parameters.d2_baseline 
        + parameters.alpha2 * (n / parameters.n2_star)
        + parameters.beta2 * (t / parameters.tau2)
}

fn get_tissue_weight(tissue_type: &str) -> f64 {
    // Default tissue weights based on MCOA framework
    // These should be calibrated per tissue type
    match tissue_type.to_lowercase().as_str() {
        "blood" | "leukocyte" => 0.8,
        "fibroblast" | "skin" => 0.7,
        "liver" => 0.6,
        "brain" => 0.5,
        "muscle" => 0.4,
        "bone_marrow" => 0.9,
        _ => 0.5, // Default weight
    }
}