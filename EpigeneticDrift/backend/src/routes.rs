use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{
        EpigeneticDriftCounter, EpigeneticDriftMeasurement, EpigeneticDriftParameters,
        NewEpigeneticDriftCounter, NewEpigeneticDriftMeasurement, NewEpigeneticDriftParameters,
    },
};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: String,
    timestamp: DateTime<Utc>,
}

pub async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
    })
}

// EpigeneticDriftCounter routes
async fn list_counters(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EpigeneticDriftCounter>>, AppError> {
    let counters = EpigeneticDriftCounter::list(&pool).await?;
    Ok(Json(counters))
}

async fn get_counter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EpigeneticDriftCounter>, AppError> {
    let counter = EpigeneticDriftCounter::find_by_id(&pool, id).await?;
    Ok(Json(counter))
}

async fn create_counter(
    State(pool): State<PgPool>,
    Json(new_counter): Json<NewEpigeneticDriftCounter>,
) -> Result<Json<EpigeneticDriftCounter>, AppError> {
    let counter = EpigeneticDriftCounter::create(&pool, new_counter).await?;
    Ok(Json(counter))
}

async fn update_counter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<NewEpigeneticDriftCounter>,
) -> Result<Json<EpigeneticDriftCounter>, AppError> {
    let counter = EpigeneticDriftCounter::update(&pool, id, update).await?;
    Ok(Json(counter))
}

async fn delete_counter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    EpigeneticDriftCounter::delete(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// EpigeneticDriftMeasurement routes
async fn list_measurements(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EpigeneticDriftMeasurement>>, AppError> {
    let measurements = EpigeneticDriftMeasurement::list(&pool).await?;
    Ok(Json(measurements))
}

async fn get_measurement(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EpigeneticDriftMeasurement>, AppError> {
    let measurement = EpigeneticDriftMeasurement::find_by_id(&pool, id).await?;
    Ok(Json(measurement))
}

async fn create_measurement(
    State(pool): State<PgPool>,
    Json(new_measurement): Json<NewEpigeneticDriftMeasurement>,
) -> Result<Json<EpigeneticDriftMeasurement>, AppError> {
    let measurement = EpigeneticDriftMeasurement::create(&pool, new_measurement).await?;
    Ok(Json(measurement))
}

async fn update_measurement(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<NewEpigeneticDriftMeasurement>,
) -> Result<Json<EpigeneticDriftMeasurement>, AppError> {
    let measurement = EpigeneticDriftMeasurement::update(&pool, id, update).await?;
    Ok(Json(measurement))
}

async fn delete_measurement(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    EpigeneticDriftMeasurement::delete(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// EpigeneticDriftParameters routes
async fn list_parameters(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EpigeneticDriftParameters>>, AppError> {
    let parameters = EpigeneticDriftParameters::list(&pool).await?;
    Ok(Json(parameters))
}

async fn get_parameters(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EpigeneticDriftParameters>, AppError> {
    let parameters = EpigeneticDriftParameters::find_by_id(&pool, id).await?;
    Ok(Json(parameters))
}

async fn create_parameters(
    State(pool): State<PgPool>,
    Json(new_parameters): Json<NewEpigeneticDriftParameters>,
) -> Result<Json<EpigeneticDriftParameters>, AppError> {
    let parameters = EpigeneticDriftParameters::create(&pool, new_parameters).await?;
    Ok(Json(parameters))
}

async fn update_parameters(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(update): Json<NewEpigeneticDriftParameters>,
) -> Result<Json<EpigeneticDriftParameters>, AppError> {
    let parameters = EpigeneticDriftParameters::update(&pool, id, update).await?;
    Ok(Json(parameters))
}

async fn delete_parameters(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    EpigeneticDriftParameters::delete(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/health", get(health_check))
        // Counter routes
        .route("/counters", get(list_counters).post(create_counter))
        .route("/counters/:id", get(get_counter).put(update_counter).delete(delete_counter))
        // Measurement routes
        .route("/measurements", get(list_measurements).post(create_measurement))
        .route("/measurements/:id", get(get_measurement).put(update_measurement).delete(delete_measurement))
        // Parameters routes
        .route("/parameters", get(list_parameters).post(create_parameters))
        .route("/parameters/:id", get(get_parameters).put(update_parameters).delete(delete_parameters))
}