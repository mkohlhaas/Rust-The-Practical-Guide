fn main() {
  let my_trip = TravelType::Car(60.0);
  let allowance = my_trip.travel_allowance(); // no inputs
  println!("Travel allowance: ${}", allowance);
}
