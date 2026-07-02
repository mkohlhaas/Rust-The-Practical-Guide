fn main() {
  let a = Rc::new(RefCell::new(Node { next: None }));
  println!("a strong count: {:?}", Rc::strong_count(&a),);

  let b = Rc::new(RefCell::new(Node {
    next: Some(Rc::downgrade(&a)), // Using downgrade now
  }));

  println!("\nB is created:");
  println!("a strong count: {:?},", Rc::strong_count(&a),);
  println!("b strong count: {:?}", Rc::strong_count(&b),);

  let c = Rc::new(RefCell::new(Node {
    next: Some(Rc::downgrade(&b)), // using downgrade now
  }));
  println!("\nC is created:");
  println!("a strong count: {:?}, ", Rc::strong_count(&a),);
  println!("b strong count: {:?},", Rc::strong_count(&b),);
  println!("c strong count: {:?}", Rc::strong_count(&c),);

  (*a).borrow_mut().next = Some(Rc::downgrade(&c)); // using downgrade now

  println!("\nAfter creating cycle:");
  println!("a strong count: {:?},", Rc::strong_count(&a),);
  println!("b strong count: {:?},", Rc::strong_count(&b),);
  println!("c strong count: {:?},", Rc::strong_count(&c),);
}
