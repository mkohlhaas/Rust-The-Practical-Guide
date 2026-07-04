#[derive(Debug)]
pub struct Square {
  side: f64,
  drawing_info: crate::drawing::DrawingInfo,
}

impl Square {
  pub fn new(side: f64, drawing_info: crate::drawing::DrawingInfo) -> Self {
    Self { side, drawing_info }
  }

  // fn calculate_area(&self) {
  //   println!("The area is: {}", self.side * self.side);
  // }
}

impl crate::traits::Draw for Square {
  fn draw_object(&self) {
    println!("Drawing a Square!");
  }
}

impl crate::traits::Shape for Square {
  fn area(&self) -> f64 {
    self.side * self.side
  }

  fn perimeter(&self) -> f64 {
    self.side * 4.0
  }
}
