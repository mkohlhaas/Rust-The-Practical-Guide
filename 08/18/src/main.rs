impl Shape for Rectangle {
  fn area(&self) -> f32 {
    let area_of_rect = self.length * self.width;
    println!("Rectangle area: {}", area_of_rect);
    area_of_rect
  }
}
