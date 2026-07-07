#![allow(unused_imports)]

use foo::Student;

fn main() {
  let student1 = Student::default();
  let student2: Student = Default::default();
  let student3 = Student::new("Joseph123".to_string()).unwrap_or_default();
  let student4 = Student::new("joseph".to_string()).unwrap_or_default();

  println!("{:?}", student1);
  println!("{:?}", student2);
  println!("{:?}", student3);
  println!("{:?}", student4);
}
