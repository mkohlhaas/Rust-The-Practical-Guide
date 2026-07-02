use std::{sync::mpsc, thread, time::Duration};
fn main() {
    ...
    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value is {:?}", received_value);
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}