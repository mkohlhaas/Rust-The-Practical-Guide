use std::thread;

// Scoped Threads

fn main() {
  let vec = vec![1, 2, 3];

  thread::scope(|s| {
    s.spawn(|| {
      println!("Thread inside scope");
      println!("{:?}", vec);
    });
  });
}
