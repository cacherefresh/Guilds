use actix_web::{web, HttpResponse, get, post, put, delete};
use log::{error, info};
use uuid::Uuid;
use crate::models::language_pack::{
    LanguagePackCreate, LanguagePackUpdate, LanguagePackListResponse
};
use crate::repository::language_pack::LanguagePackRepository;

#[get("/language-packs")]
pub async fn get_language_packs(
    db: web::Data<LanguagePackRepository>,
    query: web::Query<web::Query<(Option<String>, Option<i64>, Option<i64>)>>,
) -> HttpResponse {
    let (language_code, offset, limit) = query.0;
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(100);
    
    let language_code_ref = language_code.as_deref();
    
    match db.get_language_packs(offset, limit, language_code_ref).await {
        Ok((language_packs, total)) => {
            let response = LanguagePackListResponse {
                data: language_packs,
                meta: crate::models::PaginationMeta {
                    total,
                    offset,
                    limit,
                },
            };
            
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            error!("Failed to get language packs: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to get language packs: {}", e))
        }
    }
}

#[get("/language-packs/{id}")]
pub async fn get_language_pack_by_id(
    db: web::Data<LanguagePackRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    
    match db.get_language_pack_by_id(id).await {
        Ok(language_pack) => HttpResponse::Ok().json(language_pack),
        Err(e) => {
            error!("Failed to get language pack: {}", e);
            
            if e.to_string().contains("no rows") {
                HttpResponse::NotFound().body(format!("Language pack with id {} not found", id))
            } else {
                HttpResponse::InternalServerError().body(format!("Failed to get language pack: {}", e))
            }
        }
    }
}

#[get("/language-packs/current")]
pub async fn get_current_language_pack(
    db: web::Data<LanguagePackRepository>,
) -> HttpResponse {
    match db.get_current_language_pack().await {
        Ok(language_pack) => HttpResponse::Ok().json(language_pack),
        Err(e) => {
            error!("Failed to get current language pack: {}", e);
            
            if e.to_string().contains("no rows") {
                HttpResponse::NotFound().body("No current language pack found")
            } else {
                HttpResponse::InternalServerError().body(format!("Failed to get current language pack: {}", e))
            }
        }
    }
}

#[post("/language-packs")]
pub async fn create_language_pack(
    db: web::Data<LanguagePackRepository>,
    pack: web::Json<LanguagePackCreate>,
) -> HttpResponse {
    match db.create_language_pack(pack.into_inner()).await {
        Ok(language_pack) => {
            info!("Created language pack with id: {}", language_pack.id);
            HttpResponse::Created().json(language_pack)
        },
        Err(e) => {
            error!("Failed to create language pack: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to create language pack: {}", e))
        }
    }
}

#[put("/language-packs/{id}")]
pub async fn update_language_pack(
    db: web::Data<LanguagePackRepository>,
    path: web::Path<Uuid>,
    pack: web::Json<LanguagePackUpdate>,
) -> HttpResponse {
    let id = path.into_inner();
    
    match db.update_language_pack(id, pack.into_inner()).await {
        Ok(language_pack) => {
            info!("Updated language pack with id: {}", language_pack.id);
            HttpResponse::Ok().json(language_pack)
        },
        Err(e) => {
            error!("Failed to update language pack: {}", e);
            
            if e.to_string().contains("no rows") {
                HttpResponse::NotFound().body(format!("Language pack with id {} not found", id))
            } else {
                HttpResponse::InternalServerError().body(format!("Failed to update language pack: {}", e))
            }
        }
    }
}

#[put("/language-packs/{id}/set-current")]
pub async fn set_current_language_pack(
    db: web::Data<LanguagePackRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    
    match db.set_current_language_pack(id).await {
        Ok(_) => {
            info!("Set language pack with id {} as current", id);
            HttpResponse::Ok().body(format!("Language pack with id {} set as current", id))
        },
        Err(e) => {
            error!("Failed to set language pack as current: {}", e);
            
            if e.to_string().contains("no rows") {
                HttpResponse::NotFound().body(format!("Language pack with id {} not found", id))
            } else {
                HttpResponse::InternalServerError().body(format!("Failed to set language pack as current: {}", e))
            }
        }
    }
}

#[delete("/language-packs/{id}")]
pub async fn delete_language_pack(
    db: web::Data<LanguagePackRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    
    match db.delete_language_pack(id).await {
        Ok(_) => {
            info!("Deleted language pack with id: {}", id);
            HttpResponse::NoContent().finish()
        },
        Err(e) => {
            error!("Failed to delete language pack: {}", e);
            
            if e.to_string().contains("no rows") {
                HttpResponse::NotFound().body(format!("Language pack with id {} not found", id))
            } else if e.to_string().contains("Cannot delete the current active language pack") {
                HttpResponse::BadRequest().body("Cannot delete the current active language pack")
            } else {
                HttpResponse::InternalServerError().body(format!("Failed to delete language pack: {}", e))
            }
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_language_packs)
       .service(get_language_pack_by_id)
       .service(get_current_language_pack)
       .service(create_language_pack)
       .service(update_language_pack)
       .service(set_current_language_pack)
       .service(delete_language_pack);
} 