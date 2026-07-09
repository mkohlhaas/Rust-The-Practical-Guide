#![allow(unused_variables)]

// Thread Communication

use std::sync::mpsc;

fn main() {
  let (tx, rx) = mpsc::channel::<String>();
}
