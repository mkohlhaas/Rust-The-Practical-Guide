fn takes_ownership(vec: Vec<i32>) {
  println!("vec is: {:?}", vec);
}

fn main() {
  let vec1 = vec![1, 2, 3];

  // Vec<…> is not Copy but Clone.
  takes_ownership(vec1.clone());

  println!("vec1: {:?}", vec1);
}
