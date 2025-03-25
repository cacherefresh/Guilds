use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use log::{error, info};

use crate::{
    models::terminology::{TerminologySetting, TerminologyCreate, TerminologyUpdate},
    AppState, Error,
};

/// Get all terminology settings
#[utoipa::path(
    get,
    path = "/api/terminology",
    tag = "Terminology",
    responses(
        (status = 200, description = "List of terminology settings retrieved successfully", body = Vec<TerminologySetting>),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_all_terminology_settings(
    State(state): State<AppState>,
) -> Result<Json<Vec<TerminologySetting>>, Error> {
    let terminology_repo = state.terminology_repository();
    let terminology_settings = terminology_repo
        .get_all_terminology_settings()
        .await
        .map_err(|e| {
            error!("Failed to get terminology settings: {}", e);
            Error::internal_server_error()
        })?;
    
    Ok(Json(terminology_settings))
}

/// Get terminology setting by ID
#[utoipa::path(
    get,
    path = "/api/terminology/{id}",
    tag = "Terminology",
    params(
        ("id" = Uuid, Path, description = "Terminology setting ID"),
    ),
    responses(
        (status = 200, description = "Terminology setting retrieved successfully", body = TerminologySetting),
        (status = 404, description = "Terminology setting not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_terminology_setting_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TerminologySetting>, Error> {
    let terminology_repo = state.terminology_repository();
    let terminology_setting = terminology_repo
        .get_terminology_setting_by_id(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Terminology setting with ID {} not found", id))
                }
                _ => {
                    error!("Failed to get terminology setting: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(Json(terminology_setting))
}

/// Get current active terminology
#[utoipa::path(
    get,
    path = "/api/terminology/current",
    tag = "Terminology",
    responses(
        (status = 200, description = "Current terminology retrieved successfully", body = TerminologySetting),
        (status = 404, description = "No active terminology found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_current_terminology(
    State(state): State<AppState>,
) -> Result<Json<TerminologySetting>, Error> {
    let terminology_repo = state.terminology_repository();
    let current_terminology = terminology_repo
        .get_current_terminology()
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found("No active terminology setting found")
                }
                _ => {
                    error!("Failed to get current terminology: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(Json(current_terminology))
}

/// Create a new terminology setting
#[utoipa::path(
    post,
    path = "/api/terminology",
    tag = "Terminology",
    request_body = TerminologyCreate,
    responses(
        (status = 201, description = "Terminology setting created successfully", body = TerminologySetting),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn create_terminology_setting(
    State(state): State<AppState>,
    Json(terminology_create): Json<TerminologyCreate>,
) -> Result<(StatusCode, Json<TerminologySetting>), Error> {
    let terminology_repo = state.terminology_repository();
    let terminology_setting = terminology_repo
        .create_terminology_setting(terminology_create)
        .await
        .map_err(|e| {
            error!("Failed to create terminology setting: {}", e);
            Error::internal_server_error()
        })?;
    
    Ok((StatusCode::CREATED, Json(terminology_setting)))
}

/// Update an existing terminology setting
#[utoipa::path(
    patch,
    path = "/api/terminology/{id}",
    tag = "Terminology",
    params(
        ("id" = Uuid, Path, description = "Terminology setting ID"),
    ),
    request_body = TerminologyUpdate,
    responses(
        (status = 200, description = "Terminology setting updated successfully", body = TerminologySetting),
        (status = 400, description = "Invalid input", body = ErrorResponse),
        (status = 404, description = "Terminology setting not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn update_terminology_setting(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(terminology_update): Json<TerminologyUpdate>,
) -> Result<Json<TerminologySetting>, Error> {
    let terminology_repo = state.terminology_repository();
    let terminology_setting = terminology_repo
        .update_terminology_setting(id, terminology_update)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Terminology setting with ID {} not found", id))
                }
                _ => {
                    error!("Failed to update terminology setting: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(Json(terminology_setting))
}

/// Set a terminology setting as the current active one
#[utoipa::path(
    post,
    path = "/api/terminology/{id}/set-current",
    tag = "Terminology",
    params(
        ("id" = Uuid, Path, description = "Terminology setting ID to set as current"),
    ),
    responses(
        (status = 200, description = "Terminology setting set as current successfully", body = TerminologySetting),
        (status = 404, description = "Terminology setting not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn set_current_terminology(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TerminologySetting>, Error> {
    let terminology_repo = state.terminology_repository();
    
    // First verify the terminology setting exists
    let terminology_setting = terminology_repo
        .get_terminology_setting_by_id(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Terminology setting with ID {} not found", id))
                }
                _ => {
                    error!("Failed to get terminology setting: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    // Then set it as current
    terminology_repo
        .set_current_terminology(id)
        .await
        .map_err(|e| {
            error!("Failed to set current terminology: {}", e);
            Error::internal_server_error()
        })?;
    
    Ok(Json(terminology_setting))
}

/// Delete a terminology setting
#[utoipa::path(
    delete,
    path = "/api/terminology/{id}",
    tag = "Terminology",
    params(
        ("id" = Uuid, Path, description = "Terminology setting ID"),
    ),
    responses(
        (status = 204, description = "Terminology setting deleted successfully"),
        (status = 404, description = "Terminology setting not found", body = ErrorResponse),
        (status = 400, description = "Cannot delete the current active terminology setting", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_terminology_setting(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, Error> {
    let terminology_repo = state.terminology_repository();
    
    // First check if it's the current setting
    let current = terminology_repo.get_current_terminology().await;
    if let Ok(current_setting) = current {
        if current_setting.id == id {
            return Err(Error::bad_request("Cannot delete the current active terminology setting"));
        }
    }
    
    terminology_repo
        .delete_terminology_setting(id)
        .await
        .map_err(|e| {
            match e {
                crate::repository::db::DbError::NoDataReturned => {
                    Error::not_found(&format!("Terminology setting with ID {} not found", id))
                }
                _ => {
                    error!("Failed to delete terminology setting: {}", e);
                    Error::internal_server_error()
                }
            }
        })?;
    
    Ok(StatusCode::NO_CONTENT)
} 