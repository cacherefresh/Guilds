use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::GatewayConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGameRequest {
    pub game_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameListResponse {
    pub games: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageModeRequest {
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageModeResponse {
    pub mode: String,
}

// Create a new game save
pub async fn create_new_game(
    req: web::Json<NewGameRequest>,
    config: web::Data<std::sync::Mutex<GatewayConfig>>,
) -> impl Responder {
    let mut config_guard = match config.lock() {
        Ok(guard) => guard,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to acquire config lock"),
    };

    // Generate a UUID if no game ID provided
    let game_id = match &req.game_id {
        Some(id) => id.clone(),
        None => Uuid::new_v4().to_string(),
    };

    // Initialize new game save
    match config_guard.init_new_game(&game_id) {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "game_id": game_id,
            "message": "Game save created successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to create game save: {}", e)
        })),
    }
}

// Get a list of available games
pub async fn list_games(
    config: web::Data<std::sync::Mutex<GatewayConfig>>,
) -> impl Responder {
    let config_guard = match config.lock() {
        Ok(guard) => guard,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to acquire config lock"),
    };

    match config_guard.list_games() {
        Ok(games) => HttpResponse::Ok().json(GameListResponse { games }),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to list games: {}", e)
        })),
    }
}

// Switch to a different game
pub async fn switch_game(
    game_id: web::Path<String>,
    config: web::Data<std::sync::Mutex<GatewayConfig>>,
) -> impl Responder {
    let mut config_guard = match config.lock() {
        Ok(guard) => guard,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to acquire config lock"),
    };

    config_guard.game_id = Some(game_id.to_string());
    std::env::set_var("GAME_ID", game_id.as_str());

    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Switched to game: {}", game_id),
        "game_id": game_id.as_str()
    }))
}

// Set storage mode
pub async fn set_storage_mode(
    req: web::Json<StorageModeRequest>,
    config: web::Data<std::sync::Mutex<GatewayConfig>>,
) -> impl Responder {
    let mut config_guard = match config.lock() {
        Ok(guard) => guard,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to acquire config lock"),
    };

    let mode = req.mode.to_lowercase();
    let mode_str = mode.as_str();

    // Set storage mode
    config_guard.storage_mode = match mode_str {
        "filesystem" | "file" => crate::config::StorageMode::FileSystem,
        "postgres" | "db" | "database" => crate::config::StorageMode::Postgres,
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid storage mode. Use 'filesystem' or 'postgres'"
            }));
        }
    };

    // Set environment variable
    std::env::set_var("STORAGE_MODE", mode_str);

    HttpResponse::Ok().json(StorageModeResponse { mode })
}

// Get current storage mode
pub async fn get_storage_mode(
    config: web::Data<std::sync::Mutex<GatewayConfig>>,
) -> impl Responder {
    let config_guard = match config.lock() {
        Ok(guard) => guard,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to acquire config lock"),
    };

    let mode = match config_guard.storage_mode {
        crate::config::StorageMode::Postgres => "postgres",
        crate::config::StorageMode::FileSystem => "filesystem",
    };

    HttpResponse::Ok().json(StorageModeResponse { mode: mode.to_string() })
}

// Register game save routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/game-save")
            .route("", web::post().to(create_new_game))
            .route("", web::get().to(list_games))
            .route("/{game_id}", web::put().to(switch_game))
            .route("/storage-mode", web::put().to(set_storage_mode))
            .route("/storage-mode", web::get().to(get_storage_mode)),
    );
} 