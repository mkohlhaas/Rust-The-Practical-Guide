#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
  next: Option<Weak<RefCell<Node>>>, // using weak's
}

impl Drop for Node {
  fn drop(&mut self) {
    println!("Dropping {:?}", self);
  }
}

fn main() {
  let a = Rc::new(RefCell::new(Node { next: None }));
  println!("a strong count: {:?}", Rc::strong_count(&a),); // 1

  let b = Rc::new(RefCell::new(Node {
    next: Some(Rc::downgrade(&a)), // NOTE: using downgrade(…)
  }));

  println!("\nB is created:");
  println!("a strong count: {:?}", Rc::strong_count(&a),); // 1
  println!("b strong count: {:?}", Rc::strong_count(&b),); // 1

  let c = Rc::new(RefCell::new(Node {
    next: Some(Rc::downgrade(&b)), // NOTE: using downgrade(…)
  }));

  println!("\nC is created:");
  println!("a strong count: {:?}", Rc::strong_count(&a),); // 1
  println!("b strong count: {:?}", Rc::strong_count(&b),); // 1
  println!("c strong count: {:?}", Rc::strong_count(&c),); // 1

  (*a).borrow_mut().next = Some(Rc::downgrade(&c)); // using downgrade now

  println!("\nAfter creating cycle:");
  println!("a strong count: {:?}", Rc::strong_count(&a),); // 1
  println!("b strong count: {:?}", Rc::strong_count(&b),); // 1
  println!("c strong count: {:?}", Rc::strong_count(&c),); // 1

  println!("\nDone!\n");
}
