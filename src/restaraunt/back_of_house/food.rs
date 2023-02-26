pub struct Breakfast {
    pub toast: String, // public field
    seasonal_fruit: String, // private field
}

impl Breakfast {
    pub fn breakfast_with_toast(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }

    pub fn seasonal_fruit(&self) -> String {
        self.seasonal_fruit.clone()
    }
}