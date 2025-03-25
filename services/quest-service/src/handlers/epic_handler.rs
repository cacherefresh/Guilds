use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use log::{error, info};

use crate::{
    models::epic::{Epic, EpicCreate, EpicUpdate, EpicListResponse, EpicDetail, EpicQuery},
    AppState, Error,
};

/// Get a list of epics with pagination and filtering options
#[utoipa::path(
    get,
    path = "/api/epics",
    tag = "Epics",
    params(
        EpicQuery
    ),
    responses(
        (status = 200, description = "List of epics retrieved successfully", body = EpicListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_epics(
    State(state): State<AppState>,
    Query(query): Query<EpicQuery>,
) -> Result<Json<EpicListResponse>, Error> {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20);
    
    let epic_repo = state.epic_repository();
    let (epics, total) = epic_repo
        .get_epics(offset, limit, query.status.as_deref(), query.adventure_id)
        .await
        .map_err(|e| {
            error!("Failed to get epics: {}", e);
            Error::internal_server_error()
        })?;
    
    let response = EpicListResponse {
        epics,
        total,
        offset,
        limit,
    };
    
    Ok(Json(response))
}

/// Get an epic by ID
#[utoipa::path(
    get,
    path = "/api/epics/{id}",
    tag = "Epics",
    params(
        ("id" = Uuid, Path, description = "Epic ID"),
    ),
    responses(
        (status = 200, description = "Epic retrieved successfully", body = Epic),
        (status = 404, description = "Epic not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_epic_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Epic>, Error> {
    let epic_repo = state.epic_repository();
    let epic = epic_repo
        .get_epic_by_id(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Epic with ID {} not found", id))
                }
                _ => {
                    error!("Failed to get epic: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(Json(epic))
}

/// Get detailed information about an epic including its quests
#[utoipa::path(
    get,
    path = "/api/epics/{id}/details",
    tag = "Epics",
    params(
        ("id" = Uuid, Path, description = "Epic ID"),
    ),
    responses(
        (status = 200, description = "Epic details retrieved successfully", body = EpicDetail),
        (status = 404, description = "Epic not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_epic_details(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EpicDetail>, Error> {
    let epic_repo = state.epic_repository();
    let quest_repo = state.quest_repository();
    
    // Get the epic
    let epic = epic_repo
        .get_epic_by_id(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Epic with ID {} not found", id))
                }
                _ => {
                    error!("Failed to get epic: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    // Get quests for this epic
    let (quests, _) = quest_repo
        .get_quests(0, 100, None, Some(id), None, None, None)
        .await
        .map_err(|e| {
            error!("Failed to get quests for epic: {}", e);
            Error::internal_server_error()
        })?;
    
    let epic_detail = EpicDetail {
        epic,
        quests,
    };
    
    Ok(Json(epic_detail))
}

/// Create a new epic
#[utoipa::path(
    post,
    path = "/api/epics",
    tag = "Epics",
    request_body = EpicCreate,
    responses(
        (status = 201, description = "Epic created successfully", body = Epic),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn create_epic(
    State(state): State<AppState>,
    Json(epic_create): Json<EpicCreate>,
) -> Result<(StatusCode, Json<Epic>), Error> {
    let epic_repo = state.epic_repository();
    let epic = epic_repo
        .create_epic(epic_create)
        .await
        .map_err(|e| {
            error!("Failed to create epic: {}", e);
            Error::internal_server_error()
        })?;
    
    Ok((StatusCode::CREATED, Json(epic)))
}

/// Update an existing epic
#[utoipa::path(
    patch,
    path = "/api/epics/{id}",
    tag = "Epics",
    params(
        ("id" = Uuid, Path, description = "Epic ID"),
    ),
    request_body = EpicUpdate,
    responses(
        (status = 200, description = "Epic updated successfully", body = Epic),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 404, description = "Epic not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn update_epic(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(epic_update): Json<EpicUpdate>,
) -> Result<Json<Epic>, Error> {
    let epic_repo = state.epic_repository();
    let epic = epic_repo
        .update_epic(id, epic_update)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Epic with ID {} not found", id))
                }
                _ => {
                    error!("Failed to update epic: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(Json(epic))
}

/// Delete an epic
#[utoipa::path(
    delete,
    path = "/api/epics/{id}",
    tag = "Epics",
    params(
        ("id" = Uuid, Path, description = "Epic ID"),
    ),
    responses(
        (status = 204, description = "Epic deleted successfully"),
        (status = 404, description = "Epic not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_epic(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    let epic_repo = state.epic_repository();
    epic_repo
        .delete_epic(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Epic with ID {} not found", id))
                }
                crate::repository::db::DbError::Other(msg) => {
                    if msg.contains("quests associated with it") {
                        Error::bad_request(&msg)
                    } else {
                        error!("Failed to delete epic: {}", e);
                        Error::internal_server_error()
                    }
                }
                _ => {
                    error!("Failed to delete epic: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(StatusCode::NO_CONTENT)
} 