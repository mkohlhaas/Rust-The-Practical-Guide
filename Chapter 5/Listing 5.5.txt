struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
} 
fn display_car_info(car: &car) {
	println!(
	“owner: {}, Year: {}, Price: {}”, 
	car.owner, car.year, car.price);
}
fn main() {
     let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000,
    };
}