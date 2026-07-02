#![allow(unused_variables, unused_assignments)]

fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];
  let mut reference = &mut &vec1;
  reference = &mut &vec2;
}
