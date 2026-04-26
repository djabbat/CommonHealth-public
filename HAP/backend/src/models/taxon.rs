use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Taxon {
    pub id: Uuid,
    pub name: String,
    pub has_hepatic_organ: bool,
    pub steroid_regulators: String,
    pub bbb_permeability: String,
    pub affect: bool,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateTaxon {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub has_hepatic_organ: bool,
    #[validate(length(min = 1))]
    pub steroid_regulators: String,
    pub bbb_permeability: String,
    pub affect: bool,
    #[validate(length(min = 1, max = 50))]
    pub status: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateTaxon {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub has_hepatic_organ: Option<bool>,
    #[validate(length(min = 1))]
    pub steroid_regulators: Option<String>,
    pub bbb_permeability: Option<String>,
    pub affect: Option<bool>,
    #[validate(length(min = 1, max = 50))]
    pub status: Option<String>,
}