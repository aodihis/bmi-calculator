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
        let send_req = |payload, app| {
            test::TestRequest::post()
                .uri("/api/bmi")
                .set_json(payload)
                .send_request(app)
        };

        let check_body =
            |body: Value, expected_status: &str, expected_bmi: f64, expected_class: String| {
                assert_eq!(body["status"], expected_status);
                assert_eq!(body["data"]["bmi"].as_f64().unwrap(), expected_bmi);
                assert_eq!(body["data"]["classification"], expected_class);
            };
        let app = test::init_service(App::new().configure(bmi_routes_init)).await;

        // Underweight
        let payload = json!({
            "weight": 55,
            "height": 1.9,
        });

        let resp = send_req(&payload, &app).await;
        assert!(resp.status().is_success());

        let body: Value = test::read_body_json(resp).await;
        check_body(
            body,
            "success",
            15.24,
            BmiClass::Underweight.as_str().to_string(),
        );

        // Healthy
        let payload = json!({
            "weight": 55,
            "height": 1.61,
        });

        let resp = send_req(&payload, &app).await;
        assert!(resp.status().is_success());

        let body: Value = test::read_body_json(resp).await;
        check_body(
            body,
            "success",
            21.22,
            BmiClass::Healthy.as_str().to_string(),
        );

        //Overweight
        let payload = json!({
            "weight": 70,
            "height": 1.61,
        });

        let resp = send_req(&payload, &app).await;

        let body: Value = test::read_body_json(resp).await;
        check_body(
            body,
            "success",
            27.01,
            BmiClass::Overweight.as_str().to_string(),
        );

        //Obese 1
        let payload = json!({
            "weight": 80,
            "height": 1.61,
        });

        let resp = send_req(&payload, &app).await;

        let body: Value = test::read_body_json(resp).await;
        check_body(
            body,
            "success",
            30.86,
            BmiClass::Obese1.as_str().to_string(),
        );

        //Obese 2
        let payload = json!({
            "weight": 100,
            "height": 1.61,
        });

        let resp = send_req(&payload, &app).await;

        let body: Value = test::read_body_json(resp).await;
        check_body(
            body,
            "success",
            38.58,
            BmiClass::Obese2.as_str().to_string(),
        );

        //Obese 3
        let payload = json!({
            "weight": 100,
            "height": 1.51,
        });

        let resp = send_req(&payload, &app).await;

        let body: Value = test::read_body_json(resp).await;
        check_body(
            body,
            "success",
            43.86,
            BmiClass::Obese3.as_str().to_string(),
        );
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
            .send_request(&app)
            .await;

        assert!(resp.status().is_client_error());

        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], "Invalid parameters");

        // Invalid weight
        let payload = json!({
            "weight": 2000,
            "height": 1.61,
        });

        let resp = test::TestRequest::post()
            .uri("/api/bmi")
            .set_json(&payload)
            .send_request(&app)
            .await;

        assert!(resp.status().is_client_error());

        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], "Invalid parameters");

        // Invalid height
        let payload = json!({
            "weight": 100,
            "height": 7,
        });

        let resp = test::TestRequest::post()
            .uri("/api/bmi")
            .set_json(&payload)
            .send_request(&app)
            .await;

        assert!(resp.status().is_client_error());

        let body: Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "error");
        assert_eq!(body["message"], "Invalid parameters");
    }
}
