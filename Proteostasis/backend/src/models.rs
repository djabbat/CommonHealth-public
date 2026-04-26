use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProteostasisParameter {
    pub id: Uuid,
    pub tissue_type: String,
    pub d50: f64,
    pub alpha5: f64,
    pub n5_critical: f64,
    pub beta5: f64,
    pub tau5: f64,
    pub gamma51: f64,
    pub gamma52: f64,
    pub gamma53: f64,
    pub gamma54: f64,
    pub gamma55: f64,
    pub weight: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for ProteostasisParameter {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            tissue_type: "default".to_string(),
            d50: 0.0,
            alpha5: 0.05,
            n5_critical: 50.0,
            beta5: 0.1,
            tau5: 10.0,
            gamma51: 0.0,
            gamma52: 0.0,
            gamma53: 0.0,
            gamma54: 0.0,
            gamma55: 0.0,
            weight: 0.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProteostasisTimeSeries {
    pub id: Uuid,
    pub subject_id: String,
    pub timestamp: DateTime<Utc>,
    pub cell_divisions: f64,
    pub chronological_time: f64,
    pub d5_value: f64,
    pub parameter_set_id: Option<Uuid>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for ProteostasisTimeSeries {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            subject_id: "".to_string(),
            timestamp: Utc::now(),
            cell_divisions: 0.0,
            chronological_time: 0.0,
            d5_value: 0.0,
            parameter_set_id: None,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageComputationRequest {
    pub cell_divisions: f64,
    pub chronological_time: f64,
    pub parameter_set_id: Option<Uuid>,
}