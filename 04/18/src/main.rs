fn borrows_vec(vec: &Vec<i32>) {
  println!("vec: {:?}", vec);
}

fn main() {
  let vec1 = vec![1, 2, 3];
  let ref1 = &vec1;

  borrows_vec(ref1);
  println!("vec1: {:?}", vec1);
  println!("ref1: {:?}", ref1);
}
