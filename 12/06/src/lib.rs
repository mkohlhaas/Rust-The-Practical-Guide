#![allow(dead_code)]

#[derive(Debug, Default)]
pub struct Student {
  id: u8,
  pub age: u8,
  pub name: String,
}

impl Student {
  pub fn new(std_name: String) -> Result<Self, String> {
    if std_name.chars().all(|c| matches!(c, 'a'..='z')) {
      Ok(Self {
        id: 0,
        age: 20,
        name: std_name,
      })
    } else {
      Err("The name is invalid.".to_string())
    }
  }
}
