#![allow(unused_variables)]

// Lifetime Elision

// fn return_str<'a>(s_1: &'a str) -> &'a str {
fn return_str(s_1: &str) -> &str {
  s_1
}

fn main() {
  let str_1 = "some str";
  let received_str = return_str(&str_1);
}
