#[derive(Debug)]
enum Value {
  // Add code here
}

fn main() {
  let some_val = vec![Value::Integer(12), Value::Float(15.5)];
  for i in some_val {
    match i {
      Value::Integer(num) => println!("Integer: {} ", num),
      Value::Float(num) => println!("Float: {}", num),
    }
  }
}
