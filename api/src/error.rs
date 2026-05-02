use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use thiserror::Error;
use tokio_postgres::Error as PgError;
use std::fmt;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] PgError),
    
    #[error("Pool error: {0}")]
    PoolError(#[from] PoolError),
    
    #[error("Resource not found: {0}")]
    NotFoundError(String),
    
    #[error("Conflict: {0}")]
    ConflictError(String),
    
    #[error("Bad request: {0}")]
    BadRequestError(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    
    #[error("Unauthorized: {0}")]
    UnauthorizedError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.get_error_type(),
            message: self.to_string(),
        };
        
        match self {
            ApiError::NotFoundError(_) => {
                HttpResponse::NotFound().json(error_response)
            }
            ApiError::ConflictError(_) => {
                HttpResponse::Conflict().json(error_response)
            }
            ApiError::BadRequestError(_) => {
                HttpResponse::BadRequest().json(error_response)
            }
            ApiError::UnauthorizedError(_) => {
                HttpResponse::Unauthorized().json(error_response)
            }
            _ => HttpResponse::InternalServerError().json(error_response),
        }
    }
}

impl ApiError {
    fn get_error_type(&self) -> String {
        match self {
            ApiError::DatabaseError(_) => "DATABASE_ERROR".to_string(),
            ApiError::PoolError(_) => "CONNECTION_POOL_ERROR".to_string(),
            ApiError::NotFoundError(_) => "NOT_FOUND".to_string(),
            ApiError::ConflictError(_) => "CONFLICT".to_string(),
            ApiError::BadRequestError(_) => "BAD_REQUEST".to_string(),
            ApiError::InternalServerError(_) => "INTERNAL_SERVER_ERROR".to_string(),
            ApiError::UnauthorizedError(_) => "UNAUTHORIZED".to_string(),
        }
    }
} 