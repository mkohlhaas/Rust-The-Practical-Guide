use foo::{Circle, DrawingInfo, Rectangle, Shape, Square};

fn shape_properties(shape: impl Shape) -> (f64, f64) {
  shape.draw_object();
  (shape.area(), shape.perimeter())
}

fn returns_shape() -> impl Shape {
  Square::new(5.0, DrawingInfo::new(2, (0, 0, 0)))
}

// fn returns_shape1() -> impl Shape { // ⚠️ error
fn returns_shape1(b: bool) -> Box<dyn Shape> {
  let sq = Square::new(5.0, DrawingInfo::new(2, (0, 0, 0)));
  let rect = Rectangle::new(5.0, 10.0, DrawingInfo::new(2, (0, 0, 0)));

  // if b { sq } else { rect } // ⚠️ error
  if b { Box::new(sq) } else { Box::new(rect) }
}

fn main() {
  let sq = Square::new(4.2, DrawingInfo::new(2, (100, 100, 100)));
  let rect = Rectangle::new(4.2, 5.3, DrawingInfo::new(3, (50, 60, 0)));
  let circle = Circle::new(2.5, DrawingInfo::new(3, (50, 60, 0)));

  println!("{:?}", sq);
  println!("{:?}", rect);
  println!("{:?}", circle);

  println!();

  println!("Areas:");
  println!("Rectangle: {}", rect.area());
  println!("Square: {}", sq.area());
  println!("Circle: {}", circle.area());

  println!();

  println!("Perimeters:");
  println!("Rectangle: {}", rect.perimeter());
  println!("Square: {}", sq.perimeter());
  println!("Circle: {}", circle.perimeter());

  println!();

  let (area, perimeter) = shape_properties(sq);
  println!("Square: {area} {perimeter}");
  let (area, perimeter) = shape_properties(rect);
  println!("Rectangle: {area} {perimeter}");
  let (area, perimeter) = shape_properties(circle);
  println!("Circle: {area} {perimeter}");

  println!();

  let shape1 = returns_shape();
  println!("{:?}", shape1);
  println!("Shape area: {}", shape1.area());

  println!();

  let shape2 = returns_shape1(true);
  println!("{:?}", shape2);
  println!("Shape area: {}", shape2.area());

  println!();

  println!("Done!");
}
