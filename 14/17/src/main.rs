use std::{sync::mpsc, thread};

fn main() {
  let (tx, rx) = mpsc::channel();

  for i in 1..=10 {
    let tx_clone = tx.clone();

    thread::spawn(move || {
      println!("Sending Value: {i}");
      // all messages will be received
      // no early closing of channel
      // we can use .unwrap()
      tx_clone.send(i).unwrap();
    });
  }

  drop(tx); // close original transmitter

  // treat Receiver as an iterator to receive all messages
  // channel closes when all transmitters are dropped
  // but original `tx` is still alive -> program will be kept in a running state
  for msg in rx {
    println!("Received: {msg}");
  }

  println!("Done!"); // will be printed
}
