fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
  let result = match divisor {
    0.0 => Err(String::from("Error: Division by zero")),
    _ => Ok(dividend / divisor),
  };

  let quotient = result?;
  println!("The quotient is: {:?}", quotient);
  Ok(quotient)
}

fn main() {
  println!("Result: {:?}", division(9.0, 3.0));
  println!("Result: {:?}", division(4.0, 0.0));
  println!("Result: {:?}", division(0.0, 2.0));
}
