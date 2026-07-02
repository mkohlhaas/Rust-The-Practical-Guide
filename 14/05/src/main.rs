use std::thread;
fn main() {
    let x = "some string".to_string();
    thread::spawn(|| {   // Error
    println!("{x}");
    });
}