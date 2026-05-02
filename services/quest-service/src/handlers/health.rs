use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    service: String,
}

/// Health check endpoint
#[get("/health")]
pub async fn health_check() -> impl Responder {
    let response = HealthResponse {
        status: "UP".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        service: "quest-service".to_string(),
    };
    
    HttpResponse::Ok().json(response)
} 