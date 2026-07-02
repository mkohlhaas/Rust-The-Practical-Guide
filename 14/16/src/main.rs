use std::{sync::mpsc, thread};
fn main() {
  let (tx, rx) = mpsc::channel();
  for i in 0..10 {
    let tx_clone = tx.clone();
    thread::spawn(move || {
      println!("Sending Value: {i}");
      tx_clone.send(i).unwrap();
    });
  }
  for message in rx {
    println!("Received: {message}");
  }
}
