use std::{
  sync::{Arc, Barrier, Mutex},
  thread,
};
fn main() {
  let mut threads_vec = Vec::new();
  let task = Arc::new(Mutex::new(vec![]));

  for i in 0..5 {
    let task = task.clone();
    let handle = thread::spawn(move || {
      // Tasks 1
      task
        .lock()
        .unwrap()
        .push(format!("Thread {i}, Completed its part on Task 1"));

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
  for contents in task_lock {
    println!("{contents}");
  }
}
