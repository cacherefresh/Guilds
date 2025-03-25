mod config;
mod handlers;
mod middleware;
mod proxy;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use config::Config;
use log::info;
use std::io;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Load configuration
    let config = Config::from_env().expect("Server configuration");
    
    info!("Starting API Gateway at http://{}:{}", config.host, config.port);
    
    // Initialize config
    let config_data = web::Data::new(Mutex::new(config));
    
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
            .app_data(config_data.clone())
            // API routes will be added here
            .service(
                web::scope("/api")
                    .configure(handlers::configure_routes)
            )
            // Add other handlers here
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
} 