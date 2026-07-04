#[derive(Debug)]
pub struct DrawingInfo {
  line_width: u8,
  color: (u8, u8, u8),
}

impl DrawingInfo {
  pub fn new(line_width: u8, color: (u8, u8, u8)) -> Self {
    Self { line_width, color }
  }
}
