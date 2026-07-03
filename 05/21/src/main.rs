#![allow(dead_code)]

enum TravelType {
  Car(f32),
  Train(f32),
  Airplane(f32),
}

impl TravelType {
  fn travel_allowance(&self) -> f32 {
    match self {
      TravelType::Car(miles) => miles * 2.0,
      TravelType::Train(miles) => miles * 3.0,
      TravelType::Airplane(miles) => miles * 5.0,
    }
  }
}

fn main() {
  {
    let my_trip = TravelType::Car(42.0);
    let allowance = my_trip.travel_allowance();
    println!("Travel allowance: ${}", allowance);
  }

  // same example
  {
    let my_trip = TravelType::Car(60.0);
    let allowance = my_trip.travel_allowance();
    println!("Travel allowance: ${}", allowance);
  }
}
