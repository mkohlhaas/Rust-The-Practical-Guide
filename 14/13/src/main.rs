use std::sync::mpsc::{self, Sender};
use std::thread;

// mpsc - multiple producers/senders/transmitters, one consumer/receiver

fn main() {
  let (tx, rx) = mpsc::channel::<String>();

  for i in 1..=10 {
    // Sender<…> is Clone, but not Copy
    // Sender<…> is Send and Sync
    // https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.Sender.html
    let tx_clone: Sender<String> = tx.clone();

    thread::spawn(move || {
      let mut val = ". Hi from thread".to_string();
      val = i.to_string() + &val;
      println!("Sending Value: {val}");
      let _ = tx_clone.send(val); // unwrap could panic
    });
  }

  // Receiver<…> is not Copy, not Clone, not Sync, only Send
  // https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
  let received_val = rx.recv().unwrap();
  println!("Received: {received_val}");
}
