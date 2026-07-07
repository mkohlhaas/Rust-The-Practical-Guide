#![allow(unused_imports)]

use foo::Student;

fn main() {
  let student1 = Student::new("Joseph".to_string());
  let student2 = Student::new("joseph".to_string());

  println!("{:?}", student1);
  println!("{:?}", student2);
}
