use sqlx::{PgPool, postgres::PgPoolOptions, migrate::MigrateDatabase};
use tracing::info;
use crate::error::{AppError, AppResult};

#[derive(Clone)]
pub struct DbPool(PgPool);

impl DbPool {
    pub async fn connect(database_url: &str) -> AppResult<Self> {
        // Create database if it doesn't exist
        if !sqlx::Postgres::database_exists(database_url).await? {
            info!("Database does not exist, creating...");
            sqlx::Postgres::create_database(database_url).await?;
        }

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(Self(pool))
    }

    pub async fn run_migrations(&self) -> AppResult<()> {
        info!("Running database migrations...");
        
        sqlx::migrate!("./migrations")
            .run(&self.0)
            .await
            .map_err(AppError::from)?;
        
        info!("Database migrations completed");
        Ok(())
    }
}

impl std::ops::Deref for DbPool {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}