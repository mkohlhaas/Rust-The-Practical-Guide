use std::sync::{Arc, Barrier, Mutex};
use std::thread;

const NUM_THREADS: usize = 5;

fn main() {
  let mut threads_vec = Vec::new();
  let task = Arc::new(Mutex::new(vec![]));
  // Barrier is Send and Sync
  // https://doc.rust-lang.org/std/sync/struct.Barrier.html
  // Creates a new barrier that can block 5 threads.
  let barrier = Arc::new(Barrier::new(NUM_THREADS));

  for i in 1..=NUM_THREADS {
    let task = task.clone();
    let barrier = barrier.clone();

    let handle = thread::spawn(move || {
      // Tasks 1
      task
        .lock()
        .unwrap()
        .push(format!("Thread {i}, Completed its part on Task 1"));
      barrier.wait();
      // Task 2
      task
        .lock()
        .unwrap()
        .push(format!("Thread {i}, Completed its part on Task 2"));
    });
    threads_vec.push(handle);
  }

  for handle in threads_vec {
    handle.join().unwrap();
  }

  let task_lock = &*task.lock().unwrap();

  for msg in task_lock {
    println!("{msg}");
  }

  println!("The tasks are now completed in sequential order.");
}
