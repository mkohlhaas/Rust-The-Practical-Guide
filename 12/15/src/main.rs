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
      name,
      ..Default::default()
    }
  }

  fn new2(name: String, username: String) -> Self {
    Customer {
      name,
      username,
      ..Default::default()
    }
  }

  fn new3(name: String, username: String, membership: Membershiptype) -> Self {
    Customer {
      name,
      username,
      membership,
      ..Default::default()
    }
  }
}

#[derive(Default, Debug)]
struct CustomerBuilder {
  name: String,
  username: Option<String>,
  membership: Option<Membershiptype>,
  gender: Option<char>,
  country: Option<String>,
  age: Option<u8>,
}

impl CustomerBuilder {
  fn username(&mut self, username: String) -> &mut Self {
    self.username = Some(username);
    self
  }

  fn membership(&mut self, membership: Membershiptype) -> &mut Self {
    self.membership = Some(membership);
    self
  }

  fn gender(&mut self, gender: char) -> &mut Self {
    self.gender = Some(gender);
    self
  }
  fn country(&mut self, country: String) -> &mut Self {
    self.country = Some(country);
    self
  }
  fn age(&mut self, age: u8) -> &mut Self {
    self.age = Some(age);
    self
  }

  // build method (use default value for None's)
  // use .take(…) or .clone(…)
  fn build(&mut self) -> Customer {
    println!("{:?}", self);
    let new_customer = Customer {
      name: self.name.clone(),
      username: self.username.take().unwrap_or_default(),
      membership: self.membership.take().unwrap_or_default(),
      gender: self.gender.unwrap_or_default(),
      country: self.country.take().unwrap_or_default(),
      age: self.age.unwrap_or_default(),
    };
    println!("{:?}", self);
    new_customer
  }
}

#[derive(Debug, Clone, Default)]
enum Membershiptype {
  #[default]
  New,
  Causual,
  Loyal,
}

// impl Default for Membershiptype {
//   fn default() -> Self {
//     Membershiptype::New
//   }
// }

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
