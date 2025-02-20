use crate::models::generic::ErrorResponse;
use crate::models::generic::StatusResponse;
use actix_web::http::StatusCode;
use actix_web::{error, HttpRequest, HttpResponse};

pub  async fn not_found() -> actix_web::Result<HttpResponse> {

    let error_response = ErrorResponse {
        status: StatusResponse::Error,
        message: "Not found".to_string(),
    };

    let response = HttpResponse::build(StatusCode::NOT_FOUND)
        .json(error_response);

    Ok(response)

}


pub fn handle_json_error(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let error_resp = ErrorResponse{
        status: StatusResponse::Error,
        message: "Invalid params".to_string(),
    };

    error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(error_resp)
    ).into()
}