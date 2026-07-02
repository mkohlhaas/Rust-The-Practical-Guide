use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug)]
struct Node {
  next: Option<Weak<RefCell<Node>>>,
}
