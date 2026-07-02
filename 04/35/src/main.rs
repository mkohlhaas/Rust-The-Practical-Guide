#![allow(unused_variables)]

fn main() {
  let mut x: i32 = 45;
  let z: &mut i32 = &mut x;
  let y: &i32 = &*z;
}
