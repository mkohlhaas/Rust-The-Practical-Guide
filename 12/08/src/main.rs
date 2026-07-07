#![allow(dead_code)]

#[derive(Debug, Default, Clone)]
struct Customer {
  name: String,
  username: String,
  membership: Membershiptype,
  gender: char,
  country: String,
  age: u8,
}

#[derive(Debug, Clone)]
enum Membershiptype {
  New,
  Causual,
  Loyal,
}

impl Default for Membershiptype {
  fn default() -> Self {
    Membershiptype::New
  }
}

fn main() {}
