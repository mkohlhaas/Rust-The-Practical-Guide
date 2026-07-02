impl Shape for Rectangle {
  fn perimeter(&self) -> f32 {
    let perimeter_of_rect = 2.0 * (self.length + self.width);
    println!("Rectangle Perimeter: {}", perimeter_of_rect);
    perimeter_of_rect
  }
}
