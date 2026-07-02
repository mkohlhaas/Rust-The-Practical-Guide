fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
  for student in student_db {
    if student.name == *student_name {
      return Ok(());
    }
  }
  Err(String::from("Student not found"))
}
