fn invoker<O>(operation: O, operand: i32) -> i32
// This needs to be updated
where
  O: Fn(i32) -> i32,
{
  operation(operand)
}
/* A square function needs to be added here */
fn main() {
  let square = |x: i32| x * x;
  let result = invoker(square, 4);
  println!("Result is: {}", result);
}
