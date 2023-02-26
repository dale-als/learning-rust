pub fn celsius_and_fahrenheit(degree: f64, scale: &str) -> String {
    if scale != "C" && scale != "F" {
        return "Incorrect scale".to_string();
    }

    
    let converted_degree: f64 = if scale == "C" {
        degree * 9.0/5.0 + 32.0
    } else {
        (degree - 32.0) * 5.0/9.0  
    };

    return format!("{:.1}{}", converted_degree, if scale == "C" {"F"} else {"C"}); 
}