use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusResponse {
    Success,
    Error,
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub status: StatusResponse,
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: StatusResponse,
    pub message: String,
}
