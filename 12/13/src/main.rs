#![allow(dead_code, unused_variables)]

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
  fn new1(name: String) -> Self {
    Customer {
      name: name,
      ..Default::default()
    }
  }

  fn new2(name: String, username: String) -> Self {
    Customer {
      name: name,
      username: username,
      ..Default::default()
    }
  }

  fn new3(name: String, username: String, membership: Membershiptype) -> Self {
    Customer {
      name: name,
      username: username,
      membership: membership,
      ..Default::default()
    }
  }
}

// NOTE: start of builder pattern
// using optional values
#[derive(Default)]
struct CustomerBuilder {
  name: String,
  username: Option<String>,
  membership: Option<Membershiptype>,
  gender: Option<char>,
  country: Option<String>,
  age: Option<u8>,
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
  let customer1 = Customer::new1("AliceNouman".to_string());
  let customer2 = Customer::new2("Joseph".to_string(), "joe123".to_string());
  let customer3 = Customer::new3(
    "Micheal".to_string(),
    "micheal2000".to_string(),
    Membershiptype::Loyal,
  );

  println!("{:?}", customer1);
  println!("{:?}", customer2);
  println!("{:?}", customer3);
}
