#![allow(unused_imports, unused_variables)]

use foo::Student;

fn main() {
  let student1 = Student {
    id: 11, // ⚠️ Error
    age: 20,
    name: "Joseph".to_string(),
  };
}
