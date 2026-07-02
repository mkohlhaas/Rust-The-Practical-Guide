use std::{sync::Mutex, thread};
fn main() {
  let m = Mutex::new(5);
  {
    let mut num = m.lock().unwrap();
    *num = 10;
  }
  let lock_m = m.lock().unwrap();
  println!("m is: {:?}", *lock_m);
}
