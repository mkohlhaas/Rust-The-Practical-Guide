#![allow(dead_code)]

struct Student {
  name: String,
  grade: Option<u32>,
}

fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
  for student in student_db {
    if student.name == *student_name {
      return Ok(());
    }
  }
  Err(String::from("Student not found"))
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

  check_student(&String::from("Alice"), &student_db).unwrap();
}
