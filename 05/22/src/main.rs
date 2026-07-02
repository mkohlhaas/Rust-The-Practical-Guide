struct Student {
  name: String,
  grade: u32,
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
  ];
}
