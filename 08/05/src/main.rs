Impl<T, U> Point<T, U> {
	fn new(x: y, y: U) -> Point<T, U> {
		Point {x, y}
	}
} 
fn main() {
    	let origin = Point::new(0, 0);
    	let p1 = Point::new(1.0, 4.0);
    	let p2 = Point::new(5, 5.0);
}