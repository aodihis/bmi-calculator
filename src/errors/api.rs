use crate::models::generic::{ErrorResponse, StatusResponse};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display("Validation error: {_0}")]
    ValidationError(String),

    #[display("Deserialization error: {_0}")]
    DeserializationError(String),
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
