use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Counter3Record - Stores time-series data for MitoROS counter
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Counter3Record {
    pub id: Uuid,
    pub tissue_id: Uuid,
    pub n_cell_divisions: f64,
    pub t_time: f64,
    pub d3_value: f64,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter3RecordCreate {
    pub tissue_id: Uuid,
    pub n_cell_divisions: f64,
    pub t_time: f64,
    pub d3_value: f64,
    pub metadata: Option<serde_json::Value>,
}

// Counter3Parameters - Stores parameters for MitoROS model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Counter3Parameters {
    pub id: Uuid,
    pub tissue_id: Uuid,
    pub d3_0: f64,
    pub alpha3: f64,
    pub n3_star: f64,
    pub beta3: f64,
    pub tau3: f64,
    pub gamma3: f64,  // Default 0 per CORRECTIONS_2026-04-22
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter3ParametersCreate {
    pub tissue_id: Uuid,
    pub d3_0: f64,
    pub alpha3: f64,
    pub n3_star: f64,
    pub beta3: f64,
    pub tau3: f64,
    pub gamma3: Option<f64>,  // Optional, default to 0
    pub metadata: Option<serde_json::Value>,
}

impl Counter3ParametersCreate {
    pub fn gamma3(&self) -> f64 {
        self.gamma3.unwrap_or(0.0)  // Default 0 per canonical rules
    }
}

// Tissue - Biological tissue types with MCOA weights
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tissue {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub mitotic_index: f64,  // 0-1 scale, proportion of dividing cells
    pub metabolic_rate: f64, // Relative metabolic rate
    pub weight_w3: Option<f64>, // Weight for Counter #3 in MCOA L_tissue computation
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TissueCreate {
    pub name: String,
    pub description: Option<String>,
    pub mitotic_index: f64,
    pub metabolic_rate: f64,
    pub weight_w3: Option<f64>,
}