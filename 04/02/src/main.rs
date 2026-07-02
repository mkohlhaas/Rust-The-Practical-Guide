#![allow(unused)]

fn main() {
  let s1 = String::from("world");

  {
    let s2 = s1.clone();
  }

  println!("s2 is: {s2}"); // ⚠️ Error
}
