fn perform_operation(value: i32) -> Result<(), String> {
  if value > 0 {
    println!("Operation performed successfully with value: {}", value);
    Ok(()) // Return unit type on success
  } else {
    Err(String::from("Error: Value must be positive")) // Return error message
  }
}
fn main() {
  let positive_result = perform_operation(10);
  match positive_result {
    Ok(_) => println!("Positive operation completed."),
    Err(e) => println!("{}", e),
  }
  let negative_result = perform_operation(-5);
  match negative_result {
    Ok(_) => println!("This will not print."),
    Err(e) => println!("{}", e),
  }
}
