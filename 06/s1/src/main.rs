// Fixing visibility issues in nested modules

#![allow(dead_code)]

mod m1 {
  struct A {
    d: m2::D,
  }

  mod m2 {
    // Child module items are not visible to the parent module
    pub enum D {
      B,
      C,
    }
  }
}

fn main() {}
