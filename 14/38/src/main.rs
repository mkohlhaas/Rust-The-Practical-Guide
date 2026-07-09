use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Motivating Example for Thread Parking

// In this scenario, thread_1 should only read the shared data after that data has been updated by thread_2.

fn main() {
  let data = Arc::new(Mutex::new(5));
  let data_clone = data.clone();

  // `data` is moved
  let thread_1 = thread::spawn(move || {
    println!("Thread 1: I am doing some work");
    println!("Thread 1: I am doing some more work");
    println!("Thread 1: printing the data");
    println!("Thread 1: data: {:?}", *data.lock().unwrap()); // should be 10
  });

  // `data_clone` is moved
  let thread_2 = thread::spawn(move || {
    println!("Thread 2: I am working on updating the data");
    thread::sleep(Duration::from_millis(200));
    *data_clone.lock().unwrap() = 10;
    println!("Thread 2: data updated completed");
    println!("Thread 2: data: {:?}", *data_clone.lock().unwrap()); // should be 10
  });

  let _ = thread_2.join();
  let _ = thread_1.join();
}
