// Executing Tasks on a Single Thread

// The Tokio Current-Thread Runtime (formerly single-threaded runtime) executes all asynchronous
// tasks and drives I/O and timers entirely on a single thread. It is ideal for single-threaded
// applications, environments with strict CPU affinities, or when working with !Send data structures
// like Rc.

const NUM_TASKS: u8 = 10;

async fn printing(i: u8) {
  println!("Task {:0>2}", i);
}

// single-threaded runtime aka as the current_thread runtime
// https://docs.rs/tokio/latest/tokio/attr.main.html#current-thread
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
