#![allow(dead_code, unused_variables)]

// Rc offers multiple ownership, RefCell offers (interior) mutability for every owner
// Rc: clone()
// RefCell: borrow(), borrow_mut()
// NOTE: possibly better definition
// type Pointer = Rc<RefCell<Node>>;
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

  fn add(&mut self, element: i32) {
    let new_head = Node::new(element);

    match self.head.take() {
      Some(old_head) => {
        old_head.borrow_mut().previous = Some(new_head.clone());
        new_head.borrow_mut().next = Some(old_head.clone());
        self.head = Some(new_head); // move (self.head should be the owner)
      }
      None => {
        self.tail = Some(new_head.clone());
        self.head = Some(new_head); // move (self.head should be the owner)
      }
    }
  }
}

#[derive(Debug)]
struct Node {
  element: i32,
  next: Pointer,
  previous: Pointer,
}

impl Node {
  fn new(element: i32) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node {
      element: element,
      next: None,
      previous: None,
    }))
  }
}

fn main() {}
