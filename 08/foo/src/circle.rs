use std::f64::consts::PI;

use crate::{
  drawing::DrawingInfo,
  traits::{Draw, Shape},
};

#[derive(Debug)]
pub struct Circle {
  radius: f64,
  drawing_info: DrawingInfo,
}

impl Circle {
  pub fn new(radius: f64, drawing_info: DrawingInfo) -> Self {
    Self {
      radius,
      drawing_info,
    }
  }
}

impl Draw for Circle {
  fn draw_object(&self) {
    println!("Drawing a Circle!");
  }
}

impl Shape for Circle {
  fn area(&self) -> f64 {
    PI * self.radius * self.radius
  }

  fn perimeter(&self) -> f64 {
    2.0 * PI * self.radius
  }
}
