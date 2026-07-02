fn division_status(divident: f64, divisor: f64) -> Result<(), String> {
    let answer = match divisor {
        0.0 => Err(String::from("Error: Division by zero")),
        _ => {
            println!("The division is invalid");
            Ok(())
        }
    };
    answer
}