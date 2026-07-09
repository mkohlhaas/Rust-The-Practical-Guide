use std::{thread, time::Duration};

fn main() {
  // i32 - a primitive - is Copy
  let x: i32 = 5;

  // move makes a copy of x
  thread::spawn(move || {
    println!("{x}");
  });

  thread::sleep(Duration::from_millis(1));

  println!("{}", x + 1);
}
