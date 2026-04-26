use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BhcaCriterion {
    pub id: Uuid,
    pub criterion: String,
    pub score: i32,
    pub comment: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateBhcaCriterion {
    #[validate(length(min = 1, max = 255))]
    pub criterion: String,
    #[validate(range(min = 0, max = 3))]
    pub score: i32,
    pub comment: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateBhcaCriterion {
    #[validate(length(min = 1, max = 255))]
    pub criterion: Option<String>,
    #[validate(range(min = 0, max = 3))]
    pub score: Option<i32>,
    pub comment: Option<String>,
}