mod m1 {
  struct A {
    d: m2::D,
  }
  pub mod m2 {
    // public items of a private child module, are only accessible by parent module.
    // we need to make the child module m2 pub, so that its public items can be used outside the parent module.
    pub enum D {
      B,
      C,
    }
  }
}
mod m3 {
  struct C {
    e: crate::m1::m2::D,
  }
}
fn main() {}
