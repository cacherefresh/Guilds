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