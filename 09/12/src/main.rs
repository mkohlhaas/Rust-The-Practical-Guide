fn validate_user_simple(name: &str) -> bool {
    name.len() != 0 
}
struct User {
	...
}
fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
..

fn main() {
    let person_1 = User {
        ...
    };
    //    let validate_user_simple = move |name: &str| name.len() != 0;  // removed
    let validate_user_advance = |age: u8| age >= 30;
    println!(
        "User validity {}",
        is_valid_user(
            &person_1.name,
            person_1.age,
            validate_user_simple, // this is now functional pointer
            validate_user_advance
        )
    );  
}