use std::{sync::mpsc, thread};

fn main() {
  let (tx, rx) = mpsc::channel::<String>();

  for _ in 0..10 {
    // ⚠️ Error: use of moved value `tx`
    thread::spawn(move || {
      let val = "Hi from thread".to_string();
      println!("Sending Value: {val}");
      tx.send(val).unwrap();
    });
  }

  let received_val = rx.recv().unwrap();
  println!("Received: {received_val}");
}
