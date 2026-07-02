...
fn main() {
    ...
    let thread_1 = thread::spawn(move || {
        println!("Thread 1: I am doing some work");
        println!("Thread 1: I am doing some more work");
        println!("Thread 1: Parked");
        thread::park_timeout(Duration::from_secs(4));
        ...
    });

    let thread_2 = thread::spawn(move || {
    ...
    });
    thread_2.join();
    thread_1.thread().unpark();
    thread_1.join();
}