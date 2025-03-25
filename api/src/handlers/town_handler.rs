use crate::error::ApiError;
use crate::models::{Town, TownCreate, TownUpdate, TownListResponse};
use crate::repository::TownRepository;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/towns")
            .service(get_towns)
            .service(get_town_by_id)
            .service(create_town)
            .service(update_town)
            .service(delete_town)
            .service(search_towns)
            .service(get_towns_by_region),
    );
}

#[derive(Deserialize)]
struct ListParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Deserialize)]
struct SearchParams {
    query: String,
}

#[derive(Deserialize)]
struct RegionParams {
    region: String,
}

/// Get all towns with pagination
#[get("")]
async fn get_towns(
    params: web::Query<ListParams>,
    town_repo: web::Data<Arc<TownRepository>>,
) -> Result<impl Responder, ApiError> {
    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);
    
    let (towns, total) = town_repo.get_towns(limit, offset).await?;
    
    Ok(HttpResponse::Ok().json(TownListResponse {
        towns,
        total,
        limit,
        offset,
    }))
}

/// Get a specific town by ID
#[get("/{id}")]
async fn get_town_by_id(
    path: web::Path<Uuid>,
    town_repo: web::Data<Arc<TownRepository>>,
) -> Result<impl Responder, ApiError> {
    let town_id = path.into_inner();
    let town = town_repo.get_town_by_id(town_id).await?;
    
    Ok(HttpResponse::Ok().json(town))
}

/// Create a new town
#[post("")]
async fn create_town(
    town: web::Json<TownCreate>,
    town_repo: web::Data<Arc<TownRepository>>,
) -> Result<impl Responder, ApiError> {
    let created_town = town_repo.create_town(town.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(created_town))
}

/// Update an existing town
#[put("/{id}")]
async fn update_town(
    path: web::Path<Uuid>,
    town: web::Json<TownUpdate>,
    town_repo: web::Data<Arc<TownRepository>>,
) -> Result<impl Responder, ApiError> {
    let town_id = path.into_inner();
    let updated_town = town_repo.update_town(town_id, town.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(updated_town))
}

/// Delete a town
#[delete("/{id}")]
async fn delete_town(
    path: web::Path<Uuid>,
    town_repo: web::Data<Arc<TownRepository>>,
) -> Result<impl Responder, ApiError> {
    let town_id = path.into_inner();
    town_repo.delete_town(town_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

/// Search towns by name or region
#[get("/search")]
async fn search_towns(
    params: web::Query<SearchParams>,
    town_repo: web::Data<Arc<TownRepository>>,
) -> Result<impl Responder, ApiError> {
    let towns = town_repo.search_towns(&params.query).await?;
    
    Ok(HttpResponse::Ok().json(towns))
}

/// Get towns by region
#[get("/region/{region}")]
async fn get_towns_by_region(
    path: web::Path<String>,
    town_repo: web::Data<Arc<TownRepository>>,
) -> Result<impl Responder, ApiError> {
    let region = path.into_inner();
    let towns = town_repo.get_towns_by_region(&region).await?;
    
    Ok(HttpResponse::Ok().json(towns))
} 