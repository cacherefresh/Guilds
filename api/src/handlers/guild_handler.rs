use crate::error::ApiError;
use crate::models::{
    Guild, GuildCreate, GuildUpdate, GuildListResponse, GuildSummary,
    GuildMemberRecord, GuildMembership, GuildSkillMatch,
};
use crate::repository::GuildRepository;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;
use log::error;
use std::sync::Arc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/guilds")
            .service(get_guilds)
            .service(get_guild_by_id)
            .service(create_guild)
            .service(update_guild)
            .service(delete_guild)
            .service(add_character_to_guild)
            .service(remove_character_from_guild)
            .service(set_active_guild)
            .service(get_character_guilds)
            .service(find_guilds_by_skills)
            .service(search_guilds_by_name),
    );
}

#[derive(Deserialize)]
struct ListParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Deserialize)]
struct GuildMemberParams {
    character_id: Uuid,
    set_active: Option<bool>,
}

#[derive(Deserialize)]
struct SkillSearchParams {
    skill_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
struct NameSearchParams {
    name: String,
}

/// Get all guilds with pagination
#[get("")]
async fn get_guilds(
    params: web::Query<ListParams>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);
    
    let (guilds, total) = guild_repo.get_guilds(limit, offset).await?;
    
    Ok(HttpResponse::Ok().json(GuildListResponse {
        guilds,
        total,
        limit,
        offset,
    }))
}

/// Get a specific guild by ID
#[get("/{id}")]
async fn get_guild_by_id(
    path: web::Path<Uuid>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let guild_id = path.into_inner();
    let guild = guild_repo.get_guild_by_id(guild_id).await?;
    
    Ok(HttpResponse::Ok().json(guild))
}

/// Create a new guild
#[post("")]
async fn create_guild(
    guild: web::Json<GuildCreate>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let created_guild = guild_repo.create_guild(guild.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(created_guild))
}

/// Update an existing guild
#[put("/{id}")]
async fn update_guild(
    path: web::Path<Uuid>,
    guild: web::Json<GuildUpdate>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let guild_id = path.into_inner();
    let updated_guild = guild_repo.update_guild(guild_id, guild.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(updated_guild))
}

/// Delete a guild
#[delete("/{id}")]
async fn delete_guild(
    path: web::Path<Uuid>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let guild_id = path.into_inner();
    guild_repo.delete_guild(guild_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

/// Add a character to a guild
#[post("/{id}/members")]
async fn add_character_to_guild(
    path: web::Path<Uuid>,
    params: web::Json<GuildMemberParams>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let guild_id = path.into_inner();
    let character_id = params.character_id;
    let set_active = params.set_active.unwrap_or(false);
    
    let result = guild_repo
        .add_character_to_guild(guild_id, character_id, set_active)
        .await?;
    
    Ok(HttpResponse::Ok().json(result))
}

/// Remove a character from a guild
#[delete("/{id}/members/{character_id}")]
async fn remove_character_from_guild(
    path: web::Path<(Uuid, Uuid)>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let (guild_id, character_id) = path.into_inner();
    
    guild_repo.remove_character_from_guild(guild_id, character_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

/// Set a guild as active for a character
#[put("/{id}/members/{character_id}/active")]
async fn set_active_guild(
    path: web::Path<(Uuid, Uuid)>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let (guild_id, character_id) = path.into_inner();
    
    let result = guild_repo.set_active_guild(character_id, guild_id).await?;
    
    Ok(HttpResponse::Ok().json(result))
}

/// Get guilds that a character is a member of
#[get("/character/{character_id}")]
async fn get_character_guilds(
    path: web::Path<Uuid>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let character_id = path.into_inner();
    
    let guilds = guild_repo.get_character_guilds(character_id).await?;
    
    Ok(HttpResponse::Ok().json(guilds))
}

/// Find guilds by skills
#[post("/search/skills")]
async fn find_guilds_by_skills(
    params: web::Json<SkillSearchParams>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let skill_ids = params.skill_ids.clone();
    
    let guilds = guild_repo.find_guilds_by_skills(&skill_ids).await?;
    
    Ok(HttpResponse::Ok().json(guilds))
}

/// Search guilds by name
#[get("/search")]
async fn search_guilds_by_name(
    params: web::Query<NameSearchParams>,
    guild_repo: web::Data<Arc<GuildRepository>>,
) -> Result<impl Responder, ApiError> {
    let name = &params.name;
    
    let guilds = guild_repo.search_guilds_by_name(name).await?;
    
    Ok(HttpResponse::Ok().json(guilds))
} 