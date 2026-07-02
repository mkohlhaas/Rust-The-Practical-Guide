fn is_valid_user(
  name: &str,
  age: u8,
  banned_user_name: &str,                   //updated
  simple_validator: fn(&str, &str) -> bool, //updated
  advance_validator: fn(u8) -> bool,
) -> bool {
  simple_validator(name, banned_user_name) && advance_validator(age)
}
