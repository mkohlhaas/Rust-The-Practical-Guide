#![allow(dead_code)]

enum TravelType {
  Car,
  Train,
  Airplane,
}

impl TravelType {
  fn travel_allowance(&self, miles: f32) -> f32 {
    match self {
      TravelType::Car => miles * 2.0,
      TravelType::Train => miles * 3.0,
      TravelType::Airplane => miles * 5.0,
    }
  }
}

fn main() {
  let my_trip = TravelType::Car;
  let allowance = my_trip.travel_allowance(60.0);
  println!("Travel allowance: ${}", allowance);
}
