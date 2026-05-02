use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum StorageMode {
    Postgres,
    FileSystem,
}

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub storage_mode: StorageMode,
    pub database_url: String,
    pub game_save_path: String,
    pub game_id: Option<String>,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            storage_mode: StorageMode::Postgres,
            database_url: "postgres://postgres:postgres@localhost:5432/guilds".to_string(),
            game_save_path: "GameSave".to_string(),
            game_id: None,
        }
    }
}

impl ServiceConfig {
    pub fn new() -> Self {
        let mut config = ServiceConfig::default();
        
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
        
        config
    }
    
    // Get the path for a specific table's CSV file
    pub fn get_csv_path(&self, table_name: &str) -> String {
        match &self.game_id {
            Some(game_id) => {
                format!("{}/{}/guild-service/{}.csv", self.game_save_path, game_id, table_name)
            }
            None => {
                // Fallback to template if no game ID is set
                format!("GameSaveTemplate/guild-service/{}.csv", table_name)
            }
        }
    }
    
    // Initialize a new game save
    pub fn init_game_save(&mut self, game_id: &str) -> Result<(), String> {
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
        
        Ok(())
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

// CSV related utilities
pub mod csv {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Write};
    use std::path::Path;
    
    // Escape commas in CSV data
    pub fn escape_csv(text: &str) -> String {
        text.replace(",", "&COMMA")
    }
    
    // Unescape commas in CSV data
    pub fn unescape_csv(text: &str) -> String {
        text.replace("&COMMA", ",")
    }
    
    // Write data to CSV file
    pub fn write_csv<P: AsRef<Path>>(path: P, headers: &[&str], rows: &[Vec<String>]) -> io::Result<()> {
        let dir = path.as_ref().parent().unwrap();
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }
        
        let mut file = File::create(path)?;
        
        // Write headers
        let header_line = headers.join(",") + "\n";
        file.write_all(header_line.as_bytes())?;
        
        // Write rows
        for row in rows {
            let row_escaped: Vec<String> = row.iter().map(|cell| escape_csv(cell)).collect();
            let row_line = row_escaped.join(",") + "\n";
            file.write_all(row_line.as_bytes())?;
        }
        
        Ok(())
    }
    
    // Read data from CSV file
    pub fn read_csv<P: AsRef<Path>>(path: P) -> io::Result<(Vec<String>, Vec<Vec<String>>)> {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    // Create an empty file if it doesn't exist
                    let dummy_header = vec!["id"];
                    write_csv(&path, &dummy_header, &[])?;
                    File::open(&path)?
                } else {
                    return Err(e);
                }
            }
        };
        
        let reader = BufReader::new(file);
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        
        // Read and parse lines
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let columns: Vec<String> = line
                .split(',')
                .map(|cell| unescape_csv(cell.trim()))
                .collect();
            
            if i == 0 {
                headers = columns;
            } else {
                rows.push(columns);
            }
        }
        
        Ok((headers, rows))
    }
} 