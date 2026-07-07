#![allow(dead_code)]

#[derive(Debug, Default)]
pub struct Student {
  id: u8,
  pub age: u8,
  pub name: String,
}

impl Student {
  pub fn new(std_name: String) -> Self {
    Self {
      id: 0,
      age: 20,
      name: std_name,
    }
  }
}
