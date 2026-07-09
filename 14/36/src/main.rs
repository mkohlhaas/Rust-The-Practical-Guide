use std::thread;

fn main() {
  let mut vec = vec![1, 2, 3];

  // The thread scopes simplifies the borrowing, and you only need to pay attention to the borrowing
  // rules inside the scope.

  thread::scope(|s| {
    // immutable borrow
    s.spawn(|| {
      println!("Thread inside scope.");
      println!("vec: {:?}", vec);
    });

    // immutable borrow
    s.spawn(|| {
      println!("Another Thread inside scope.");
      println!("vec: {:?}", vec);
    });
  });

  println!("The scope finished.");
  vec.push(5);
  println!("vec: {:?}", vec);

  println!("Done!");
}
