use std::env;
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3008".to_string())
            .parse()
            .unwrap_or(3008);
            
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://cn:cn@localhost/proteostasis_db".to_string());
            
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        
        Ok(Self {
            port,
            database_url,
            log_level,
        })
    }
}