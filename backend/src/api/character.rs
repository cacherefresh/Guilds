use crate::db;
use crate::error::AppError;
use crate::models::character::{CharacterLocationDto, CreateCharacterDto, UpdateCharacterDto};
use actix_web::{get, post, put, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::{Pool, Postgres};
use sqlx::types::Uuid;
use crate::auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::basic(auth::validator);
    
    cfg.service(
        web::scope("/characters")
            .wrap(auth)
            .service(get_all)
            .service(get_by_id)
            .service(create)
            .service(update)
            .service(update_location),
    );
}

#[get("")]
async fn get_all(pool: web::Data<Pool<Postgres>>) -> Result<impl Responder, AppError> {
    let characters = db::character::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(characters))
}

#[get("/{id}")]
async fn get_by_id(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let character = db::character::get_by_id(&pool, id).await?;
    Ok(HttpResponse::Ok().json(character))
}

#[post("")]
async fn create(
    pool: web::Data<Pool<Postgres>>,
    dto: web::Json<CreateCharacterDto>,
) -> Result<impl Responder, AppError> {
    let character = db::character::create(&pool, dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(character))
}

#[put("/{id}")]
async fn update(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<Uuid>,
    dto: web::Json<UpdateCharacterDto>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let character = db::character::update(&pool, id, dto.into_inner()).await?;
    Ok(HttpResponse::Ok().json(character))
}

#[put("/{id}/location")]
async fn update_location(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<Uuid>,
    location: web::Json<CharacterLocationDto>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    db::character::update_location(&pool, id, location.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
} 