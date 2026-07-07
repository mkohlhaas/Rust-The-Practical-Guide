#![allow(dead_code)]

// Decomposing the A struct:
// fn1 uses f2
// fn2 uses f1, f3

struct A {
  b: B,
  c: C,
}

// will only used by fn1
struct B {
  f2: u32,
}

// will only used by fn2
struct C {
  f1: u32,
  f3: u32,
}

fn main() {}
