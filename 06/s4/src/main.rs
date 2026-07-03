// Fixing access to private fields in a struct

#![allow(dead_code)]

mod university {
  pub struct Student {
    pub name: String, // fields need to be made public
    pub marks: u8,
    pub grade: char,
  }
}

use university::Student;

fn main() {
  let student1 = Student {
    name: String::from("Alice"),
    marks: 75,
    grade: 'A',
  };

  println!("{} got {} grade", student1.name, student1.grade);
}
