use std::{sync::mpsc, thread};
fn main() {
    ...
    let received_val = rx.recv().unwrap();
    println!("Received: {received_val}");

    let received_val = rx.recv().unwrap();
    println!("Received: {received_val}");
}