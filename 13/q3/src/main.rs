struct FlexibleStruct {
  fixed_part: u32,
  dynamic_part: str,
}
fn main() {
  let instance = FlexibleStruct {
    fixed_part: 42,
    dynamic_part: "Hello", // Error
  };
}
