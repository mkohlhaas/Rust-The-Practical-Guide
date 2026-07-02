use std::thread;
fn main() {
    let mut vec = vec![1, 2, 3];
    thread::spawn(|| {
        println!("{:?}", vec);
    });
}