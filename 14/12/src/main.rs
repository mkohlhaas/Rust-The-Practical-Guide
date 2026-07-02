use std::{sync::mpsc, thread};
fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    for i in 0..10 {
        thread::spawn(move || { // Error
            let val = "Hi from thread".to_string();
            println!("Sending Value: {val}");
            tx.send(val).unwrap();
        });
    }
    let received_val = rx.recv().unwrap();
    println!("Received: {received_val}");
}