#[derive(Debug)]
struct Employee {
  employee_id: u32,
  pub age: u8,
  pub name: String,
}
impl Employee {
  pub fn new(emp_name: String) -> Self {
    Self {
      employee_id: 0,
      age: 30,
      name: emp_name,
    }
  }
}
fn main() {
  let emp_1 = Employee::new("Alice".to_string());
}
