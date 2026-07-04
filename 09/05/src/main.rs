fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
  V1: Fn(&str) -> bool,
  V2: Fn(u8) -> bool,
{
  simple_validator(name) && advance_validator(age)
}
