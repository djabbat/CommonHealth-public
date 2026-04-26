use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;
use crate::error::AppError;

pub struct DbPool(PgPool);

impl DbPool {
    pub async fn connect(database_url: &str) -> Result<Self, AppError> {
        info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        info!("Database connection established");
        Ok(Self(pool))
    }
}

impl std::ops::Deref for DbPool {
    type Target = PgPool;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}