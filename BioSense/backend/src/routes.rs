use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use validator::Validate;

use crate::db;
use crate::error::AppError;
use crate::models;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub fn api_routes() -> Router<PgPool> {
    Router::new()
        .route("/devices", get(list_devices).post(create_device))
        .route("/devices/:id", get(get_device).put(update_device).delete(delete_device))
        .route("/eeg_measurements", get(list_eeg_measurements).post(create_eeg_measurement))
        .route("/eeg_measurements/:id", get(get_eeg_measurement))
        .route("/hrv_measurements", get(list_hrv_measurements).post(create_hrv_measurement))
        .route("/hrv_measurements/:id", get(get_hrv_measurement))
        .route("/olfaction_measurements", get(list_olfaction_measurements).post(create_olfaction_measurement))
        .route("/olfaction_measurements/:id", get(get_olfaction_measurement))
        .route("/sessions", get(list_sessions).post(create_session))
        .route("/sessions/:id", get(get_session).put(update_session).delete(delete_session))
}

// Device routes
async fn list_devices(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(50);
    let offset = (page - 1) * per_page;

    let devices = sqlx::query_as::<_, models::Device>(
        "SELECT * FROM devices ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await?;

    Ok(Json(devices))
}

async fn create_device(
    State(pool): State<PgPool>,
    Json(payload): Json<models::CreateDevice>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let device = sqlx::query_as::<_, models::Device>(
        r#"
        INSERT INTO devices (id, name, serial_number, device_type, firmware_version, 
                            hardware_parameters, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(&payload.name)
    .bind(&payload.serial_number)
    .bind(&payload.device_type)
    .bind(&payload.firmware_version)
    .bind(&payload.hardware_parameters)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(device)))
}

async fn get_device(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let device = sqlx::query_as::<_, models::Device>(
        "SELECT * FROM devices WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match device {
        Some(device) => Ok(Json(device)),
        None => Err(AppError::NotFound(format!("Device with id {} not found", id))),
    }
}

async fn update_device(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<models::UpdateDevice>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let device = sqlx::query_as::<_, models::Device>(
        r#"
        UPDATE devices
        SET name = COALESCE($1, name),
            serial_number = COALESCE($2, serial_number),
            device_type = COALESCE($3, device_type),
            firmware_version = COALESCE($4, firmware_version),
            hardware_parameters = COALESCE($5, hardware_parameters),
            updated_at = $6
        WHERE id = $7
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.serial_number)
    .bind(payload.device_type)
    .bind(payload.firmware_version)
    .bind(payload.hardware_parameters)
    .bind(Utc::now())
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match device {
        Some(device) => Ok(Json(device)),
        None => Err(AppError::NotFound(format!("Device with id {} not found", id))),
    }
}

async fn delete_device(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query(
        "DELETE FROM devices WHERE id = $1"
    )
    .bind(id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Device with id {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

// EEG Measurement routes
async fn list_eeg_measurements(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(50);
    let offset = (page - 1) * per_page;

    let measurements = sqlx::query_as::<_, models::EegMeasurement>(
        "SELECT * FROM eeg_measurements ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await?;

    Ok(Json(measurements))
}

async fn create_eeg_measurement(
    State(pool): State<PgPool>,
    Json(payload): Json<models::CreateEegMeasurement>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let measurement = sqlx::query_as::<_, models::EegMeasurement>(
        r#"
        INSERT INTO eeg_measurements (
            id, device_id, session_id, subject_id, recording_started_at, recording_ended_at,
            sampling_rate_hz, channel_labels, channel_data, metadata, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(payload.device_id)
    .bind(payload.session_id)
    .bind(payload.subject_id)
    .bind(payload.recording_started_at)
    .bind(payload.recording_ended_at)
    .bind(payload.sampling_rate_hz)
    .bind(payload.channel_labels)
    .bind(payload.channel_data)
    .bind(payload.metadata)
    .bind(Utc::now())
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(measurement)))
}

async fn get_eeg_measurement(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let measurement = sqlx::query_as::<_, models::EegMeasurement>(
        "SELECT * FROM eeg_measurements WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match measurement {
        Some(measurement) => Ok(Json(measurement)),
        None => Err(AppError::NotFound(format!("EEG measurement with id {} not found", id))),
    }
}

// HRV Measurement routes (similar pattern)
async fn list_hrv_measurements(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(50);
    let offset = (page - 1) * per_page;

    let measurements = sqlx::query_as::<_, models::HrvMeasurement>(
        "SELECT * FROM hrv_measurements ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await?;

    Ok(Json(measurements))
}

async fn create_hrv_measurement(
    State(pool): State<PgPool>,
    Json(payload): Json<models::CreateHrvMeasurement>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let measurement = sqlx::query_as::<_, models::HrvMeasurement>(
        r#"
        INSERT INTO hrv_measurements (
            id, device_id, session_id, subject_id, recording_started_at, recording_ended_at,
            rr_intervals_ms, metadata, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(payload.device_id)
    .bind(payload.session_id)
    .bind(payload.subject_id)
    .bind(payload.recording_started_at)
    .bind(payload.recording_ended_at)
    .bind(payload.rr_intervals_ms)
    .bind(payload.metadata)
    .bind(Utc::now())
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(measurement)))
}

async fn get_hrv_measurement(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let measurement = sqlx::query_as::<_, models::HrvMeasurement>(
        "SELECT * FROM hrv_measurements WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match measurement {
        Some(measurement) => Ok(Json(measurement)),
        None => Err(AppError::NotFound(format!("HRV measurement with id {} not found", id))),
    }
}

// Olfaction Measurement routes
async fn list_olfaction_measurements(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(50);
    let offset = (page - 1) * per_page;

    let measurements = sqlx::query_as::<_, models::OlfactionMeasurement>(
        "SELECT * FROM olfaction_measurements ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await?;

    Ok(Json(measurements))
}

async fn create_olfaction_measurement(
    State(pool): State<PgPool>,
    Json(payload): Json<models::CreateOlfactionMeasurement>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let measurement = sqlx::query_as::<_, models::OlfactionMeasurement>(
        r#"
        INSERT INTO olfaction_measurements (
            id, device_id, session_id, subject_id, recording_started_at,
            sensor_readings, metadata, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(payload.device_id)
    .bind(payload.session_id)
    .bind(payload.subject_id)
    .bind(payload.recording_started_at)
    .bind(payload.sensor_readings)
    .bind(payload.metadata)
    .bind(Utc::now())
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(measurement)))
}

async fn get_olfaction_measurement(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let measurement = sqlx::query_as::<_, models::OlfactionMeasurement>(
        "SELECT * FROM olfaction_measurements WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match measurement {
        Some(measurement) => Ok(Json(measurement)),
        None => Err(AppError::NotFound(format!("Olfaction measurement with id {} not found", id))),
    }
}

// Session routes
async fn list_sessions(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(50);
    let offset = (page - 1) * per_page;

    let sessions = sqlx::query_as::<_, models::Session>(
        "SELECT * FROM sessions ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await?;

    Ok(Json(sessions))
}

async fn create_session(
    State(pool): State<PgPool>,
    Json(payload): Json<models::CreateSession>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let session = sqlx::query_as::<_, models::Session>(
        r#"
        INSERT INTO sessions (id, subject_id, device_id, protocol_type, 
                            environment_conditions, metadata, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(payload.subject_id)
    .bind(payload.device_id)
    .bind(payload.protocol_type)
    .bind(payload.environment_conditions)
    .bind(payload.metadata)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(session)))
}

async fn get_session(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let session = sqlx::query_as::<_, models::Session>(
        "SELECT * FROM sessions WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match session {
        Some(session) => Ok(Json(session)),
        None => Err(AppError::NotFound(format!("Session with id {} not found", id))),
    }
}

async fn update_session(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<models::UpdateSession>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let session = sqlx::query_as::<_, models::Session>(
        r#"
        UPDATE sessions
        SET subject_id = COALESCE($1, subject_id),
            device_id = COALESCE($2, device_id),
            protocol_type = COALESCE($3, protocol_type),
            environment_conditions = COALESCE($4, environment_conditions),
            metadata = COALESCE($5, metadata),
            updated_at = $6
        WHERE id = $7
        RETURNING *
        "#
    )
    .bind(payload.subject_id)
    .bind(payload.device_id)
    .bind(payload.protocol_type)
    .bind(payload.environment_conditions)
    .bind(payload.metadata)
    .bind(Utc::now())
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match session {
        Some(session) => Ok(Json(session)),
        None => Err(AppError::NotFound(format!("Session with id {} not found", id))),
    }
}

async fn delete_session(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query(
        "DELETE FROM sessions WHERE id = $1"
    )
    .bind(id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Session with id {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}