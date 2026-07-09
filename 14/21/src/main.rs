use std::sync::Mutex;

// Sharing States

fn main() {
  let m = Mutex::new(5);

  {
    // .lock() is blocking
    let mut num = m.lock().unwrap();
    println!("{:?}", m);
    *num = 10;
  } // num goes out of scope and will unlock/release the mutex

  println!("{:?}", m);
}
