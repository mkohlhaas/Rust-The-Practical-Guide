#![allow(dead_code)]

// Methods

// Unsized Coercion with Traits

trait SomeTrait {
  fn method(&self);
}

// implementation for unsized raw array slices
impl<T> SomeTrait for [T] {
  fn method(&self) {}
}

fn main() {
  let slice: &[i32] = &[1];
  let vec: Vec<i32> = vec![1];
  let array: [i32; 3] = [1, 2, 3];

  slice.method();
  vec.method(); // deref coercion (type changes)
  array.method(); // unsized coercion (type doesn't change)
}
