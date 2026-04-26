use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::steroid::{Steroid, CreateSteroid, UpdateSteroid},
};

pub async fn list_steroids(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Steroid>>, AppError> {
    let steroids = sqlx::query_as::<_, Steroid>(
        r#"
        SELECT * FROM steroids
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch steroids: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    Ok(Json(steroids))
}

pub async fn get_steroid(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Steroid>, AppError> {
    let steroid = sqlx::query_as::<_, Steroid>(
        r#"
        SELECT * FROM steroids
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch steroid {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Steroid {} not found", id);
        AppError::NotFound(format!("Steroid {} not found", id))
    })?;

    Ok(Json(steroid))
}

pub async fn create_steroid(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSteroid>,
) -> Result<Json<Steroid>, AppError> {
    let steroid = sqlx::query_as::<_, Steroid>(
        r#"
        INSERT INTO steroids (
            category, examples, bbb_permeability,
            source, affective_modulation
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(payload.category)
    .bind(payload.examples)
    .bind(payload.bbb_permeability)
    .bind(payload.source)
    .bind(payload.affective_modulation)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create steroid: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    tracing::info!("Created steroid: {}", steroid.id);
    Ok(Json(steroid))
}

pub async fn update_steroid(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSteroid>,
) -> Result<Json<Steroid>, AppError> {
    let steroid = sqlx::query_as::<_, Steroid>(
        r#"
        UPDATE steroids
        SET
            category = COALESCE($1, category),
            examples = COALESCE($2, examples),
            bbb_permeability = COALESCE($3, bbb_permeability),
            source = COALESCE($4, source),
            affective_modulation = COALESCE($5, affective_modulation),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $6 AND deleted_at IS NULL
        RETURNING *
        "#
    )
    .bind(payload.category)
    .bind(payload.examples)
    .bind(payload.bbb_permeability)
    .bind(payload.source)
    .bind(payload.affective_modulation)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update steroid {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Steroid {} not found for update", id);
        AppError::NotFound(format!("Steroid {} not found", id))
    })?;

    tracing::info!("Updated steroid: {}", id);
    Ok(Json(steroid))
}

pub async fn delete_steroid(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE steroids
        SET deleted_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete steroid {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    if result.rows_affected() == 0 {
        tracing::warn!("Steroid {} not found for deletion", id);
        return Err(AppError::NotFound(format!("Steroid {} not found", id)));
    }

    tracing::info!("Soft deleted steroid: {}", id);
    Ok(Json(()))
}