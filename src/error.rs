use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not Found")]
    NotFound,

    #[error("Validation Error: {0}")]
    Validation(String),

    #[error("Database Error: {0}")]
    Database(String),

    #[error("Not Implemented: {0}")]
    NotImplemented(String),

    #[error("Internal Server Error")]
    InternalServerError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match self {
            APIError::Unauthorized => StatusCode::UNAUTHORIZED,
            APIError::NotFound => StatusCode::NOT_FOUND,
            APIError::Validation(_) => StatusCode::BAD_REQUEST,
            APIError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let payload = ErrorResponse {
            error: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(payload)
    }
}
