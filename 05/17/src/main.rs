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
  let my_travel = TravelType::Car;
  my_travel.travel_allowance(42.0);
}
