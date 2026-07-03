#![allow(dead_code, unused_variables)]

struct Car {
  owner: String,
  year: u32,
  fuel_level: f32,
  price: u32,
}

fn main() {
  let owner = String::from("John Doe");
  let year = 2021;
  let fuel_level = 45.5;
  let price = 10_000;

  let my_car = Car {
    owner,
    year,
    fuel_level,
    price,
  };
}
