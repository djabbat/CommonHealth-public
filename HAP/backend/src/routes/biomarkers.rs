use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::biomarker::{Biomarker, CreateBiomarker, UpdateBiomarker},
};

pub async fn list_biomarkers(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Biomarker>>, AppError> {
    let biomarkers = sqlx::query_as::<_, Biomarker>(
        r#"
        SELECT * FROM biomarkers
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch biomarkers: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    Ok(Json(biomarkers))
}

pub async fn get_biomarker(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Biomarker>, AppError> {
    let biomarker = sqlx::query_as::<_, Biomarker>(
        r#"
        SELECT * FROM biomarkers
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch biomarker {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Biomarker {} not found", id);
        AppError::NotFound(format!("Biomarker {} not found", id))
    })?;

    Ok(Json(biomarker))
}

pub async fn create_biomarker(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateBiomarker>,
) -> Result<Json<Biomarker>, AppError> {
    let biomarker = sqlx::query_as::<_, Biomarker>(
        r#"
        INSERT INTO biomarkers (
            name, biomarker_type, hepatic_affective_joint,
            measurement_method, normal_range, unit
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.biomarker_type)
    .bind(payload.hepatic_affective_joint)
    .bind(payload.measurement_method)
    .bind(payload.normal_range)
    .bind(payload.unit)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create biomarker: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    tracing::info!("Created biomarker: {}", biomarker.id);
    Ok(Json(biomarker))
}

pub async fn update_biomarker(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBiomarker>,
) -> Result<Json<Biomarker>, AppError> {
    let biomarker = sqlx::query_as::<_, Biomarker>(
        r#"
        UPDATE biomarkers
        SET
            name = COALESCE($1, name),
            biomarker_type = COALESCE($2, biomarker_type),
            hepatic_affective_joint = COALESCE($3, hepatic_affective_joint),
            measurement_method = COALESCE($4, measurement_method),
            normal_range = COALESCE($5, normal_range),
            unit = COALESCE($6, unit),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $7 AND deleted_at IS NULL
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.biomarker_type)
    .bind(payload.hepatic_affective_joint)
    .bind(payload.measurement_method)
    .bind(payload.normal_range)
    .bind(payload.unit)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update biomarker {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Biomarker {} not found for update", id);
        AppError::NotFound(format!("Biomarker {} not found", id))
    })?;

    tracing::info!("Updated biomarker: {}", id);
    Ok(Json(biomarker))
}

pub async fn delete_biomarker(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE biomarkers
        SET deleted_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete biomarker {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    if result.rows_affected() == 0 {
        tracing::warn!("Biomarker {} not found for deletion", id);
        return Err(AppError::NotFound(format!("Biomarker {} not found", id)));
    }

    tracing::info!("Soft deleted biomarker: {}", id);
    Ok(Json(()))
}