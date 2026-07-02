#![allow(unused)]

fn main() {
  // String is not Copy
  let s1 = String::from("world");
  let s2 = s1;

  println!("s1: {s1}"); // ⚠️ error
}
