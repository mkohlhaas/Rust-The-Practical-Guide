#![allow(dead_code)]

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

fn main() {
  // empty
  let a: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node { next: None }));
  println!("a strong count: {:?}", Rc::strong_count(&a)); // 1
  println!("{:?}", a);

  // using a in b
  let b: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
    next: Some(Rc::clone(&a)),
  }));

  println!("\nB is created:");
  println!("a strong count: {:?}, ", Rc::strong_count(&a)); // 2
  println!("b strong count: {:?}", Rc::strong_count(&b)); // 1
  println!("{:?}", a);
  println!("{:?}", b);

  // using b in c
  let c: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
    next: Some(Rc::clone(&b)),
  }));

  // c -> b -> a
  println!("\nC is created:");
  println!("a strong count: {:?}", Rc::strong_count(&a)); // 2
  println!("b strong count: {:?}", Rc::strong_count(&b)); // 2
  println!("c strong count: {:?}", Rc::strong_count(&c)); // 1
  println!();
  println!("{:?}", a);
  println!("{:?}", b);
  println!("{:?}", c);
  println!();

  // creating a cycle
  // c -> b -> a -> c -> …
  (*a).borrow_mut().next = Some(Rc::clone(&c));
  println!("\nAfter creating cycle:");
  println!("a strong count: {:?}", Rc::strong_count(&a)); // 2
  println!("b strong count: {:?}", Rc::strong_count(&b)); // 2
  println!("c strong count: {:?}", Rc::strong_count(&c)); // 2

  // println!("{:?}", a); // ⚠️ Stack Overflow

  println!("\nDone!");
} // no drops are taking place

