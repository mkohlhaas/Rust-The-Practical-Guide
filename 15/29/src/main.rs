#![allow(unused_mut)]

macro_rules! string_concat {
    ($($some_str:expr),*) => {{
            let mut temp_str = String::new();
            $(temp_str.push_str($some_str);)*
            temp_str
    }};
}

fn main() {
  let str_null = string_concat!();
  let str_single = string_concat!("First");
  let str_double = string_concat!("First", "Second");

  println!("{str_null}");
  println!("{str_single}");
  println!("{str_double}");
}
