use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct TelomereMeasurement {
    pub id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub subject_id: Uuid,
    
    #[validate(length(max = 255))]
    pub sample_id: Option<String>,
    
    pub measured_at: DateTime<Utc>,
    
    #[validate(range(min = 0.0))]
    pub telomere_length_bp: f64,
    
    #[validate(range(min = 0.0))]
    pub telomere_deficit_bp: f64,
    
    #[validate(range(min = 0.0))]
    pub population_doublings: Option<f64>,
    
    #[validate(range(min = 0.0))]
    pub time_elapsed_years: Option<f64>,
    
    #[serde(default)]
    pub oxidative_stress_marker: Option<f64>,
    
    #[serde(default)]
    pub shelterin_expression: Option<f64>,
    
    #[serde(default)]
    pub telomerase_activity: Option<f64>,
    
    #[serde(default)]
    #[validate(length(max = 1000))]
    pub measurement_method: Option<String>,
    
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for TelomereMeasurement {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            subject_id: Uuid::nil(),
            sample_id: None,
            measured_at: Utc::now(),
            telomere_length_bp: 0.0,
            telomere_deficit_bp: 0.0,
            population_doublings: None,
            time_elapsed_years: None,
            oxidative_stress_marker: None,
            shelterin_expression: None,
            telomerase_activity: None,
            measurement_method: None,
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct TelomereParameters {
    pub id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub subject_id: Uuid,
    
    #[serde(rename = "d2_baseline")]
    #[validate(range(min = 0.0, max = 20000.0))]
    pub d2_baseline: f64,  // D₂,₀ in bp
    
    #[serde(rename = "alpha2")]
    #[validate(range(min = 0.0, max = 500.0))]
    pub alpha2: f64,  // α₂ in bp/PD
    
    #[serde(rename = "beta2")]
    #[validate(range(min = 0.0, max = 100.0))]
    pub beta2: f64,  // β₂ in bp/year
    
    #[serde(rename = "n2_star")]
    #[validate(range(min = 1.0, max = 200.0))]
    pub n2_star: f64,  // n₂* in PD
    
    #[serde(rename = "tau2")]
    #[validate(range(min = 0.1))]
    pub tau2: f64,  // τ₂ in years
    
    // Coupling coefficients (γ) - all zero by default per CORRECTIONS_2026-04-22
    #[serde(rename = "gamma_21")]
    #[validate(range(min = 0.0))]
    pub gamma_21: f64,  // Γ₂,₁: Centriolar → Telomere
    
    #[serde(rename = "gamma_23")]
    #[validate(range(min = 0.0))]
    pub gamma_23: f64,  // Γ₂,₃: MitoROS → Telomere
    
    #[serde(rename = "gamma_24")]
    #[validate(range(min = 0.0))]
    pub gamma_24: f64,  // Γ₂,₄: Epigenetic → Telomere
    
    #[serde(rename = "gamma_25")]
    #[validate(range(min = 0.0))]
    pub gamma_25: f64,  // Γ₂,₅: Proteostasis → Telomere
    
    #[serde(default)]
    pub is_default: bool,
    
    #[serde(default)]
    #[validate(length(max = 1000))]
    pub notes: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for TelomereParameters {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            subject_id: Uuid::nil(),
            d2_baseline: 12500.0,  // 10-15kb range midpoint
            alpha2: 125.0,         // 50-200 bp/PD midpoint
            beta2: 35.0,           // 20-50 bp/year midpoint
            n2_star: 50.0,         // 40-60 PD midpoint
            tau2: 1.0,             // 1 year default
            gamma_21: 0.0,         // Default zero per corrections
            gamma_23: 0.0,
            gamma_24: 0.0,
            gamma_25: 0.0,
            is_default: true,
            notes: Some("Default parameters from PARAMETERS.md".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterRegistry {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub equation: String,
    pub parameters: Vec<String>,
    pub units: String,
    pub coupling: Vec<(String, String)>,
    pub default_gamma: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TissueLoadRequest {
    #[validate(required)]
    pub subject_id: Option<Uuid>,
    
    #[validate(required, length(min = 1))]
    pub tissue_type: Option<String>,
    
    #[serde(default)]
    pub include_coupling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TissueLoadResponse {
    pub subject_id: Uuid,
    pub tissue_type: String,
    pub measurement_id: Uuid,
    pub d2_value: f64,
    pub tissue_weight: f64,
    pub tissue_load: f64,
    pub computed_at: DateTime<Utc>,
    pub parameters_used: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeasurementQuery {
    pub subject_id: Option<Uuid>,
    pub sample_id: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub min_length: Option<f64>,
    pub max_length: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub details: Option<serde_json::Value>,
}