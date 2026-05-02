use axum::Json;
use serde_json::json;

/// Health check endpoint for monitoring service status
#[utoipa::path(
    get,
    path = "/api/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "quest-service",
    }))
} 