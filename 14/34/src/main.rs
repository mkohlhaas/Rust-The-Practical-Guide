use std::thread;

fn main() {
  let mut vec = vec![1, 2, 3];

  thread::scope(|s| {
    s.spawn(|| {
      println!("Thread inside scope");
      println!("{:?}", vec);
    });
  });

  // At the end of the scope, the thread is guaranteed to have finished its execution.
  // Therefore, no references to vec exist, and we can therefore use it.

  println!("The scope finished.");

  vec.push(4);
  println!("vec: {:?}", vec);

  println!("Done!");
}
