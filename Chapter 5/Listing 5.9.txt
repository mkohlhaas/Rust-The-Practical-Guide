fn main() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000,
    };
    let new_owner = my_car.sell();
    my_car.refuel(10.5); // Error
}