#![allow(dead_code, unused_variables)]

struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn new(x: T, y: U) -> Self {
    Self { x, y }
  }
}

fn main() {
  let origin = Point::new(0, 0);
  let p1 = Point::new(1.0, 4.0);
  let p2 = Point::new(5, 5.0);
}
