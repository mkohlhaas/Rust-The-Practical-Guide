use std::sync::mpsc;
use std::thread;
fn thread_fn(d: i32, tx: mpsc::Sender<i32>) {
  thread::spawn(move || {
    println!("{} send!", d);
    tx.send(d).unwrap();
  });
}
fn main() {
  let (tx, rx) = mpsc::channel();
  for i in 0..5 {
    thread_fn(i, tx.clone());
  }
  drop(tx);
  for recieving_val in rx {
    println!("{} recieved!", recieving_val);
  }
}
