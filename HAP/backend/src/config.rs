use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3010".to_string())
            .parse()
            .unwrap_or(3010);
        
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://cn:cn@localhost/hap_db".to_string());

        Ok(Config {
            port,
            database_url,
        })
    }
}