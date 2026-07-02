use std::{sync::Mutex, thread};
fn main() {
    ...
    let lock_m = m.lock().unwrap();
    println!("m is: {:?}", *lock_m);
    drop(lock_m); 
    let lock_m1 = m.lock().unwrap();
    println!("This code is blocked");
}