use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Serialize, Deserialize, Validate)]
pub struct BmiRequest {
    #[validate(range(min = 0.1, max = 5.0))]
    pub height  : f32,
    #[validate(range(min = 10_f32, max = 1000_f32))]
    pub weight  : f32,
}

#[derive(Serialize)]
pub struct BmiResponse {
    pub bmi: f32,
    pub classification: String,
}

