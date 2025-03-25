use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use utoipa::ToSchema;
use serde::Serialize;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    NotFound(String),
    InternalServerError,
    Unauthorized(String),
    Forbidden(String),
}

impl Error {
    pub fn bad_request(message: &str) -> Self {
        Self::BadRequest(message.to_string())
    }

    pub fn not_found(message: &str) -> Self {
        Self::NotFound(message.to_string())
    }

    pub fn internal_server_error() -> Self {
        Self::InternalServerError
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::Unauthorized(message.to_string())
    }

    pub fn forbidden(message: &str) -> Self {
        Self::Forbidden(message.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            Error::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            Error::Forbidden(message) => (StatusCode::FORBIDDEN, message),
        };

        let body = Json(json!({
            "status": status.as_u16().to_string(),
            "message": message,
        }));

        (status, body).into_response()
    }
} 