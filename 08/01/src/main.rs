#![allow(dead_code, unused_variables)]

struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
}

fn main() {
  let origin1 = Point { x: 0, y: 0 };
  let origin2 = Point::new(0, 0);
}
