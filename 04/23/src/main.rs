fn borrows_vec(vec: &Vec<i32>) {
  println!("vec: {:?}", vec);
}

// name and signature updated
fn mutably_borrows_vec(vec: &mut Vec<i32>) {
  vec.push(10);
}

fn main() {
  let mut vec1 = vec![1, 2, 3];
  let ref1 = &vec1;

  borrows_vec(ref1);

  let ref2 = &mut vec1;
  mutably_borrows_vec(ref2);

  println!("vec1: {:?}", vec1);
}
