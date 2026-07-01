enum CustomNeverType {}
fn function_never_succeeds() -> Result<CustomNeverType, i32> {
    Err(100)  // Return an arbitrary error code
}
fn function_never_fails() -> Result<i32, CustomNeverType> {
    Ok(200)  // Return a success value
}
fn main() {
    match function_never_succeeds() {
        Ok(_) => println!("This will never print because success is not possible"),
        Err(e) => println!("Error occurred as expected: {}", e),
    }
    match function_never_fails() {
        Ok(value) => println!("Success as expected: {}", value),
        Err(_) => println!("This will never print because failure is not possible"),
    }
} 