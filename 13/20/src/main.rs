#![allow(unused_variables)]

// Functions

// Unsized Coercion

// accepts any input that can be coerced into an array slice
fn array_slice_fn<T>(s: &[T]) {}

fn main() {
  let slice: &[i32] = &[1];
  let vec: Vec<i32> = vec![1];
  let array: [i32; 3] = [1, 2, 3];

  array_slice_fn(slice);
  array_slice_fn(&vec); // deref coercion https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-Deref-for-Vec%3CT,+A%3E
  array_slice_fn(&array); // unsized coercion (from reference of sized array to unsized array slice)
}
