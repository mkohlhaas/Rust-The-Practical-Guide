#![allow(dead_code, unused_variables)]

struct A {
  f1: u32,
  f2: u32,
  f3: u32,
}

// NOTE: immutable borrows in fn1 and fn2 would work

// NOTE: returning a u32 value would also work
fn fn1(a: &mut A) -> &u32 {
  &a.f2
}

fn fn2(a: &mut A) -> u32 {
  a.f1 + a.f3
}

// NOTE: Rust doesn’t allow the borrowing of individual fields independently of each other!
fn fn3(a: &mut A) {
  let x = fn1(a); // 1st mutable borrow
  let y = fn2(a); // 2nd mutable borrow

  println!("{x}"); // ⚠️ Error
}

fn main() {}
