use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Error};
use tracing::info;

pub async fn get_pool(database_url: &str) -> Result<PgPool, Error> {
    info!("Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;
    
    info!("Database connection established");
    
    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), Error> {
    info!("Running database migrations...");
    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    info!("Migrations completed");
    
    Ok(())
}