fn main() {
  let slice: &[i32] = &[1];
  let vec = vec![1];
  let array = [1, 2, 3];

  slice.method();
  vec.method(); // deref coercion
  array.method(); // Unsized coercion
}
