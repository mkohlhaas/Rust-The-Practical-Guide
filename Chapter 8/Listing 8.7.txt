 impl Point<i32, i32> {
    ...
    fn new(x: i32, y: i32) -> Point<i32, i32> { // Error
	Point {x, y}
    }
}