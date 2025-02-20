use actix_web::{web, HttpResponse};
use validator::Validate;
use crate::models::bmi::{BmiRequest, BmiResponse};
use crate::models::generic::{ApiResponse, StatusResponse};
use crate::utils::bmi::{bmi_classification, calculate_bmi};

pub async fn bmi(data: web::Json<BmiRequest>) -> HttpResponse {

    if let Err(err) = data.validate() {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: StatusResponse::Error,
            data: None,
            message: Option::from(err.to_string())
        });
    }

    let bmi_value = calculate_bmi(data.weight,data.height);
    let classification = bmi_classification(bmi_value);

    HttpResponse::Ok().json(
        ApiResponse {
            status: StatusResponse::Success,
            message: None,
            data: Some(BmiResponse {
                bmi: bmi_value,
                classification,
            }),
        }
    )
}