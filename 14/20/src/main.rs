use std::{sync::mpsc, thread, time::Duration};
fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let msg = "some_value".to_string();
    println!("Sending:  {msg}");
    thread::sleep(Duration::from_millis(2000));
    tx.send(msg).unwrap();
  });

  loop {
    // .try_recv() does not block
    match rx.try_recv() {
      Ok(msg) => {
        println!("Received: {}", msg);
        break;
      }
      Err(_) => {}
    }
  }

  println!("Done!");
}
