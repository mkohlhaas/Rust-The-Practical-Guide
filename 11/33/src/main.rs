fn main() {
  let a = Rc::new(RefCell::new(Node { next: None }));
  println!("a strong count: {:?}", Rc::strong_count(&a));
  let b = Rc::new(RefCell::new(Node {
    next: Some(Rc::clone(&a)),
  }));

  println!("\nB is created:");
  println!("a strong count: {:?}, ", Rc::strong_count(&a));
  println!("b strong count: {:?}", Rc::strong_count(&b));
  let c = Rc::new(RefCell::new(Node {
    next: Some(Rc::clone(&b)),
  }));
  println!("\nC is created:");
  println!("a strong count: {:?}", Rc::strong_count(&a));
  println!("b strong count: {:?}", Rc::strong_count(&b));
  println!("c strong count: {:?}", Rc::strong_count(&c));
}
