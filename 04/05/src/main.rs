fn main() {
  let vec_2 = gives_ownership();
  println!("vec 2 is: {:?}", vec_2);
}
fn gives_ownership() -> Vec<i32> {
  vec![4, 5, 6]
}
