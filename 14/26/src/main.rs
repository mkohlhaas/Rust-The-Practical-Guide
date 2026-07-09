use std::{rc::Rc, sync::Mutex, thread};

struct File {
  text: Vec<String>,
}

fn main() {
  let file = Rc::new(Mutex::new(File { text: vec![] }));

  let mut thread_vec = vec![];

  for i in 0..10 {
    let file = Rc::clone(&file);
    // ⚠️ Error: Rc is not thread-safe, is not Send
    let handle = thread::spawn(move || {
      let mut file_lock = file.lock().unwrap();
      file_lock.text.push(format!("Hello from Thread {i}"));
    });

    thread_vec.push(handle);
  }
}
