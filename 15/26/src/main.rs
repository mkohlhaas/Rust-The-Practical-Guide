#![allow(unused_macros)]

macro_rules! string_concat {
  () => {
    String::new()
  };

  ($some_str: expr) => {{
    let mut temp_str = String::new();
    temp_str.push_str($some_str);
    temp_str
  }};
}

fn main() {
  let s = string_concat!("foo");
  println!("{s}");
}
