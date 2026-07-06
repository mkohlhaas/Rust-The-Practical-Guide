#![allow(unused_variables)]

// 1.
fn return_str<'a>(s_1: &'a str, s_2: &str) -> &'a str {
  s_1
}

// 2.
// ⚠️ Error
// fn return_str<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &str {
//   s_1
// }

// 3.
// fn return_str<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &'a str {
//   s_1
// }

fn main() {
  let str_1 = "some str";
  let str_2 = "other str";
  let received_str = return_str(&str_1, &str_2);
}
