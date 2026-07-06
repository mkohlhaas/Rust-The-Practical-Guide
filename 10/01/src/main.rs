#![allow(unused_variables)]

// The lifetime of a value begins when it is created and ends when the value is dropped or moved
// from its memory location, often due to a change in ownership.

fn main() {
  let i = 5; // i lifetime starts
  let j = i; // j lifetime starts

  println!("{i}");
} // i,j lifetimes end
