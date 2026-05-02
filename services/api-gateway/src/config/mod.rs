use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum StorageMode {
    Postgres,
    FileSystem,
}

#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub storage_mode: StorageMode,
    pub database_url: String,
    pub game_save_path: String,
    pub game_id: Option<String>,
    pub service_urls: ServiceUrls,
}

#[derive(Debug, Clone)]
pub struct ServiceUrls {
    pub quest_service: String,
    pub character_service: String,
    pub guild_service: String,
    pub reward_service: String,
}

impl Default for ServiceUrls {
    fn default() -> Self {
        Self {
            quest_service: "http://quest-service:8081".to_string(),
            character_service: "http://character-service:8082".to_string(),
            guild_service: "http://guild-service:8083".to_string(),
            reward_service: "http://reward-service:8084".to_string(),
        }
    }
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            storage_mode: StorageMode::Postgres,
            database_url: "postgres://postgres:postgres@localhost:5432/guilds".to_string(),
            game_save_path: "GameSave".to_string(),
            game_id: None,
            service_urls: ServiceUrls::default(),
        }
    }
}

impl GatewayConfig {
    pub fn new() -> Self {
        let mut config = GatewayConfig::default();
        
        // Read storage mode from environment
        if let Ok(mode) = env::var("STORAGE_MODE") {
            config.storage_mode = match mode.to_lowercase().as_str() {
                "filesystem" | "file" => StorageMode::FileSystem,
                _ => StorageMode::Postgres,
            };
        }
        
        // Read database URL if using Postgres
        if let Ok(url) = env::var("DATABASE_URL") {
            config.database_url = url;
        }
        
        // Read game save path
        if let Ok(path) = env::var("GAME_SAVE_PATH") {
            config.game_save_path = path;
        }
        
        // Read game ID if set
        if let Ok(id) = env::var("GAME_ID") {
            config.game_id = Some(id);
        }
        
        // Read service URLs
        if let Ok(url) = env::var("QUEST_SERVICE_URL") {
            config.service_urls.quest_service = url;
        }
        
        if let Ok(url) = env::var("CHARACTER_SERVICE_URL") {
            config.service_urls.character_service = url;
        }
        
        if let Ok(url) = env::var("GUILD_SERVICE_URL") {
            config.service_urls.guild_service = url;
        }
        
        if let Ok(url) = env::var("REWARD_SERVICE_URL") {
            config.service_urls.reward_service = url;
        }
        
        config
    }
    
    // Initialize a new game
    pub fn init_new_game(&mut self, game_id: &str) -> Result<(), String> {
        self.game_id = Some(game_id.to_string());
        
        // Create game save directory if it doesn't exist
        let game_save_dir = format!("{}/{}", self.game_save_path, game_id);
        if !Path::new(&game_save_dir).exists() {
            fs::create_dir_all(&game_save_dir)
                .map_err(|e| format!("Failed to create game save directory: {}", e))?;
            
            // Copy template structure
            copy_dir_all("GameSaveTemplate", &game_save_dir)
                .map_err(|e| format!("Failed to copy template: {}", e))?;
        }
        
        // Set environment variable for microservices
        env::set_var("GAME_ID", game_id);
        
        Ok(())
    }
    
    // List available games
    pub fn list_games(&self) -> Result<Vec<String>, String> {
        let path = Path::new(&self.game_save_path);
        if !path.exists() {
            return Ok(vec![]);
        }
        
        let mut games = Vec::new();
        
        for entry in fs::read_dir(path).map_err(|e| format!("Failed to read GameSave directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(game_id) = path.file_name().and_then(|n| n.to_str()) {
                    games.push(game_id.to_string());
                }
            }
        }
        
        Ok(games)
    }
}

// Helper function to recursively copy directories
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.as_ref().join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(src_path, dst_path)?;
        } else {
            fs::copy(src_path, dst_path)?;
        }
    }
    
    Ok(())
} 