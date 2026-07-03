#![allow(dead_code)]

trait Shape {}
struct Square {
  side: f64,
  line_width: u8,
  color: String,
}
struct Rectangle {
  length: f64,
  width: f64,
  line_width: u8,
  color: String,
}

impl Shape for Square {}
impl Shape for Rectangle {}

// fn returns_shape() -> impl Shape {
fn returns_shape() -> Box<dyn Shape> {
  let sq = Square {
    side: 5.0,
    line_width: 5,
    color: String::from("Red"),
  };
  let rect = Rectangle {
    length: 5.0,
    width: 10.0,
    line_width: 5,
    color: String::from("Red"),
  };

  let x = false;

  if x { Box::new(sq) } else { Box::new(rect) }
}

fn main() {}
