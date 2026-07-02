fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
  (num1 * num2, num1 + num2, num1 - num2)
}

fn main() {
  basic_math(9, 3);
}
