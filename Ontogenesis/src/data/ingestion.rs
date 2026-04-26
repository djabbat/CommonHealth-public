// Ontogenesis v4.1 — src/data/ingestion.rs
// DataIngestion: import CSV, JSON, text-based sources
// Normalizes to unified DataRecord with age in months

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::path::Path;

/// Type of developmental parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    /// Anatomical: height, weight, head circumference, bone density, etc.
    Anatomical,
    /// Endocrine: GH, IGF-1, sex steroids, cortisol, etc.
    Endocrine,
    /// Cognitive: WISC/WAIS normative scores
    Cognitive,
    /// Psychological: CBCL/ASEBA normative data
    Psychological,
    /// Social: GSS/ESS events, network size
    Social,
}

/// A single measurement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRecord {
    /// Age in months (0–300)
    pub age_months: f64,
    /// Measured value (normalized to standard units)
    pub value: f64,
    /// Optional individual ID (for longitudinal data; None for cross-sectional)
    pub individual_id: Option<String>,
    /// Parameter name (e.g., "height_cm", "IGF1_ng_ml")
    pub parameter: String,
    /// Data type
    pub data_type: DataType,
    /// Data source label
    pub source: Option<String>,
    /// Sex: "M", "F", "mixed", or None
    pub sex: Option<String>,
}

/// Errors during ingestion
#[derive(Error, Debug)]
pub enum IngestionError {
    #[error("CSV parse error: {0}")]
    Csv(#[from] csv::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid age value: {0}")]
    InvalidAge(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Raw CSV row (flexible schema)
#[derive(Debug, Deserialize)]
struct CsvRow {
    /// Age — can be "age_months", "age_years", "age_days"
    #[serde(alias = "age_months", alias = "age_years", alias = "age_days", alias = "age")]
    age: f64,
    /// Unit of age: "months", "years", "days"
    #[serde(default)]
    age_unit: String,
    value: f64,
    parameter: String,
    #[serde(default)]
    individual_id: Option<String>,
    #[serde(default)]
    data_type: Option<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    sex: Option<String>,
}

/// Data ingestion pipeline
pub struct DataIngestion;

impl DataIngestion {
    /// Load records from a CSV file.
    /// Supports age in months, years, or days (auto-detected from `age_unit` column or filename).
    pub fn from_csv<P: AsRef<Path>>(path: P, data_type: DataType) -> Result<Vec<DataRecord>, IngestionError> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_path(path)?;

        let mut records = Vec::new();
        for result in reader.deserialize::<CsvRow>() {
            let row = result?;
            let age_months = Self::to_months(row.age, &row.age_unit)?;
            records.push(DataRecord {
                age_months,
                value: row.value,
                individual_id: row.individual_id,
                parameter: row.parameter,
                data_type: row.data_type
                    .as_deref()
                    .map(|s| Self::parse_data_type(s))
                    .unwrap_or_else(|| data_type.clone()),
                source: row.source,
                sex: row.sex,
            });
        }
        Ok(records)
    }

    /// Load records from a JSON array of DataRecord
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Vec<DataRecord>, IngestionError> {
        let content = std::fs::read_to_string(path)?;
        let records: Vec<DataRecord> = serde_json::from_str(&content)?;
        Ok(records)
    }

    /// Convert age to months based on unit string
    fn to_months(age: f64, unit: &str) -> Result<f64, IngestionError> {
        if age < 0.0 {
            return Err(IngestionError::InvalidAge(format!("Negative age: {age}")));
        }
        let months = match unit.to_lowercase().trim() {
            "years" | "year" | "y" | "yr" => age * 12.0,
            "days" | "day" | "d" => age / 30.4375,
            "weeks" | "week" | "w" | "wk" => age * 7.0 / 30.4375,
            _ => age, // default: months
        };
        if months > 600.0 {
            return Err(IngestionError::InvalidAge(format!("Age too large: {months} months")));
        }
        Ok(months)
    }

    fn parse_data_type(s: &str) -> DataType {
        match s.to_lowercase().as_str() {
            "anatomical" | "anat" => DataType::Anatomical,
            "endocrine" | "endo" => DataType::Endocrine,
            "cognitive" | "cog" => DataType::Cognitive,
            "psychological" | "psych" => DataType::Psychological,
            "social" => DataType::Social,
            _ => DataType::Anatomical,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_months_years() {
        let m = DataIngestion::to_months(2.0, "years").unwrap();
        assert!((m - 24.0).abs() < 0.001);
    }

    #[test]
    fn test_to_months_default() {
        let m = DataIngestion::to_months(18.0, "").unwrap();
        assert!((m - 18.0).abs() < 0.001);
    }

    #[test]
    fn test_negative_age_rejected() {
        assert!(DataIngestion::to_months(-1.0, "months").is_err());
    }
}
