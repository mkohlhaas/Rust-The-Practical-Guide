fn gives_ownership() -> Vec<i32> {
  vec![4, 5, 6]
}

fn main() {
  let vec1 = gives_ownership();

  println!("vec1: {:?}", vec1);
}
