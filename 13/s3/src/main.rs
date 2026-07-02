struct FlexibleStruct<T: ?Sized> {
  fixed_part: u32,
  dynamic_part: T,
}
fn main() {
  let instance = FlexibleStruct {
    fixed_part: 42,
    dynamic_part: "Hello",
  };
}
