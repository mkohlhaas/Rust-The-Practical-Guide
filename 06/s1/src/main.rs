mod m1 {
  struct A {
    d: m2::D,
  }
  mod m2 {
    pub enum D {
      // Child module items are not visible to the parent module
      B,
      C,
    }
  }
}
fn main() {}
