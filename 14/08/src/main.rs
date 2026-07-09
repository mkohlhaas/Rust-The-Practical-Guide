use std::{thread, time::Duration};

fn main() {
  let x = "some string".to_string();

  // x is moved into the thread
  // closure is FnOnce; no `move` (before `||`) necessary
  thread::spawn(|| {
    let y = x;
    println!("{y}");
  });

  thread::sleep(Duration::from_millis(1));

  // println!("{}", x); // ⚠️ Error: x moved
}
