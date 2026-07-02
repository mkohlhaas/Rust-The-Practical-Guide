fn main() {
  let mut x = 45;
  let z = &mut x;
  let y = &*z;
}
