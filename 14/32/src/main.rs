use std::thread;

// Motivating Example

fn main() {
  let mut vec = vec![1, 2, 3];

  thread::spawn(|| {
    println!("{:?}", vec);
  });
}
