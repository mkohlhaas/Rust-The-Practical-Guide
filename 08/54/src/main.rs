impl Addition<i32, Point> for Point {
  fn add(self, rhs: i32) -> Point {
    Point {
      x: self.x + rhs,
      y: self.y + rhs,
    }
  }
}
