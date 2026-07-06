// heap-allocated data

#![allow(unused_variables)]

fn main() {
  let str_1 = String::from("abc"); // str_1 lifetime starts 
  let str_2 = str_1; // str_1 lifetime ends 

  println!("str_1: {str_1}"); // ⚠️ Error
}
