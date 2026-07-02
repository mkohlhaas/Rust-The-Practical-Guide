use std::thread;
fn main() {
  let x = 5;
  thread::spawn(move || {
    println!("{x}");
  });
  println!("{x}");
}
