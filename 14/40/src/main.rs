use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Temporarily Blocking a Thread

fn main() {
  let data = Arc::new(Mutex::new(5));
  let data_clone = data.clone();

  let thread_1 = thread::spawn(move || {
    println!("Thread 1: I am doing some work");
    println!("Thread 1: I am doing some more work");
    println!("Thread 1: parked");
    thread::park(); // waiting till thread_2 is finished (park() is blocking)
    println!("Thread 1: printing the updated data");
    println!("Thread 1: data: {:?}", *data.lock().unwrap());
  });

  let thread_2 = thread::spawn(move || {
    println!("Thread 2: I am working on updating the data");
    thread::sleep(Duration::from_secs(1));
    *data_clone.lock().unwrap() = 10;
    println!("Thread 2: data updated completed");
  });

  let _ = thread_2.join();
  thread_1.thread().unpark(); // wake up thread_1 (get thread from join handle)
  let _ = thread_1.join();
}
