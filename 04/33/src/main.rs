fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];
  let reference = &mut &vec1; // reference to a reference
  *reference = &mut &vec2;
}
