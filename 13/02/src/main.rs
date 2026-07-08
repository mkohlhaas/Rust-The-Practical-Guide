#![allow(dead_code)]

use std::rc::Rc;

struct Point {
  x: bool, // size 1
  y: i64,  // size 8
}

fn main() {
  // Struct
  println!("Size of Point:                 {}", size_of::<Point>()); // padding for alignment

  println!();

  // References
  println!("Size of i32 reference:         {}", size_of::<&i32>());
  println!("Size of i32 mutable reference: {}", size_of::<&mut i32>());

  println!();

  // Machine word size
  // using a reference to unit type
  println!("Size of machine word:          {}", size_of::<&()>());

  println!();

  // Smart Pointers (not fat pointers: sizes are 8 bytes)
  println!("Size of Box<i32>:              {}", size_of::<Box<i32>>());
  println!(
    "Size of fn(i32) -> i32:        {}",
    size_of::<fn(i32) -> i32>()
  );
  println!("Size of Rc<i32>:               {}", size_of::<Rc<i32>>());

  println!();

  // Unsized Types //

  // raw array slice
  // println!("[i32] size is: {}", size_of::<[i32]>()); // ⚠️ Error
  // let a: [i32]; // ⚠️ Error

  // array
  let _a: [i32; 3];

  // array slice
  // consists of a pointer (8 bytes) and a length (8 bytes)
  // https://saidvandeklundert.net/img/rust_slice.png
  // https://saidvandeklundert.net/2021-08-14-rust-slice/
  println!("Size of &[i32]:                {}", size_of::<&[i32]>()); // 16

  // raw string slice
  // println!("str size is: {}", size_of::<str>()); // ⚠️ Error

  // string slice
  println!("Size of &str:                  {}", size_of::<&str>());
}
