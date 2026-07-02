fn main() {
  let r1 = Rectangle {
    width: 5.0,
    length: 4.0,
    line_width: 1,
    color: String::from("Red"),
  };
  let s1 = Square {
    side: 3.2,
    line_width: 1,
    color: String::from("Red"),
  };
  let c1 = Circle { radius: 5.0 };
  shape_properties(r1);
  shape_properties(s1);
  shape_properties(c1); // Error
}
