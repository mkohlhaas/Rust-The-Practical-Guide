#![allow(dead_code)]

// Rc offers multiple ownership, RefCell offers (interior) mutability for every owner
type Pointer = Option<Rc<RefCell<Node>>>;

use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct DoublyLinkedlist {
  head: Pointer,
  tail: Pointer,
}

impl DoublyLinkedlist {
  fn new() -> Self {
    DoublyLinkedlist {
      head: None,
      tail: None,
    }
  }
}

#[derive(Debug)]
struct Node {
  element: i32,
  next: Pointer,
  previous: Pointer,
}

fn main() {}
