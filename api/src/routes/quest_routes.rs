use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use uuid::Uuid;
use log::error;

use crate::models::{QuestCreate, QuestUpdate, QuestsResponse, AvailableQuestsResponse};
use crate::repository::quest_repository::QuestRepository;
use crate::error::ApiError;

// Configure quest routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/quests")
            .service(get_quests)
            .service(get_quest_by_id)
            .service(create_quest)
            .service(update_quest)
            .service(delete_quest)
            .service(get_available_quests_for_character)
    );
}

// Get all quests with optional filtering
#[get("")]
async fn get_quests(
    repository: web::Data<QuestRepository>,
    query: web::Query<QueryParams>,
) -> Result<impl Responder, ApiError> {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);
    
    // Parse skill IDs if provided
    let skill_ids = if let Some(ref skills) = query.skills {
        let skill_ids: Result<Vec<Uuid>, _> = skills
            .split(',')
            .map(|s| s.trim().parse::<Uuid>())
            .collect();
            
        match skill_ids {
            Ok(ids) => Some(ids),
            Err(e) => {
                error!("Error parsing skill IDs: {}", e);
                return Err(ApiError::BadRequestError("Invalid skill ID format".to_string()));
            }
        }
    } else {
        None
    };
    
    let exact_match = query.exact_match;
    
    let (quests, total_count) = repository.get_quests(skill_ids, exact_match, limit, offset).await?;
    
    Ok(HttpResponse::Ok().json(QuestsResponse {
        data: quests,
        total: total_count,
        offset,
        limit,
    }))
}

// Get a quest by ID
#[get("/{id}")]
async fn get_quest_by_id(
    repository: web::Data<QuestRepository>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let quest_id = path.into_inner();
    let quest = repository.get_quest_by_id(quest_id).await?;
    
    Ok(HttpResponse::Ok().json(quest))
}

// Create a new quest
#[post("")]
async fn create_quest(
    repository: web::Data<QuestRepository>,
    quest: web::Json<QuestCreate>,
) -> Result<impl Responder, ApiError> {
    let quest = repository.create_quest(quest.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(quest))
}

// Update an existing quest
#[put("/{id}")]
async fn update_quest(
    repository: web::Data<QuestRepository>,
    path: web::Path<Uuid>,
    quest: web::Json<QuestUpdate>,
) -> Result<impl Responder, ApiError> {
    let quest_id = path.into_inner();
    let quest = repository.update_quest(quest_id, quest.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(quest))
}

// Delete a quest
#[delete("/{id}")]
async fn delete_quest(
    repository: web::Data<QuestRepository>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, ApiError> {
    let quest_id = path.into_inner();
    repository.delete_quest(quest_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

// Get available quests for a character
#[get("/available/{character_id}")]
async fn get_available_quests_for_character(
    repository: web::Data<QuestRepository>,
    path: web::Path<Uuid>,
    query: web::Query<AvailableParams>,
) -> Result<impl Responder, ApiError> {
    let character_id = path.into_inner();
    let include_team = query.include_team.unwrap_or(false);
    
    let quests = repository.get_available_quests_for_character(character_id, include_team).await?;
    
    Ok(HttpResponse::Ok().json(AvailableQuestsResponse {
        data: quests,
        character_id,
        include_team,
    }))
}

// Query parameters for filtering quests
#[derive(serde::Deserialize)]
struct QueryParams {
    skills: Option<String>,
    exact_match: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
}

// Query parameters for available quests
#[derive(serde::Deserialize)]
struct AvailableParams {
    include_team: Option<bool>,
} 