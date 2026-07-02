use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct Node {
  next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
  fn drop(&mut self) {
    println!("Dropping {:?}", self);
  }
}
