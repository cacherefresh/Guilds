mod config;
mod handlers;
mod models;
mod repository;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use config::Config;
use repository::db::init_pool;
use log::info;
use std::io;
use dotenvy::dotenv;
use log::{info, error};
use std::env;
use tokio_postgres::{NoTls, Error};
use utoipa::{OpenApi, openapi::security::{SecurityScheme, ApiKeyAuth, ApiKey, Location}};
use utoipa_swagger_ui::SwaggerUi;

use quest_service::{
    AppState,
    routes::create_router,
    models::{quest, epic, adventure, terminology},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Quest handlers
        quest_service::handlers::get_quests,
        quest_service::handlers::get_quest_by_id,
        quest_service::handlers::create_quest,
        quest_service::handlers::update_quest,
        quest_service::handlers::delete_quest,
        // Epic handlers
        quest_service::handlers::get_epics,
        quest_service::handlers::get_epic_by_id,
        quest_service::handlers::get_epic_details,
        quest_service::handlers::create_epic,
        quest_service::handlers::update_epic,
        quest_service::handlers::delete_epic,
        // Adventure handlers
        quest_service::handlers::get_adventures,
        quest_service::handlers::get_adventure_by_id,
        quest_service::handlers::get_adventure_details,
        quest_service::handlers::create_adventure,
        quest_service::handlers::update_adventure,
        quest_service::handlers::delete_adventure,
        // Terminology handlers
        quest_service::handlers::get_all_terminology_settings,
        quest_service::handlers::get_terminology_setting_by_id,
        quest_service::handlers::get_current_terminology,
        quest_service::handlers::create_terminology_setting,
        quest_service::handlers::update_terminology_setting,
        quest_service::handlers::set_current_terminology,
        quest_service::handlers::delete_terminology_setting,
        // Health check
        quest_service::handlers::health_check,
    ),
    components(
        schemas(
            // Quest schemas
            quest::Quest,
            quest::QuestCreate,
            quest::QuestUpdate,
            quest::QuestListResponse,
            quest::QuestDetail,
            quest::QuestPrerequisite,
            quest::PrerequisiteRequest,
            quest::QuestQuery,
            // Epic schemas
            epic::Epic,
            epic::EpicCreate,
            epic::EpicUpdate,
            epic::EpicListResponse,
            epic::EpicDetail,
            epic::EpicQuery,
            // Adventure schemas
            adventure::Adventure,
            adventure::AdventureCreate,
            adventure::AdventureUpdate,
            adventure::AdventureListResponse,
            adventure::AdventureDetail,
            adventure::AdventureQuery,
            // Terminology schemas
            terminology::TerminologySetting, 
            terminology::TerminologyCreate,
            terminology::TerminologyUpdate,
            // Error response
            quest_service::error::ErrorResponse,
        )
    ),
    tags(
        (name = "Quests", description = "Quest management endpoints"),
        (name = "Epics", description = "Epic management endpoints"),
        (name = "Adventures", description = "Adventure management endpoints"),
        (name = "Terminology", description = "Terminology settings endpoints"),
        (name = "Health", description = "Health check endpoints"),
    ),
    info(
        title = "Guild Quest API",
        description = "API for managing quests, epics, adventures, and terminology",
        version = "1.0.0"
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Load configuration
    let config = Config::from_env().expect("Server configuration");
    
    // Initialize database connection pool
    let pool = init_pool(&config.database_url).expect("Failed to initialize database pool");
    
    info!("Starting Quest Service at http://{}:{}", config.host, config.port);
    
    // Create and start HTTP server
    HttpServer::new(move || {
        // CORS configuration
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(pool.clone()))
            // Create repositories
            .app_data(web::Data::new(repository::quest::QuestRepository::new(pool.clone())))
            .app_data(web::Data::new(repository::epic::EpicRepository::new(pool.clone())))
            .app_data(web::Data::new(repository::adventure::AdventureRepository::new(pool.clone())))
            .app_data(web::Data::new(repository::terminology::TerminologyRepository::new(pool.clone())))
            .app_data(web::Data::new(repository::language_pack::LanguagePackRepository::new(pool.clone())))
            // API routes will be added here
            .service(
                web::scope("/quests")
                    .configure(handlers::quest::configure_routes)
            )
            .service(handlers::health::health_check)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Get database connection details from environment variables
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "guilds".to_string());
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    
    // Build connection string
    let conn_string = format!(
        "host={} port={} dbname={} user={} password={}",
        db_host, db_port, db_name, db_user, db_password
    );
    
    // Connect to the database
    info!("Connecting to database at {}:{}/{}", db_host, db_port, db_name);
    let (client, connection) = tokio_postgres::connect(&conn_string, NoTls).await?;
    
    // Spawn a task to run the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Database connection error: {}", e);
        }
    });
    
    // Create app state
    let app_state = AppState::new(client);
    
    // Get the port from environment variables or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
    
    // Create the router
    let app = create_router(app_state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
    
    // Start the server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    
    Ok(())
} 