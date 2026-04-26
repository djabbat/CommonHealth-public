use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::error::DatabaseError;
use thiserror::Error;
use tracing::error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("Error occurred: {}", self);
        
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_unique_violation() {
                        (StatusCode::CONFLICT, "Unique constraint violation".to_string())
                    } else if db_err.is_foreign_key_violation() {
                        (StatusCode::BAD_REQUEST, "Foreign key constraint violation".to_string())
                    } else {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
                    }
                } else if e.to_string().contains("no rows returned") {
                    (StatusCode::NOT_FOUND, "Resource not found".to_string())
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
                }
            }
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error".to_string()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "type": match self {
                    AppError::Database(_) => "database_error",
                    AppError::Config(_) => "config_error",
                    AppError::Validation(_) => "validation_error",
                    AppError::NotFound(_) => "not_found",
                    AppError::Internal => "internal_error",
                }
            }
        }));

        (status, body).into_response()
    }
}