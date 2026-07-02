fn main() {
  let mut vec1 = vec![4, 5, 6];
  let ref1 = &vec1;
  let ref2 = &vec1;

  println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

  let ref3 = &mut vec1;
  println!("ref3: {:?}", ref3);
}
