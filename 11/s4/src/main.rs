use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct Doubly_Linklist {
  head: pointer,
  tail: pointer,
}

#[derive(Debug)]
struct Node {
  element: i32,
  next: pointer,
  prev: pointer,
}

type pointer = Option<Rc<RefCell<Node>>>;

impl Doubly_Linklist {
  fn remove_back(&mut self) {
    if self.tail.is_none() {
      println!("list is emtpy so we can not remove");
    } else {
      self
        .tail
        .take()
        .map(|old_tail| match old_tail.borrow_mut().prev.take() {
          Some(new_tail) => {
            new_tail.borrow_mut().next.take();
            self.tail = Some(new_tail);
            self.tail.clone()
          }
          None => {
            self.head.take();
            println!("List is empty after removal");
            None
          }
        });
    }
  }
}

impl Node {
  fn new(element: i32) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node {
      element: element,
      next: None,
      prev: None,
    }))
  }
}
fn main() {}
