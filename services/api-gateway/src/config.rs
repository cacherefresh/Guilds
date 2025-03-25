use serde::Deserialize;
use config::{Config as ConfigLib, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub quest_service_url: String,
    pub guild_service_url: String,
    pub character_service_url: String,
    pub reward_service_url: String,
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
        // E.g. `APP_DEBUG=1` would set the `debug` key
        config.merge(Environment::with_prefix("APP").separator("_"))?;
        
        // Try to convert the configuration values it read into our Config type
        config.try_into()
    }
} 