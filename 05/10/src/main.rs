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

  fn refuel(&mut self, gallons: f32) {
    self.fuel_level += gallons;
  }

  fn sell(self) -> Self {
    self
  }

  fn monthly_insurance() -> u32 {
    123
  }
}

fn main() {
  {
    let mut my_car = Car {
      owner: String::from("ABC"),
      year: 2010,
      fuel_level: 0.0,
      price: 5_000,
    };
    my_car.display_car_info();

    my_car.refuel(10.0);
    my_car.display_car_info();

    Car::sell(my_car);
  }

  {
    let mut my_car = Car {
      owner: String::from("ABC"),
      year: 2010,
      fuel_level: 0.0,
      price: 5_000,
    };

    my_car.refuel(10.5);
    my_car.sell();
  }

  {
    Car::monthly_insurance();
  }
}
