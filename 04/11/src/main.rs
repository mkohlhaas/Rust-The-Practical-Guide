fn main() {
  let mut vec1 = vec![4, 5, 6];
  let ref1 = &vec1;
  let ref2 = &vec1;
  let ref3 = &mut vec1; // ⚠️ Error

  println!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref1, ref2, ref3);
}
