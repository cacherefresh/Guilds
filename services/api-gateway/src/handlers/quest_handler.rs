use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::proxy::{ProxyFactory, ProxyError};
use crate::config::Config;
use log::{error, info};

// Request and response models for quests
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub required_skills: Vec<String>,
    pub reward: String,
    pub xp_reward: i32,
    pub gold_reward: i32,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestsListResponse {
    pub quests: Vec<QuestResponse>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Debug, Deserialize)]
pub struct QuestQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub difficulty: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestRequest {
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub required_skills: Vec<String>,
    pub reward: String,
    pub xp_reward: i32,
    pub gold_reward: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub required_skills: Option<Vec<String>>,
    pub reward: Option<String>,
    pub xp_reward: Option<i32>,
    pub gold_reward: Option<i32>,
    pub status: Option<String>,
}

// Handler functions
#[get("")]
pub async fn get_quests(
    query: web::Query<QuestQuery>,
    config: web::Data<Config>,
) -> impl Responder {
    info!("Fetching quests with query: {:?}", query);
    
    let factory = ProxyFactory::new();
    let quest_proxy = factory.create_quest_proxy(config.quest_service_url.clone());
    
    // Build query string
    let mut query_parts = Vec::new();
    if let Some(offset) = query.offset {
        query_parts.push(format!("offset={}", offset));
    }
    if let Some(limit) = query.limit {
        query_parts.push(format!("limit={}", limit));
    }
    if let Some(ref difficulty) = query.difficulty {
        query_parts.push(format!("difficulty={}", difficulty));
    }
    if let Some(ref status) = query.status {
        query_parts.push(format!("status={}", status));
    }
    
    let query_str = if !query_parts.is_empty() {
        format!("?{}", query_parts.join("&"))
    } else {
        String::new()
    };
    
    match quest_proxy.get::<QuestsListResponse>(&format!("/quests{}", query_str)).await {
        Ok(response) => {
            HttpResponse::Ok().json(response)
        },
        Err(err) => handle_proxy_error(err),
    }
}

#[get("/{id}")]
pub async fn get_quest_by_id(
    path: web::Path<Uuid>,
    config: web::Data<Config>,
) -> impl Responder {
    let quest_id = path.into_inner();
    info!("Fetching quest with ID: {}", quest_id);
    
    let factory = ProxyFactory::new();
    let quest_proxy = factory.create_quest_proxy(config.quest_service_url.clone());
    
    match quest_proxy.get::<QuestResponse>(&format!("/quests/{}", quest_id)).await {
        Ok(response) => {
            HttpResponse::Ok().json(response)
        },
        Err(err) => handle_proxy_error(err),
    }
}

#[post("")]
pub async fn create_quest(
    quest: web::Json<CreateQuestRequest>,
    config: web::Data<Config>,
) -> impl Responder {
    info!("Creating new quest: {}", quest.title);
    
    let factory = ProxyFactory::new();
    let quest_proxy = factory.create_quest_proxy(config.quest_service_url.clone());
    
    match quest_proxy.post::<_, QuestResponse>("/quests", &quest.into_inner()).await {
        Ok(response) => {
            HttpResponse::Created().json(response)
        },
        Err(err) => handle_proxy_error(err),
    }
}

#[put("/{id}")]
pub async fn update_quest(
    path: web::Path<Uuid>,
    quest: web::Json<UpdateQuestRequest>,
    config: web::Data<Config>,
) -> impl Responder {
    let quest_id = path.into_inner();
    info!("Updating quest with ID: {}", quest_id);
    
    let factory = ProxyFactory::new();
    let quest_proxy = factory.create_quest_proxy(config.quest_service_url.clone());
    
    match quest_proxy.put::<_, QuestResponse>(&format!("/quests/{}", quest_id), &quest.into_inner()).await {
        Ok(response) => {
            HttpResponse::Ok().json(response)
        },
        Err(err) => handle_proxy_error(err),
    }
}

#[delete("/{id}")]
pub async fn delete_quest(
    path: web::Path<Uuid>,
    config: web::Data<Config>,
) -> impl Responder {
    let quest_id = path.into_inner();
    info!("Deleting quest with ID: {}", quest_id);
    
    let factory = ProxyFactory::new();
    let quest_proxy = factory.create_quest_proxy(config.quest_service_url.clone());
    
    match quest_proxy.delete::<()>(&format!("/quests/{}", quest_id)).await {
        Ok(_) => {
            HttpResponse::NoContent().finish()
        },
        Err(err) => handle_proxy_error(err),
    }
}

// Helper function to handle proxy errors
fn handle_proxy_error(err: ProxyError) -> HttpResponse {
    match err {
        ProxyError::RequestFailed(e) => {
            error!("Request to service failed: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to communicate with service",
                "message": e.to_string()
            }))
        },
        ProxyError::ServiceUnavailable(msg) => {
            error!("Service unavailable: {}", msg);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service unavailable",
                "message": msg
            }))
        },
        ProxyError::Timeout => {
            error!("Request to service timed out");
            HttpResponse::GatewayTimeout().json(serde_json::json!({
                "error": "Request timed out",
                "message": "The service took too long to respond"
            }))
        },
        ProxyError::AuthError(msg) => {
            error!("Authentication error with service: {}", msg);
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication error",
                "message": msg
            }))
        },
        ProxyError::Other(msg) => {
            error!("Unexpected error from service: {}", msg);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Unexpected error",
                "message": msg
            }))
        },
    }
} 