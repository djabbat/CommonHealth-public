use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgQueryResult};
use tracing::{info, warn};
use uuid::Uuid;

use crate::error::{AppError, DbError};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EpigeneticDriftCounter {
    pub id: Uuid,
    pub individual_id: String,
    pub tissue_type: String,
    pub d4_state: f64,
    pub d4_baseline: f64,
    pub beta4: f64,
    pub tau4: f64,
    pub alpha4: f64,
    pub n4_star: f64,
    // Interaction coefficients (γ) - default 0 per canonical rules
    pub gamma_centriolar: f64,
    pub gamma_telomere: f64,
    pub gamma_mitoros: f64,
    pub gamma_proteostasis: f64,
    pub gamma_autocatalytic: f64,
    pub weight_tissue: f64,
    pub measured_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEpigeneticDriftCounter {
    pub individual_id: String,
    pub tissue_type: String,
    pub d4_state: f64,
    pub d4_baseline: f64,
    pub beta4: f64,
    pub tau4: f64,
    pub alpha4: f64,
    pub n4_star: f64,
    pub gamma_centriolar: Option<f64>,
    pub gamma_telomere: Option<f64>,
    pub gamma_mitoros: Option<f64>,
    pub gamma_proteostasis: Option<f64>,
    pub gamma_autocatalytic: Option<f64>,
    pub weight_tissue: f64,
    pub measured_at: DateTime<Utc>,
}

impl EpigeneticDriftCounter {
    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM epigenetic_drift_counters ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Self, AppError> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM epigenetic_drift_counters WHERE id = $1"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)
    }

    pub async fn create(pool: &PgPool, new: NewEpigeneticDriftCounter) -> Result<Self, AppError> {
        info!("Creating new EpigeneticDriftCounter for individual: {}", new.individual_id);
        
        let row = sqlx::query_as::<_, Self>(
            r#"
            INSERT INTO epigenetic_drift_counters (
                individual_id, tissue_type, d4_state, d4_baseline, beta4, tau4, alpha4, n4_star,
                gamma_centriolar, gamma_telomere, gamma_mitoros, gamma_proteostasis, gamma_autocatalytic,
                weight_tissue, measured_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(new.individual_id)
        .bind(new.tissue_type)
        .bind(new.d4_state)
        .bind(new.d4_baseline)
        .bind(new.beta4)
        .bind(new.tau4)
        .bind(new.alpha4)
        .bind(new.n4_star)
        .bind(new.gamma_centriolar.unwrap_or(0.0))
        .bind(new.gamma_telomere.unwrap_or(0.0))
        .bind(new.gamma_mitoros.unwrap_or(0.0))
        .bind(new.gamma_proteostasis.unwrap_or(0.0))
        .bind(new.gamma_autocatalytic.unwrap_or(0.0))
        .bind(new.weight_tissue)
        .bind(new.measured_at)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        Ok(row)
    }

    pub async fn update(pool: &PgPool, id: Uuid, update: NewEpigeneticDriftCounter) -> Result<Self, AppError> {
        info!("Updating EpigeneticDriftCounter: {}", id);
        
        let row = sqlx::query_as::<_, Self>(
            r#"
            UPDATE epigenetic_drift_counters
            SET individual_id = $2, tissue_type = $3, d4_state = $4, d4_baseline = $5,
                beta4 = $6, tau4 = $7, alpha4 = $8, n4_star = $9,
                gamma_centriolar = $10, gamma_telomere = $11, gamma_mitoros = $12,
                gamma_proteostasis = $13, gamma_autocatalytic = $14,
                weight_tissue = $15, measured_at = $16, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(update.individual_id)
        .bind(update.tissue_type)
        .bind(update.d4_state)
        .bind(update.d4_baseline)
        .bind(update.beta4)
        .bind(update.tau4)
        .bind(update.alpha4)
        .bind(update.n4_star)
        .bind(update.gamma_centriolar.unwrap_or(0.0))
        .bind(update.gamma_telomere.unwrap_or(0.0))
        .bind(update.gamma_mitoros.unwrap_or(0.0))
        .bind(update.gamma_proteostasis.unwrap_or(0.0))
        .bind(update.gamma_autocatalytic.unwrap_or(0.0))
        .bind(update.weight_tissue)
        .bind(update.measured_at)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        Ok(row)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        info!("Deleting EpigeneticDriftCounter: {}", id);
        
        let result: PgQueryResult = sqlx::query(
            "DELETE FROM epigenetic_drift_counters WHERE id = $1"
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        if result.rows_affected() == 0 {
            warn!("No EpigeneticDriftCounter found with id: {}", id);
            return Err(AppError::NotFound(format!("Counter with id {} not found", id)));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EpigeneticDriftMeasurement {
    pub id: Uuid,
    pub counter_id: Uuid,
    pub measurement_type: String, // "DNAm" or "ATAC"
    pub raw_data: serde_json::Value, // JSON of CpG values or accessibility scores
    pub computed_d4: f64,
    pub measured_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEpigeneticDriftMeasurement {
    pub counter_id: Uuid,
    pub measurement_type: String,
    pub raw_data: serde_json::Value,
    pub computed_d4: f64,
    pub measured_at: DateTime<Utc>,
}

impl EpigeneticDriftMeasurement {
    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM epigenetic_drift_measurements ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Self, AppError> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM epigenetic_drift_measurements WHERE id = $1"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)
    }

    pub async fn create(pool: &PgPool, new: NewEpigeneticDriftMeasurement) -> Result<Self, AppError> {
        info!("Creating new EpigeneticDriftMeasurement for counter: {}", new.counter_id);
        
        let row = sqlx::query_as::<_, Self>(
            r#"
            INSERT INTO epigenetic_drift_measurements (counter_id, measurement_type, raw_data, computed_d4, measured_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(new.counter_id)
        .bind(new.measurement_type)
        .bind(new.raw_data)
        .bind(new.computed_d4)
        .bind(new.measured_at)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        Ok(row)
    }

    pub async fn update(pool: &PgPool, id: Uuid, update: NewEpigeneticDriftMeasurement) -> Result<Self, AppError> {
        info!("Updating EpigeneticDriftMeasurement: {}", id);
        
        let row = sqlx::query_as::<_, Self>(
            r#"
            UPDATE epigenetic_drift_measurements
            SET counter_id = $2, measurement_type = $3, raw_data = $4,
                computed_d4 = $5, measured_at = $6, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(update.counter_id)
        .bind(update.measurement_type)
        .bind(update.raw_data)
        .bind(update.computed_d4)
        .bind(update.measured_at)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        Ok(row)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        info!("Deleting EpigeneticDriftMeasurement: {}", id);
        
        let result: PgQueryResult = sqlx::query(
            "DELETE FROM epigenetic_drift_measurements WHERE id = $1"
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        if result.rows_affected() == 0 {
            warn!("No EpigeneticDriftMeasurement found with id: {}", id);
            return Err(AppError::NotFound(format!("Measurement with id {} not found", id)));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EpigeneticDriftParameters {
    pub id: Uuid,
    pub tissue_type: String,
    pub beta4: f64,
    pub tau4: f64,
    pub alpha4: f64,
    pub n4_star: f64,
    // Interaction coefficients (γ) - default 0 per canonical rules
    pub gamma_centriolar: f64,
    pub gamma_telomere: f64,
    pub gamma_mitoros: f64,
    pub gamma_proteostasis: f64,
    pub gamma_autocatalytic: f64,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEpigeneticDriftParameters {
    pub tissue_type: String,
    pub beta4: f64,
    pub tau4: f64,
    pub alpha4: f64,
    pub n4_star: f64,
    pub gamma_centriolar: Option<f64>,
    pub gamma_telomere: Option<f64>,
    pub gamma_mitoros: Option<f64>,
    pub gamma_proteostasis: Option<f64>,
    pub gamma_autocatalytic: Option<f64>,
    pub is_default: bool,
}

impl EpigeneticDriftParameters {
    pub async fn list(pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM epigenetic_drift_parameters ORDER BY tissue_type, created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Self, AppError> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM epigenetic_drift_parameters WHERE id = $1"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)
    }

    pub async fn create(pool: &PgPool, new: NewEpigeneticDriftParameters) -> Result<Self, AppError> {
        info!("Creating new EpigeneticDriftParameters for tissue: {}", new.tissue_type);
        
        let row = sqlx::query_as::<_, Self>(
            r#"
            INSERT INTO epigenetic_drift_parameters (
                tissue_type, beta4, tau4, alpha4, n4_star,
                gamma_centriolar, gamma_telomere, gamma_mitoros, gamma_proteostasis, gamma_autocatalytic,
                is_default
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(new.tissue_type)
        .bind(new.beta4)
        .bind(new.tau4)
        .bind(new.alpha4)
        .bind(new.n4_star)
        .bind(new.gamma_centriolar.unwrap_or(0.0))
        .bind(new.gamma_telomere.unwrap_or(0.0))
        .bind(new.gamma_mitoros.unwrap_or(0.0))
        .bind(new.gamma_proteostasis.unwrap_or(0.0))
        .bind(new.gamma_autocatalytic.unwrap_or(0.0))
        .bind(new.is_default)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        Ok(row)
    }

    pub async fn update(pool: &PgPool, id: Uuid, update: NewEpigeneticDriftParameters) -> Result<Self, AppError> {
        info!("Updating EpigeneticDriftParameters: {}", id);
        
        let row = sqlx::query_as::<_, Self>(
            r#"
            UPDATE epigenetic_drift_parameters
            SET tissue_type = $2, beta4 = $3, tau4 = $4, alpha4 = $5, n4_star = $6,
                gamma_centriolar = $7, gamma_telomere = $8, gamma_mitoros = $9,
                gamma_proteostasis = $10, gamma_autocatalytic = $11,
                is_default = $12, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(update.tissue_type)
        .bind(update.beta4)
        .bind(update.tau4)
        .bind(update.alpha4)
        .bind(update.n4_star)
        .bind(update.gamma_centriolar.unwrap_or(0.0))
        .bind(update.gamma_telomere.unwrap_or(0.0))
        .bind(update.gamma_mitoros.unwrap_or(0.0))
        .bind(update.gamma_proteostasis.unwrap_or(0.0))
        .bind(update.gamma_autocatalytic.unwrap_or(0.0))
        .bind(update.is_default)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        Ok(row)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        info!("Deleting EpigeneticDriftParameters: {}", id);
        
        let result: PgQueryResult = sqlx::query(
            "DELETE FROM epigenetic_drift_parameters WHERE id = $1"
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(DbError::from)
        .map_err(AppError::from)?;

        if result.rows_affected() == 0 {
            warn!("No EpigeneticDriftParameters found with id: {}", id);
            return Err(AppError::NotFound(format!("Parameters with id {} not found", id)));
        }

        Ok(())
    }
}