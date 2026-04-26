use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::taxon::{Taxon, CreateTaxon, UpdateTaxon},
};

pub async fn list_taxa(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Taxon>>, AppError> {
    let taxa = sqlx::query_as::<_, Taxon>(
        r#"
        SELECT * FROM taxa
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch taxa: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    Ok(Json(taxa))
}

pub async fn get_taxon(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Taxon>, AppError> {
    let taxon = sqlx::query_as::<_, Taxon>(
        r#"
        SELECT * FROM taxa
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch taxon {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Taxon {} not found", id);
        AppError::NotFound(format!("Taxon {} not found", id))
    })?;

    Ok(Json(taxon))
}

pub async fn create_taxon(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTaxon>,
) -> Result<Json<Taxon>, AppError> {
    let taxon = sqlx::query_as::<_, Taxon>(
        r#"
        INSERT INTO taxa (
            name, has_hepatic_organ, steroid_regulators,
            bbb_permeability, affect, status
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.has_hepatic_organ)
    .bind(payload.steroid_regulators)
    .bind(payload.bbb_permeability)
    .bind(payload.affect)
    .bind(payload.status)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create taxon: {}", e);
        AppError::DatabaseError(e.to_string())
    })?;

    tracing::info!("Created taxon: {}", taxon.id);
    Ok(Json(taxon))
}

pub async fn update_taxon(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaxon>,
) -> Result<Json<Taxon>, AppError> {
    let taxon = sqlx::query_as::<_, Taxon>(
        r#"
        UPDATE taxa
        SET
            name = COALESCE($1, name),
            has_hepatic_organ = COALESCE($2, has_hepatic_organ),
            steroid_regulators = COALESCE($3, steroid_regulators),
            bbb_permeability = COALESCE($4, bbb_permeability),
            affect = COALESCE($5, affect),
            status = COALESCE($6, status),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $7 AND deleted_at IS NULL
        RETURNING *
        "#
    )
    .bind(payload.name)
    .bind(payload.has_hepatic_organ)
    .bind(payload.steroid_regulators)
    .bind(payload.bbb_permeability)
    .bind(payload.affect)
    .bind(payload.status)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update taxon {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?
    .ok_or_else(|| {
        tracing::warn!("Taxon {} not found for update", id);
        AppError::NotFound(format!("Taxon {} not found", id))
    })?;

    tracing::info!("Updated taxon: {}", id);
    Ok(Json(taxon))
}

pub async fn delete_taxon(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE taxa
        SET deleted_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND deleted_at IS NULL
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete taxon {}: {}", id, e);
        AppError::DatabaseError(e.to_string())
    })?;

    if result.rows_affected() == 0 {
        tracing::warn!("Taxon {} not found for deletion", id);
        return Err(AppError::NotFound(format!("Taxon {} not found", id)));
    }

    tracing::info!("Soft deleted taxon: {}", id);
    Ok(Json(()))
}