#![allow(dead_code)]

struct A {
  b: B,
  c: C,
}

struct B {
  f2: u32,
}

struct C {
  f1: u32,
  f3: u32,
}

// we can mutable borrow separately

fn fn1(b: &mut B) -> &u32 {
  &b.f2
}

fn fn2(c: &mut C) -> u32 {
  c.f1 + c.f3
}

fn main() {}
