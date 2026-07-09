use std::sync::{Arc, Barrier, Mutex};
use std::thread;

fn main() {
  let mut threads_vec = Vec::new();
  let task = Arc::new(Mutex::new(vec![]));
  let barrier = Arc::new(Barrier::new(5));

  for i in 1..=5 {
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
      barrier.wait(); // NOTE: the same barrier
      // Task 3
      task
        .lock()
        .unwrap()
        .push(format!("Thread {i}, Completed its part on Task 3"));
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
