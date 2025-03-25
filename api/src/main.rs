mod config;
mod db;
mod error;
mod handlers;
mod models;
mod repository;
mod schema;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::env;
use std::sync::Arc;

use crate::config::Config;
use crate::db::init_db_pool;
use crate::handlers::{character_handler, guild_handler, quest_handler, skill_handler, town_handler};
use crate::schema::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

use repository::{
    CharacterRepository, GuildRepository, QuestRepository, SkillRepository, TownRepository,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Configure database connection
    let config = Config {
        user: Some(env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string())),
        password: Some(env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string())),
        host: Some(env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string())),
        port: Some(env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap()),
        dbname: Some(env::var("DB_NAME").unwrap_or_else(|_| "guilds".to_string())),
        ..Default::default()
    };
    
    // Create database connection pool
    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    
    // Test database connection
    let client = pool.get().await.expect("Failed to connect to database");
    let result = client.query("SELECT 1", &[]).await.expect("Failed to execute query");
    assert_eq!(result[0].get::<_, i32>(0), 1);
    println!("Successfully connected to database");
    
    // Create Arc<Pool> to share across repositories
    let pool_arc = Arc::new(pool);
    
    // Initialize repositories
    let character_repo = Arc::new(CharacterRepository::new(pool_arc.clone()));
    let quest_repo = Arc::new(QuestRepository::new(pool_arc.clone()));
    let skill_repo = Arc::new(SkillRepository::new(pool_arc.clone()));
    let guild_repo = Arc::new(GuildRepository::new(pool_arc.clone()));
    let town_repo = Arc::new(TownRepository::new(pool_arc.clone()));
    
    // Create HTTP server
    let server = HttpServer::new(move || {
        // CORS configuration
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        // OpenAPI documentation
        let api_doc = ApiDoc::openapi();
        
        App::new()
            // Enable logger
            .wrap(middleware::Logger::default())
            // Enable CORS
            .wrap(cors)
            // App data
            .app_data(web::Data::new(pool_arc.clone()))
            .app_data(web::Data::new(character_repo.clone()))
            .app_data(web::Data::new(quest_repo.clone()))
            .app_data(web::Data::new(skill_repo.clone()))
            .app_data(web::Data::new(guild_repo.clone()))
            .app_data(web::Data::new(town_repo.clone()))
            // API routes
            .service(
                web::scope("/api/v1")
                    // Quest routes
                    .configure(quest_handler::configure)
                    // Skill routes
                    .configure(skill_handler::configure)
                    // Character routes
                    .configure(character_handler::configure)
                    // Guild routes
                    .configure(guild_handler::configure)
                    // Town routes
                    .configure(town_handler::configure)
            )
            // Swagger UI
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-docs/openapi.json", api_doc)
            )
    });
    
    // Get server host and port from environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();
    
    // Start server
    println!("Starting server at http://{}:{}", host, port);
    server.bind((host, port))?.run().await
} 