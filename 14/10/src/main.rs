#![allow(unused_variables)]

// Message Passing

use std::{sync::mpsc, thread};

fn main() {
  // NOTE: tx and rx are traditionally used vars for channels

  // The channel is said to be closed if either the transmitter or the receiver half is dropped.
  // let (tx, rx) = mpsc::channel::<String>();
  let (tx, rx) = mpsc::channel();

  let t = thread::spawn(move || {
    let val = "Hi from thread".to_string();
    println!("Sending Value: {val}");

    tx.send(val).unwrap();
  });

  t.join().unwrap();
}
