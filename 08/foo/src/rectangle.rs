#[derive(Debug)]
pub struct Rectangle {
  length: f64,
  width: f64,
  drawing_info: crate::drawing::DrawingInfo,
}

impl Rectangle {
  pub fn new(length: f64, width: f64, drawing_info: crate::drawing::DrawingInfo) -> Self {
    Self {
      length,
      width,
      drawing_info,
    }
  }

  // fn area(&self) -> f64 {
  //   print!("From implementation.");
  //   self.length * self.width
  // }
}

impl crate::traits::Draw for Rectangle {
  fn draw_object(&self) {
    println!("Drawing a Rectangle!");
  }
}

impl crate::traits::Shape for Rectangle {
  fn area(&self) -> f64 {
    self.width * self.length
  }

  fn perimeter(&self) -> f64 {
    2.0 * (self.width + self.length)
  }
}
