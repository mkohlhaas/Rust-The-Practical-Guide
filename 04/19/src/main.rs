fn borrows_vec(vec: &Vec<i32>) {
  println!("vec: {:?}", vec);
}

fn main() {
  let mut vec1 = vec![1, 2, 3]; // vec_1 has to be mutable, in order to create a mutable reference to it
  let ref1 = &vec1;
  let ref2 = &mut vec1; // ⚠️ Error
  borrows_vec(ref1);

  println!("vec1: {:?}", vec1);
}
