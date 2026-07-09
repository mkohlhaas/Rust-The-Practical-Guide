use std::thread::{self, JoinHandle};

fn main() {
  println!("This will be printed");
  println!("This will also be printed");
  println!("The concurrency will start after this line");

  println!("----------------------------------------------");
  let t: JoinHandle<()> = thread::spawn(|| {
    println!("Hello 1 from the thread");
    println!("Hello 2 from the thread");
    println!("Hello 3 from the thread");
    println!("Hello 4 from the thread");
    println!("Hello 5 from the thread");
    println!("Hello 6 from the thread");
    println!("Hello 7 from the thread");
  });
  t.join().unwrap();
  println!("----------------------------------------------");

  println!("Hello 1 from the main");
  println!("Hello 2 from the main");
}
