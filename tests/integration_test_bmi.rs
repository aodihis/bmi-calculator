//! # BMI Integration Test
//!
//! This the integration test for BMI endpoint.
//! 1. Normal test case.
//! 2. Invalid Parameters.
use actix_web::{test, App};


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    use bmi_calculator::routes::bmi_route::init as bmi_routes_init;
    use bmi_calculator::utils::bmi::BmiClass;

    #[actix_web::test]
    async fn test_bmi_endpoint() {

        let app = test::init_service(App::new().configure(bmi_routes_init)).await;

        // Underweight
        let payload = json!({
            "weight": 55,
            "height": 1.9,
        });

        let resp = test::TestRequest::post()
            .uri("/api/bmi")
            .set_json(&payload)
            .send_request(&app).await;
        assert!(resp.status().is_success());

        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "success");
        assert_eq!(body["data"]["bmi"].as_f64().unwrap(), 15.24);
        assert_eq!(body["data"]["classification"], BmiClass::Underweight.as_str().to_string());

        // Healthy
        let payload = json!({
            "weight": 55,
            "height": 1.61,
        });

        let resp = test::TestRequest::post()
                                    .uri("/api/bmi")
                                    .set_json(&payload)
                                    .send_request(&app).await;
        assert!(resp.status().is_success());

        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "success");
        assert_eq!(body["data"]["bmi"].as_f64().unwrap(), 21.22);
        assert_eq!(body["data"]["classification"], BmiClass::Healthy.as_str().to_string());


        //Overweight
        let payload = json!({
            "weight": 70,
            "height": 1.61,
        });

        let resp = test::TestRequest::post()
            .uri("/api/bmi")
            .set_json(&payload)
            .send_request(&app).await;
        assert!(resp.status().is_success());

        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "success");
        assert_eq!(body["data"]["bmi"].as_f64().unwrap(), 27.01);
        assert_eq!(body["data"]["classification"], BmiClass::Overweight.as_str().to_string());
    }

    #[actix_web::test]
    async fn test_invalid_params() {

        let app = test::init_service(App::new().configure(bmi_routes_init)).await;
        let payload = json!({
            "weight": -55,
            "height": 1.61,
        });

        let resp = test::TestRequest::post()
            .uri("/api/bmi")
            .set_json(&payload)
            .send_request(&app).await;

        assert!(resp.status().is_client_error());

        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], "Invalid parameters");
    }

}
