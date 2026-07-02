use std::{sync::mpsc, thread};
fn main() {
  let (tx, rx) = mpsc::channel::<String>();
  thread::spawn(move || {
    let val = "Hi from thread".to_string();
    println!("Sending Value: {val}");
    tx.send(val).unwrap();
  });
}
