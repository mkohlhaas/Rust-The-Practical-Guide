use std::cell::RefCell;

fn main() {
  let a = RefCell::new(10);
  let mut ref1 = a.borrow_mut();
  *ref1 = 15;
  drop(ref1);
  println!("{:?}", a);
}
