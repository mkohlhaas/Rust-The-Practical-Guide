use std::{sync::Mutex, thread};
fn main() {
    let file = Mutex::new(File { text: vec![] });
    let mut thread_vec = vec![];
    for i in 0..10 {
        let handle = thread::spawn(move || { // Error
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("Hello from Thread {i}"));
        });
        thread_vec.push(handle);
    }
}