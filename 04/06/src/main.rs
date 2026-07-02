fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
  vec.push(10);
  vec
}

fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = takes_and_gives_ownership(vec1);

  println!("vec2: {:?}", vec2);
}
