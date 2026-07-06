#![allow(unused_variables)]

use std::{cell::RefCell, rc::Rc};

fn main() {
  let a = Rc::new(RefCell::new(String::from("c++")));

  println!("{:?}", a);
}
