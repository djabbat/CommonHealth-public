use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Knowledge {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateKnowledge {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub content: String,
    #[validate(length(min = 1, max = 100))]
    pub category: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateKnowledge {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    pub content: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub category: Option<String>,
}