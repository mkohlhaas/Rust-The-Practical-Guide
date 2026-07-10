#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;

#[macro_use]
extern crate std;

fn main() {
  let mut x = 4;
  x = x + 1;
}
