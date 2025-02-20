use serde::Serialize;


#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusResponse {
    Success,
    Error,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: StatusResponse,
    pub message: Option<String>,
    pub data: Option<T>,
}