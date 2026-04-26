use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct CounterRegistry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub gamma_i: f64, // γ_i = 0 by default per CORRECTIONS
    pub damage_model: String,
    pub parameters: Vec<CounterParameter>,
}

#[derive(Debug, Serialize)]
pub struct CounterParameter {
    pub name: String,
    pub default_value: f64,
    pub description: String,
}

pub async fn get_registry(
    State(_pool): State<PgPool>,
) -> Result<Json<Vec<CounterRegistry>>, AppError> {
    // HAP-specific counter registry
    let registry = vec![
        CounterRegistry {
            id: "hap_hepatic_affective".to_string(),
            name: "Hepatic-Affective Joint Biomarker".to_string(),
            description: "Tracks joint hepatic and affective biomarkers for HAP theory validation".to_string(),
            gamma_i: 0.0, // Default per CORRECTIONS §1.3
            damage_model: "D_HAP(n,t) = α * stress_episodes + β * temporal_dysregulation".to_string(),
            parameters: vec![
                CounterParameter {
                    name: "α".to_string(),
                    default_value: 0.0,
                    description: "Stress episode coefficient".to_string(),
                },
                CounterParameter {
                    name: "β".to_string(),
                    default_value: 0.0,
                    description: "Temporal dysregulation coefficient".to_string(),
                },
            ],
        },
    ];

    Ok(Json(registry))
}