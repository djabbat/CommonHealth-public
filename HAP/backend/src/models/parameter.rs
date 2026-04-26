use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Parameter {
    pub id: Uuid,
    pub name: String,
    pub value: String,
    pub unit: Option<String>,
    pub source: String,
    pub justification: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateParameter {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1))]
    pub value: String,
    #[validate(length(max = 50))]
    pub unit: Option<String>,
    #[validate(length(min = 1))]
    pub source: String,
    pub justification: String,
    #[validate(length(min = 1, max = 50))]
    pub status: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateParameter {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub value: Option<String>,
    #[validate(length(max = 50))]
    pub unit: Option<String>,
    #[validate(length(min = 1))]
    pub source: Option<String>,
    pub justification: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub status: Option<String>,
}