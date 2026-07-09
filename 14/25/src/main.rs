use std::{sync::Mutex, thread};

struct File {
  text: Vec<String>,
}

fn main() {
  // Mutex is Send and Sync
  // `file` is not declared mutable.
  // Mutexes uses interior mutability.
  let file = Mutex::new(File { text: vec![] });

  let mut thread_vec = vec![];
  for i in 1..=10 {
    // ⚠️ Error: we can move Mutex only once
    let handle = thread::spawn(move || {
      let mut file_lock = file.lock().unwrap();
      file_lock.text.push(format!("Hello from Thread {i}"));
    });

    thread_vec.push(handle);
  }
}
