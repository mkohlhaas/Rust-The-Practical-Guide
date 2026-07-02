fn main() {
  println!(
    "Size of a reference to sized type: {}",
    size_of::<&[i32; 3]>()
  );
  println!(
    "Size of a reference to unsized type: {}",
    size_of::<&[i32]>()
  );
}
