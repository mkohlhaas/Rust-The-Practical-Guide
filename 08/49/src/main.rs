impl Addition for Point { // Error
    type Rhs = i32;
    type Output = Point;
    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}