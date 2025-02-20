use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Deserialize, Validate)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_bmi_request() {
        let bmi_req = BmiRequest {
            height: 1.0,
            weight: 10.0,
        };
        assert!(bmi_req.validate().is_ok());
    }

    #[test]
    fn test_negative_bmi_request() {
        let bmi_req = BmiRequest {
            height: 5.1,
            weight: 5.0,
        };
        assert!(bmi_req.validate().is_err());

        let bmi_req = BmiRequest {
            height: 1.0,
            weight: -5.0,
        };
        assert!(bmi_req.validate().is_err());
    }
    #[test]
    fn test_invalid_bmi_request() {
        let bmi_req = BmiRequest {
            height: -1.0,
            weight: 5.0,
        };
        assert!(bmi_req.validate().is_err());

        let bmi_req = BmiRequest {
            height: 1.0,
            weight: 1000.1,
        };
        assert!(bmi_req.validate().is_err());
    }
}
