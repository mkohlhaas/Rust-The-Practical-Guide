struct drawing_info {
  line_width: u8,
  color: String,
}
struct Square {
  side: f32,
  info: drawing_info,
}
struct Rectangle {
  length: f32,
  width: f32,
  info: drawing_info,
}
