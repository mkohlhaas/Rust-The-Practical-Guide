use std::thread;
fn main() {
    let v = vec![1, 2, 3];
    let x = 5;
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        println!("Here's a variable : {:?}", x);
    });
    println!("The variable x is still alive {}", x);
    println!("The variable v is not alive {}", v);   // something wrong here
    handle.join();
}