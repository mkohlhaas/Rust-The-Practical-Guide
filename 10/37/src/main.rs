use std::{cell::RefCell, rc::Rc};
fn main() {
  let a = Rc::new(RefCell::new(String::from("c++")));
  let b = Rc::clone(&a);
  *b.borrow_mut() = String::from("rust");
  println!("{:?}", a);
}
