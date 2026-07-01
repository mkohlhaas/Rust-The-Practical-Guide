use std::{sync::mpsc, thread};
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = "some_val".to_string();
        println!("Sending Value: {val}");
        tx.send(val).unwrap();
    });
    let received_val = rx.recv().unwrap();
}