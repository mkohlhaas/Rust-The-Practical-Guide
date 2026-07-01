struct Square {
    side: f32,
    line_width: u8,
    color: String,
}
struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}
trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        ...
    }
    fn perimeter(&self) -> f32 {
        ...
    }
}
impl Shape for Square {
    fn area(&self) -> f32 {
        ...
    }
}