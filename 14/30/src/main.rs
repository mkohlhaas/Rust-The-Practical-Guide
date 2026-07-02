use std::{sync::{Arc, Barrier, Mutex}, thread};
fn main() {
    let mut threads_vec = Vec::new();
    let task = Arc::new(Mutex::new(vec![]));
    let barrier = Arc::new(Barrier::new(5)); // added
    for i in 0..5 {
        let task = task.clone();
        let barrier = barrier.clone(); // added
        let handle = thread::spawn(move || {
            // Tasks 1
            task.lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 1"));
            barrier.wait(); // added
                            // Task 2
            task.lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 2"));
        });
        threads_vec.push(handle);
    }
    ...
}