use serde::Deserialize;
use crate::error::AppError;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenv::dotenv().ok();
        
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://cn:cn@localhost/mitoros_db".to_string());
        
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3006".to_string())
            .parse::<u16>()
            .map_err(|e| AppError::ConfigError(format!("Invalid PORT: {}", e)))?;
        
        Ok(Self {
            database_url,
            port,
        })
    }
}