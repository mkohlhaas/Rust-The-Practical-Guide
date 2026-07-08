#![allow(dead_code)]

trait Shape {}

struct Circle;
struct Rectangle;

impl Shape for Circle {}
impl Shape for Rectangle {}

fn main() {
  // references to structs (thin pointers)
  println!("Size of &Circle:    {}", size_of::<&Circle>()); //    8
  println!("Size of &Rectangle: {}", size_of::<&Rectangle>()); // 8

  // trait object is a fat pointer (pointer + vtable)
  println!("Size of &dyn Shape: {}", size_of::<&dyn Shape>()); // 16
}
