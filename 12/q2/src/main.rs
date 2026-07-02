#[derive(Default)]
struct Car {
    car_id: u32,
    model_name: String,
    year: u16,
}

impl Car {
    pub fn new(car_model_name: String, car_year: u16) -> Result<Self, String> {
        // Validation: year must be 1990 or later
    }
}

fn main() {
    let car_1 = Car::new("MODEL_X".to_string(), 1985);
}