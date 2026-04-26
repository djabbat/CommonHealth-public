use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;
use decimal::Decimal;
use validator::Validate;

// Phase entity - represents one of the 5 neurobiological phases
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Phase {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[validate(range(min = 0, max = 120))]
    pub age_start: i32,
    #[validate(range(min = 0, max = 120))]
    pub age_end: i32,
    pub neurobio_characteristics: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePhase {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    #[validate(range(min = 0, max = 120))]
    pub age_start: i32,
    #[validate(range(min = 0, max = 120))]
    pub age_end: i32,
    pub neurobio_characteristics: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePhase {
    pub name: Option<String>,
    pub description: Option<String>,
    pub age_start: Option<i32>,
    pub age_end: Option<i32>,
    pub neurobio_characteristics: Option<String>,
}

// Domain entity - the four domains: Morphology, Physiology, Psychology, Sociology
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Domain {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDomain {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDomain {
    pub name: Option<String>,
    pub description: Option<String>,
}

// Parameter entity - quantitative parameters in each domain
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Parameter {
    pub id: Uuid,
    pub domain_id: Uuid,
    pub domain_name: Option<String>, // Joined from domains table
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<Decimal>,
    pub unit: Option<String>,
    pub source: Option<String>,
    pub status: Option<String>, // Measured, Estimated, TBD
    pub min_value: Option<Decimal>,
    pub max_value: Option<Decimal>,
    pub gamma: Decimal, // γ_i coupling coefficient, default 0 per CORRECTIONS
    pub is_scaffold: bool, // For Telomere, MitoROS, EpigeneticDrift, Proteostasis
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateParameter {
    pub domain_id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<Decimal>,
    pub unit: Option<String>,
    pub source: Option<String>,
    pub status: Option<String>,
    pub min_value: Option<Decimal>,
    pub max_value: Option<Decimal>,
    pub gamma: Option<Decimal>, // Default to 0 if not provided
    pub is_scaffold: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParameter {
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_value: Option<Decimal>,
    pub unit: Option<String>,
    pub source: Option<String>,
    pub status: Option<String>,
    pub min_value: Option<Decimal>,
    pub max_value: Option<Decimal>,
    pub gamma: Option<Decimal>,
    pub is_scaffold: Option<bool>,
}

// Individual entity - represents a simulated individual
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Individual {
    pub id: Uuid,
    pub cohort_id: Option<Uuid>,
    pub birth_date: Option<NaiveDate>,
    pub sex: Option<String>, // "M", "F", or other
    pub simulated: bool,
    #[validate(range(min = 0.0, max = 120.0))]
    pub initial_age: f64,
    #[validate(range(min = 0.0, max = 120.0))]
    pub max_age: f64,
    pub transition_count: Option<i64>,
    pub metamorphosis_count: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateIndividual {
    pub cohort_id: Option<Uuid>,
    pub birth_date: Option<NaiveDate>,
    pub sex: Option<String>,
    pub simulated: Option<bool>,
    #[validate(range(min = 0.0, max = 120.0))]
    pub initial_age: Option<f64>,
    #[validate(range(min = 0.0, max = 120.0))]
    pub max_age: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIndividual {
    pub cohort_id: Option<Uuid>,
    pub birth_date: Option<NaiveDate>,
    pub sex: Option<String>,
    pub simulated: Option<bool>,
    pub initial_age: Option<f64>,
    pub max_age: Option<f64>,
}

// Transition entity - parameter change at a specific age
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Transition {
    pub id: Uuid,
    pub individual_id: Uuid,
    pub parameter_id: Uuid,
    pub parameter_name: Option<String>, // Joined from parameters
    pub domain_name: Option<String>, // Joined from domains via parameters
    #[validate(range(min = 0.0, max = 120.0))]
    pub age: f64, // Age in years
    pub value: Decimal,
    pub change_score: Option<Decimal>, // LCS change score
    pub previous_value: Option<Decimal>,
    pub domain_coupling: Option<Decimal>, // γ coupling from other domains
    pub is_significant: bool,
    pub fdr_q_value: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTransition {
    pub individual_id: Uuid,
    pub parameter_id: Uuid,
    #[validate(range(min = 0.0, max = 120.0))]
    pub age: f64,
    pub value: Decimal,
    pub change_score: Option<Decimal>,
    pub previous_value: Option<Decimal>,
    pub domain_coupling: Option<Decimal>,
    pub is_significant: Option<bool>,
    pub fdr_q_value: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTransition {
    pub age: Option<f64>,
    pub value: Option<Decimal>,
    pub change_score: Option<Decimal>,
    pub previous_value: Option<Decimal>,
    pub domain_coupling: Option<Decimal>,
    pub is_significant: Option<bool>,
    pub fdr_q_value: Option<Decimal>,
}

// Metamorphosis entity - cluster of transitions in ≥2 domains within 6 months
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Metamorphosis {
    pub id: Uuid,
    pub individual_id: Uuid,
    #[validate(range(min = 0.0, max = 120.0))]
    pub age: f64, // Central age of the cluster
    pub cluster_radius: f64, // In months, default 6
    pub domain_count: i32, // Should be ≥2
    pub transition_count: Option<i64>,
    pub is_valid: bool,
    pub fdr_corrected: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateMetamorphosis {
    pub individual_id: Uuid,
    #[validate(range(min = 0.0, max = 120.0))]
    pub age: f64,
    pub cluster_radius: Option<f64>,
    #[validate(range(min = 2))]
    pub domain_count: i32,
    pub transition_ids: Option<Vec<Uuid>>,
    pub is_valid: Option<bool>,
    pub fdr_corrected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMetamorphosis {
    pub age: Option<f64>,
    pub cluster_radius: Option<f64>,
    pub domain_count: Option<i32>,
    pub is_valid: Option<bool>,
    pub fdr_corrected: Option<bool>,
}