use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::error::AppError;

pub type DbPool = Pool<Postgres>;

pub async fn init_pool(database_url: &str) -> Result<DbPool, AppError> {
    info!("Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;

    // Test connection
    sqlx::query("SELECT 1").execute(&pool).await?;
    info!("Database connection established successfully");

    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), AppError> {
    info!("Running database migrations...");
    
    // Create migrations table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _sqlx_migrations (
            version BIGINT PRIMARY KEY,
            description TEXT NOT NULL,
            installed_on TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            success BOOLEAN NOT NULL,
            checksum BYTEA NOT NULL,
            execution_time BIGINT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    // Run migrations from file system (we'll have a separate migration system)
    // For now, we'll rely on the initial.sql being run manually or via separate tool
    info!("Migrations table ready - run initial.sql manually");
    
    Ok(())
}