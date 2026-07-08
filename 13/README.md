- Thin pointer: reference consisting of one word  (pointer)
- Fat  pointer: reference consisting of two words (pointer and length)
  - [Rust slice data types - Diagram](https://saidvandeklundert.net/img/rust_slice.png)
  - [Rust slice data types](https://saidvandeklundert.net/2021-08-14-rust-slice/)

- The Sized trait is that it serves as both an auto trait and a marker trait.
- All auto traits are marker traits.

- The Send trait indicates whether a type can be safely transferred across threads.
- The Sync trait signifies whether references of a type can be safely shared between threads. 

- The `?Sized` syntax (also referred to as `optionally sized traits`) allows for flexibility when working with generics by accommodating both sized and unsized types.
- The `?Sized` trait bound is also referred to as a `widening, expanded, or relaxed bound` because it loosens the limitations on the type parameter rather than restricting it.

- Unsized coercion allows a sized type to be transformed into an unsized type.
- In deref coercion, the type changes, while in unsized coercion, the type does not change.
- Instead, the property of the type changes from sized to unsized, which impacts the way the compiler handles the type.
- Unsized coercion works with functions and methods. 
  - 20/src/main.rs
  - 21/src/main.rs

| Never Type                                                  | Unit Type `()`                                               |
| ----------------------------------------------------------- | ------------------------------------------------------------ |
| Never produces a value                                      | No meaningful value                                          |
| Function returning `never` will never return normally       | Function returning unit always returns normally              |
| No associated value and can be coerced into all other types | Single value `()` which cannot be coerced to any other type  |

- A `Unit struct` is a struct with no fields. 
