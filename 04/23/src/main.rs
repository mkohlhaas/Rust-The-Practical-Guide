fn main() {
  let mut vec_1 = vec![1, 2, 3];
  let ref1 = &vec_1;
  borrows_vec(ref1);
  let ref2 = &mut vec_1;
  mutably_borrows_vec(ref2);
  println!("vec 1 is: {:?}", vec_1);
}
fn borrows_vec(vec: &Vec<i32>) {
  println!("vec is: {:?}", vec);
}
fn mutably_borrows_vec(vec: &mut Vec<i32>) {
  // name and signature updated
  vec.push(10);
}
