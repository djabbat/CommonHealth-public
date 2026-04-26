use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::knowledge::{Knowledge, CreateKnowledge, UpdateKnowledge},
};

pub async fn list_knowledges(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Knowledge>>, AppError> {
    let knowledges = sqlx::query_as::<_, Knowledge>(
        r#"
        SELECT * FROM knowledges
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch knowledge entries: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    Ok(Json(knowledges))
}

pub async fn get_knowledge(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Knowledge>, AppError> {
    let knowledge = sqlx::query_as::<_, Knowledge>(
        r#"
        SELECT * FROM knowledges
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch knowledge entry {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Knowledge entry {} not found", id);
        AppError::NotFound(format!("Knowledge entry {} not found", id))
    })?;

    Ok(Json(knowledge))
}

pub async fn create_knowledge(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateKnowledge>,
) -> Result<Json<Knowledge>, AppError> {
    let knowledge = sqlx::query_as::<_, Knowledge>(
        r#"
        INSERT INTO knowledges (title, content, category)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.category)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create knowledge entry: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    tracing::info!("Created knowledge entry: {}", knowledge.id);
    Ok(Json(knowledge))
}

pub async fn update_knowledge(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateKnowledge>,
) -> Result<Json<Knowledge>, AppError> {
    let knowledge = sqlx::query_as::<_, Knowledge>(
        r#"
        UPDATE knowledges
        SET
            title = COALESCE($1, title),
            content = COALESCE($2, content),
            category = COALESCE($3, category),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4 AND deleted_at IS NULL
        RETURNING *
        "#
    )
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.category)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update knowledge entry {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Knowledge entry {} not found for update", id);
        AppError::NotFound(format!("Knowledge entry {} not found", id))
    })?;

    tracing::info!("Updated knowledge entry: {}", id);
    Ok(Json(knowledge))
}

pub async fn delete_knowledge(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE knowledges
        SET deleted_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete knowledge entry {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    if result.rows_affected() == 0 {
        tracing::warn!("Knowledge entry {} not found for deletion", id);
        return Err(AppError::NotFound(format!("Knowledge entry {} not found", id)));
    }

    tracing::info!("Soft deleted knowledge entry: {}", id);
    Ok(Json(()))
}