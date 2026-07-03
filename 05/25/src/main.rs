#![allow(dead_code, unused_variables)]

struct Student {
  name: String,
  grade: Option<u32>,
}

fn main() {
  let student_db = vec![
    Student {
      name: String::from("Alice"),
      grade: Some(95),
    },
    Student {
      name: String::from("Bob"),
      grade: Some(87),
    },
    Student {
      name: String::from("Charlie"),
      grade: None,
    },
  ];
}
