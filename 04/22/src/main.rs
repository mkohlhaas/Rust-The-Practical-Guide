fn borrows_vec(vec: &Vec<i32>) {
  println!("vec: {:?}", vec);
}

// NOTE: vec1 becomes mutable!
fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
  vec.push(10);
  vec
}

fn main() {
  let vec1 = vec![1, 2, 3];
  let ref1 = &vec1;

  borrows_vec(ref1);

  let vec1 = takes_and_gives_ownership(vec1);

  println!("vec: {:?}", vec1);
}
