use std::{sync::mpsc, thread};

fn main() {
  let (tx, rx) = mpsc::channel(); // type annotation not needed 

  for i in 1..=10 {
    let tx_clone = tx.clone();

    thread::spawn(move || {
      println!("Sending Value: {i}");
      let _ = tx_clone.send(i);
    });
  }

  let received_val = rx.recv().unwrap();
  println!("Received: {received_val}");
}
