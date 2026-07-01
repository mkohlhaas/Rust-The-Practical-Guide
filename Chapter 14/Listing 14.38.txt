use std::{sync::{Arc, Mutex}, thread, time::Duration};
fn main() {
    let data = Arc::new(Mutex::new(5));
    let data_clone = data.clone();
    let thread_1 = thread::spawn(move || {
        println!("Thread 1: I am doing some work");
        println!("Thread 1: I am doing some more work");
        println!("Thread 1: Printing the updated data");
        println!("Thread 1: Data: {:?}", *data.lock().unwrap());
    });

    let thread_2 = thread::spawn(move || {
        println!("Thread 2: I am working on updating the data");
        thread::sleep(Duration::from_secs(1));
        *data_clone.lock().unwrap() = 10;
        println!("Thread 2: Data updated completed");
    });
    thread_2.join();
    thread_1.join();
}