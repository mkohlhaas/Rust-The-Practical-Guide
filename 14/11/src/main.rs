#![allow(unused_variables)]

use std::{sync::mpsc, thread};

fn main() {
  let (tx, rx) = mpsc::channel::<String>();

  thread::spawn(move || {
    let val = "Hi from thread".to_string();
    println!("Sending:  {val}");
    tx.send(val).unwrap(); // val is consumed/moved (by value); primitives would be copied (Copy trait)
    // println!("Val is: {val}"); // ⚠️ Error: val moved
  });

  let received_val = rx.recv().unwrap(); // .recv() blocks
  println!("Received: {received_val}");
}
