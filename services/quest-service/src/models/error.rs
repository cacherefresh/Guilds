use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;
use crate::repository::db::DbError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error: {}", _0)]
    InternalServerError(String),

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(ref message) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: self.to_string(),
                    message: message.clone(),
                })
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: self.to_string(),
                    message: message.clone(),
                })
            }
            ServiceError::NotFound(ref message) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: self.to_string(),
                    message: message.clone(),
                })
            }
        }
    }
}

impl From<DbError> for ServiceError {
    fn from(error: DbError) -> ServiceError {
        match error {
            DbError::NoDataReturned => {
                ServiceError::NotFound("No data found".to_string())
            }
            _ => ServiceError::InternalServerError(error.to_string()),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
} 