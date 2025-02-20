
#[derive(Debug,PartialEq)]
pub enum BmiClass {
    Underweight,
    Healthy,
    Overweight,
    Obese1,
    Obese2,
    Obese3
}

impl BmiClass {
    pub fn as_str(&self) -> &'static str {
        match *self {
            BmiClass::Underweight => {"Underweight"}
            BmiClass::Healthy => {"Healthy weight"}
            BmiClass::Overweight => {"Overweight but not obese"}
            BmiClass::Obese1 => {"Obese class I"}
            BmiClass::Obese2 => {"Obese class II"}
            BmiClass::Obese3 => {"Obese class III"}
        }
    }
}
pub fn calculate_bmi(weight: f32, height: f32) -> f32 {
     (weight / (height*height)).max(0.0)
}

pub fn bmi_classification(bmi: f32) -> BmiClass {
    match bmi {
        bmi if bmi < 18.5 => {
            BmiClass::Underweight
        },
        bmi if bmi < 25.0 => {
            BmiClass::Healthy
        },
        bmi if bmi < 30.0 => {
            BmiClass::Overweight
        },
        bmi if bmi < 35.0 => {
            BmiClass::Obese1
        },
        bmi if bmi < 40.0 => {
            BmiClass::Obese2
        },
        _ => {
            BmiClass::Obese3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_decimals(num: f32) -> f32 {
        (num * 100.0).floor() / 100.0
    }
    #[test]
    fn test_calculate_bmi() {
        let res = calculate_bmi(50.0, 1.3);
        assert_eq!(two_decimals(res), 29.58);

        let res = calculate_bmi(90.0, 1.7);
        assert_eq!(two_decimals(res), 31.14);
    }

    #[test]
    fn test_bmi_min() {
        let res = calculate_bmi(-60.0, 1.3);
        assert_eq!(res, 0.0);
    }
    #[test]
    fn test_calculate_bmi_classification() {
        let res = bmi_classification(-18.4);
        assert_eq!(res, BmiClass::Underweight);

        let res = bmi_classification(18.4);
        assert_eq!(res, BmiClass::Underweight);

        let res = bmi_classification(22.4);
        assert_eq!(res, BmiClass::Healthy);

        let res = bmi_classification(27.4);
        assert_eq!(res, BmiClass::Overweight);

        let res = bmi_classification(32.4);
        assert_eq!(res, BmiClass::Obese1);

        let res = bmi_classification(35.4);
        assert_eq!(res, BmiClass::Obese2);

        let res = bmi_classification(90.4);
        assert_eq!(res, BmiClass::Obese3);
    }
}