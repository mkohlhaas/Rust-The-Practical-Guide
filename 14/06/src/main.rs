use std::{thread, time::Duration};

fn main() {
  let x = "some string".to_string();

  // move x into the thread
  thread::spawn(move || {
    println!("{x}");
  });

  thread::sleep(Duration::from_millis(1));

  // println!("{x}"); // ⚠️ Error: moved value x
}
