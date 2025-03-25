use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::quest::{Quest, QuestCreate, QuestUpdate, QuestsResponse, QuestQuery};
use crate::models::error::ServiceError;
use crate::repository::quest::QuestRepository;
use deadpool_postgres::Pool;
use log::{info, error};

/// Configure quest routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_quests)
        .service(get_quest_by_id)
        .service(create_quest)
        .service(update_quest)
        .service(delete_quest);
}

/// Get all quests with filtering and pagination
#[get("")]
async fn get_quests(
    query: web::Query<QuestQuery>,
    db_pool: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let query = query.into_inner();
    info!("Fetching quests with query: {:?}", query);
    
    // Default values
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);
    
    let client = db_pool.get().await.map_err(|e| {
        error!("Failed to get DB client: {}", e);
        ServiceError::InternalServerError(e.to_string())
    })?;
    
    let repository = QuestRepository::new(client);
    let (quests, total) = repository.get_quests(
        offset, 
        limit, 
        query.difficulty.as_deref(), 
        query.status.as_deref()
    ).await?;
    
    Ok(HttpResponse::Ok().json(QuestsResponse {
        quests,
        total,
        offset,
        limit,
    }))
}

/// Get a quest by ID
#[get("/{id}")]
async fn get_quest_by_id(
    path: web::Path<Uuid>,
    db_pool: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let quest_id = path.into_inner();
    info!("Fetching quest with ID: {}", quest_id);
    
    let client = db_pool.get().await.map_err(|e| {
        error!("Failed to get DB client: {}", e);
        ServiceError::InternalServerError(e.to_string())
    })?;
    
    let repository = QuestRepository::new(client);
    let quest = repository.get_quest_by_id(quest_id).await?;
    
    Ok(HttpResponse::Ok().json(quest))
}

/// Create a new quest
#[post("")]
async fn create_quest(
    quest: web::Json<QuestCreate>,
    db_pool: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    info!("Creating new quest: {}", quest.title);
    
    let client = db_pool.get().await.map_err(|e| {
        error!("Failed to get DB client: {}", e);
        ServiceError::InternalServerError(e.to_string())
    })?;
    
    let repository = QuestRepository::new(client);
    let quest = repository.create_quest(quest.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(quest))
}

/// Update an existing quest
#[put("/{id}")]
async fn update_quest(
    path: web::Path<Uuid>,
    quest: web::Json<QuestUpdate>,
    db_pool: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let quest_id = path.into_inner();
    info!("Updating quest with ID: {}", quest_id);
    
    let client = db_pool.get().await.map_err(|e| {
        error!("Failed to get DB client: {}", e);
        ServiceError::InternalServerError(e.to_string())
    })?;
    
    let repository = QuestRepository::new(client);
    let quest = repository.update_quest(quest_id, quest.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(quest))
}

/// Delete a quest
#[delete("/{id}")]
async fn delete_quest(
    path: web::Path<Uuid>,
    db_pool: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let quest_id = path.into_inner();
    info!("Deleting quest with ID: {}", quest_id);
    
    let client = db_pool.get().await.map_err(|e| {
        error!("Failed to get DB client: {}", e);
        ServiceError::InternalServerError(e.to_string())
    })?;
    
    let repository = QuestRepository::new(client);
    repository.delete_quest(quest_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
} 