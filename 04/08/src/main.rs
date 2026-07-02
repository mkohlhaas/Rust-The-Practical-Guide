fn main() {
  let mut vec1 = vec![4, 5, 6];
  let ref1 = &mut vec1;
  let ref2 = &mut vec1; // ⚠️ Error

  println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
}
