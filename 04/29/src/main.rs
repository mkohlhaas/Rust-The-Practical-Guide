#![allow(unused)]

fn main() {
  let mut vec1 = vec![1, 2, 3];
  let mut vec2 = vec![4, 5, 6];
  let reference = &vec1;
  reference = &vec2; // ⚠️ Error 
}
