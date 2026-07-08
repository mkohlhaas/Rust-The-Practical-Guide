#![allow(dead_code, unused_variables)]

// Rust enforces two requirements for an unsized struct:
// - the struct must have exactly one unsized field
// - the unsized field must be the last field of the struct

struct UnSizedStruct {
  sized_field: i32,
  unsized_field: [i32], // raw array slice is unsized
}

// ⚠️ Error: unsized field not the last one
// struct UnSizedStructBad {
//   sized_field1:   i32,
//   unsized_field: [i32], // ⚠️ Error
//   sized_field2:   i32,
// }

// ⚠️ Error: more than one unsized field
// struct UnSizedStructBad {
//   sized_field1:    i32,
//   unsized_field:  [i32],// ⚠️ Error
//   unsized_field2: [i32],
// }

struct UnSizedStructRelaxed<T: ?Sized> {
  sized_field: i32,
  unsized_field: T,
}

fn main() {
  // println!("Size of UnSizedStruct: {}", size_of::<UnSizedStruct>()); // ⚠️ Error

  // let u = UnSizedStruct { // ⚠️ Error
  //   sized_field:    3,
  //   unsized_field: [3],
  // };

  // The generic T can be any type that is optionally sized.
  // The reason this approach works is because the ?Sized trait made the field unsized_field optionally sized,
  // thereby relaxing Rust’s size constraint.
  let u = UnSizedStructRelaxed {
    sized_field: 3,
    unsized_field: [3],
  };
}
