#[derive(Default)]
struct Car {
    car_id: u32,
    model_name: String,
    year: u16,
}

impl Car {
    pub fn new(car_model_name: String, car_year: u16) -> Result<Self, String> {
       if car_year < 1990 {
            return Err("The car year must not be older than 1990.".to_string());
        }

        Ok(Self {
            car_id: 12345, // Auto-assigned ID
            model_name: car_model_name,
            year: car_year,
        })
    }
}

fn main() {
    let car_1 = Car::new("MODEL_X".to_string(), 1985);
}