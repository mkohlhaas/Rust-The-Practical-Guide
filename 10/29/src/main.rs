fn main() {
  let mut x = 50;
  let ref1 = &x;
  let ref2 = &x;
  let ref3 = &mut x;
  println!("{} {} ", ref1, ref2); // Error
}
