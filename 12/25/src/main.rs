#![allow(dead_code, unused_variables)]

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

fn fn1(b: &mut B) -> &u32 {
  &b.f2
}

fn fn2(c: &mut C) -> u32 {
  c.f1 + c.f3
}

fn fn3(a: &mut A) {
  let x = fn1(&a.b); // ⚠️ Error: immutable borrow, but we need to borrow mutably
  let y = fn2(&a.c); // ⚠️ Error: immutable borrow, but we need to borrow mutably

  println!("{x}");
}

fn main() {}
