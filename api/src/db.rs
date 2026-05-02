use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use std::str::FromStr;

pub async fn init_db_pool(database_url: &str) -> Result<Pool, Box<dyn std::error::Error>> {
    let mut cfg = Config::new();
    
    // Parse the database URL
    let pg_config = tokio_postgres::Config::from_str(database_url)?;
    
    cfg.host = pg_config.get_hosts().iter().next().map(|h| h.to_string());
    cfg.port = pg_config.get_ports().iter().next().cloned();
    cfg.user = pg_config.get_user().map(|u| u.to_string());
    cfg.password = pg_config.get_password().map(|p| p.to_string());
    cfg.dbname = pg_config.get_dbname().map(|d| d.to_string());
    
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    
    // Verify that the connection works
    let client = pool.get().await?;
    let result = client.query_one("SELECT 1", &[]).await?;
    
    log::info!("Database connection verified: {}", result.get::<_, i32>(0));
    
    Ok(pool)
} 