fn main() {
  let vec_1 = vec![1, 2, 3];
  let vec_2 = takes_and_gives_ownership(vec_1);
  println!("vec 2 is: {:?}", vec_2);
}
fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
  vec.push(10);
  vec
}
