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

  fn remove(&mut self) -> Option<i32> {
    if self.head.is_none() {
      // list is empty so we can not remove
      None
    } else {
      // get to be removed value
      let removed_val = self.head.as_ref().unwrap().borrow().element;
      // adjust Pointer's
      self
        .head
        .take()
        .map(|old_head| match old_head.borrow_mut().next.take() {
          Some(new_head) => {
            // removes from head
            new_head.borrow_mut().previous = None;
            self.head = Some(new_head); // point to new head
          }
          None => {
            self.tail = None;
          }
        });
      // return removed value
      Some(removed_val)
    }
  }

  fn printing(&self) {
    let mut list_traversal = self.head.clone();
    while list_traversal.is_some() {
      print!("{} ", list_traversal.as_ref().unwrap().borrow().element);
      list_traversal = list_traversal.unwrap().borrow().next.clone();
    }
    println!();
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

fn main() {
  let mut list1 = DoublyLinkedlist::new();
  list1.add(1);
  list1.add(2);
  list1.add(3);
  list1.add(4);
  list1.add(5);

  list1.printing(); // 5 4 3 2 1

  list1.remove();
  list1.printing(); // 4 3 2 1

  list1.remove();
  list1.printing(); // 3 2 1

  list1.remove();
  list1.printing(); // 2 1

  list1.remove();
  list1.printing(); // 1

  list1.remove();
  list1.printing(); // (empty)

  println!("Done!");
}

