

pub fn calculate_bmi(weight: f32, height: f32) -> f32 {
     (weight / (height*height)).max(0.0)
}

pub fn bmi_classification(bmi: f32) -> String {
    match bmi {
        bmi if bmi < 18.5 => {
            "Underweight".to_owned()
        },
        bmi if bmi < 25.0 => {
            "Healthy weight".to_owned()
        },
        bmi if bmi < 30.0 => {
            "Overweight but not obese".to_owned()
        },
        bmi if bmi < 35.0 => {
            "Obese class I".to_owned()
        },
        bmi if bmi < 40.0 => {
            "Obese class II".to_owned()
        },
        _ => {
            "Obese class III".to_owned()
        }
    }
}