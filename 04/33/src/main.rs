fn main() {
  let mut vec_1 = vec![1, 2, 3];
  let mut vec_2 = vec![4, 5, 6];
  let reference = &mut &vec_1;
  *reference = &mut &vec_2;
}
