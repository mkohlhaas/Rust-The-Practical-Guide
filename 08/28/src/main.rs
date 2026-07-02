fn returns_shape() -> impl Shape {
  let sq = Square {
    side: 5.0,
    line_width: 5,
    color: String::from("Red"),
  };
  let rect = Rectangle {
    length: 5.0,
    width: 10.0,
    line_width: 5,
    color: String::from("Red"),
  };
  let x = false;
  if x {
    sq
  } else {
    Rect // Error 
  }
}
