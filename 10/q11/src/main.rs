use std::cell::RefCell;

fn main() {
  let x = RefCell::new(5);
  let x_ref1 = x.borrow();
  let x_ref2 = x.borrow();
  println!("x_ref1: {}, x_ref2: {}", x_ref1, x_ref2);

  /* Code for Task 1 */

  let mut x_ref3 = x.borrow_mut();
  *x_ref3 = 6;

  /* Code for Task 2 */

  println!("Stored value: {:?}", x);
}
