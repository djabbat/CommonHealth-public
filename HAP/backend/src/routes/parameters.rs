use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::parameter::{Parameter, CreateParameter, UpdateParameter},
};

pub async fn list_parameters(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Parameter>>, AppError> {
    let parameters = sqlx::query_as::<_, Parameter>(
        r#"
        SELECT * FROM parameters
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch parameters: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    Ok(Json(parameters))
}

pub async fn get_parameter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Parameter>, AppError> {
    let parameter = sqlx::query_as::<_, Parameter>(
        r#"
        SELECT * FROM parameters
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch parameter {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Parameter {} not found", id);
        AppError::NotFound(format!("Parameter {} not found", id))
    })?;

    Ok(Json(parameter))
}

pub async fn create_parameter(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateParameter>,
) -> Result<Json<Parameter>, AppError> {
    let parameter = sqlx::query_as::<_, Parameter>(
        r#"
        INSERT INTO parameters (
            name, value, unit, source,
            justification, status
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.value)
    .bind(payload.unit)
    .bind(payload.source)
    .bind(payload.justification)
    .bind(payload.status)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create parameter: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    tracing::info!("Created parameter: {}", parameter.id);
    Ok(Json(parameter))
}

pub async fn update_parameter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateParameter>,
) -> Result<Json<Parameter>, AppError> {
    let parameter = sqlx::query_as::<_, Parameter>(
        r#"
        UPDATE parameters
        SET
            name = COALESCE($1, name),
            value = COALESCE($2, value),
            unit = COALESCE($3, unit),
            source = COALESCE($4, source),
            justification = COALESCE($5, justification),
            status = COALESCE($6, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $7 AND deleted_at IS NULL
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.value)
    .bind(payload.unit)
    .bind(payload.source)
    .bind(payload.justification)
    .bind(payload.status)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update parameter {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Parameter {} not found for update", id);
        AppError::NotFound(format!("Parameter {} not found", id))
    })?;

    tracing::info!("Updated parameter: {}", id);
    Ok(Json(parameter))
}

pub async fn delete_parameter(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE parameters
        SET deleted_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete parameter {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    if result.rows_affected() == 0 {
        tracing::warn!("Parameter {} not found for deletion", id);
        return Err(AppError::NotFound(format!("Parameter {} not found", id)));
    }

    tracing::info!("Soft deleted parameter: {}", id);
    Ok(Json(()))
}