use serde::Deserialize;
use std::env;
use crate::error::AppResult;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn load() -> AppResult<Self> {
        dotenvy::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3002".to_string())
            .parse()
            .unwrap_or(3002);

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://cn:cn@localhost/mcoa_db".to_string());

        Ok(Self {
            port,
            database_url,
        })
    }
}