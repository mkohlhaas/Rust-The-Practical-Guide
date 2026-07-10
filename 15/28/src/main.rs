// Repeating Patterns

macro_rules! string_concat {
    ($($some_str:expr),*) => {{
            let mut temp_str = String::new();
            $(temp_str.push_str($some_str);)*
            temp_str
    }};
}

fn main() {
  let s = string_concat!("foo", " you", " and me");
  println!("{s}");
}
