#![allow(dead_code)]

#[derive(Debug, Default, Clone)]
struct Customer {
  name: String,
  username: String,
  membership: Membershiptype,
  gender: char,
  country: String,
  age: u8,
}

impl Customer {
  fn new(name: String) -> Self {
    Customer {
      name: name,
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone)]
enum Membershiptype {
  New,
  Causual,
  Loyal,
}

impl Default for Membershiptype {
  fn default() -> Self {
    Membershiptype::New
  }
}

fn main() {
  let customer1 = Customer::default();
  let customer2: Customer = Default::default();
  let customer3 = Customer::new("Joseph123".to_string());
  let customer4 = Customer::new("joseph".to_string());

  println!("{:?}", customer1);
  println!("{:?}", customer2);
  println!("{:?}", customer3);
  println!("{:?}", customer4);
}
