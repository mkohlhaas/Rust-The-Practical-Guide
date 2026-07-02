fn main() {
	let my_trip = TravelType::Car;
	let allowance = my_trip.travel_allowance(60.0);
	println!("Travel allowance: ${}", allowance);
}
