fn main() {
  let vec1 = {
    let vec2 = vec![1, 2, 3];
    &vec2
  };

  println!("Vec1: {:?}", vec1)
}
