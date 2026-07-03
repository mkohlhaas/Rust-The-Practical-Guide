struct Car {
  owner: String,
  year: u32,
  fuel_level: f32,
  price: u32,
}

impl Car {
  fn display_car_info(&self) {
    println!(
      "owner: {}, Year: {}, Price: {}, Fuel Level: {}",
      self.owner, self.year, self.price, self.fuel_level
    );
  }
}

fn main() {
  let my_car = Car {
    owner: String::from("ABC"),
    year: 2010,
    fuel_level: 0.0,
    price: 5_000,
  };

  my_car.display_car_info();
}
