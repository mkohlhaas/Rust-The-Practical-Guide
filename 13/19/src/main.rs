#![allow(unused_variables)]

// Deref Coercion

fn str_slice_fn(s: &str) {}

fn main() {
  let some_string = String::from("String");

  // String is coerced to str (or rather &String -> &str)
  // https://doc.rust-lang.org/std/string/struct.String.html#impl-Deref-for-String
  str_slice_fn(&some_string);
}
