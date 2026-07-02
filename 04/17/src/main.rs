fn takes_ownership(vec: &Vec<i32>) {
  println!("vec: {:?}", vec);
}

fn main() {
  let vec1 = vec![1, 2, 3];
  let ref1 = &vec1;

  takes_ownership(ref1);
  println!("vec 1 is: {:?}", vec1);
}
