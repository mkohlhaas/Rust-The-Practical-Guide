use std::thread;

fn main() {
  let mut vec = vec![1, 2, 3];

  thread::scope(|s| {
    // immutable borrow of vec
    s.spawn(|| {
      println!("Thread inside scope");
      println!("{:?}", vec);
    });

    // mutable borrow of vec
    // ⚠️ Error: cannot borrow `vec` as mutable because it is also borrowed as immutable
    s.spawn(|| {
      println!("Another Thread inside scope");
      vec.push(4);
      println!("vec: {:?}", vec);
    });
  });

  println!("The scope finished.");
  vec.push(5);
  println!("vec: {:?}", vec);
  println!("Done!");
}
