use std::sync::{Arc, Mutex};
use std::thread;

// Motivating Example for Barriers

fn main() {
  let vec_string = Arc::new(Mutex::new(vec![]));

  let mut threads_vec = Vec::new();

  // 5 threads, 2 tasks
  // Second Task should only start processing once Task 1 is complete.
  for i in 1..=5 {
    let vec_string_clone = vec_string.clone();
    let handle = thread::spawn(move || {
      // Tasks 1
      vec_string_clone
        .lock()
        .unwrap()
        .push(format!("Thread {i}, Completed its part on Task 1"));
      // Task 2
      vec_string_clone
        .lock()
        .unwrap()
        .push(format!("Thread {i}, Completed its part on Task 2"));
    });
    threads_vec.push(handle);
  }

  for handle in threads_vec {
    handle.join().unwrap();
  }

  let task_lock: &Vec<String> = &*vec_string.lock().unwrap();

  for msg in task_lock {
    println!("{msg}");
  }

  println!("Done!");
}
