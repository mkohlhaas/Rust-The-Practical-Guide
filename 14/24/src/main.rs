#![allow(unused_variables)]

use std::sync::Mutex;

fn main() {
  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 10;
  }

  let lock_m1 = m.lock().unwrap();
  println!("m is: {:?}", *lock_m1);

  drop(lock_m1);

  println!("This code is not blocked");
  let lock_m2 = m.lock().unwrap();
  println!("m is: {:?}", *lock_m2);
  println!("Done!");
}
