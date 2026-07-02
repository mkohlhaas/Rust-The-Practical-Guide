use std::{sync::mpsc, thread};
fn main() {
    let (tx, rx) = mpsc::channel(); 
    for i in 0..10 {
        ...
    } 
	drop(tx);
    ...
}