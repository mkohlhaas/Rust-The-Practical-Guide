fn takes_ownership(vec: Vec<i32>) {
  println!("vec is: {:?}", vec);
}

fn main() {
  let vec1 = vec![1, 2, 3];
  takes_ownership(vec1);
}
