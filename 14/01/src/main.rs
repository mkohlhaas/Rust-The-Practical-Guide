use std::thread;
fn main() {
  println!("This will be printed");
  println!("This will also be printed");
  println!("The concurrency will start after this line");

  thread::spawn(|| {
    println!("Hello 1 from the thread");
    println!("Hello 2 from the thread");
    println!("Hello 3 from the thread");
    println!("Hello 4 from the thread");
    println!("Hello 5 from the thread");
    println!("Hello 6 from the thread");
    println!("Hello 7 from the thread");
  });
  println!("Hello 1 from the main");
  println!("Hello 2 from the main");
}
