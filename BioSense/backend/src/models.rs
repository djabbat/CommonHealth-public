use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use std::collections::HashMap;

// Device models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Device {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub serial_number: String,
    pub device_type: String,
    pub firmware_version: String,
    #[serde(default)]
    pub hardware_parameters: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDevice {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub serial_number: String,
    pub device_type: String,
    pub firmware_version: String,
    #[serde(default)]
    pub hardware_parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDevice {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub serial_number: Option<String>,
    pub device_type: Option<String>,
    pub firmware_version: Option<String>,
    pub hardware_parameters: Option<serde_json::Value>,
}

// EEG Measurement models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EegMeasurement {
    pub id: Uuid,
    pub device_id: Uuid,
    pub session_id: Option<Uuid>,
    pub subject_id: Option<String>,
    pub recording_started_at: DateTime<Utc>,
    pub recording_ended_at: DateTime<Utc>,
    pub sampling_rate_hz: i32,
    pub channel_labels: Vec<String>,
    pub channel_data: serde_json::Value, // JSON array of arrays
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateEegMeasurement {
    pub device_id: Uuid,
    pub session_id: Option<Uuid>,
    #[validate(length(max = 100))]
    pub subject_id: Option<String>,
    #[validate(required)]
    pub recording_started_at: Option<DateTime<Utc>>,
    #[validate(required)]
    pub recording_ended_at: Option<DateTime<Utc>>,
    #[validate(range(min = 1, max = 10000))]
    pub sampling_rate_hz: i32,
    #[validate(length(min = 1, max = 32))]
    pub channel_labels: Vec<String>,
    #[validate(required)]
    pub channel_data: Option<serde_json::Value>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

// HRV Measurement models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HrvMeasurement {
    pub id: Uuid,
    pub device_id: Uuid,
    pub session_id: Option<Uuid>,
    pub subject_id: Option<String>,
    pub recording_started_at: DateTime<Utc>,
    pub recording_ended_at: DateTime<Utc>,
    pub rr_intervals_ms: Vec<f64>, // PostgreSQL array of floats
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateHrvMeasurement {
    pub device_id: Uuid,
    pub session_id: Option<Uuid>,
    #[validate(length(max = 100))]
    pub subject_id: Option<String>,
    #[validate(required)]
    pub recording_started_at: Option<DateTime<Utc>>,
    #[validate(required)]
    pub recording_ended_at: Option<DateTime<Utc>>,
    #[validate(length(min = 1))]
    pub rr_intervals_ms: Vec<f64>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

// Olfaction Measurement models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OlfactionMeasurement {
    pub id: Uuid,
    pub device_id: Uuid,
    pub session_id: Option<Uuid>,
    pub subject_id: Option<String>,
    pub recording_started_at: DateTime<Utc>,
    pub sensor_readings: serde_json::Value, // JSON array of sensor values
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOlfactionMeasurement {
    pub device_id: Uuid,
    pub session_id: Option<Uuid>,
    #[validate(length(max = 100))]
    pub subject_id: Option<String>,
    #[validate(required)]
    pub recording_started_at: Option<DateTime<Utc>>,
    #[validate(required)]
    pub sensor_readings: Option<serde_json::Value>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

// Session models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Session {
    pub id: Uuid,
    #[validate(length(max = 100))]
    pub subject_id: Option<String>,
    pub device_id: Uuid,
    pub protocol_type: String,
    #[serde(default)]
    pub environment_conditions: serde_json::Value,
    #[serde(default)]
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateSession {
    #[validate(length(max = 100))]
    pub subject_id: Option<String>,
    pub device_id: Uuid,
    pub protocol_type: String,
    #[serde(default)]
    pub environment_conditions: serde_json::Value,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateSession {
    #[validate(length(max = 100))]
    pub subject_id: Option<String>,
    pub device_id: Option<Uuid>,
    pub protocol_type: Option<String>,
    pub environment_conditions: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

// Note: No χ_Ze computation or storage - only raw data storage per CORRECTIONS
// Ze analysis is performed client-side or in separate processing pipelines