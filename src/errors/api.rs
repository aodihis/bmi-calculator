use std::fmt::{Display, Formatter};
use crate::models::generic::{ErrorResponse, StatusResponse};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub enum ApiError {
    ValidationError(String),
    DeserializationError(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ValidationError(message) => write!(f, "Invalid parameters: {}", message),
            ApiError::DeserializationError(message) => write!(f, "Invalid parameters: {}", message),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::DeserializationError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        HttpResponse::build(status).json(ErrorResponse {
            status: StatusResponse::Error,
            message: match self {
                ApiError::ValidationError(_) => "Invalid parameters",
                ApiError::DeserializationError(_) => "Invalid parameters",
            }
            .to_string(),
        })
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::DeserializationError(err.to_string())
    }
}
