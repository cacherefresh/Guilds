use deadpool_postgres::{Config, Pool, PoolError, Runtime};
use tokio_postgres::NoTls;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database pool error: {0}")]
    PoolError(#[from] PoolError),
    
    #[error("Database query error: {0}")]
    PostgresError(#[from] tokio_postgres::Error),
    
    #[error("No data returned")]
    NoDataReturned,
    
    #[error("Unknown error: {0}")]
    Other(String),
}

pub fn init_pool(connection_string: &str) -> Result<Pool, PoolError> {
    let mut config = Config::new();
    config.url = Some(connection_string.to_string());
    config.manager = Some(deadpool_postgres::ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast
    });
    
    config.create_pool(Some(Runtime::Tokio1), NoTls)
} 