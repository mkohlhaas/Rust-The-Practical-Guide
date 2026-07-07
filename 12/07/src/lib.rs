#![allow(dead_code)]

#[derive(Debug)]
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

impl Default for Student {
  fn default() -> Self {
    Self {
      id: 0,
      age: 20,
      name: "unknown".to_string(),
    }
  }
}
