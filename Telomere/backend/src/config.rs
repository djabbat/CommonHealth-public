use serde::Deserialize;
use config::{Config as ConfigBuilder, File, Environment};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
    pub cors_origins: Vec<String>,
    pub enable_compression: bool,
    pub enable_tracing: bool,
}

impl Config {
    pub fn load() -> Result<Self, crate::error::AppError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        
        let config = ConfigBuilder::builder()
            // Start with default values
            .set_default("port", 3005)?
            .set_default("database_url", "postgres://cn:cn@localhost/telomere_db")?
            .set_default("log_level", "info")?
            .set_default("cors_origins", Vec::<String>::new())?
            .set_default("enable_compression", true)?
            .set_default("enable_tracing", true)?
            // Add configuration file
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            // Add environment variables with prefix "TELOMERE_"
            .add_source(Environment::with_prefix("TELOMERE").separator("__"))
            .build()?;
        
        let config: Config = config.try_deserialize()?;
        
        // Override with environment variables if present
        if let Ok(port) = env::var("PORT") {
            if let Ok(port) = port.parse() {
                let mut config = config;
                config.port = port;
                return Ok(config);
            }
        }
        
        Ok(config)
    }
}