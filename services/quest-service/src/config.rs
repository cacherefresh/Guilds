use serde::Deserialize;
use config::{Config as ConfigLib, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub database_schema: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut config = ConfigLib::default();
        
        // Path to configuration file
        let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "./config".to_string());
        
        // Load default configuration
        config.merge(File::with_name(&format!("{}/default", config_path)).required(false))?;
        
        // Load environment-specific configuration
        let env = env::var("RUN_ENV").unwrap_or_else(|_| "development".to_string());
        config.merge(File::with_name(&format!("{}/{}", config_path, env)).required(false))?;
        
        // Load secrets from environment variables
        if let Ok(secret_path) = env::var("SECRETS_PATH") {
            let secrets_file = format!("{}/database.env", secret_path);
            config.merge(File::with_name(&secrets_file).required(false))?;
        }
        
        // Add in settings from environment variables (with a prefix of APP and '_' as separator)
        config.merge(Environment::with_prefix("APP").separator("_"))?;
        
        // Build database URL from environment variables if not explicitly set
        if let Ok(user) = env::var("DB_USER") {
            if let Ok(password) = env::var("DB_PASSWORD") {
                if let Ok(host) = env::var("DB_HOST") {
                    if let Ok(port) = env::var("DB_PORT") {
                        if let Ok(name) = env::var("DB_NAME") {
                            let database_url = format!("postgres://{}:{}@{}:{}/{}", 
                                user, password, host, port, name);
                            let mut cfg_map = config.clone().try_into::<std::collections::HashMap<String, String>>()?;
                            cfg_map.insert("database_url".to_string(), database_url);
                            config = ConfigLib::try_from(cfg_map)?;
                        }
                    }
                }
            }
        }
        
        // Try to convert the configuration values it read into our Config type
        config.try_into()
    }
} 