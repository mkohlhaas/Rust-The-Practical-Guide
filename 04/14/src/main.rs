#![allow(unused_variables)]

fn main() {
  // Vec<…> is not Copy, only Clone.
  let mut vec1 = vec![4, 5, 6];
  let ref1 = &mut vec1;
  let ref2 = ref1;
  let ref3 = ref1; // ⚠️ Error
}
