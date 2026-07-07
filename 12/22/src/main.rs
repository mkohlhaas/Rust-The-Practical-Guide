#![allow(dead_code, unused_variables)]

struct A {
  f1: u32,
  f2: u32,
  f3: u32,
}

// only immutable borrows
fn fn1(a: &A) -> &u32 {
  &a.f2
}

fn fn2(a: &A) -> u32 {
  a.f1 + a.f3
}

fn fn3(a: &mut A) {
  let x = fn1(a);
  let y = fn2(a);
  println!("{x}");
}

fn main() {}
