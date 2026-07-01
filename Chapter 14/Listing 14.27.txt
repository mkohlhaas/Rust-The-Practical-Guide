use std::{sync::{Arc, Mutex}, thread};
fn main() {
    let file = Arc::new(Mutex::new(File { text: vec![] })); // updated
    let mut thread_vec = vec![];
    for i in 0..10 {
        let file = Arc::clone(&file); // updated
        ...
    }
}