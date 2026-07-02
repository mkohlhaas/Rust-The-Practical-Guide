mod utilities {
  pub mod math {
    pub fn multiply(a: i32, b: i32) -> i32 {
      a * b
    }
  }

  pub fn calculate() {
    let result = math::multiply(3, 4); // relative path with no prefix
    println!("Multiplication result: {}", result);

    let result_self = self::math::multiply(5, 6); // relative path with self prefix
    println!("Result using `self`: {}", result_self);
  }
}
