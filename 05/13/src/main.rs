fn main() {
     // Instead of using syntax below
     let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000,
    };
    // easier way to create new instances
    let new_car = Car::new("ABC".to_string(), 2010);	
}