#![allow(dead_code, unused_variables)]

struct A {
  f1: u32,
  f2: u32,
  f3: u32,
}

// functions with mutable borrows

fn fn1(a: &mut A) -> &u32 {
  &a.f2
}

fn fn2(a: &mut A) -> u32 {
  a.f1 + a.f3
}

fn fn3(a: &mut A) {
  let x: &u32 = fn1(a);
  let y: u32 = fn2(a);
}

fn main() {}
