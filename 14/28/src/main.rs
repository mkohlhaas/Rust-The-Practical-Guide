use std::{sync::{Arc, Mutex}, thread};
fn main() {
    let file = Arc::new(Mutex::new(File { text: vec![] }));
    let mut thread_vec = vec![];
    for i in 0..10 {
	    ...	
    }
    for handle in thread_vec {
        handle.join().unwrap();
    }
    let file_lock = file.lock().unwrap();
    for t in &file_lock.text {
        println!("{t}");
    }
}