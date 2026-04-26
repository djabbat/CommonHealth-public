use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::bhcacriterion::{BhcaCriterion, CreateBhcaCriterion, UpdateBhcaCriterion},
};

pub async fn list_bhcacriteria(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<BhcaCriterion>>, AppError> {
    let criteria = sqlx::query_as::<_, BhcaCriterion>(
        r#"
        SELECT * FROM bhcacriteria
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch BHCA criteria: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    Ok(Json(criteria))
}

pub async fn get_bhcacriterion(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<BhcaCriterion>, AppError> {
    let criterion = sqlx::query_as::<_, BhcaCriterion>(
        r#"
        SELECT * FROM bhcacriteria
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch BHCA criterion {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("BHCA criterion {} not found", id);
        AppError::NotFound(format!("BHCA criterion {} not found", id))
    })?;

    Ok(Json(criterion))
}

pub async fn create_bhcacriterion(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateBhcaCriterion>,
) -> Result<Json<BhcaCriterion>, AppError> {
    let criterion = sqlx::query_as::<_, BhcaCriterion>(
        r#"
        INSERT INTO bhcacriteria (criterion, score, comment)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
    .bind(payload.criterion)
    .bind(payload.score)
    .bind(payload.comment)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create BHCA criterion: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    tracing::info!("Created BHCA criterion: {}", criterion.id);
    Ok(Json(criterion))
}

pub async fn update_bhcacriterion(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBhcaCriterion>,
) -> Result<Json<BhcaCriterion>, AppError> {
    let criterion = sqlx::query_as::<_, BhcaCriterion>(
        r#"
        UPDATE bhcacriteria
        SET
            criterion = COALESCE($1, criterion),
            score = COALESCE($2, score),
            comment = COALESCE($3, comment),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4 AND deleted_at IS NULL
        RETURNING *
        "#
    )
    .bind(payload.criterion)
    .bind(payload.score)
    .bind(payload.comment)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update BHCA criterion {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("BHCA criterion {} not found for update", id);
        AppError::NotFound(format!("BHCA criterion {} not found", id))
    })?;

    tracing::info!("Updated BHCA criterion: {}", id);
    Ok(Json(criterion))
}

pub async fn delete_bhcacriterion(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE bhcacriteria
        SET deleted_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete BHCA criterion {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    if result.rows_affected() == 0 {
        tracing::warn!("BHCA criterion {} not found for deletion", id);
        return Err(AppError::NotFound(format!("BHCA criterion {} not found", id)));
    }

    tracing::info!("Soft deleted BHCA criterion: {}", id);
    Ok(Json(()))
}