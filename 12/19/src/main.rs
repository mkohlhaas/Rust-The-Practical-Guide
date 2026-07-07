#![allow(dead_code)]

// Sometimes, a large, poorly designed struct can lead to borrowing issues. While individ-
// ual fields might be borrowable independently, borrowing the struct as a whole can
// block further borrows or block usage of the struct, ultimately preventing other parts of
// the code from accessing it.

struct A {
  f1: u32,
  f2: u32,
  f3: u32,
}

fn main() {}
