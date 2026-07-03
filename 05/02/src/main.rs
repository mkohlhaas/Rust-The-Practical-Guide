#![allow(dead_code, unused_variables)]

struct Car {
  owner: String,
  year: u32,
  fuel_level: f32,
  price: u32,
}

fn main() {
  let my_car = Car {
    owner: String::from("ABC"),
    year: 2010,
    fuel_level: 0.0,
    price: 5_000,
  };
}
