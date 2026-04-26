use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub env: String,
    pub log_level: String,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let environment = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        
        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .set_default("port", 3004)?
            .set_default("env", environment)?
            .set_default("log_level", "info")?
            .set_default(
                "database_url",
                "postgres://cn:cn@localhost/biosense_db",
            )?
            .build()?;

        config.try_deserialize()
    }
}