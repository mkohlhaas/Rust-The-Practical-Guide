use std::{cell::RefCell, rc::Rc};

fn main() {
  // both a and b are owners of the interior String
  let a: Rc<RefCell<String>> = Rc::new(RefCell::new(String::from("C++")));
  let b: Rc<RefCell<String>> = Rc::clone(&a);

  *b.borrow_mut() = String::from("Rust");

  println!("{:?}", a);
}
