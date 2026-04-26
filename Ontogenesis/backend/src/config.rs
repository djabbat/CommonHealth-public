use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub log_level: String,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder();

        // Try to load from .env file
        let _ = dotenv::dotenv();

        // Set defaults
        builder = builder
            .set_default("database_url", "postgres://cn:cn@localhost/ontogenesis_db")?
            .set_default("port", 3011)?
            .set_default("log_level", "info")?
            .set_default("cors_origin", "*")?;

        // Override with environment variables
        if let Ok(db_url) = env::var("DATABASE_URL") {
            builder = builder.set_override("database_url", db_url)?;
        }

        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("port", port.parse::<u16>().unwrap_or(3011))?;
        }

        if let Ok(log_level) = env::var("LOG_LEVEL") {
            builder = builder.set_override("log_level", log_level)?;
        }

        if let Ok(cors_origin) = env::var("CORS_ORIGIN") {
            builder = builder.set_override("cors_origin", cors_origin)?;
        }

        let config = builder.build()?;
        config.try_deserialize()
    }
}