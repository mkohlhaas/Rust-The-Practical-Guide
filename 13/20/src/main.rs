fn main() {
  let slice: &[i32] = &[1];
  let vec = vec![1];
  let array = [1, 2, 3];
  array_slice_fn(slice);
  array_slice_fn(&vec); // deref coercion
  array_slice_fn(&array); // Unsized coercion
}
