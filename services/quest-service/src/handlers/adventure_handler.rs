use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use log::{error, info};

use crate::{
    models::adventure::{Adventure, AdventureCreate, AdventureUpdate, AdventureListResponse, AdventureDetail, AdventureQuery},
    AppState, Error,
};

/// Get a list of adventures with pagination and filtering options
#[utoipa::path(
    get,
    path = "/api/adventures",
    tag = "Adventures",
    params(
        AdventureQuery
    ),
    responses(
        (status = 200, description = "List of adventures retrieved successfully", body = AdventureListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_adventures(
    State(state): State<AppState>,
    Query(query): Query<AdventureQuery>,
) -> Result<Json<AdventureListResponse>, Error> {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20);
    
    let adventure_repo = state.adventure_repository();
    let (adventures, total) = adventure_repo
        .get_adventures(offset, limit, query.status.as_deref())
        .await
        .map_err(|e| {
            error!("Failed to get adventures: {}", e);
            Error::internal_server_error()
        })?;
    
    let response = AdventureListResponse {
        adventures,
        total,
        offset,
        limit,
    };
    
    Ok(Json(response))
}

/// Get an adventure by ID
#[utoipa::path(
    get,
    path = "/api/adventures/{id}",
    tag = "Adventures",
    params(
        ("id" = Uuid, Path, description = "Adventure ID"),
    ),
    responses(
        (status = 200, description = "Adventure retrieved successfully", body = Adventure),
        (status = 404, description = "Adventure not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_adventure_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Adventure>, Error> {
    let adventure_repo = state.adventure_repository();
    let adventure = adventure_repo
        .get_adventure_by_id(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Adventure with ID {} not found", id))
                }
                _ => {
                    error!("Failed to get adventure: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(Json(adventure))
}

/// Get detailed information about an adventure including its epics
#[utoipa::path(
    get,
    path = "/api/adventures/{id}/details",
    tag = "Adventures",
    params(
        ("id" = Uuid, Path, description = "Adventure ID"),
    ),
    responses(
        (status = 200, description = "Adventure details retrieved successfully", body = AdventureDetail),
        (status = 404, description = "Adventure not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_adventure_details(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AdventureDetail>, Error> {
    let adventure_repo = state.adventure_repository();
    let epic_repo = state.epic_repository();
    
    // Get the adventure
    let adventure = adventure_repo
        .get_adventure_by_id(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Adventure with ID {} not found", id))
                }
                _ => {
                    error!("Failed to get adventure: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    // Get epics for this adventure
    let (epics, _) = epic_repo
        .get_epics(0, 100, None, Some(id))
        .await
        .map_err(|e| {
            error!("Failed to get epics for adventure: {}", e);
            Error::internal_server_error()
        })?;
    
    let adventure_detail = AdventureDetail {
        adventure,
        epics,
    };
    
    Ok(Json(adventure_detail))
}

/// Create a new adventure
#[utoipa::path(
    post,
    path = "/api/adventures",
    tag = "Adventures",
    request_body = AdventureCreate,
    responses(
        (status = 201, description = "Adventure created successfully", body = Adventure),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn create_adventure(
    State(state): State<AppState>,
    Json(adventure_create): Json<AdventureCreate>,
) -> Result<(StatusCode, Json<Adventure>), Error> {
    let adventure_repo = state.adventure_repository();
    let adventure = adventure_repo
        .create_adventure(adventure_create)
        .await
        .map_err(|e| {
            error!("Failed to create adventure: {}", e);
            Error::internal_server_error()
        })?;
    
    Ok((StatusCode::CREATED, Json(adventure)))
}

/// Update an existing adventure
#[utoipa::path(
    patch,
    path = "/api/adventures/{id}",
    tag = "Adventures",
    params(
        ("id" = Uuid, Path, description = "Adventure ID"),
    ),
    request_body = AdventureUpdate,
    responses(
        (status = 200, description = "Adventure updated successfully", body = Adventure),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 404, description = "Adventure not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn update_adventure(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(adventure_update): Json<AdventureUpdate>,
) -> Result<Json<Adventure>, Error> {
    let adventure_repo = state.adventure_repository();
    let adventure = adventure_repo
        .update_adventure(id, adventure_update)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Adventure with ID {} not found", id))
                }
                _ => {
                    error!("Failed to update adventure: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(Json(adventure))
}

/// Delete an adventure
#[utoipa::path(
    delete,
    path = "/api/adventures/{id}",
    tag = "Adventures",
    params(
        ("id" = Uuid, Path, description = "Adventure ID"),
    ),
    responses(
        (status = 204, description = "Adventure deleted successfully"),
        (status = 404, description = "Adventure not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_adventure(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    let adventure_repo = state.adventure_repository();
    adventure_repo
        .delete_adventure(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Adventure with ID {} not found", id))
                }
                crate::repository::db::DbError::Other(msg) => {
                    if msg.contains("epics associated with it") {
                        Error::bad_request(&msg)
                    } else {
                        error!("Failed to delete adventure: {}", e);
                        Error::internal_server_error()
                    }
                }
                _ => {
                    error!("Failed to delete adventure: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(StatusCode::NO_CONTENT)
} 