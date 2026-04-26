use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;
use std::time::Duration;

pub type DbPool = PgPool;

pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
}

impl DbConfig {
    pub fn from_url(url: &str) -> Self {
        Self {
            url: url.to_string(),
            max_connections: 20,
            min_connections: 5,
            connect_timeout: Duration::from_secs(10),
        }
    }
}

impl DbPool {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let config = DbConfig::from_url(database_url);
        
        info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect_timeout(config.connect_timeout)
            .connect(&config.url)
            .await?;
            
        info!("Database connection pool created");
        
        Ok(pool)
    }
    
    pub async fn test_connection(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1")
            .execute(self)
            .await?;
        Ok(())
    }
}