trait SomeTrait {}

fn main() {
  // trait object are unsized

  // println!(
  //   "Size of trait object: {}",
  //   size_of::<dyn SomeTrait>() // ⚠️ Error
  // );

  // Trait objects are fat pointers.
  // A pointer to some data, and a pointer to a vtable containing methods for operating on that data.

  println!(
    "Size of reference to a trait object: {}", // 16
    size_of::<&dyn SomeTrait>()
  );
}
