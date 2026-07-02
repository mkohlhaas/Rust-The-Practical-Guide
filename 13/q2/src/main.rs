fn some_fn<T: ?Sized + std::fmt::Debug>(val: T) {
  println!("{:?}", val)
}
fn main() {}
