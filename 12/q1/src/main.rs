#[derive(Debug)]
struct Employee {
  employee_id: u32,
  pub age: u8,
  pub name: String,
}

impl Employee {
  // Add constructor function here
}

fn main() {
  let emp_1 = Employee::new("Alice".to_string());
}
