fn is_valid_user(name: &str, age: u8, simple_validator: fn(&str) -> bool,
    advance_validator: fn(u8) -> bool) -> bool 
{
    simple_validator(name) && advance_validator(age)
} 
fn validate_user_advance(age: u8) -> bool {
    age >= 30
}
fn validate_user_simple(name: &str) -> bool {
    name.len() != 0 
}
struct User {
	...
}
fn main() {
    let person_1 = User {
        ...
    };
   ...
}