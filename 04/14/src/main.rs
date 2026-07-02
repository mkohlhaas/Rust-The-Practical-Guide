fn main() {
  let mut vec_1 = vec![4, 5, 6];
  let ref1 = &mut vec_1;
  let ref2 = ref1;
  let ref3 = ref1; // Error
}
