struct Car {
  owner: String,
  year: u32,
  fuel_level: f32,
  price: u32,
}

fn display_car_info(car: &Car) {
  println!(
    "owner: {}, Year: {}, Price: {}, Fuel Level: {}",
    car.owner, car.year, car.price, car.fuel_level
  );
}

fn main() {
  let my_car = Car {
    owner: String::from("ABC"),
    year: 2010,
    fuel_level: 0.0,
    price: 5_000,
  };

  display_car_info(&my_car);
}

