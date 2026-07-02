use std::{sync::mpsc, thread};
fn main() {
  let (tx, rx) = mpsc::channel(); // Type mentioning not needed now
  for i in 0..10 {
    let tx_clone = tx.clone();
    thread::spawn(move || {
      println!("Sending Value: {i}");
      tx_clone.send(i).unwrap();
    });
  }
  let received_val = rx.recv().unwrap();
  println!("Received: {received_val}");
}
