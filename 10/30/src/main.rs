#![allow(unused_variables)]

use std::cell::RefCell;

fn main() {
  let x = RefCell::new(50);
  let ref1 = x.borrow();
  let ref2 = x.borrow();
  let ref3 = x.borrow_mut(); // ⚠️ run-time error (borrowing rules are checked at run-time)

  println!("{} {} ", ref1, ref2);
}
