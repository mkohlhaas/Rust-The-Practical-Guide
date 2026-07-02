#![allow(unused)]

fn main() {
  let mut vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];
  let mut vec_ptr: &Vec<i32>;

  vec_ptr = &vec1;
  println!("vec ptr is pointing to vec_1");

  vec_ptr = &vec2;
  println!("vec ptr is updated and now pointing to vec_2");
}
