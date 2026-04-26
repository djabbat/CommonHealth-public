use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Biomarker {
    pub id: Uuid,
    pub name: String,
    pub biomarker_type: String,
    pub hepatic_affective_joint: bool,
    pub measurement_method: String,
    pub normal_range: String,
    pub unit: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateBiomarker {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub biomarker_type: String,
    pub hepatic_affective_joint: bool,
    #[validate(length(min = 1))]
    pub measurement_method: String,
    pub normal_range: String,
    #[validate(length(min = 1, max = 50))]
    pub unit: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateBiomarker {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub biomarker_type: Option<String>,
    pub hepatic_affective_joint: Option<bool>,
    #[validate(length(min = 1))]
    pub measurement_method: Option<String>,
    pub normal_range: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub unit: Option<String>,
}