mod m1 {
  struct A {
    d: m2::D,
  }
  mod m2 {
    pub enum D {
      // Child module items are not visible to parent module by default
      B,
      C,
    }
  }
}
fn main() {}
