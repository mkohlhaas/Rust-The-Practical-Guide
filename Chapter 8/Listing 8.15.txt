impl Square {
    fn calculate_area(&self) {
        println!("The area is: {}", self.side * self.side);
    }
}
impl Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
} 