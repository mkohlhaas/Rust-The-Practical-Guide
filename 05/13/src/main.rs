#![allow(dead_code, unused_variables)]

struct Car {
  owner: String,
  year: u32,
  fuel_level: f32,
  price: u32,
}

impl Car {
  fn new(name: String, year: u32) -> Self {
    Self {
      owner: name,
      year: year,
      fuel_level: 0.0,
      price: 0,
    }
  }

  fn display_car_info(&self) {
    println!(
      "owner: {}, Year: {}, Price: {}, Fuel Level: {}",
      self.owner, self.year, self.price, self.fuel_level
    );
  }

  fn refuel(&mut self, gallons: f32) {
    self.fuel_level += gallons;
  }

  fn sell(self) -> Self {
    self
  }

  fn selling_price(&self) -> u32 {
    self.price + Car::monthly_insurance()
  }

  fn monthly_insurance() -> u32 {
    123
  }
}

fn main() {
  {
    let my_car = Car::new(String::from("ABC"), 2010);
    my_car.display_car_info();
  }

  {
    // Instead of using this syntax …
    let my_car = Car {
      owner: String::from("ABC"),
      year: 2010,
      fuel_level: 0.0,
      price: 5_000,
    };

    // … easier way to create new instances.
    let new_car = Car::new("ABC".to_string(), 2010);
  }
}
