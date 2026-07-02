use std::{sync::mpsc, thread, time::Duration};
fn main() {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let x = "some_value".to_string();
    println!("Sending value {x}");
    thread::sleep(Duration::from_secs(3));
    tx.send(x).unwrap();
  });
  let received_val = rx.recv().unwrap();
  println!("I will not excute until the value is recieved");
}
