use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Experiment {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub prediction: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateExperiment {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: String,
    pub prediction: String,
    #[validate(length(min = 1, max = 50))]
    pub status: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateExperiment {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub prediction: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub status: Option<String>,
}