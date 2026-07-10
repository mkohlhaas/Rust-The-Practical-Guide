#![allow(unused_macros)]

macro_rules! string_concat {
  () => {
    String::new()
  };
}

fn main() {
  let s = string_concat!();
  println!("{s}");
}
