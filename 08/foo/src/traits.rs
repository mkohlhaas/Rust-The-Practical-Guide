use std::fmt::Debug;

pub trait Draw {
  fn draw_object(&self);
}

pub trait Shape: Draw + Debug {
  fn area(&self) -> f64;
  fn perimeter(&self) -> f64;
}
