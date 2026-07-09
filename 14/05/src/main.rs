use std::thread;

// Ownership in Threads

fn main() {
  let x = "some string".to_string();

  // ⚠️ Error: x is borrowed (immutably); closure may outlive x
  thread::spawn(|| {
    println!("{x}");
  });
}
