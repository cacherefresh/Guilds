use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFoundError(String),
    
    #[error("Bad request: {0}")]
    BadRequestError(String),
    
    #[error("Unauthorized: {0}")]
    UnauthorizedError(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(e) => {
                log::error!("Database error: {:?}", e);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: 500,
                    message: "Internal server error".to_string(),
                })
            }
            AppError::NotFoundError(msg) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    code: 404,
                    message: msg.clone(),
                })
            }
            AppError::BadRequestError(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    code: 400,
                    message: msg.clone(),
                })
            }
            AppError::UnauthorizedError(msg) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    code: 401,
                    message: msg.clone(),
                })
            }
            AppError::InternalServerError(msg) => {
                log::error!("Internal server error: {}", msg);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: 500,
                    message: "Internal server error".to_string(),
                })
            }
        }
    }
} 