println!(
  "User validity {}",
  is_valid_user(
    &person_1.name,
    person_1.age,
    banned_user, // updated
    validate_user_simple,
    validate_user_advance
  )
);
