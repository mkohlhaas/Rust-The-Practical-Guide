#![allow(dead_code)]

// Rust knows the size of Conveyance:
// size = 8, align = 0x4, no Drop

enum Conveyance {
  Car(i32),
  Train(i32),
  Air(i32),
  Walk,
}

fn main() {}
