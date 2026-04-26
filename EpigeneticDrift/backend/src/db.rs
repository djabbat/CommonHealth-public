use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::{info, error};

#[derive(Clone)]
pub struct DbPool {
    pool: Pool<Postgres>,
}

impl DbPool {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        info!("Database connection established");
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        info!("Running database migrations...");
        
        match sqlx::migrate!("./migrations").run(&self.pool).await {
            Ok(_) => {
                info!("Migrations completed successfully");
                Ok(())
            }
            Err(e) => {
                error!("Migration error: {}", e);
                Err(e)
            }
        }
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

impl std::ops::Deref for DbPool {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}