use actix_web::{web, HttpResponse};
use validator::Validate;
use crate::errors::api::ApiError;
use crate::models::bmi::{BmiRequest, BmiResponse};
use crate::models::generic::{StatusResponse, SuccessResponse};
use crate::utils::bmi::{bmi_classification, calculate_bmi};

pub async fn bmi(data: web::Json<BmiRequest>) -> Result<HttpResponse, ApiError> {
    if let Err(err) = data.validate() {
        return Err(
            ApiError::ValidationError(err.to_string())
        );
    }

    let bmi_value = calculate_bmi(data.weight,data.height);
    let classification = bmi_classification(bmi_value);

    Ok(HttpResponse::Ok().json(
        SuccessResponse {
            status: StatusResponse::Success,
            data: Some(BmiResponse {
                bmi: bmi_value,
                classification: classification.as_str().to_string(),
            }),
        }
    ))
}