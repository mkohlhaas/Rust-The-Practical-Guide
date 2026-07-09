use std::{sync::mpsc, thread, time::Duration};

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let msg = "some_value".to_string();
    println!("Sending:  {msg}");
    thread::sleep(Duration::from_millis(2000));
    tx.send(msg).unwrap();
  });

  println!("I'm waiting till I receive the message.");
  let received_val = rx.recv().unwrap(); // .recv() is blocking
  println!("Received: {received_val}");

  println!("Done!");
}
