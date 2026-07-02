use std::mem::size_of;
fn main() {
  println!("[i32] size is: {}", size_of::<&[i32]>());
}
