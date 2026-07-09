use std::sync::{Arc, Mutex};
use std::thread;

struct File {
  text: Vec<String>,
}

fn main() {
  // Arc is thread-safe
  let file = Arc::new(Mutex::new(File { text: vec![] }));

  let mut thread_vec = vec![];

  for i in 1..=10 {
    let file = Arc::clone(&file);

    let handle = thread::spawn(move || {
      let mut file_lock = file.lock().unwrap();
      file_lock.text.push(format!("Hello from Thread {i}"));
    });

    thread_vec.push(handle);
  }

  println!("{:#?}", thread_vec);
  println!("Done!");
}
