#![allow(dead_code)]

use std::mem::size_of;

enum Membershiptype {
  New,
  Causual,
  Loyal,
}

fn main() {
  let n: i32 = 42;

  println!("{n}");

  println!();

  println!("Size of i8:  {} byte", size_of::<i8>());
  println!("Size of i16: {} bytes", size_of::<i16>());
  println!("Size of i32: {} bytes", size_of::<i32>());
  println!("Size of i64: {} bytes", size_of::<i64>());

  println!();

  println!("Size of u8:  {} byte", size_of::<u8>());
  println!("Size of u16: {} bytes", size_of::<u16>());
  println!("Size of u32: {} bytes", size_of::<u32>());
  println!("Size of u64: {} bytes", size_of::<u64>());

  println!();

  println!("Size of f32: {} bytes", size_of::<f32>());
  println!("Size of f64: {} bytes", size_of::<f64>());

  println!();

  println!("Size of (i32, i32): {} bytes", size_of::<(i32, i32)>());
  println!("Size of (i64, i64): {} bytes", size_of::<(i64, i64)>());

  println!();

  println!("Size of [i32: 3]: {} bytes", size_of::<[i32; 3]>());
  println!("Size of [i32: 9]: {} bytes", size_of::<[i32; 9]>());

  println!();

  println!(
    "Size of Membershiptype: {} byte",
    size_of::<Membershiptype>()
  );
}
