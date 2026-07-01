fn main() {
    let person_1 = User {
        name: String::from("someone"),
        age: 35,
        salary: 40_000,
    }; 
    println!("User validity {}", validate_user(&person_1.name));
}