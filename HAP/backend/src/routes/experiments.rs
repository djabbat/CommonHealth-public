use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::experiment::{Experiment, CreateExperiment, UpdateExperiment},
};

pub async fn list_experiments(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Experiment>>, AppError> {
    let experiments = sqlx::query_as::<_, Experiment>(
        r#"
        SELECT * FROM experiments
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch experiments: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    Ok(Json(experiments))
}

pub async fn get_experiment(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Experiment>, AppError> {
    let experiment = sqlx::query_as::<_, Experiment>(
        r#"
        SELECT * FROM experiments
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch experiment {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Experiment {} not found", id);
        AppError::NotFound(format!("Experiment {} not found", id))
    })?;

    Ok(Json(experiment))
}

pub async fn create_experiment(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateExperiment>,
) -> Result<Json<Experiment>, AppError> {
    let experiment = sqlx::query_as::<_, Experiment>(
        r#"
        INSERT INTO experiments (name, description, prediction, status)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.prediction)
    .bind(payload.status)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create experiment: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    tracing::info!("Created experiment: {}", experiment.id);
    Ok(Json(experiment))
}

pub async fn update_experiment(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateExperiment>,
) -> Result<Json<Experiment>, AppError> {
    let experiment = sqlx::query_as::<_, Experiment>(
        r#"
        UPDATE experiments
        SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            prediction = COALESCE($3, prediction),
            status = COALESCE($4, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $5 AND deleted_at IS NULL
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.prediction)
    .bind(payload.status)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update experiment {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Experiment {} not found for update", id);
        AppError::NotFound(format!("Experiment {} not found", id))
    })?;

    tracing::info!("Updated experiment: {}", id);
    Ok(Json(experiment))
}

pub async fn delete_experiment(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE experiments
        SET deleted_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete experiment {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    if result.rows_affected() == 0 {
        tracing::warn!("Experiment {} not found for deletion", id);
        return Err(AppError::NotFound(format!("Experiment {} not found", id)));
    }

    tracing::info!("Soft deleted experiment: {}", id);
    Ok(Json(()))
}