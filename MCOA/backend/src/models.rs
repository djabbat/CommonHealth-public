use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::error::{AppError, AppResult};
use crate::db::DbPool;

// Counter entity - represents one of the five canonical counters
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Counter {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub alpha: f64, // division-driven rate
    pub beta: f64,  // time-driven rate
    pub gamma: f64, // interaction coefficient (default 0)
    pub d_critical_default: f64, // default critical damage threshold
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCounter {
    pub name: String,
    pub description: Option<String>,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: Option<f64>,
    pub d_critical_default: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCounter {
    pub name: Option<String>,
    pub description: Option<String>,
    pub alpha: Option<f64>,
    pub beta: Option<f64>,
    pub gamma: Option<f64>,
    pub d_critical_default: Option<f64>,
    pub is_active: Option<bool>,
}

impl Counter {
    pub async fn list(pool: &DbPool) -> AppResult<Vec<Self>> {
        sqlx::query_as("SELECT * FROM counters ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> AppResult<Self> {
        sqlx::query_as("SELECT * FROM counters WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn create(pool: &DbPool, payload: CreateCounter) -> AppResult<Self> {
        let gamma = payload.gamma.unwrap_or(0.0);
        sqlx::query_as(
            "INSERT INTO counters (name, description, alpha, beta, gamma, d_critical_default) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING *"
        )
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.alpha)
        .bind(payload.beta)
        .bind(gamma)
        .bind(payload.d_critical_default)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn update(pool: &DbPool, id: Uuid, payload: UpdateCounter) -> AppResult<Self> {
        sqlx::query_as(
            "UPDATE counters 
             SET name = COALESCE($1, name),
                 description = COALESCE($2, description),
                 alpha = COALESCE($3, alpha),
                 beta = COALESCE($4, beta),
                 gamma = COALESCE($5, gamma),
                 d_critical_default = COALESCE($6, d_critical_default),
                 is_active = COALESCE($7, is_active),
                 updated_at = NOW()
             WHERE id = $8
             RETURNING *"
        )
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.alpha)
        .bind(payload.beta)
        .bind(payload.gamma)
        .bind(payload.d_critical_default)
        .bind(payload.is_active)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM counters WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}

// Tissue entity - represents biological tissue types
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tissue {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub l_critical: f64, // critical load threshold for this tissue
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTissue {
    pub name: String,
    pub description: Option<String>,
    pub l_critical: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTissue {
    pub name: Option<String>,
    pub description: Option<String>,
    pub l_critical: Option<f64>,
}

impl Tissue {
    pub async fn list(pool: &DbPool) -> AppResult<Vec<Self>> {
        sqlx::query_as("SELECT * FROM tissues ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> AppResult<Self> {
        sqlx::query_as("SELECT * FROM tissues WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn create(pool: &DbPool, payload: CreateTissue) -> AppResult<Self> {
        sqlx::query_as(
            "INSERT INTO tissues (name, description, l_critical) 
             VALUES ($1, $2, $3) 
             RETURNING *"
        )
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.l_critical)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn update(pool: &DbPool, id: Uuid, payload: UpdateTissue) -> AppResult<Self> {
        sqlx::query_as(
            "UPDATE tissues 
             SET name = COALESCE($1, name),
                 description = COALESCE($2, description),
                 l_critical = COALESCE($3, l_critical),
                 updated_at = NOW()
             WHERE id = $4
             RETURNING *"
        )
        .bind(payload.name)
        .bind(payload.description)
        .bind(payload.l_critical)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM tissues WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}

// Tissue-specific counter parameters
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CounterTissueParams {
    pub id: Uuid,
    pub counter_id: Uuid,
    pub tissue_id: Uuid,
    pub n_reference: f64,  // n_i* for this counter in this tissue
    pub tau_reference: f64, // τ_i for this counter in this tissue
    pub d_critical: f64,   // tissue-specific critical damage
    pub weight: f64,       // w_i(tissue) - a-priori weight
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CounterTissueParams {
    pub async fn find_by_counter_and_tissue(pool: &DbPool, counter_id: Uuid, tissue_id: Uuid) -> AppResult<Self> {
        sqlx::query_as(
            "SELECT * FROM counter_tissue_params 
             WHERE counter_id = $1 AND tissue_id = $2"
        )
        .bind(counter_id)
        .bind(tissue_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn update_weight(pool: &DbPool, counter_id: Uuid, tissue_id: Uuid, weight: f64) -> AppResult<Self> {
        sqlx::query_as(
            "UPDATE counter_tissue_params 
             SET weight = $1, updated_at = NOW()
             WHERE counter_id = $2 AND tissue_id = $3
             RETURNING *"
        )
        .bind(weight)
        .bind(counter_id)
        .bind(tissue_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }
}

// Subject entity - represents organisms being studied
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subject {
    pub id: Uuid,
    pub identifier: String,
    pub species: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubject {
    pub identifier: String,
    pub species: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubject {
    pub identifier: Option<String>,
    pub species: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl Subject {
    pub async fn list(pool: &DbPool) -> AppResult<Vec<Self>> {
        sqlx::query_as("SELECT * FROM subjects ORDER BY identifier")
            .fetch_all(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> AppResult<Self> {
        sqlx::query_as("SELECT * FROM subjects WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn create(pool: &DbPool, payload: CreateSubject) -> AppResult<Self> {
        sqlx::query_as(
            "INSERT INTO subjects (identifier, species, metadata) 
             VALUES ($1, $2, $3) 
             RETURNING *"
        )
        .bind(payload.identifier)
        .bind(payload.species)
        .bind(payload.metadata)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn update(pool: &DbPool, id: Uuid, payload: UpdateSubject) -> AppResult<Self> {
        sqlx::query_as(
            "UPDATE subjects 
             SET identifier = COALESCE($1, identifier),
                 species = COALESCE($2, species),
                 metadata = COALESCE($3, metadata),
                 updated_at = NOW()
             WHERE id = $4
             RETURNING *"
        )
        .bind(payload.identifier)
        .bind(payload.species)
        .bind(payload.metadata)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM subjects WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}

// DamageMeasurement entity - stores damage values for counters
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DamageMeasurement {
    pub id: Uuid,
    pub subject_id: Uuid,
    pub counter_id: Uuid,
    pub tissue_id: Uuid,
    pub division_count: f64,  // n
    pub time_value: f64,      // t in seconds
    pub damage: f64,          // D_i calculated
    pub measured_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDamageMeasurement {
    pub subject_id: Uuid,
    pub counter_id: Uuid,
    pub tissue_id: Uuid,
    pub division_count: f64,
    pub time_value: f64,
    pub damage: f64,
    pub measured_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDamageMeasurement {
    pub division_count: Option<f64>,
    pub time_value: Option<f64>,
    pub damage: Option<f64>,
    pub measured_at: Option<DateTime<Utc>>,
}

impl DamageMeasurement {
    pub async fn list(pool: &DbPool) -> AppResult<Vec<Self>> {
        sqlx::query_as("SELECT * FROM damage_measurements ORDER BY measured_at DESC")
            .fetch_all(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> AppResult<Self> {
        sqlx::query_as("SELECT * FROM damage_measurements WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_subject(pool: &DbPool, subject_id: Uuid) -> AppResult<Vec<Self>> {
        sqlx::query_as("SELECT * FROM damage_measurements WHERE subject_id = $1 ORDER BY measured_at")
            .bind(subject_id)
            .fetch_all(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn create(pool: &DbPool, payload: CreateDamageMeasurement) -> AppResult<Self> {
        let measured_at = payload.measured_at.unwrap_or_else(Utc::now);
        sqlx::query_as(
            "INSERT INTO damage_measurements 
             (subject_id, counter_id, tissue_id, division_count, time_value, damage, measured_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) 
             RETURNING *"
        )
        .bind(payload.subject_id)
        .bind(payload.counter_id)
        .bind(payload.tissue_id)
        .bind(payload.division_count)
        .bind(payload.time_value)
        .bind(payload.damage)
        .bind(measured_at)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn update(pool: &DbPool, id: Uuid, payload: UpdateDamageMeasurement) -> AppResult<Self> {
        sqlx::query_as(
            "UPDATE damage_measurements 
             SET division_count = COALESCE($1, division_count),
                 time_value = COALESCE($2, time_value),
                 damage = COALESCE($3, damage),
                 measured_at = COALESCE($4, measured_at),
                 updated_at = NOW()
             WHERE id = $5
             RETURNING *"
        )
        .bind(payload.division_count)
        .bind(payload.time_value)
        .bind(payload.damage)
        .bind(payload.measured_at)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM damage_measurements WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}

// TissueLoad entity - stores computed tissue loads
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TissueLoad {
    pub id: Uuid,
    pub subject_id: Uuid,
    pub tissue_id: Uuid,
    pub load_value: f64, // L_tissue calculated
    pub computed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTissueLoad {
    pub subject_id: Uuid,
    pub tissue_id: Uuid,
    pub load_value: f64,
    pub computed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeTissueLoad {
    pub subject_id: Uuid,
    pub tissue_id: Uuid,
    pub timestamp: Option<DateTime<Utc>>,
}

impl TissueLoad {
    pub async fn list(pool: &DbPool) -> AppResult<Vec<Self>> {
        sqlx::query_as("SELECT * FROM tissue_loads ORDER BY computed_at DESC")
            .fetch_all(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> AppResult<Self> {
        sqlx::query_as("SELECT * FROM tissue_loads WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn create(pool: &DbPool, payload: CreateTissueLoad) -> AppResult<Self> {
        let computed_at = payload.computed_at.unwrap_or_else(Utc::now);
        sqlx::query_as(
            "INSERT INTO tissue_loads (subject_id, tissue_id, load_value, computed_at) 
             VALUES ($1, $2, $3, $4) 
             RETURNING *"
        )
        .bind(payload.subject_id)
        .bind(payload.tissue_id)
        .bind(payload.load_value)
        .bind(computed_at)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM tissue_loads WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    pub async fn compute(pool: &DbPool, payload: ComputeTissueLoad) -> AppResult<Self> {
        let timestamp = payload.timestamp.unwrap_or_else(Utc::now);
        
        // Get all damage measurements for this subject and tissue at or before timestamp
        let measurements = sqlx::query_as::<_, DamageMeasurement>(
            "SELECT dm.* FROM damage_measurements dm
             JOIN counter_tissue_params ctp ON dm.counter_id = ctp.counter_id 
                 AND dm.tissue_id = ctp.tissue_id
             WHERE dm.subject_id = $1 
                 AND dm.tissue_id = $2 
                 AND dm.measured_at <= $3"
        )
        .bind(payload.subject_id)
        .bind(payload.tissue_id)
        .bind(timestamp)
        .fetch_all(pool)
        .await?;

        // Get tissue weights for all counters
        let params = sqlx::query_as::<_, CounterTissueParams>(
            "SELECT * FROM counter_tissue_params WHERE tissue_id = $1"
        )
        .bind(payload.tissue_id)
        .fetch_all(pool)
        .await?;

        // Calculate L_tissue = Σ_i [ w_i(tissue) * f_i( D_i(n, t) ) ]
        // For now, f_i is identity function
        let mut total_load = 0.0;
        let mut used_weights = 0.0;
        
        for param in params {
            if let Some(measurement) = measurements.iter().find(|m| m.counter_id == param.counter_id) {
                total_load += param.weight * measurement.damage;
                used_weights += param.weight;
            }
        }

        // Normalize by total weight (should be ~1.0)
        let load_value = if used_weights > 0.0 {
            total_load / used_weights
        } else {
            0.0
        };

        // Store the computed load
        Self::create(pool, CreateTissueLoad {
            subject_id: payload.subject_id,
            tissue_id: payload.tissue_id,
            load_value,
            computed_at: Some(timestamp),
        }).await
    }
}

// CouplingMatrixEntry entity - stores Γ_ij coefficients
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CouplingMatrixEntry {
    pub id: Uuid,
    pub influencer_counter_id: Uuid, // j
    pub influenced_counter_id: Uuid, // i
    pub gamma_ij: f64,               // Γ_ij
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCouplingMatrixEntry {
    pub influencer_counter_id: Uuid,
    pub influenced_counter_id: Uuid,
    pub gamma_ij: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCouplingMatrixEntry {
    pub gamma_ij: Option<f64>,
}

impl CouplingMatrixEntry {
    pub async fn list(pool: &DbPool) -> AppResult<Vec<Self>> {
        sqlx::query_as("SELECT * FROM coupling_matrix ORDER BY influenced_counter_id, influencer_counter_id")
            .fetch_all(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &DbPool, id: Uuid) -> AppResult<Self> {
        sqlx::query_as("SELECT * FROM coupling_matrix WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)
    }

    pub async fn create(pool: &DbPool, payload: CreateCouplingMatrixEntry) -> AppResult<Self> {
        sqlx::query_as(
            "INSERT INTO coupling_matrix (influencer_counter_id, influenced_counter_id, gamma_ij) 
             VALUES ($1, $2, $3) 
             RETURNING *"
        )
        .bind(payload.influencer_counter_id)
        .bind(payload.influenced_counter_id)
        .bind(payload.gamma_ij)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn update(pool: &DbPool, id: Uuid, payload: UpdateCouplingMatrixEntry) -> AppResult<Self> {
        sqlx::query_as(
            "UPDATE coupling_matrix 
             SET gamma_ij = COALESCE($1, gamma_ij),
                 updated_at = NOW()
             WHERE id = $2
             RETURNING *"
        )
        .bind(payload.gamma_ij)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM coupling_matrix WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}