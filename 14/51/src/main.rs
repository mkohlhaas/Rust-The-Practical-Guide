// Asynchronous I/O Using the Tokio Sleep

use std::time::Duration;
use tokio::time::sleep;

const NUM_TASKS: u8 = 10;

async fn printing(i: u8) {
  sleep(Duration::from_millis(200)).await; // Tokio's sleep function
  println!("Task {:0>2}", i);
}

// single-threaded runtime aka as the current_thread runtime
// https://docs.rs/tokio/latest/tokio/attr.main.html#current-thread

// The Futures are scheduled on a single thread.
// However, these tasks do not block the other tasks on the same thread.
#[tokio::main(flavor = "current_thread")]
async fn main() {
  let mut handles = vec![];

  for i in 1..=NUM_TASKS {
    let handle = tokio::spawn(async move {
      println!("Task {:0>2}, printing, first time", i);
      printing(i).await;
      println!("Task {:0>2}, printing, second time", i);
      printing(i).await;
      println!("Task {:0>2}, completed", i);
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.await.unwrap();
  }

  println!("Done!");
}
