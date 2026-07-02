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
  r1.area();
  s1.area();
}
