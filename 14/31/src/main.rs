...
fn main() {
    ...
    for i in 0..5 {
        ...
            // Task 2
            task.lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 2"));
            barrier.wait();
            // Task 3 // added
            task.lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 3"));
        });
        threads_vec.push(handle);
    }
    ...
}