use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Steroid {
    pub id: Uuid,
    pub category: String,
    pub examples: String,
    pub bbb_permeability: String,
    pub source: String,
    pub affective_modulation: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateSteroid {
    #[validate(length(min = 1, max = 100))]
    pub category: String,
    #[validate(length(min = 1))]
    pub examples: String,
    pub bbb_permeability: String,
    #[validate(length(min = 1))]
    pub source: String,
    pub affective_modulation: bool,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateSteroid {
    #[validate(length(min = 1, max = 100))]
    pub category: Option<String>,
    #[validate(length(min = 1))]
    pub examples: Option<String>,
    pub bbb_permeability: Option<String>,
    #[validate(length(min = 1))]
    pub source: Option<String>,
    pub affective_modulation: Option<bool>,
}