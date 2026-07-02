fn main() {
  let mut stack = MaxStack::new();
  stack.push(55);
  stack.push(80);
  stack.push(120);
  stack.push(99);
  stack.push(22);
  stack.push(140);
  stack.push(145);

  print!("Maximum value of stock: ");
  println!("{:}", stack.max_value());

  println!("After going one week back");
  print!("Maximum value of stock: ");
  stack.pop();

  println!("{:}", stack.max_value());
}
