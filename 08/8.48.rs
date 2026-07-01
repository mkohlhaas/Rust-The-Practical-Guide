impl Addition for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Self::Rhs) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}