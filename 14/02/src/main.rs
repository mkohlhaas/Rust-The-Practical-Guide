use std::thread;
use std::time::Duration;

fn main() {
    println!("This will be printed");
    ...
    thread::spawn(|| {
        println!("Hello 1 from the thread");
        ...
    }); 
    thread::sleep(Duration::from_millis(1));
    println!("Hello 1 from the main");
    println!("Hello 2 from the main");
}